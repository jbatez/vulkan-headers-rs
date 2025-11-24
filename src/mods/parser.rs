use quick_xml::{
    Reader,
    events::{BytesStart, BytesText, Event, attributes::Attribute},
};

use crate::{Platform, Platforms, PlatformsElem, Registry, RegistryElem, Tag, Tags, TagsElem};

struct Parser<'a> {
    reader: Reader<&'a [u8]>,
}

enum Content<'a> {
    Text(BytesText<'a>),
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

    fn assert_is_ws(&mut self, text: BytesText) {
        for &b in text.as_ref() {
            assert!(matches!(b, b'\n' | b'\r' | b' '));
        }
    }

    fn parse_content<F>(&mut self, elem: Elem, mut f: F)
    where
        F: FnMut(&mut Parser, Content),
    {
        if elem.is_empty {
            return;
        }

        let mut buf = Vec::new();
        loop {
            match self.next_event(&mut buf) {
                Event::Text(text) => {
                    f(self, Content::Text(text));
                }
                Event::Empty(start) => {
                    f(
                        self,
                        Content::Elem(Elem {
                            is_empty: true,
                            start,
                        }),
                    );
                }
                Event::Start(start) => {
                    f(
                        self,
                        Content::Elem(Elem {
                            is_empty: false,
                            start,
                        }),
                    );
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
                        let elem = Elem { is_empty, start };
                        self.parse_registry(elem, &mut registry);
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

    fn parse_registry(&mut self, elem: Elem, registry: &mut Option<Registry>) {
        assert_eq!(*registry, None);

        for attr in elem.start.attributes() {
            let attr = attr.unwrap();
            match attr.key.as_ref() {
                _ => panic!("unexpected attr: {attr:?}"),
            }
        }

        let mut elems = Vec::new();
        self.parse_content(elem, |this, content| match content {
            Content::Text(text) => {
                this.assert_is_ws(text);
            }
            Content::Elem(elem) => match elem.start.name().as_ref() {
                b"comment" => {
                    let text = this.parse_text_elem(elem);
                    elems.push(RegistryElem::Comment(text));
                }
                b"platforms" => {
                    let platforms = this.parse_platforms(elem);
                    elems.push(RegistryElem::Platforms(platforms));
                }
                b"tags" => {
                    let tags = this.parse_tags(elem);
                    elems.push(RegistryElem::Tags(tags));
                }
                _ => {
                    panic!("unexpected elem: {elem:?}");
                }
            },
        });

        *registry = Some(Registry { elems });
    }

    fn parse_text_elem(&mut self, elem: Elem) -> String {
        for attr in elem.start.attributes() {
            let attr = attr.unwrap();
            match attr.key.as_ref() {
                _ => panic!("unexpected attr: {attr:?}"),
            }
        }

        let mut ret = String::new();
        self.parse_content(elem, |_this, content| match content {
            Content::Text(text) => {
                ret += text.decode().unwrap().as_ref();
            }
            Content::Elem(elem) => {
                panic!("unexpected elem: {elem:?}");
            }
        });
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

        let mut elems = Vec::new();
        self.parse_content(elem, |this, content| match content {
            Content::Text(text) => {
                this.assert_is_ws(text);
            }
            Content::Elem(elem) => match elem.start.name().as_ref() {
                b"platform" => {
                    let platform = this.parse_platform(elem);
                    elems.push(PlatformsElem::Platform(platform));
                }
                _ => {
                    panic!("unexpected elem: {elem:?}");
                }
            },
        });

        Platforms {
            comment: comment.unwrap(),
            elems,
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

        let mut elems = Vec::new();
        self.parse_content(elem, |this, content| match content {
            Content::Text(text) => {
                this.assert_is_ws(text);
            }
            Content::Elem(elem) => match elem.start.name().as_ref() {
                b"tag" => {
                    let tag = this.parse_tag(elem);
                    elems.push(TagsElem::Tag(tag));
                }
                _ => {
                    panic!("unexpected elem: {elem:?}");
                }
            },
        });

        Tags {
            comment: comment.unwrap(),
            elems,
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
}
