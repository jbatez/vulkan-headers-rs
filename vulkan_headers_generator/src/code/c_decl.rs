pub(crate) struct CDecl {
    pub(crate) ident: Option<String>,
    pub(crate) typ: CType,
    pub(crate) bit_field_width: Option<String>,
}

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
    pub(crate) fn parse_struct_forward_decl(s: &str, name: &str) {
        let mut parser = CDeclParser { s };
        parser.consume("struct");
        parser.consume(name);
        parser.consume(";");
        assert_eq!(parser.peek_next_token(), None);
    }

    pub(crate) fn parse_typedef(s: &str) -> CDecl {
        let mut parser = CDeclParser { s };
        parser.consume("typedef");
        let decl = parser.parse_decl();
        assert_eq!(decl.bit_field_width, None);
        parser.consume(";");
        assert_eq!(parser.peek_next_token(), None);
        decl
    }

    pub(crate) fn parse_member_decl(s: &str) -> CDecl {
        let mut parser = CDeclParser { s };
        let decl = parser.parse_decl();
        assert_eq!(parser.peek_next_token(), None);
        decl
    }
}

struct CDeclParser<'a> {
    s: &'a str,
}

impl<'a> CDeclParser<'a> {
    fn skip_ws(&mut self) {
        loop {
            match self.s.as_bytes().first() {
                Some(b'\n' | b'\r' | b' ') => self.s = &self.s[1..],
                _ => break,
            }
        }
    }

    fn peek_next_token(&mut self) -> Option<&'a str> {
        self.skip_ws();
        if self.s.is_empty() {
            return None;
        }

        let mut len = 1;
        if is_digit_or_ident_char(self.s.as_bytes()[0]) {
            while let Some(&b) = self.s.as_bytes().get(len)
                && is_digit_or_ident_char(b)
            {
                len += 1;
            }
        }

        Some(&self.s[..len])
    }

    fn consume(&mut self, expected: &str) {
        let token = self.peek_next_token().unwrap();
        assert_eq!(token, expected);
        self.s = &self.s[token.len()..];
    }

    fn consume_ident(&mut self) -> String {
        let token = self.peek_next_token().unwrap();
        assert!(is_ident(token));
        self.s = &self.s[token.len()..];
        token.to_string()
    }

    fn consume_int_or_ident(&mut self) -> String {
        let token = self.peek_next_token().unwrap();
        assert!(is_int_or_ident(token));
        self.s = &self.s[token.len()..];
        token.to_string()
    }

    fn opt_consume_ident(&mut self) -> Option<String> {
        let token = self.peek_next_token();
        if let Some(token) = token
            && is_ident(token)
        {
            self.s = &self.s[token.len()..];
            Some(token.to_string())
        } else {
            None
        }
    }

    fn parse_maybe_const_type_name(&mut self) -> CType {
        if self.peek_next_token() == Some("const") {
            self.consume("const");
            let type_name = Box::new(self.parse_type_name());
            CType::Const(type_name)
        } else {
            self.parse_type_name()
        }
    }

    fn parse_type_name(&mut self) -> CType {
        if self.peek_next_token() == Some("struct") {
            self.consume("struct");
        }

        let name = self.consume_ident();
        CType::Name(name.to_string())
    }

    fn parse_maybe_const_ptrs(&mut self, mut typ: CType) -> CType {
        loop {
            match self.peek_next_token() {
                Some("const") => {
                    self.consume("const");
                    typ = CType::Const(Box::new(typ));
                }
                Some("*") => {
                    self.consume("*");
                    typ = CType::Ptr(Box::new(typ));
                }
                _ => return typ,
            }
        }
    }

    fn parse_decl(&mut self) -> CDecl {
        let typ = self.parse_maybe_const_type_name();
        let typ = self.parse_maybe_const_ptrs(typ);
        if self.peek_next_token() == Some("(") {
            self.parse_pfn_decl(Box::new(typ))
        } else {
            let ident = self.opt_consume_ident();
            let typ = self.parse_array_extents(typ);
            let bit_field_width = self.parse_bit_field_width();
            CDecl {
                ident,
                typ,
                bit_field_width,
            }
        }
    }

    fn parse_pfn_decl(&mut self, return_type: Box<CType>) -> CDecl {
        self.consume("(");
        self.consume("VKAPI_PTR");
        self.consume("*");
        let ident = self.opt_consume_ident();
        self.consume(")");
        self.consume("(");
        let params = self.parse_params();
        self.consume(")");

        CDecl {
            ident,
            typ: CType::Pfn {
                return_type,
                params,
            },
            bit_field_width: None,
        }
    }

    fn parse_params(&mut self) -> Vec<CDecl> {
        let mut params = Vec::new();
        loop {
            params.push(self.parse_decl());
            match self.peek_next_token().unwrap() {
                "," => self.consume(","),
                ")" => return params,
                token => panic!("unexpected token: {token:?}"),
            }
        }
    }

    fn parse_array_extents(&mut self, mut typ: CType) -> CType {
        let mut extents = Vec::new();
        while self.peek_next_token() == Some("[") {
            self.consume("[");
            extents.push(self.consume_int_or_ident());
            self.consume("]");
        }

        while let Some(len) = extents.pop() {
            let elem_type = Box::new(typ);
            typ = CType::Array { elem_type, len }
        }

        typ
    }

    fn parse_bit_field_width(&mut self) -> Option<String> {
        if self.peek_next_token() == Some(":") {
            self.consume(":");
            Some(self.consume_int_or_ident())
        } else {
            None
        }
    }
}

fn is_digit(b: u8) -> bool {
    matches!(b, b'0'..=b'9')
}

fn is_ident_nondigit(b: u8) -> bool {
    matches!(b, b'A'..=b'Z' | b'_' | b'a'..=b'z')
}

fn is_digit_or_ident_char(b: u8) -> bool {
    is_digit(b) || is_ident_nondigit(b)
}

fn is_ident(token: &str) -> bool {
    is_ident_nondigit(token.as_bytes()[0]) && is_int_or_ident(&token[1..])
}

fn is_int_or_ident(token: &str) -> bool {
    for &b in token.as_bytes() {
        if !is_digit_or_ident_char(b) {
            return false;
        }
    }
    true
}
