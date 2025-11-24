use quick_xml::{
    Reader,
    events::{BytesStart, Event, attributes::Attribute},
};

use crate::{
    Command, CommandContent, Commands, CommandsContent, Deprecate, DeprecateContent, Enum, Enums,
    EnumsContent, Extension, ExtensionContent, Extensions, ExtensionsContent, Feature,
    FeatureContent, FeatureRef, GeneralRef, ImplicitExternSyncParams,
    ImplicitExternSyncParamsContent, Member, MemberContent, Param, ParamContent, Platform,
    Platforms, PlatformsContent, Proto, ProtoContent, Registry, RegistryContent, Remove,
    RemoveContent, Require, RequireContent, RequireEnum, Tag, Tags, TagsContent, Type, TypeContent,
    Types, TypesContent, Unused,
};

struct Parser<'a> {
    reader: Reader<&'a [u8]>,
}

enum Content<'a> {
    Text(&'a str),
    Elem(Elem<'a>),
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

    fn parse_contents<F>(&mut self, elem: Elem, mut f: F)
    where
        F: FnMut(&mut Parser, Content),
    {
        if elem.is_empty {
            return;
        }

        let mut buf = Vec::new();
        loop {
            match self.next_event(&mut buf) {
                Event::Comment(_) => {
                    ();
                }
                Event::Text(text) => {
                    f(self, Content::Text(text.decode().unwrap().as_ref()));
                }
                Event::GeneralRef(text) => {
                    f(self, Content::Text(text.decode().unwrap().as_ref()));
                }
                Event::Empty(start) => {
                    let is_empty = true;
                    f(self, Content::Elem(Elem { is_empty, start }));
                }
                Event::Start(start) => {
                    let is_empty = false;
                    f(self, Content::Elem(Elem { is_empty, start }));
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
        self.parse_contents(elem, |this, content| match content {
            Content::Text(text) => this.assert_is_ws(text.as_bytes()),
            Content::Elem(elem) => match elem.start.name().as_ref() {
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
                b"commands" => {
                    let commands = this.parse_commands(elem);
                    contents.push(RegistryContent::Commands(commands));
                }
                b"feature" => {
                    let feature = this.parse_feature(elem);
                    contents.push(RegistryContent::Feature(feature));
                }
                b"extensions" => {
                    let extensions = this.parse_extensions(elem);
                    contents.push(RegistryContent::Extensions(extensions));
                }
                _ => {
                    panic!("unexpected elem: {elem:?}");
                }
            },
        });

        Registry { contents }
    }

    fn parse_text_elem(&mut self, elem: Elem) -> String {
        for attr in elem.start.attributes() {
            let attr = attr.unwrap();
            match attr.key.as_ref() {
                _ => panic!("unexpected attr: {attr:?}"),
            }
        }

        let mut contents = String::new();
        self.parse_contents(elem, |_this, content| match content {
            Content::Text(text) => contents += text,
            Content::Elem(elem) => panic!("unexpected elem: {elem:?}"),
        });

        contents
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
        self.parse_contents(elem, |this, content| match content {
            Content::Text(text) => this.assert_is_ws(text.as_bytes()),
            Content::Elem(elem) => match elem.start.name().as_ref() {
                b"platform" => {
                    let platform = this.parse_platform(elem);
                    contents.push(PlatformsContent::Platform(platform));
                }
                _ => {
                    panic!("unexpected elem: {elem:?}");
                }
            },
        });

        Platforms { comment, contents }
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
            comment,
            name,
            protect,
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
        self.parse_contents(elem, |this, content| match content {
            Content::Text(text) => this.assert_is_ws(text.as_bytes()),
            Content::Elem(elem) => match elem.start.name().as_ref() {
                b"tag" => {
                    let tag = this.parse_tag(elem);
                    contents.push(TagsContent::Tag(tag));
                }
                _ => {
                    panic!("unexpected elem: {elem:?}");
                }
            },
        });

        Tags { comment, contents }
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
            author,
            contact,
            name,
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
        self.parse_contents(elem, |this, content| match content {
            Content::Text(text) => this.assert_is_ws(text.as_bytes()),
            Content::Elem(elem) => match elem.start.name().as_ref() {
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
        });

        Types { comment, contents }
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
        self.parse_contents(elem, |this, content| match content {
            Content::Text(text) => {
                contents.push(TypeContent::Text(text.to_string()));
            }
            Content::Elem(elem) => match elem.start.name().as_ref() {
                b"comment" => {
                    let comment = this.parse_text_elem(elem);
                    contents.push(TypeContent::Comment(comment));
                }
                b"type" => {
                    let type_name = this.parse_text_elem(elem);
                    contents.push(TypeContent::Type(type_name));
                }
                b"name" => {
                    assert_eq!(name, None);
                    let name = this.parse_text_elem(elem);
                    contents.push(TypeContent::Name(name));
                }
                b"member" => {
                    let member = this.parse_member(elem);
                    contents.push(TypeContent::Member(member));
                }
                _ => {
                    panic!("unexpected element: {elem:?}");
                }
            },
        });

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
        self.parse_contents(elem, |this, content| match content {
            Content::Text(text) => {
                contents.push(MemberContent::Text(text.to_string()));
            }
            Content::Elem(elem) => match elem.start.name().as_ref() {
                b"comment" => {
                    let comment = this.parse_text_elem(elem);
                    contents.push(MemberContent::Comment(comment));
                }
                b"type" => {
                    let type_name = this.parse_text_elem(elem);
                    contents.push(MemberContent::Type(type_name));
                }
                b"name" => {
                    let name = this.parse_text_elem(elem);
                    contents.push(MemberContent::Name(name));
                }
                b"enum" => {
                    let enum_name = this.parse_text_elem(elem);
                    contents.push(MemberContent::Enum(enum_name));
                }
                _ => {
                    panic!("unexpected element: {elem:?}");
                }
            },
        });

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
        self.parse_contents(elem, |this, content| match content {
            Content::Text(text) => this.assert_is_ws(text.as_bytes()),
            Content::Elem(elem) => match elem.start.name().as_ref() {
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
        });

        Enums {
            bitwidth,
            comment,
            name,
            ty,
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
            name,
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
        Unused { comment, start }
    }

    fn parse_commands(&mut self, elem: Elem) -> Commands {
        let mut comment = None;

        for attr in elem.start.attributes() {
            let attr = attr.unwrap();
            match attr.key.as_ref() {
                b"comment" => self.save_attr(attr, &mut comment),
                _ => panic!("unexpected attr: {attr:?}"),
            }
        }

        let mut contents = Vec::new();
        self.parse_contents(elem, |this, content| match content {
            Content::Text(text) => this.assert_is_ws(text.as_bytes()),
            Content::Elem(elem) => match elem.start.name().as_ref() {
                b"command" => {
                    let command = this.parse_command(elem);
                    contents.push(CommandsContent::Command(command));
                }
                _ => {
                    panic!("unexpected elem: {elem:?}");
                }
            },
        });

        Commands { comment, contents }
    }

    fn parse_command(&mut self, elem: Elem) -> Command {
        let mut alias = None;
        let mut allownoqueues = None;
        let mut api = None;
        let mut cmdbufferlevel = None;
        let mut comment = None;
        let mut conditionalrendering = None;
        let mut errorcodes = None;
        let mut export = None;
        let mut name = None;
        let mut queues = None;
        let mut renderpass = None;
        let mut successcodes = None;
        let mut tasks = None;
        let mut videocoding = None;

        for attr in elem.start.attributes() {
            let attr = attr.unwrap();
            match attr.key.as_ref() {
                b"alias" => self.save_attr(attr, &mut alias),
                b"allownoqueues" => self.save_attr(attr, &mut allownoqueues),
                b"api" => self.save_attr(attr, &mut api),
                b"cmdbufferlevel" => self.save_attr(attr, &mut cmdbufferlevel),
                b"comment" => self.save_attr(attr, &mut comment),
                b"conditionalrendering" => self.save_attr(attr, &mut conditionalrendering),
                b"errorcodes" => self.save_attr(attr, &mut errorcodes),
                b"export" => self.save_attr(attr, &mut export),
                b"name" => self.save_attr(attr, &mut name),
                b"queues" => self.save_attr(attr, &mut queues),
                b"renderpass" => self.save_attr(attr, &mut renderpass),
                b"successcodes" => self.save_attr(attr, &mut successcodes),
                b"tasks" => self.save_attr(attr, &mut tasks),
                b"videocoding" => self.save_attr(attr, &mut videocoding),
                _ => panic!("unexpected attr: {attr:?}"),
            }
        }

        let mut contents = Vec::new();
        self.parse_contents(elem, |this, content| match content {
            Content::Text(text) => this.assert_is_ws(text.as_bytes()),
            Content::Elem(elem) => match elem.start.name().as_ref() {
                b"proto" => {
                    let proto = this.parse_proto(elem);
                    contents.push(CommandContent::Proto(proto));
                }
                b"param" => {
                    let param = this.parse_param(elem);
                    contents.push(CommandContent::Param(param));
                }
                b"implicitexternsyncparams" => {
                    let params = this.parse_implicitexternsyncparams(elem);
                    contents.push(CommandContent::ImplicitExternSyncParams(params));
                }
                _ => {
                    panic!("unexpected elem: {elem:?}");
                }
            },
        });

        Command {
            alias,
            allownoqueues,
            api,
            cmdbufferlevel,
            comment,
            conditionalrendering,
            errorcodes,
            export,
            name,
            queues,
            renderpass,
            successcodes,
            tasks,
            videocoding,
            contents,
        }
    }

    fn parse_proto(&mut self, elem: Elem) -> Proto {
        for attr in elem.start.attributes() {
            let attr = attr.unwrap();
            match attr.key.as_ref() {
                _ => panic!("unexpected attr: {attr:?}"),
            }
        }

        let mut contents = Vec::new();
        self.parse_contents(elem, |this, content| match content {
            Content::Text(text) => {
                contents.push(ProtoContent::Text(text.to_string()));
            }
            Content::Elem(elem) => match elem.start.name().as_ref() {
                b"type" => {
                    let type_name = this.parse_text_elem(elem);
                    contents.push(ProtoContent::Type(type_name));
                }
                b"name" => {
                    let name = this.parse_text_elem(elem);
                    contents.push(ProtoContent::Name(name));
                }
                _ => {
                    panic!("unexpected element: {elem:?}");
                }
            },
        });

        Proto { contents }
    }

    fn parse_param(&mut self, elem: Elem) -> Param {
        let mut altlen = None;
        let mut api = None;
        let mut externsync = None;
        let mut len = None;
        let mut noautovalidity = None;
        let mut objecttype = None;
        let mut optional = None;
        let mut stride = None;
        let mut validstructs = None;

        for attr in elem.start.attributes() {
            let attr = attr.unwrap();
            match attr.key.as_ref() {
                b"altlen" => self.save_attr(attr, &mut altlen),
                b"api" => self.save_attr(attr, &mut api),
                b"externsync" => self.save_attr(attr, &mut externsync),
                b"len" => self.save_attr(attr, &mut len),
                b"noautovalidity" => self.save_attr(attr, &mut noautovalidity),
                b"objecttype" => self.save_attr(attr, &mut objecttype),
                b"optional" => self.save_attr(attr, &mut optional),
                b"stride" => self.save_attr(attr, &mut stride),
                b"validstructs" => self.save_attr(attr, &mut validstructs),
                _ => panic!("unexpected attr: {attr:?}"),
            }
        }

        let mut contents = Vec::new();
        self.parse_contents(elem, |this, content| match content {
            Content::Text(text) => {
                contents.push(ParamContent::Text(text.to_string()));
            }
            Content::Elem(elem) => match elem.start.name().as_ref() {
                b"type" => {
                    let type_name = this.parse_text_elem(elem);
                    contents.push(ParamContent::Type(type_name));
                }
                b"name" => {
                    let name = this.parse_text_elem(elem);
                    contents.push(ParamContent::Name(name));
                }
                _ => {
                    panic!("unexpected element: {elem:?}");
                }
            },
        });

        Param {
            altlen,
            api,
            externsync,
            len,
            noautovalidity,
            objecttype,
            optional,
            stride,
            validstructs,
            contents,
        }
    }

    fn parse_implicitexternsyncparams(&mut self, elem: Elem) -> ImplicitExternSyncParams {
        for attr in elem.start.attributes() {
            let attr = attr.unwrap();
            match attr.key.as_ref() {
                _ => panic!("unexpected attr: {attr:?}"),
            }
        }

        let mut contents = Vec::new();
        self.parse_contents(elem, |this, content| match content {
            Content::Text(text) => this.assert_is_ws(text.as_bytes()),
            Content::Elem(elem) => match elem.start.name().as_ref() {
                b"param" => {
                    let param = this.parse_text_elem(elem);
                    contents.push(ImplicitExternSyncParamsContent::Param(param));
                }
                _ => {
                    panic!("unexpected elem: {elem:?}");
                }
            },
        });

        ImplicitExternSyncParams { contents }
    }

    fn parse_feature(&mut self, elem: Elem) -> Feature {
        let mut api = None;
        let mut apitype = None;
        let mut comment = None;
        let mut depends = None;
        let mut name = None;
        let mut number = None;

        for attr in elem.start.attributes() {
            let attr = attr.unwrap();
            match attr.key.as_ref() {
                b"api" => self.save_attr(attr, &mut api),
                b"apitype" => self.save_attr(attr, &mut apitype),
                b"comment" => self.save_attr(attr, &mut comment),
                b"depends" => self.save_attr(attr, &mut depends),
                b"name" => self.save_attr(attr, &mut name),
                b"number" => self.save_attr(attr, &mut number),
                _ => panic!("unexpected attr: {attr:?}"),
            }
        }

        let mut contents = Vec::new();
        self.parse_contents(elem, |this, content| match content {
            Content::Text(text) => this.assert_is_ws(text.as_bytes()),
            Content::Elem(elem) => match elem.start.name().as_ref() {
                b"require" => {
                    let require = this.parse_require(elem);
                    contents.push(FeatureContent::Require(require));
                }
                b"deprecate" => {
                    let deprecate = this.parse_deprecate(elem);
                    contents.push(FeatureContent::Deprecate(deprecate));
                }
                b"remove" => {
                    let remove = this.parse_remove(elem);
                    contents.push(FeatureContent::Remove(remove));
                }
                _ => {
                    panic!("unexpected element: {elem:?}");
                }
            },
        });

        Feature {
            api,
            apitype,
            comment,
            depends,
            name,
            number,
            contents,
        }
    }

    fn parse_extensions(&mut self, elem: Elem) -> Extensions {
        let mut comment = None;

        for attr in elem.start.attributes() {
            let attr = attr.unwrap();
            match attr.key.as_ref() {
                b"comment" => self.save_attr(attr, &mut comment),
                _ => panic!("unexpected attr: {attr:?}"),
            }
        }

        let mut contents = Vec::new();
        self.parse_contents(elem, |this, content| match content {
            Content::Text(text) => this.assert_is_ws(text.as_bytes()),
            Content::Elem(elem) => match elem.start.name().as_ref() {
                b"extension" => {
                    let extension = this.parse_extension(elem);
                    contents.push(ExtensionsContent::Extension(extension));
                }
                _ => {
                    panic!("unexpected elem: {elem:?}");
                }
            },
        });

        Extensions { comment, contents }
    }

    fn parse_extension(&mut self, elem: Elem) -> Extension {
        let mut author = None;
        let mut comment = None;
        let mut contact = None;
        let mut depends = None;
        let mut deprecatedby = None;
        let mut name = None;
        let mut nofeatures = None;
        let mut number = None;
        let mut obsoletedby = None;
        let mut platform = None;
        let mut promotedto = None;
        let mut provisional = None;
        let mut ratified = None;
        let mut sortorder = None;
        let mut specialuse = None;
        let mut supported = None;
        let mut ty = None;

        for attr in elem.start.attributes() {
            let attr = attr.unwrap();
            match attr.key.as_ref() {
                b"author" => self.save_attr(attr, &mut author),
                b"comment" => self.save_attr(attr, &mut comment),
                b"contact" => self.save_attr(attr, &mut contact),
                b"depends" => self.save_attr(attr, &mut depends),
                b"deprecatedby" => self.save_attr(attr, &mut deprecatedby),
                b"name" => self.save_attr(attr, &mut name),
                b"nofeatures" => self.save_attr(attr, &mut nofeatures),
                b"number" => self.save_attr(attr, &mut number),
                b"obsoletedby" => self.save_attr(attr, &mut obsoletedby),
                b"platform" => self.save_attr(attr, &mut platform),
                b"promotedto" => self.save_attr(attr, &mut promotedto),
                b"provisional" => self.save_attr(attr, &mut provisional),
                b"ratified" => self.save_attr(attr, &mut ratified),
                b"sortorder" => self.save_attr(attr, &mut sortorder),
                b"specialuse" => self.save_attr(attr, &mut specialuse),
                b"supported" => self.save_attr(attr, &mut supported),
                b"type" => self.save_attr(attr, &mut ty),
                _ => panic!("unexpected attr: {attr:?}"),
            }
        }

        let mut contents = Vec::new();
        self.parse_contents(elem, |this, content| match content {
            Content::Text(text) => this.assert_is_ws(text.as_bytes()),
            Content::Elem(elem) => match elem.start.name().as_ref() {
                b"require" => {
                    let require = this.parse_require(elem);
                    contents.push(ExtensionContent::Require(require));
                }
                b"deprecate" => {
                    let deprecate = this.parse_deprecate(elem);
                    contents.push(ExtensionContent::Deprecate(deprecate));
                }
                b"remove" => {
                    let remove = this.parse_remove(elem);
                    contents.push(ExtensionContent::Remove(remove));
                }
                _ => {
                    panic!("unexpected element: {elem:?}");
                }
            },
        });

        Extension {
            author,
            comment,
            contact,
            depends,
            deprecatedby,
            name,
            nofeatures,
            number,
            obsoletedby,
            platform,
            promotedto,
            provisional,
            ratified,
            sortorder,
            specialuse,
            supported,
            ty,
            contents,
        }
    }

    fn parse_require(&mut self, elem: Elem) -> Require {
        let mut api = None;
        let mut comment = None;
        let mut depends = None;

        for attr in elem.start.attributes() {
            let attr = attr.unwrap();
            match attr.key.as_ref() {
                b"api" => self.save_attr(attr, &mut api),
                b"comment" => self.save_attr(attr, &mut comment),
                b"depends" => self.save_attr(attr, &mut depends),
                _ => panic!("unexpected attr: {attr:?}"),
            }
        }

        let mut contents = Vec::new();
        self.parse_contents(elem, |this, content| match content {
            Content::Text(text) => this.assert_is_ws(text.as_bytes()),
            Content::Elem(elem) => match elem.start.name().as_ref() {
                b"comment" => {
                    let comment = this.parse_text_elem(elem);
                    contents.push(RequireContent::Comment(comment));
                }
                b"type" => {
                    let type_ref = this.parse_general_ref(elem);
                    contents.push(RequireContent::Type(type_ref));
                }
                b"enum" => {
                    let enum_ref = this.parse_require_enum(elem);
                    contents.push(RequireContent::Enum(enum_ref));
                }
                b"command" => {
                    let command_ref = this.parse_general_ref(elem);
                    contents.push(RequireContent::Command(command_ref));
                }
                b"feature" => {
                    let feature_ref = this.parse_feature_ref(elem);
                    contents.push(RequireContent::Feature(feature_ref));
                }
                _ => {
                    panic!("unexpected element: {elem:?}");
                }
            },
        });

        Require {
            api,
            comment,
            depends,
            contents,
        }
    }

    fn parse_require_enum(&mut self, elem: Elem) -> RequireEnum {
        let mut alias = None;
        let mut api = None;
        let mut bitpos = None;
        let mut comment = None;
        let mut deprecated = None;
        let mut dir = None;
        let mut extends = None;
        let mut extnumber = None;
        let mut name = None;
        let mut offset = None;
        let mut protect = None;
        let mut value = None;

        for attr in elem.start.attributes() {
            let attr = attr.unwrap();
            match attr.key.as_ref() {
                b"alias" => self.save_attr(attr, &mut alias),
                b"api" => self.save_attr(attr, &mut api),
                b"bitpos" => self.save_attr(attr, &mut bitpos),
                b"comment" => self.save_attr(attr, &mut comment),
                b"deprecated" => self.save_attr(attr, &mut deprecated),
                b"dir" => self.save_attr(attr, &mut dir),
                b"extends" => self.save_attr(attr, &mut extends),
                b"extnumber" => self.save_attr(attr, &mut extnumber),
                b"name" => self.save_attr(attr, &mut name),
                b"offset" => self.save_attr(attr, &mut offset),
                b"protect" => self.save_attr(attr, &mut protect),
                b"value" => self.save_attr(attr, &mut value),
                _ => panic!("unexpected attr: {attr:?}"),
            }
        }

        assert_eq!(elem.is_empty, true);
        RequireEnum {
            alias,
            api,
            bitpos,
            comment,
            deprecated,
            dir,
            extends,
            extnumber,
            name,
            offset,
            protect,
            value,
        }
    }

    fn parse_deprecate(&mut self, elem: Elem) -> Deprecate {
        let mut explanationlink = None;

        for attr in elem.start.attributes() {
            let attr = attr.unwrap();
            match attr.key.as_ref() {
                b"explanationlink" => self.save_attr(attr, &mut explanationlink),
                _ => panic!("unexpected attr: {attr:?}"),
            }
        }

        let mut contents = Vec::new();
        self.parse_contents(elem, |this, content| match content {
            Content::Text(text) => this.assert_is_ws(text.as_bytes()),
            Content::Elem(elem) => match elem.start.name().as_ref() {
                b"type" => {
                    let type_ref = this.parse_general_ref(elem);
                    contents.push(DeprecateContent::Type(type_ref));
                }
                b"command" => {
                    let command_ref = this.parse_general_ref(elem);
                    contents.push(DeprecateContent::Command(command_ref));
                }
                _ => {
                    panic!("unexpected element: {elem:?}");
                }
            },
        });

        Deprecate {
            explanationlink,
            contents,
        }
    }

    fn parse_remove(&mut self, elem: Elem) -> Remove {
        let mut comment = None;
        let mut reasonlink = None;

        for attr in elem.start.attributes() {
            let attr = attr.unwrap();
            match attr.key.as_ref() {
                b"comment" => self.save_attr(attr, &mut comment),
                b"reasonlink" => self.save_attr(attr, &mut reasonlink),
                _ => panic!("unexpected attr: {attr:?}"),
            }
        }

        let mut contents = Vec::new();
        self.parse_contents(elem, |this, content| match content {
            Content::Text(text) => this.assert_is_ws(text.as_bytes()),
            Content::Elem(elem) => match elem.start.name().as_ref() {
                b"type" => {
                    let type_ref = this.parse_general_ref(elem);
                    contents.push(RemoveContent::Type(type_ref));
                }
                b"enum" => {
                    let enum_ref = this.parse_general_ref(elem);
                    contents.push(RemoveContent::Enum(enum_ref));
                }
                b"command" => {
                    let command_ref = this.parse_general_ref(elem);
                    contents.push(RemoveContent::Command(command_ref));
                }
                b"feature" => {
                    let feature_ref = this.parse_feature_ref(elem);
                    contents.push(RemoveContent::Feature(feature_ref));
                }
                _ => {
                    panic!("unexpected element: {elem:?}");
                }
            },
        });

        Remove {
            comment,
            reasonlink,
            contents,
        }
    }

    fn parse_general_ref(&mut self, elem: Elem) -> GeneralRef {
        let mut comment = None;
        let mut name = None;

        for attr in elem.start.attributes() {
            let attr = attr.unwrap();
            match attr.key.as_ref() {
                b"comment" => self.save_attr(attr, &mut comment),
                b"name" => self.save_attr(attr, &mut name),
                _ => panic!("unexpected attr: {attr:?}"),
            }
        }

        assert_eq!(elem.is_empty, true);
        GeneralRef { comment, name }
    }

    fn parse_feature_ref(&mut self, elem: Elem) -> FeatureRef {
        let mut name = None;
        let mut feature_struct = None;

        for attr in elem.start.attributes() {
            let attr = attr.unwrap();
            match attr.key.as_ref() {
                b"name" => self.save_attr(attr, &mut name),
                b"struct" => self.save_attr(attr, &mut feature_struct),
                _ => panic!("unexpected attr: {attr:?}"),
            }
        }

        assert_eq!(elem.is_empty, true);
        FeatureRef {
            name,
            feature_struct,
        }
    }
}
