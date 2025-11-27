pub(crate) struct CDecl<'a> {
    pub(crate) typ: CType<'a>,
    pub(crate) name: &'a str,
}

pub(crate) enum CType<'a> {
    Name(&'a str),
    Const(Box<CType<'a>>),
    Ptr(Box<CType<'a>>),
    FuncPtr {
        return_type: Box<CType<'a>>,
        params: Vec<CDecl<'a>>,
    },
    Array {
        elem_type: Box<CType<'a>>,
        len: &'a str,
    },
}

impl<'a> CDecl<'a> {
    pub(crate) fn parse(s: &'a str) -> Self {
        let mut parser = CDeclParser {
            bytes: s.as_bytes(),
        };
        let decl = parser.parse_decl();
        assert_eq!(parser.peek_next_token(), None);
        decl
    }
}

struct CDeclParser<'a> {
    bytes: &'a [u8],
}

impl<'a> CDeclParser<'a> {
    fn is_ident_char(b: u8) -> bool {
        matches!(b, b'0'..=b'9' | b'A'..=b'Z' | b'_' | b'a'..=b'z')
    }

    fn peek_next_token(&mut self) -> Option<&'a [u8]> {
        while !self.bytes.is_empty() {
            match self.bytes[0] {
                b'\n' | b'\r' | b' ' => self.bytes = &self.bytes[1..],
                _ => break,
            }
        }

        if self.bytes.is_empty() {
            return None;
        }

        let mut len = 1;
        if Self::is_ident_char(self.bytes[0]) {
            while self.bytes.len() > len && Self::is_ident_char(self.bytes[len]) {
                len += 1;
            }
        }

        Some(&self.bytes[..len])
    }

    fn next_token(&mut self) -> Option<&'a [u8]> {
        let token = self.peek_next_token();
        if let Some(token) = token {
            self.bytes = &self.bytes[token.len()..];
        }
        token
    }

    fn parse_type_name(&mut self) -> CType<'a> {
        let token = self.next_token().unwrap();
        if token == b"const" {
            return CType::Const(Box::new(self.parse_type_name()));
        }

        assert!(Self::is_ident_char(token[0]));
        CType::Name(str::from_utf8(token).unwrap())
    }

    fn parse_decl(&mut self) -> CDecl<'a> {
        let mut typ = self.parse_type_name();

        let mut token = self.next_token().unwrap();
        loop {
            match token {
                b"const" => typ = CType::Const(Box::new(typ)),
                b"*" => typ = CType::Ptr(Box::new(typ)),
                _ => break,
            }
            token = self.next_token().unwrap();
        }

        if token == b"(" {
            assert_eq!(self.next_token().unwrap(), b"VKAPI_PTR");
            assert_eq!(self.next_token().unwrap(), b"*");

            token = self.next_token().unwrap();
            assert!(Self::is_ident_char(token[0]));
            let name = str::from_utf8(token).unwrap();

            assert_eq!(self.next_token().unwrap(), b")");
            assert_eq!(self.next_token().unwrap(), b"(");

            let mut params = Vec::new();
            while self.peek_next_token().unwrap() != b")" {
                params.push(self.parse_decl());
            }

            assert_eq!(self.next_token().unwrap(), b")");
            assert_eq!(self.next_token().unwrap(), b";");

            return CDecl {
                typ: CType::FuncPtr {
                    return_type: Box::new(typ),
                    params,
                },
                name,
            };
        }

        assert!(Self::is_ident_char(token[0]));
        let name = str::from_utf8(token).unwrap();

        let mut lens = Vec::new();
        let mut token = self.next_token();
        while token == Some(b"[") {
            let len = self.next_token().unwrap();
            assert!(Self::is_ident_char(len[0]));
            lens.push(str::from_utf8(len).unwrap());

            assert_eq!(self.next_token().unwrap(), b"]");
            token = self.next_token();
        }

        while let Some(len) = lens.pop() {
            typ = CType::Array {
                elem_type: Box::new(typ),
                len,
            };
        }

        assert!(matches!(token, None | Some(b"," | b";")));
        CDecl { typ, name }
    }
}
