use quick_xml::{
    Reader,
    events::{BytesStart, Event, attributes::Attribute},
};

use crate::{
    Enum, Enums, EnumsContent, Member, MemberContent, Platform, Platforms, PlatformsContent,
    Registry, RegistryContent, Tag, Tags, TagsContent, Type, TypeContent, Types, TypesContent,
    Unused,
};

struct Parser<'a> {
    reader: Reader<&'a [u8]>,
}

#[derive(Debug)]
struct Elem<'a> {
    is_empty: bool,
    start: BytesStart<'a>,
}

impl Registry {
    pub fn vk() -> Self {
        Self::parse(include_str!("../data/vk.xml"))
    }

    pub fn parse(xml: &str) -> Self {
        let mut parser = Parser {
            reader: Reader::from_str(xml),
        };
        parser.parse_file()
    }
}

impl<'a> Parser<'a> {
    fn next_event<'b>(&mut self, buf: &'b mut Vec<u8>) -> Event<'b> {
        self.reader.read_event_into(buf).unwrap()
    }

    fn save_attr(&mut self, attr: Attribute, out: &mut Option<String>) {
        assert_eq!(*out, None);
        let decoder = self.reader.decoder();
        let value = attr.decode_and_unescape_value(decoder).unwrap();
        *out = Some(value.to_string());
    }

    fn assert_is_ws(&mut self, text: &[u8]) {
        for &b in text {
            assert!(matches!(b, b'\n' | b'\r' | b' '));
        }
    }

    fn parse_content<TextF, ElemF>(&mut self, elem: Elem, mut text_f: TextF, mut elem_f: ElemF)
    where
        TextF: FnMut(&mut Parser, &str),
        ElemF: FnMut(&mut Parser, Elem),
    {
        if elem.is_empty {
            return;
        }

        let mut buf = Vec::new();
        loop {
            match self.next_event(&mut buf) {
                Event::Text(text) => {
                    text_f(self, text.decode().unwrap().as_ref());
                }
                Event::GeneralRef(text) => {
                    text_f(self, text.decode().unwrap().as_ref());
                }
                Event::Empty(start) => {
                    let is_empty = true;
                    elem_f(self, Elem { is_empty, start });
                }
                Event::Start(start) => {
                    let is_empty = false;
                    elem_f(self, Elem { is_empty, start });
                }
                Event::End(end) => {
                    assert_eq!(end.name(), elem.start.name());
                    break;
                }
                event => {
                    panic!("unexpected event: {event:?}");
                }
            }
            buf.clear();
        }
    }

    fn parse_file(&mut self) -> Registry {
        let mut registry = None;

        let mut buf = Vec::new();
        loop {
            match self.next_event(&mut buf) {
                Event::Decl(_) => {
                    ();
                }
                Event::Text(text) => {
                    self.assert_is_ws(text.as_ref());
                }
                Event::Start(start) => match start.name().as_ref() {
                    b"registry" => {
                        let is_empty = false;
                        assert_eq!(registry, None);
                        registry = Some(self.parse_registry(Elem { is_empty, start }));
                    }
                    _ => {
                        panic!("unexpected elem: {start:?}");
                    }
                },
                event => {
                    panic!("unexpected event: {event:?}");
                }
            }
            buf.clear();
        }
    }

    fn parse_registry(&mut self, elem: Elem) -> Registry {
        for attr in elem.start.attributes() {
            let attr = attr.unwrap();
            match attr.key.as_ref() {
                _ => panic!("unexpected attr: {attr:?}"),
            }
        }

        let mut contents = Vec::new();
        self.parse_content(
            elem,
            |this, text| this.assert_is_ws(text.as_bytes()),
            |this, elem| match elem.start.name().as_ref() {
                b"comment" => {
                    let text = this.parse_text_elem(elem);
                    contents.push(RegistryContent::Comment(text));
                }
                b"platforms" => {
                    let platforms = this.parse_platforms(elem);
                    contents.push(RegistryContent::Platforms(platforms));
                }
                b"tags" => {
                    let tags = this.parse_tags(elem);
                    contents.push(RegistryContent::Tags(tags));
                }
                b"types" => {
                    let types = this.parse_types(elem);
                    contents.push(RegistryContent::Types(types));
                }
                b"enums" => {
                    let enums = this.parse_enums(elem);
                    contents.push(RegistryContent::Enums(enums));
                }
                _ => {
                    panic!("unexpected elem: {elem:?}");
                }
            },
        );

        Registry { contents }
    }

    fn parse_text_elem(&mut self, elem: Elem) -> String {
        for attr in elem.start.attributes() {
            let attr = attr.unwrap();
            match attr.key.as_ref() {
                _ => panic!("unexpected attr: {attr:?}"),
            }
        }

        let mut ret = String::new();
        self.parse_content(
            elem,
            |_this, text| ret += text,
            |_this, elem| panic!("unexpected elem: {elem:?}"),
        );
        ret
    }

    fn parse_platforms(&mut self, elem: Elem) -> Platforms {
        let mut comment = None;
        for attr in elem.start.attributes() {
            let attr = attr.unwrap();
            match attr.key.as_ref() {
                b"comment" => self.save_attr(attr, &mut comment),
                _ => panic!("unexpected attr: {attr:?}"),
            }
        }

        let mut contents = Vec::new();
        self.parse_content(
            elem,
            |this, text| this.assert_is_ws(text.as_bytes()),
            |this, elem| match elem.start.name().as_ref() {
                b"platform" => {
                    let platform = this.parse_platform(elem);
                    contents.push(PlatformsContent::Platform(platform));
                }
                _ => {
                    panic!("unexpected elem: {elem:?}");
                }
            },
        );

        Platforms {
            comment: comment.unwrap(),
            contents,
        }
    }

    fn parse_platform(&mut self, elem: Elem) -> Platform {
        let mut comment = None;
        let mut name = None;
        let mut protect = None;

        for attr in elem.start.attributes() {
            let attr = attr.unwrap();
            match attr.key.as_ref() {
                b"comment" => self.save_attr(attr, &mut comment),
                b"name" => self.save_attr(attr, &mut name),
                b"protect" => self.save_attr(attr, &mut protect),
                _ => panic!("unexpected attr: {attr:?}"),
            }
        }

        assert_eq!(elem.is_empty, true);
        Platform {
            comment: comment.unwrap(),
            name: name.unwrap(),
            protect: protect.unwrap(),
        }
    }

    fn parse_tags(&mut self, elem: Elem) -> Tags {
        let mut comment = None;
        for attr in elem.start.attributes() {
            let attr = attr.unwrap();
            match attr.key.as_ref() {
                b"comment" => self.save_attr(attr, &mut comment),
                _ => panic!("unexpected attr: {attr:?}"),
            }
        }

        let mut contents = Vec::new();
        self.parse_content(
            elem,
            |this, text| this.assert_is_ws(text.as_bytes()),
            |this, elem| match elem.start.name().as_ref() {
                b"tag" => {
                    let tag = this.parse_tag(elem);
                    contents.push(TagsContent::Tag(tag));
                }
                _ => {
                    panic!("unexpected elem: {elem:?}");
                }
            },
        );

        Tags {
            comment: comment.unwrap(),
            contents,
        }
    }

    fn parse_tag(&mut self, elem: Elem) -> Tag {
        let mut author = None;
        let mut contact = None;
        let mut name = None;

        for attr in elem.start.attributes() {
            let attr = attr.unwrap();
            match attr.key.as_ref() {
                b"author" => self.save_attr(attr, &mut author),
                b"contact" => self.save_attr(attr, &mut contact),
                b"name" => self.save_attr(attr, &mut name),
                _ => panic!("unexpected attr: {attr:?}"),
            }
        }

        assert_eq!(elem.is_empty, true);
        Tag {
            author: author.unwrap(),
            contact: contact.unwrap(),
            name: name.unwrap(),
        }
    }

    fn parse_types(&mut self, elem: Elem) -> Types {
        let mut comment = None;
        for attr in elem.start.attributes() {
            let attr = attr.unwrap();
            match attr.key.as_ref() {
                b"comment" => self.save_attr(attr, &mut comment),
                _ => panic!("unexpected attr: {attr:?}"),
            }
        }

        let mut contents = Vec::new();
        self.parse_content(
            elem,
            |this, text| this.assert_is_ws(text.as_bytes()),
            |this, elem| match elem.start.name().as_ref() {
                b"comment" => {
                    let comment = this.parse_text_elem(elem);
                    contents.push(TypesContent::Comment(comment));
                }
                b"type" => {
                    let type_name = this.parse_type(elem);
                    contents.push(TypesContent::Type(type_name));
                }
                _ => {
                    panic!("unexpected elem: {elem:?}");
                }
            },
        );

        Types {
            comment: comment.unwrap(),
            contents,
        }
    }

    fn parse_type(&mut self, elem: Elem) -> Type {
        let mut alias = None;
        let mut allowduplicate = None;
        let mut api = None;
        let mut bitvalues = None;
        let mut category = None;
        let mut comment = None;
        let mut name = None;
        let mut objtypeenum = None;
        let mut parent = None;
        let mut requiredlimittype = None;
        let mut requires = None;
        let mut returnedonly = None;
        let mut structextends = None;

        for attr in elem.start.attributes() {
            let attr = attr.unwrap();
            match attr.key.as_ref() {
                b"alias" => self.save_attr(attr, &mut alias),
                b"allowduplicate" => self.save_attr(attr, &mut allowduplicate),
                b"api" => self.save_attr(attr, &mut api),
                b"bitvalues" => self.save_attr(attr, &mut bitvalues),
                b"category" => self.save_attr(attr, &mut category),
                b"comment" => self.save_attr(attr, &mut comment),
                b"name" => self.save_attr(attr, &mut name),
                b"objtypeenum" => self.save_attr(attr, &mut objtypeenum),
                b"parent" => self.save_attr(attr, &mut parent),
                b"requiredlimittype" => self.save_attr(attr, &mut requiredlimittype),
                b"requires" => self.save_attr(attr, &mut requires),
                b"returnedonly" => self.save_attr(attr, &mut returnedonly),
                b"structextends" => self.save_attr(attr, &mut structextends),
                _ => panic!("unexpected attr: {attr:?}"),
            }
        }

        let mut contents = Vec::new();
        if !elem.is_empty {
            let mut buf = Vec::new();
            loop {
                match self.next_event(&mut buf) {
                    Event::Text(text) => {
                        let text = text.decode().unwrap().to_string();
                        contents.push(TypeContent::Text(text));
                    }
                    Event::GeneralRef(text) => {
                        let text = text.decode().unwrap().to_string();
                        contents.push(TypeContent::Text(text));
                    }
                    Event::Start(start) => {
                        let is_empty = false;
                        let elem = Elem { is_empty, start };
                        match elem.start.name().as_ref() {
                            b"comment" => {
                                let comment = self.parse_text_elem(elem);
                                contents.push(TypeContent::Comment(comment));
                            }
                            b"type" => {
                                let type_name = self.parse_text_elem(elem);
                                contents.push(TypeContent::Type(type_name));
                            }
                            b"name" => {
                                assert_eq!(name, None);
                                let name = self.parse_text_elem(elem);
                                contents.push(TypeContent::Name(name));
                            }
                            b"member" => {
                                let member = self.parse_member(elem);
                                contents.push(TypeContent::Member(member));
                            }
                            _ => {
                                panic!("unexpected element: {elem:?}");
                            }
                        }
                    }
                    Event::End(end) => {
                        assert_eq!(end.name(), elem.start.name());
                        break;
                    }
                    event => {
                        panic!("unexpected event: {event:?}");
                    }
                }
                buf.clear();
            }
        }

        Type {
            alias,
            allowduplicate,
            api,
            bitvalues,
            category,
            comment,
            name,
            objtypeenum,
            parent,
            requiredlimittype,
            requires,
            returnedonly,
            structextends,
            contents,
        }
    }

    fn parse_member(&mut self, elem: Elem) -> Member {
        let mut altlen = None;
        let mut api = None;
        let mut deprecated = None;
        let mut externsync = None;
        let mut featurelink = None;
        let mut len = None;
        let mut limittype = None;
        let mut noautovalidity = None;
        let mut objecttype = None;
        let mut optional = None;
        let mut selection = None;
        let mut selector = None;
        let mut values = None;

        for attr in elem.start.attributes() {
            let attr = attr.unwrap();
            match attr.key.as_ref() {
                b"altlen" => self.save_attr(attr, &mut altlen),
                b"api" => self.save_attr(attr, &mut api),
                b"deprecated" => self.save_attr(attr, &mut deprecated),
                b"externsync" => self.save_attr(attr, &mut externsync),
                b"featurelink" => self.save_attr(attr, &mut featurelink),
                b"len" => self.save_attr(attr, &mut len),
                b"limittype" => self.save_attr(attr, &mut limittype),
                b"noautovalidity" => self.save_attr(attr, &mut noautovalidity),
                b"objecttype" => self.save_attr(attr, &mut objecttype),
                b"optional" => self.save_attr(attr, &mut optional),
                b"selection" => self.save_attr(attr, &mut selection),
                b"selector" => self.save_attr(attr, &mut selector),
                b"values" => self.save_attr(attr, &mut values),
                _ => panic!("unexpected attr: {attr:?}"),
            }
        }

        let mut contents = Vec::new();
        if !elem.is_empty {
            let mut buf = Vec::new();
            loop {
                match self.next_event(&mut buf) {
                    Event::Text(text) => {
                        let text = text.decode().unwrap().to_string();
                        contents.push(MemberContent::Text(text));
                    }
                    Event::Start(start) => {
                        let is_empty = false;
                        let elem = Elem { is_empty, start };
                        match elem.start.name().as_ref() {
                            b"comment" => {
                                let comment = self.parse_text_elem(elem);
                                contents.push(MemberContent::Comment(comment));
                            }
                            b"type" => {
                                let type_name = self.parse_text_elem(elem);
                                contents.push(MemberContent::Type(type_name));
                            }
                            b"name" => {
                                let name = self.parse_text_elem(elem);
                                contents.push(MemberContent::Name(name));
                            }
                            b"enum" => {
                                let enum_name = self.parse_text_elem(elem);
                                contents.push(MemberContent::Enum(enum_name));
                            }
                            _ => {
                                panic!("unexpected element: {elem:?}");
                            }
                        }
                    }
                    Event::End(end) => {
                        assert_eq!(end.name(), elem.start.name());
                        break;
                    }
                    event => {
                        panic!("unexpected event: {event:?}");
                    }
                }
                buf.clear();
            }
        }

        Member {
            altlen,
            api,
            deprecated,
            externsync,
            featurelink,
            len,
            limittype,
            noautovalidity,
            objecttype,
            optional,
            selection,
            selector,
            values,
            contents,
        }
    }

    fn parse_enums(&mut self, elem: Elem) -> Enums {
        let mut bitwidth = None;
        let mut comment = None;
        let mut name = None;
        let mut ty = None;

        for attr in elem.start.attributes() {
            let attr = attr.unwrap();
            match attr.key.as_ref() {
                b"bitwidth" => self.save_attr(attr, &mut bitwidth),
                b"comment" => self.save_attr(attr, &mut comment),
                b"name" => self.save_attr(attr, &mut name),
                b"type" => self.save_attr(attr, &mut ty),
                _ => panic!("unexpected attr: {attr:?}"),
            }
        }

        let mut contents = Vec::new();
        self.parse_content(
            elem,
            |this, text| this.assert_is_ws(text.as_bytes()),
            |this, elem| match elem.start.name().as_ref() {
                b"comment" => {
                    let comment = this.parse_text_elem(elem);
                    contents.push(EnumsContent::Comment(comment));
                }
                b"enum" => {
                    let new_enum = this.parse_enum(elem);
                    contents.push(EnumsContent::Enum(new_enum));
                }
                b"unused" => {
                    let unused = this.parse_unused(elem);
                    contents.push(EnumsContent::Unused(unused));
                }
                _ => {
                    panic!("unexpected elem: {elem:?}");
                }
            },
        );

        Enums {
            bitwidth,
            comment,
            name: name.unwrap(),
            ty: ty.unwrap(),
            contents,
        }
    }

    fn parse_enum(&mut self, elem: Elem) -> Enum {
        let mut alias = None;
        let mut api = None;
        let mut bitpos = None;
        let mut comment = None;
        let mut deprecated = None;
        let mut name = None;
        let mut ty = None;
        let mut value = None;

        for attr in elem.start.attributes() {
            let attr = attr.unwrap();
            match attr.key.as_ref() {
                b"alias" => self.save_attr(attr, &mut alias),
                b"api" => self.save_attr(attr, &mut api),
                b"bitpos" => self.save_attr(attr, &mut bitpos),
                b"comment" => self.save_attr(attr, &mut comment),
                b"deprecated" => self.save_attr(attr, &mut deprecated),
                b"name" => self.save_attr(attr, &mut name),
                b"type" => self.save_attr(attr, &mut ty),
                b"value" => self.save_attr(attr, &mut value),
                _ => panic!("unexpected attr: {attr:?}"),
            }
        }

        assert_eq!(elem.is_empty, true);
        Enum {
            alias,
            api,
            bitpos,
            comment,
            deprecated,
            name: name.unwrap(),
            ty,
            value,
        }
    }

    fn parse_unused(&mut self, elem: Elem) -> Unused {
        let mut comment = None;
        let mut start = None;

        for attr in elem.start.attributes() {
            let attr = attr.unwrap();
            match attr.key.as_ref() {
                b"comment" => self.save_attr(attr, &mut comment),
                b"start" => self.save_attr(attr, &mut start),
                _ => panic!("unexpected attr: {attr:?}"),
            }
        }

        assert_eq!(elem.is_empty, true);
        Unused {
            comment: comment.unwrap(),
            start: start.unwrap(),
        }
    }
}
