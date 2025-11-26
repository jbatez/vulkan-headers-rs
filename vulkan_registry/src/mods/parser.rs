use quick_xml::{
    Reader,
    escape::resolve_xml_entity,
    events::{BytesStart, Event, attributes::Attribute},
};

use crate::mods::*;

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
                    let text = text.xml_content().unwrap();
                    f(self, Content::Text(&text));
                }
                Event::GeneralRef(text) => {
                    let text = text.xml_content().unwrap();
                    let text = resolve_xml_entity(&text).unwrap();
                    f(self, Content::Text(text));
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
                    self.assert_is_ws(&text);
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
                Event::Eof => {
                    break;
                }
                event => {
                    panic!("unexpected event: {event:?}");
                }
            }
            buf.clear();
        }

        registry.unwrap()
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
                b"formats" => {
                    let formats = this.parse_formats(elem);
                    contents.push(RegistryContent::Formats(formats));
                }
                b"spirvextensions" => {
                    let extensions = this.parse_spirv_extensions(elem);
                    contents.push(RegistryContent::SpirvExtensions(extensions));
                }
                b"spirvcapabilities" => {
                    let capabilities = this.parse_spirv_capabilities(elem);
                    contents.push(RegistryContent::SpirvCapabilities(capabilities));
                }
                b"sync" => {
                    let syncs = this.parse_syncs(elem);
                    contents.push(RegistryContent::Syncs(syncs));
                }
                b"videocodecs" => {
                    let codecs = this.parse_video_codecs(elem);
                    contents.push(RegistryContent::VideoCodecs(codecs));
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
                    let typ = this.parse_type(elem);
                    contents.push(TypesContent::Type(typ));
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
                    let typ = this.parse_text_elem(elem);
                    contents.push(TypeContent::Type(typ));
                }
                b"name" => {
                    let name = this.parse_text_elem(elem);
                    contents.push(TypeContent::Name(name));
                }
                b"member" => {
                    let member = this.parse_member(elem);
                    contents.push(TypeContent::Member(member));
                }
                _ => {
                    panic!("unexpected elem: {elem:?}");
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
                    let typ = this.parse_text_elem(elem);
                    contents.push(MemberContent::Type(typ));
                }
                b"name" => {
                    let name = this.parse_text_elem(elem);
                    contents.push(MemberContent::Name(name));
                }
                b"enum" => {
                    let enu = this.parse_text_elem(elem);
                    contents.push(MemberContent::Enum(enu));
                }
                _ => {
                    panic!("unexpected elem: {elem:?}");
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
        let mut typ = None;

        for attr in elem.start.attributes() {
            let attr = attr.unwrap();
            match attr.key.as_ref() {
                b"bitwidth" => self.save_attr(attr, &mut bitwidth),
                b"comment" => self.save_attr(attr, &mut comment),
                b"name" => self.save_attr(attr, &mut name),
                b"type" => self.save_attr(attr, &mut typ),
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
                    let enu = this.parse_enum(elem);
                    contents.push(EnumsContent::Enum(enu));
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
            typ,
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
        let mut typ = None;
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
                b"type" => self.save_attr(attr, &mut typ),
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
            typ,
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
                    let params = this.parse_implicit_extern_sync_params(elem);
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
                    let typ = this.parse_text_elem(elem);
                    contents.push(ProtoContent::Type(typ));
                }
                b"name" => {
                    let name = this.parse_text_elem(elem);
                    contents.push(ProtoContent::Name(name));
                }
                _ => {
                    panic!("unexpected elem: {elem:?}");
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
                    let typ = this.parse_text_elem(elem);
                    contents.push(ParamContent::Type(typ));
                }
                b"name" => {
                    let name = this.parse_text_elem(elem);
                    contents.push(ParamContent::Name(name));
                }
                _ => {
                    panic!("unexpected elem: {elem:?}");
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

    fn parse_implicit_extern_sync_params(&mut self, elem: Elem) -> ImplicitExternSyncParams {
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
                    panic!("unexpected elem: {elem:?}");
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
        let mut typ = None;

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
                b"type" => self.save_attr(attr, &mut typ),
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
                    panic!("unexpected elem: {elem:?}");
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
            typ,
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
                    let typ = this.parse_general_ref(elem);
                    contents.push(RequireContent::Type(typ));
                }
                b"enum" => {
                    let enu = this.parse_require_enum(elem);
                    contents.push(RequireContent::Enum(enu));
                }
                b"command" => {
                    let command = this.parse_general_ref(elem);
                    contents.push(RequireContent::Command(command));
                }
                b"feature" => {
                    let feature = this.parse_feature_ref(elem);
                    contents.push(RequireContent::Feature(feature));
                }
                _ => {
                    panic!("unexpected elem: {elem:?}");
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
                    let typ = this.parse_general_ref(elem);
                    contents.push(DeprecateContent::Type(typ));
                }
                b"command" => {
                    let command = this.parse_general_ref(elem);
                    contents.push(DeprecateContent::Command(command));
                }
                _ => {
                    panic!("unexpected elem: {elem:?}");
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
                    let typ = this.parse_general_ref(elem);
                    contents.push(RemoveContent::Type(typ));
                }
                b"enum" => {
                    let enu = this.parse_general_ref(elem);
                    contents.push(RemoveContent::Enum(enu));
                }
                b"command" => {
                    let command = this.parse_general_ref(elem);
                    contents.push(RemoveContent::Command(command));
                }
                b"feature" => {
                    let feature = this.parse_feature_ref(elem);
                    contents.push(RemoveContent::Feature(feature));
                }
                _ => {
                    panic!("unexpected elem: {elem:?}");
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
        let mut struc = None;

        for attr in elem.start.attributes() {
            let attr = attr.unwrap();
            match attr.key.as_ref() {
                b"name" => self.save_attr(attr, &mut name),
                b"struct" => self.save_attr(attr, &mut struc),
                _ => panic!("unexpected attr: {attr:?}"),
            }
        }

        assert_eq!(elem.is_empty, true);
        FeatureRef { name, struc }
    }

    fn parse_formats(&mut self, elem: Elem) -> Formats {
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
                b"format" => {
                    let format = this.parse_format(elem);
                    contents.push(FormatsContent::Format(format));
                }
                _ => {
                    panic!("unexpected elem: {elem:?}");
                }
            },
        });

        Formats { contents }
    }

    fn parse_format(&mut self, elem: Elem) -> Format {
        let mut block_extent = None;
        let mut block_size = None;
        let mut chroma = None;
        let mut class = None;
        let mut compressed = None;
        let mut name = None;
        let mut packed = None;
        let mut texels_per_block = None;

        for attr in elem.start.attributes() {
            let attr = attr.unwrap();
            match attr.key.as_ref() {
                b"blockExtent" => self.save_attr(attr, &mut block_extent),
                b"blockSize" => self.save_attr(attr, &mut block_size),
                b"chroma" => self.save_attr(attr, &mut chroma),
                b"class" => self.save_attr(attr, &mut class),
                b"compressed" => self.save_attr(attr, &mut compressed),
                b"name" => self.save_attr(attr, &mut name),
                b"packed" => self.save_attr(attr, &mut packed),
                b"texelsPerBlock" => self.save_attr(attr, &mut texels_per_block),
                _ => panic!("unexpected attr: {attr:?}"),
            }
        }

        let mut contents = Vec::new();
        self.parse_contents(elem, |this, content| match content {
            Content::Text(text) => this.assert_is_ws(text.as_bytes()),
            Content::Elem(elem) => match elem.start.name().as_ref() {
                b"component" => {
                    let component = this.parse_component(elem);
                    contents.push(FormatContent::Component(component));
                }
                b"plane" => {
                    let plane = this.parse_plane(elem);
                    contents.push(FormatContent::Plane(plane));
                }
                b"spirvimageformat" => {
                    let format = this.parse_spirv_image_format(elem);
                    contents.push(FormatContent::SpirvImageFormat(format));
                }
                _ => {
                    panic!("unexpected elem: {elem:?}");
                }
            },
        });

        Format {
            block_extent,
            block_size,
            chroma,
            class,
            compressed,
            name,
            packed,
            texels_per_block,
            contents,
        }
    }

    fn parse_component(&mut self, elem: Elem) -> Component {
        let mut bits = None;
        let mut name = None;
        let mut numeric_format = None;
        let mut plane_index = None;

        for attr in elem.start.attributes() {
            let attr = attr.unwrap();
            match attr.key.as_ref() {
                b"bits" => self.save_attr(attr, &mut bits),
                b"name" => self.save_attr(attr, &mut name),
                b"numericFormat" => self.save_attr(attr, &mut numeric_format),
                b"planeIndex" => self.save_attr(attr, &mut plane_index),
                _ => panic!("unexpected attr: {attr:?}"),
            }
        }

        assert_eq!(elem.is_empty, true);
        Component {
            bits,
            name,
            numeric_format,
            plane_index,
        }
    }

    fn parse_plane(&mut self, elem: Elem) -> Plane {
        let mut compatible = None;
        let mut height_divisor = None;
        let mut index = None;
        let mut width_divisor = None;

        for attr in elem.start.attributes() {
            let attr = attr.unwrap();
            match attr.key.as_ref() {
                b"compatible" => self.save_attr(attr, &mut compatible),
                b"heightDivisor" => self.save_attr(attr, &mut height_divisor),
                b"index" => self.save_attr(attr, &mut index),
                b"widthDivisor" => self.save_attr(attr, &mut width_divisor),
                _ => panic!("unexpected attr: {attr:?}"),
            }
        }

        assert_eq!(elem.is_empty, true);
        Plane {
            compatible,
            height_divisor,
            index,
            width_divisor,
        }
    }

    fn parse_spirv_image_format(&mut self, elem: Elem) -> SpirvImageFormat {
        let mut name = None;

        for attr in elem.start.attributes() {
            let attr = attr.unwrap();
            match attr.key.as_ref() {
                b"name" => self.save_attr(attr, &mut name),
                _ => panic!("unexpected attr: {attr:?}"),
            }
        }

        assert_eq!(elem.is_empty, true);
        SpirvImageFormat { name }
    }

    fn parse_spirv_extensions(&mut self, elem: Elem) -> SpirvExtensions {
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
                b"spirvextension" => {
                    let extension = this.parse_spirv_extension(elem);
                    contents.push(SpirvExtensionsContent::SpirvExtension(extension));
                }
                _ => {
                    panic!("unexpected elem: {elem:?}");
                }
            },
        });

        SpirvExtensions { comment, contents }
    }

    fn parse_spirv_extension(&mut self, elem: Elem) -> SpirvExtension {
        let mut name = None;

        for attr in elem.start.attributes() {
            let attr = attr.unwrap();
            match attr.key.as_ref() {
                b"name" => self.save_attr(attr, &mut name),
                _ => panic!("unexpected attr: {attr:?}"),
            }
        }

        let mut contents = Vec::new();
        self.parse_contents(elem, |this, content| match content {
            Content::Text(text) => this.assert_is_ws(text.as_bytes()),
            Content::Elem(elem) => match elem.start.name().as_ref() {
                b"enable" => {
                    let enable = this.parse_spirv_extension_enable(elem);
                    contents.push(SpirvExtensionContent::Enable(enable));
                }
                _ => {
                    panic!("unexpected elem: {elem:?}");
                }
            },
        });

        SpirvExtension { name, contents }
    }

    fn parse_spirv_extension_enable(&mut self, elem: Elem) -> SpirvExtensionEnable {
        let mut extension = None;
        let mut version = None;

        for attr in elem.start.attributes() {
            let attr = attr.unwrap();
            match attr.key.as_ref() {
                b"extension" => self.save_attr(attr, &mut extension),
                b"version" => self.save_attr(attr, &mut version),
                _ => panic!("unexpected attr: {attr:?}"),
            }
        }

        assert_eq!(elem.is_empty, true);
        SpirvExtensionEnable { extension, version }
    }

    fn parse_spirv_capabilities(&mut self, elem: Elem) -> SpirvCapabilities {
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
                b"spirvcapability" => {
                    let capability = this.parse_spirv_capability(elem);
                    contents.push(SpirvCapabilitiesContent::SpirvCapability(capability));
                }
                _ => {
                    panic!("unexpected elem: {elem:?}");
                }
            },
        });

        SpirvCapabilities { comment, contents }
    }

    fn parse_spirv_capability(&mut self, elem: Elem) -> SpirvCapability {
        let mut name = None;

        for attr in elem.start.attributes() {
            let attr = attr.unwrap();
            match attr.key.as_ref() {
                b"name" => self.save_attr(attr, &mut name),
                _ => panic!("unexpected attr: {attr:?}"),
            }
        }

        let mut contents = Vec::new();
        self.parse_contents(elem, |this, content| match content {
            Content::Text(text) => this.assert_is_ws(text.as_bytes()),
            Content::Elem(elem) => match elem.start.name().as_ref() {
                b"enable" => {
                    let enable = this.parse_spirv_capability_enable(elem);
                    contents.push(SpirvCapabilityContent::Enable(enable));
                }
                _ => {
                    panic!("unexpected elem: {elem:?}");
                }
            },
        });

        SpirvCapability { name, contents }
    }

    fn parse_spirv_capability_enable(&mut self, elem: Elem) -> SpirvCapabilityEnable {
        let mut alias = None;
        let mut extension = None;
        let mut feature = None;
        let mut member = None;
        let mut property = None;
        let mut requires = None;
        let mut struc = None;
        let mut value = None;
        let mut version = None;

        for attr in elem.start.attributes() {
            let attr = attr.unwrap();
            match attr.key.as_ref() {
                b"alias" => self.save_attr(attr, &mut alias),
                b"extension" => self.save_attr(attr, &mut extension),
                b"feature" => self.save_attr(attr, &mut feature),
                b"member" => self.save_attr(attr, &mut member),
                b"property" => self.save_attr(attr, &mut property),
                b"requires" => self.save_attr(attr, &mut requires),
                b"struct" => self.save_attr(attr, &mut struc),
                b"value" => self.save_attr(attr, &mut value),
                b"version" => self.save_attr(attr, &mut version),
                _ => panic!("unexpected attr: {attr:?}"),
            }
        }

        assert_eq!(elem.is_empty, true);
        SpirvCapabilityEnable {
            alias,
            extension,
            feature,
            member,
            property,
            requires,
            struc,
            value,
            version,
        }
    }

    fn parse_syncs(&mut self, elem: Elem) -> Syncs {
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
                b"syncstage" => {
                    let stage = this.parse_sync_stage(elem);
                    contents.push(SyncsContent::SyncStage(stage));
                }
                b"syncaccess" => {
                    let access = this.parse_sync_access(elem);
                    contents.push(SyncsContent::SyncAccess(access));
                }
                b"syncpipeline" => {
                    let pipeline = this.parse_sync_pipeline(elem);
                    contents.push(SyncsContent::SyncPipeline(pipeline));
                }
                _ => {
                    panic!("unexpected elem: {elem:?}");
                }
            },
        });

        Syncs { comment, contents }
    }

    fn parse_sync_stage(&mut self, elem: Elem) -> SyncStage {
        let mut alias = None;
        let mut name = None;

        for attr in elem.start.attributes() {
            let attr = attr.unwrap();
            match attr.key.as_ref() {
                b"alias" => self.save_attr(attr, &mut alias),
                b"name" => self.save_attr(attr, &mut name),
                _ => panic!("unexpected attr: {attr:?}"),
            }
        }

        let mut contents = Vec::new();
        self.parse_contents(elem, |this, content| match content {
            Content::Text(text) => this.assert_is_ws(text.as_bytes()),
            Content::Elem(elem) => match elem.start.name().as_ref() {
                b"syncsupport" => {
                    let support = this.parse_sync_stage_support(elem);
                    contents.push(SyncStageContent::SyncSupport(support));
                }
                b"syncequivalent" => {
                    let equivalent = this.parse_sync_stage_equivalent(elem);
                    contents.push(SyncStageContent::SyncEquivalent(equivalent));
                }
                _ => {
                    panic!("unexpected elem: {elem:?}");
                }
            },
        });

        SyncStage {
            alias,
            name,
            contents,
        }
    }

    fn parse_sync_stage_support(&mut self, elem: Elem) -> SyncStageSupport {
        let mut queues = None;

        for attr in elem.start.attributes() {
            let attr = attr.unwrap();
            match attr.key.as_ref() {
                b"queues" => self.save_attr(attr, &mut queues),
                _ => panic!("unexpected attr: {attr:?}"),
            }
        }

        assert_eq!(elem.is_empty, true);
        SyncStageSupport { queues }
    }

    fn parse_sync_stage_equivalent(&mut self, elem: Elem) -> SyncStageEquivalent {
        let mut stage = None;

        for attr in elem.start.attributes() {
            let attr = attr.unwrap();
            match attr.key.as_ref() {
                b"stage" => self.save_attr(attr, &mut stage),
                _ => panic!("unexpected attr: {attr:?}"),
            }
        }

        assert_eq!(elem.is_empty, true);
        SyncStageEquivalent { stage }
    }

    fn parse_sync_access(&mut self, elem: Elem) -> SyncAccess {
        let mut alias = None;
        let mut name = None;

        for attr in elem.start.attributes() {
            let attr = attr.unwrap();
            match attr.key.as_ref() {
                b"alias" => self.save_attr(attr, &mut alias),
                b"name" => self.save_attr(attr, &mut name),
                _ => panic!("unexpected attr: {attr:?}"),
            }
        }

        let mut contents = Vec::new();
        self.parse_contents(elem, |this, content| match content {
            Content::Text(text) => this.assert_is_ws(text.as_bytes()),
            Content::Elem(elem) => match elem.start.name().as_ref() {
                b"comment" => {
                    let comment = this.parse_text_elem(elem);
                    contents.push(SyncAccessContent::Comment(comment));
                }
                b"syncsupport" => {
                    let support = this.parse_sync_access_support(elem);
                    contents.push(SyncAccessContent::SyncSupport(support));
                }
                b"syncequivalent" => {
                    let equivalent = this.parse_sync_access_equivalent(elem);
                    contents.push(SyncAccessContent::SyncEquivalent(equivalent));
                }
                _ => {
                    panic!("unexpected elem: {elem:?}");
                }
            },
        });

        SyncAccess {
            alias,
            name,
            contents,
        }
    }

    fn parse_sync_access_support(&mut self, elem: Elem) -> SyncAccessSupport {
        let mut stage = None;

        for attr in elem.start.attributes() {
            let attr = attr.unwrap();
            match attr.key.as_ref() {
                b"stage" => self.save_attr(attr, &mut stage),
                _ => panic!("unexpected attr: {attr:?}"),
            }
        }

        assert_eq!(elem.is_empty, true);
        SyncAccessSupport { stage }
    }

    fn parse_sync_access_equivalent(&mut self, elem: Elem) -> SyncAccessEquivalent {
        let mut access = None;

        for attr in elem.start.attributes() {
            let attr = attr.unwrap();
            match attr.key.as_ref() {
                b"access" => self.save_attr(attr, &mut access),
                _ => panic!("unexpected attr: {attr:?}"),
            }
        }

        assert_eq!(elem.is_empty, true);
        SyncAccessEquivalent { access }
    }

    fn parse_sync_pipeline(&mut self, elem: Elem) -> SyncPipeline {
        let mut depends = None;
        let mut name = None;

        for attr in elem.start.attributes() {
            let attr = attr.unwrap();
            match attr.key.as_ref() {
                b"depends" => self.save_attr(attr, &mut depends),
                b"name" => self.save_attr(attr, &mut name),
                _ => panic!("unexpected attr: {attr:?}"),
            }
        }

        let mut contents = Vec::new();
        self.parse_contents(elem, |this, content| match content {
            Content::Text(text) => this.assert_is_ws(text.as_bytes()),
            Content::Elem(elem) => match elem.start.name().as_ref() {
                b"syncpipelinestage" => {
                    let stage = this.parse_sync_pipeline_stage(elem);
                    contents.push(SyncPipelineContent::SyncPipelineStage(stage));
                }
                _ => {
                    panic!("unexpected elem: {elem:?}");
                }
            },
        });

        SyncPipeline {
            depends,
            name,
            contents,
        }
    }

    fn parse_sync_pipeline_stage(&mut self, elem: Elem) -> SyncPipelineStage {
        let mut before = None;
        let mut order = None;

        for attr in elem.start.attributes() {
            let attr = attr.unwrap();
            match attr.key.as_ref() {
                b"before" => self.save_attr(attr, &mut before),
                b"order" => self.save_attr(attr, &mut order),
                _ => panic!("unexpected attr: {attr:?}"),
            }
        }

        let mut contents = String::new();
        self.parse_contents(elem, |_this, content| match content {
            Content::Text(text) => contents += text,
            Content::Elem(elem) => panic!("unexpected elem: {elem:?}"),
        });

        SyncPipelineStage {
            before,
            order,
            contents,
        }
    }

    fn parse_video_codecs(&mut self, elem: Elem) -> VideoCodecs {
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
                b"videocodec" => {
                    let codec = this.parse_video_codec(elem);
                    contents.push(VideoCodecsContent::VideoCodec(codec));
                }
                _ => {
                    panic!("unexpected elem: {elem:?}");
                }
            },
        });

        VideoCodecs { contents }
    }

    fn parse_video_codec(&mut self, elem: Elem) -> VideoCodec {
        let mut extend = None;
        let mut name = None;
        let mut value = None;

        for attr in elem.start.attributes() {
            let attr = attr.unwrap();
            match attr.key.as_ref() {
                b"extend" => self.save_attr(attr, &mut extend),
                b"name" => self.save_attr(attr, &mut name),
                b"value" => self.save_attr(attr, &mut value),
                _ => panic!("unexpected attr: {attr:?}"),
            }
        }

        let mut contents = Vec::new();
        self.parse_contents(elem, |this, content| match content {
            Content::Text(text) => this.assert_is_ws(text.as_bytes()),
            Content::Elem(elem) => match elem.start.name().as_ref() {
                b"videocapabilities" => {
                    let capabilities = this.parse_video_capabilities(elem);
                    contents.push(VideoCodecContent::VideoCapabilities(capabilities));
                }
                b"videoformat" => {
                    let format = this.parse_video_format(elem);
                    contents.push(VideoCodecContent::VideoFormat(format));
                }
                b"videoprofiles" => {
                    let profiles = this.parse_video_profiles(elem);
                    contents.push(VideoCodecContent::VideoProfiles(profiles));
                }
                _ => {
                    panic!("unexpected elem: {elem:?}");
                }
            },
        });

        VideoCodec {
            extend,
            name,
            value,
            contents,
        }
    }

    fn parse_video_capabilities(&mut self, elem: Elem) -> VideoCapabilities {
        let mut struc = None;

        for attr in elem.start.attributes() {
            let attr = attr.unwrap();
            match attr.key.as_ref() {
                b"struct" => self.save_attr(attr, &mut struc),
                _ => panic!("unexpected attr: {attr:?}"),
            }
        }

        assert_eq!(elem.is_empty, true);
        VideoCapabilities { struc }
    }

    fn parse_video_format(&mut self, elem: Elem) -> VideoFormat {
        let mut extend = None;
        let mut name = None;
        let mut usage = None;

        for attr in elem.start.attributes() {
            let attr = attr.unwrap();
            match attr.key.as_ref() {
                b"extend" => self.save_attr(attr, &mut extend),
                b"name" => self.save_attr(attr, &mut name),
                b"usage" => self.save_attr(attr, &mut usage),
                _ => panic!("unexpected attr: {attr:?}"),
            }
        }

        let mut contents = Vec::new();
        self.parse_contents(elem, |this, content| match content {
            Content::Text(text) => this.assert_is_ws(text.as_bytes()),
            Content::Elem(elem) => match elem.start.name().as_ref() {
                b"videorequirecapabilities" => {
                    let capabilities = this.parse_video_require_capabilities(elem);
                    contents.push(VideoFormatContent::VideoRequireCapabilities(capabilities));
                }
                b"videoformatproperties" => {
                    let properties = this.parse_video_format_properties(elem);
                    contents.push(VideoFormatContent::VideoFormatProperties(properties));
                }
                _ => {
                    panic!("unexpected elem: {elem:?}");
                }
            },
        });

        VideoFormat {
            extend,
            name,
            usage,
            contents,
        }
    }

    fn parse_video_require_capabilities(&mut self, elem: Elem) -> VideoRequireCapabilities {
        let mut member = None;
        let mut struc = None;
        let mut value = None;

        for attr in elem.start.attributes() {
            let attr = attr.unwrap();
            match attr.key.as_ref() {
                b"member" => self.save_attr(attr, &mut member),
                b"struct" => self.save_attr(attr, &mut struc),
                b"value" => self.save_attr(attr, &mut value),
                _ => panic!("unexpected attr: {attr:?}"),
            }
        }

        assert_eq!(elem.is_empty, true);
        VideoRequireCapabilities {
            member,
            struc,
            value,
        }
    }

    fn parse_video_format_properties(&mut self, elem: Elem) -> VideoFormatProperties {
        let mut struc = None;

        for attr in elem.start.attributes() {
            let attr = attr.unwrap();
            match attr.key.as_ref() {
                b"struct" => self.save_attr(attr, &mut struc),
                _ => panic!("unexpected attr: {attr:?}"),
            }
        }

        assert_eq!(elem.is_empty, true);
        VideoFormatProperties { struc }
    }

    fn parse_video_profiles(&mut self, elem: Elem) -> VideoProfiles {
        let mut struc = None;

        for attr in elem.start.attributes() {
            let attr = attr.unwrap();
            match attr.key.as_ref() {
                b"struct" => self.save_attr(attr, &mut struc),
                _ => panic!("unexpected attr: {attr:?}"),
            }
        }

        let mut contents = Vec::new();
        self.parse_contents(elem, |this, content| match content {
            Content::Text(text) => this.assert_is_ws(text.as_bytes()),
            Content::Elem(elem) => match elem.start.name().as_ref() {
                b"videoprofilemember" => {
                    let member = this.parse_video_profile_member(elem);
                    contents.push(VideoProfilesContent::VideoProfileMember(member));
                }
                _ => {
                    panic!("unexpected elem: {elem:?}");
                }
            },
        });

        VideoProfiles { struc, contents }
    }

    fn parse_video_profile_member(&mut self, elem: Elem) -> VideoProfileMember {
        let mut name = None;

        for attr in elem.start.attributes() {
            let attr = attr.unwrap();
            match attr.key.as_ref() {
                b"name" => self.save_attr(attr, &mut name),
                _ => panic!("unexpected attr: {attr:?}"),
            }
        }

        let mut contents = Vec::new();
        self.parse_contents(elem, |this, content| match content {
            Content::Text(text) => this.assert_is_ws(text.as_bytes()),
            Content::Elem(elem) => match elem.start.name().as_ref() {
                b"videoprofile" => {
                    let profile = this.parse_video_profile(elem);
                    contents.push(VideoProfileMemberContent::VideoProfile(profile));
                }
                _ => {
                    panic!("unexpected elem: {elem:?}");
                }
            },
        });

        VideoProfileMember { name, contents }
    }

    fn parse_video_profile(&mut self, elem: Elem) -> VideoProfile {
        let mut name = None;
        let mut value = None;

        for attr in elem.start.attributes() {
            let attr = attr.unwrap();
            match attr.key.as_ref() {
                b"name" => self.save_attr(attr, &mut name),
                b"value" => self.save_attr(attr, &mut value),
                _ => panic!("unexpected attr: {attr:?}"),
            }
        }

        assert_eq!(elem.is_empty, true);
        VideoProfile { name, value }
    }
}
