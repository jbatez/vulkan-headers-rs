use quick_xml::{
    Reader,
    events::{BytesStart, BytesText, Event, attributes::Attribute},
};

use crate::{
    Platform, Platforms, PlatformsContent, Registry, RegistryContent, Tag, Tags, TagsContent, Type,
    TypeContent, Types, TypesContent,
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

    fn assert_is_ws(&mut self, text: BytesText) {
        for &b in text.as_ref() {
            assert!(matches!(b, b'\n' | b'\r' | b' '));
        }
    }

    fn parse_content<TextF, ElemF>(&mut self, elem: Elem, mut text_f: TextF, mut elem_f: ElemF)
    where
        TextF: FnMut(&mut Parser, BytesText),
        ElemF: FnMut(&mut Parser, Elem),
    {
        if elem.is_empty {
            return;
        }

        let mut buf = Vec::new();
        loop {
            match self.next_event(&mut buf) {
                Event::Text(text) => {
                    text_f(self, text);
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
                    self.assert_is_ws(text);
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
            |this, text| this.assert_is_ws(text),
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
            |_this, text| ret += text.decode().unwrap().as_ref(),
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
            |this, text| this.assert_is_ws(text),
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
            |this, text| this.assert_is_ws(text),
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
            |this, text| this.assert_is_ws(text),
            |this, elem| match elem.start.name().as_ref() {
                b"comment" => {
                    let comment = this.parse_text_elem(elem);
                    contents.push(TypesContent::Comment(comment));
                }
                b"type" => {
                    let ty = this.parse_type(elem);
                    contents.push(TypesContent::Type(ty));
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
        let mut api = None;
        let mut bitvalues = None;
        let mut category = None;
        let mut comment = None;
        let mut name = None;
        let mut objtypeenum = None;
        let mut parent = None;
        let mut requires = None;

        for attr in elem.start.attributes() {
            let attr = attr.unwrap();
            match attr.key.as_ref() {
                b"alias" => self.save_attr(attr, &mut alias),
                b"api" => self.save_attr(attr, &mut api),
                b"bitvalues" => self.save_attr(attr, &mut bitvalues),
                b"category" => self.save_attr(attr, &mut category),
                b"comment" => self.save_attr(attr, &mut comment),
                b"name" => self.save_attr(attr, &mut name),
                b"objtypeenum" => self.save_attr(attr, &mut objtypeenum),
                b"parent" => self.save_attr(attr, &mut parent),
                b"requires" => self.save_attr(attr, &mut requires),
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
                    Event::Start(start) => match start.name().as_ref() {
                        b"name" => {
                            assert_eq!(name, None);
                            let is_empty = false;
                            let name = self.parse_text_elem(Elem { is_empty, start });
                            contents.push(TypeContent::Name(name));
                        }
                        b"type" => {
                            let is_empty = false;
                            let ty = self.parse_text_elem(Elem { is_empty, start });
                            contents.push(TypeContent::Type(ty));
                        }
                        _ => {
                            panic!("unexpected element: {start:?}");
                        }
                    },
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
            api,
            bitvalues,
            category,
            comment,
            name,
            objtypeenum,
            parent,
            requires,
            contents,
        }
    }
}
