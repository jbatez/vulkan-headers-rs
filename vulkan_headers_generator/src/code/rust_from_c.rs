use crate::code::*;

pub(crate) fn rust_type_from_c_type_name(name: &str) -> &str {
    match name {
        "char" => "c_char",
        "double" => "f64",
        "float" => "f32",
        "int" => "c_int",
        "int16_t" => "i16",
        "int32_t" => "i32",
        "int64_t" => "i64",
        "int8_t" => "i8",
        "size_t" => "usize",
        "uint16_t" => "u16",
        "uint32_t" => "u32",
        "uint64_t" => "u64",
        "uint8_t" => "u8",
        "void" => "c_void",
        name => name,
    }
}

pub(crate) fn rust_type_from_c_type(c_typ: &CType) -> String {
    match c_typ {
        CType::Name(name) => {
            return rust_type_from_c_type_name(name).to_string();
        }
        CType::Const(_) => {
            panic!("unexpected C const type");
        }
        CType::Ptr(pointee_type) => {
            let (constness, pointee_type) = match pointee_type.as_ref() {
                CType::Const(pointee_type) => ("const", pointee_type.as_ref()),
                pointee_type => ("mut", pointee_type),
            };
            let pointee_type = rust_type_from_c_type(pointee_type);
            return format!("*{constness} {pointee_type}");
        }
        CType::Pfn { .. } => {
            panic!("unexpected C function pointer type");
        }
        CType::Array { elem_type, len } => {
            let elem_type = rust_type_from_c_type(elem_type);
            return format!("[{elem_type}; {len} as usize]");
        }
    }
}

pub(crate) fn rust_fn_signature_from_c(return_type: &CType, params: &[CDecl]) -> String {
    let mut s = "(".to_string();
    for (i, param) in params.iter().enumerate() {
        if i > 0 {
            s += ", ";
        } else if param.typ == "void" {
            assert_eq!(param.ident, None);
            assert_eq!(params.len(), 1);
            break;
        }

        rust_decl_from_c(&mut s, param);
    }
    s += ")";

    if *return_type != "void" {
        s += " -> ";
        s += &rust_type_from_c_type(return_type);
    }

    s
}

pub(crate) fn rust_decl_from_c(s: &mut String, c_decl: &CDecl) {
    *s += match c_decl.ident.as_ref().map(String::as_str) {
        Some("type") => "typ",
        Some(name) => name,
        None => "_",
    };

    *s += ": ";
    *s += &rust_type_from_c_type(&c_decl.typ);
}

pub(crate) fn rust_type_from_c_value(c_value: &str) -> &str {
    if c_value.starts_with('"') {
        assert!(c_value.ends_with('"'));
        "&CStr"
    } else {
        "u32"
    }
}

pub(crate) fn rust_value_from_c_value(mut c_value: &str) -> String {
    if c_value.starts_with('"') {
        assert!(c_value.ends_with('"'));
        format!("c{c_value}")
    } else {
        if c_value.starts_with('(') {
            assert!(c_value.ends_with(')'));
            c_value = &c_value[1..c_value.len() - 1];
        }

        if c_value.ends_with('F') && !c_value.contains("0x") {
            c_value = &c_value[..c_value.len() - 1];
        }

        if c_value.ends_with('L') {
            c_value = &c_value[..c_value.len() - 1];
            if c_value.ends_with('L') {
                c_value = &c_value[..c_value.len() - 1];
            }
        }

        if c_value.ends_with('U') {
            c_value = &c_value[..c_value.len() - 1];
        }

        if c_value.starts_with('~') {
            format!("!{}", &c_value[1..])
        } else {
            c_value.to_string()
        }
    }
}
