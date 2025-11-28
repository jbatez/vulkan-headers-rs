#[derive(Debug)]
pub(crate) struct CDecl {
    pub(crate) name: Option<String>,
    pub(crate) typ: CType,
}

#[derive(Debug)]
pub(crate) enum CType {
    Name(String),
    Const(Box<CType>),
    Ptr(Box<CType>),
    Pfn {
        return_type: Box<CType>,
        params: Vec<CDecl>,
    },
    Array {
        elem_type: Box<CType>,
        len: String,
    },
}

impl CDecl {
    pub(crate) fn parse(s: &str) -> Self {
        let mut parser = CDeclParser { s };
        let decl = parser.parse_decl();

        let mut token = parser.peek_next_token();
        if token == Some(":") {
            // TODO
            return decl;
        }
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
        loop {
            match self.s.as_bytes().first() {
                Some(b'\n' | b'\r' | b' ') => self.s = &self.s[1..],
                Some(_) => break,
                None => return None,
            }
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

    fn assert_next_token_ident(&mut self) -> &'a str {
        let token = self.peek_next_token().unwrap();
        assert!(Self::is_ident(token));
        self.consume(token);
        token
    }

    fn consume(&mut self, token: &'a str) {
        self.s = &self.s[token.len()..];
    }

    fn consume_opt_ident(&mut self) -> Option<&'a str> {
        match self.peek_next_token() {
            Some(token) if Self::is_ident(token) => {
                self.consume(token);
                Some(token)
            }
            _ => None,
        }
    }

    fn parse_type_name(&mut self) -> CType {
        let mut token = self.peek_next_token().unwrap();
        if token == "const" {
            self.consume(token);
            return CType::Const(Box::new(self.parse_type_name()));
        }

        if token == "struct" {
            self.consume(token);
            token = self.peek_next_token().unwrap();
        }

        if Self::is_ident(token) {
            self.consume(token);
            return CType::Name(token.to_string());
        }

        panic!("unexpected token: {token:?}");
    }

    fn parse_ptrs(&mut self, mut typ: CType) -> CType {
        loop {
            let token = self.peek_next_token();
            match token {
                Some("const") => {
                    self.consume(token.unwrap());
                    typ = CType::Const(Box::new(typ));
                }
                Some("*") => {
                    self.consume(token.unwrap());
                    typ = CType::Ptr(Box::new(typ));
                }
                _ => break typ,
            }
        }
    }

    fn parse_fn_ptr(&mut self, return_type: CType) -> CDecl {
        self.assert_next_token("(");
        self.assert_next_token("VKAPI_PTR");
        self.assert_next_token("*");
        let name = self.consume_opt_ident();
        self.assert_next_token(")");
        self.assert_next_token("(");
        let params = self.parse_params();
        self.assert_next_token(")");

        CDecl {
            name: name.map(str::to_string),
            typ: CType::Pfn {
                return_type: Box::new(return_type),
                params,
            },
        }
    }

    fn parse_params(&mut self) -> Vec<CDecl> {
        let mut params = Vec::new();
        loop {
            let param = self.parse_decl();
            let token = self.peek_next_token().unwrap();
            if let CType::Name(typ_name) = &param.typ
                && typ_name == "void"
            {
                assert!(param.name.is_none());
                assert_eq!(token, ")");
                assert_eq!(params.len(), 0);
                break params;
            }

            params.push(param);
            match token {
                "," => self.consume(token),
                ")" => break params,
                _ => panic!("unexpected token: {token:?}"),
            }
        }
    }

    fn parse_decl(&mut self) -> CDecl {
        let typ = self.parse_type_name();
        let typ = self.parse_ptrs(typ);

        if self.peek_next_token() == Some("(") {
            self.parse_fn_ptr(typ)
        } else {
            let name = self.consume_opt_ident().map(str::to_string);
            let typ = self.parse_array_extents(typ);
            CDecl { typ, name }
        }
    }

    fn parse_array_extents(&mut self, mut typ: CType) -> CType {
        let mut extents = Vec::new();
        loop {
            let token = self.peek_next_token();
            if token == Some("[") {
                self.consume(token.unwrap());
                extents.push(self.assert_next_token_ident());
                self.assert_next_token("]");
            } else {
                break;
            }
        }

        while let Some(len) = extents.pop() {
            typ = CType::Array {
                elem_type: Box::new(typ),
                len: len.to_string(),
            };
        }

        typ
    }
}
