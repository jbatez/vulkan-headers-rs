#[derive(Debug)]
pub(crate) struct CDecl<'a> {
    pub(crate) typ: CType<'a>,
    pub(crate) name: Option<&'a str>,
}

#[derive(Debug)]
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
        let mut parser = CDeclParser { s };
        let decl = parser.parse_decl();

        let mut token = parser.peek_next_token();
        if token == Some(";") {
            parser.consume(token.unwrap());
            token = parser.peek_next_token();
        }
        assert_eq!(token, None);

        decl
    }
}

struct CDeclParser<'a> {
    s: &'a str,
}

impl<'a> CDeclParser<'a> {
    fn is_ident_char(b: u8) -> bool {
        matches!(b, b'0'..=b'9' | b'A'..=b'Z' | b'_' | b'a'..=b'z')
    }

    fn is_ident(token: &'a str) -> bool {
        Self::is_ident_char(token.as_bytes()[0])
    }

    fn peek_next_token(&mut self) -> Option<&'a str> {
        while !self.s.is_empty() {
            match self.s.as_bytes()[0] {
                b'\n' | b'\r' | b' ' => self.s = &self.s[1..],
                _ => break,
            }
        }

        if self.s.is_empty() {
            return None;
        }

        let mut len = 1;
        if Self::is_ident_char(self.s.as_bytes()[0]) {
            while self.s.len() > len {
                if Self::is_ident_char(self.s.as_bytes()[len]) {
                    len += 1;
                } else {
                    break;
                }
            }
        }

        Some(&self.s[..len])
    }

    fn assert_next_token(&mut self, expected: &str) {
        let token = self.peek_next_token().unwrap();
        assert_eq!(token, expected);
        self.consume(token);
    }

    fn consume(&mut self, token: &'a str) {
        self.s = &self.s[token.len()..];
    }

    fn parse_type_name(&mut self) -> CType<'a> {
        let token = self.peek_next_token().unwrap();
        if token == "const" {
            self.consume(token);
            return CType::Const(Box::new(self.parse_type_name()));
        } else if Self::is_ident(token) {
            self.consume(token);
            CType::Name(token)
        } else {
            panic!("unexpected token: {token:?}");
        }
    }

    fn parse_decl(&mut self) -> CDecl<'a> {
        let mut typ = self.parse_type_name();
        let mut token = self.peek_next_token();

        loop {
            match token {
                Some("const") => {
                    self.consume(token.unwrap());
                    typ = CType::Const(Box::new(typ));
                }
                Some("*") => {
                    self.consume(token.unwrap());
                    typ = CType::Ptr(Box::new(typ));
                }
                _ => break,
            }
            token = self.peek_next_token();
        }

        if token == Some("(") {
            self.consume(token.unwrap());
            self.assert_next_token("VKAPI_PTR");
            self.assert_next_token("*");

            let name = match self.peek_next_token() {
                Some(name) if Self::is_ident(name) => {
                    self.consume(name);
                    Some(name)
                }
                _ => None,
            };

            self.assert_next_token(")");
            self.assert_next_token("(");

            let mut params = Vec::new();
            loop {
                params.push(self.parse_decl());
                let token = self.peek_next_token().unwrap();
                if token == "," {
                    self.consume(token);
                } else if token == ")" {
                    self.consume(token);
                    break;
                } else {
                    panic!("unexpected token: {token:?}")
                }
            }

            if params.len() == 1 {
                let param = &params[0];
                if matches!(param.typ, CType::Name("void")) && param.name.is_none() {
                    params.clear();
                }
            }

            return CDecl {
                typ: CType::FuncPtr {
                    return_type: Box::new(typ),
                    params,
                },
                name,
            };
        }

        let name = match token {
            Some(name) if Self::is_ident(name) => {
                self.consume(name);
                token = self.peek_next_token();
                Some(name)
            }
            _ => None,
        };

        let mut lens = Vec::new();
        while token == Some("[") {
            self.consume(token.unwrap());

            let len = self.peek_next_token().unwrap();
            if Self::is_ident(len) {
                self.consume(len);
            } else {
                panic!("unexpected token: {len:?}");
            }
            lens.push(len);

            self.assert_next_token("]");
            token = self.peek_next_token();
        }

        while let Some(len) = lens.pop() {
            typ = CType::Array {
                elem_type: Box::new(typ),
                len,
            };
        }

        CDecl { typ, name }
    }
}
