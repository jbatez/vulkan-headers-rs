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
            panic!("unexpected const C type");
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
