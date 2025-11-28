use std::{collections::HashSet, fs::File, io::Write};

use vulkan_registry::*;

use crate::code::*;

pub(crate) struct Generator<'a> {
    index: &'a RegistryIndex<'a>,
    require_names: HashSet<&'a str>,
    structs: Vec<(String, String)>,
    enums: Vec<(String, String)>,
    constants: Vec<(String, String)>,
    type_aliases: Vec<(String, String)>,
    unions: Vec<(String, String)>,
}

impl<'a> Generator<'a> {
    pub(crate) fn generate() {
        let registry = Registry::vk();
        let index = RegistryIndex::new(&registry);

        let mut generator = Generator {
            index: &index,
            require_names: HashSet::new(),
            structs: Vec::new(),
            enums: Vec::new(),
            constants: Vec::new(),
            type_aliases: Vec::new(),
            unions: Vec::new(),
        };

        for features in index.features.values() {
            for &feature in features {
                generator.add_feature(feature);
            }
        }

        let mut out = File::create("vulkan_headers/src/lib.rs").unwrap();
        out.write_all(
            b"\
#![allow(nonstandard_style)]
#![no_std]

use core::{ffi::{c_char, c_float, c_void}, ptr::NonNull};
        ",
        )
        .unwrap();

        generator.structs.sort_by(|a, b| a.0.cmp(&b.0));
        for (_, text) in &generator.structs {
            writeln!(out).unwrap();
            writeln!(out, "{text}").unwrap();
        }

        generator.enums.sort_by(|a, b| a.0.cmp(&b.0));
        for (_, text) in &generator.enums {
            writeln!(out).unwrap();
            writeln!(out, "{text}").unwrap();
        }

        writeln!(out).unwrap();
        generator.constants.sort_by(|a, b| a.0.cmp(&b.0));
        for (_, text) in &generator.constants {
            writeln!(out, "{text}").unwrap();
        }

        writeln!(out).unwrap();
        generator.type_aliases.sort_by(|a, b| a.0.cmp(&b.0));
        for (_, text) in &generator.type_aliases {
            writeln!(out, "{text}").unwrap();
        }

        generator.unions.sort_by(|a, b| a.0.cmp(&b.0));
        for (_, text) in &generator.unions {
            writeln!(out).unwrap();
            writeln!(out, "{text}").unwrap();
        }
    }

    fn api_matches_vulkan(api: &'a Option<String>) -> bool {
        if let Some(api) = api.as_ref() {
            api.split(',').find(|&api| api == "vulkan").is_some()
        } else {
            true
        }
    }

    fn add_feature(&mut self, feature: &'a Feature) {
        if !Self::api_matches_vulkan(&feature.api) {
            return;
        }

        for feature_content in &feature.contents {
            if let FeatureContent::Require(require) = feature_content {
                self.add_require(require);
            }
        }
    }

    fn add_require(&mut self, require: &'a Require) {
        if !Self::api_matches_vulkan(&require.api) {
            return;
        }

        for require_content in &require.contents {
            match require_content {
                RequireContent::Comment(_) => (),
                RequireContent::Type(typ) => self.require_type(typ),
                RequireContent::Enum(enu) => self.require_enum(enu),
                RequireContent::Command(command) => self.require_command(command),
                RequireContent::Feature(_) => (),
            }
        }
    }

    fn require_type(&mut self, typ: &'a GeneralRef) {
        let name = typ.name.as_ref().unwrap().as_str();
        if self.require_names.insert(name) {
            for &typ in &self.index.types[name] {
                self.add_type(name, typ);
            }
        }
    }

    fn add_type(&mut self, name: &'a str, typ: &'a Type) {
        if !Self::api_matches_vulkan(&typ.api) {
            return;
        }

        if let Some(alias) = typ.alias.as_ref() {
            self.add_type_alias(name.to_string(), alias);
            return;
        }

        match typ.category.as_ref().map(String::as_str) {
            Some("basetype") => self.add_other_type(name, typ),
            Some("bitmask") => self.add_other_type(name, typ),
            Some("define") => (),
            Some("enum") => self.add_enum_type(name),
            Some("funcpointer") => self.add_other_type(name, typ),
            Some("handle") => self.add_handle_type(name),
            Some("include") => (),
            Some("struct") => self.add_struct_type(name, typ),
            Some("union") => self.add_union_type(name, typ),
            Some(category) => panic!("unexpected type category: {category:?}"),
            None => self.add_other_type(name, typ),
        }
    }

    fn add_type_alias(&mut self, name: String, alias: &str) {
        let text = format!("pub type {name} = {alias};");
        self.type_aliases.push((name, text));
    }

    fn add_enum_type(&mut self, name: &'a str) {
        for &enums in &self.index.enums[name] {
            let alias = match enums.typ.as_ref().unwrap().as_str() {
                "bitmask" => match enums.bitwidth.as_ref().map(String::as_str) {
                    None => "VkFlags",
                    Some("64") => "VkFlags64",
                    Some(bitwidth) => panic!("unexpected enums bitwidth: {bitwidth:?}"),
                },
                "enum" => "i32",
                typ => panic!("unexpected enums type: {typ:?}"),
            };
            self.add_type_alias(name.to_string(), alias);

            for enums_content in &enums.contents {
                if let EnumsContent::Enum(enu) = enums_content {
                    self.add_enum(enums, enu);
                }
            }
        }
    }

    fn add_enum(&mut self, group: &'a Enums, enu: &'a Enum) {
        if !Self::api_matches_vulkan(&enu.api) {
            return;
        }

        let name = enu.name.clone().unwrap();
        let typ = group.name.as_ref().unwrap();

        let value = if let Some(alias) = enu.alias.as_ref() {
            alias.to_string()
        } else if let Some(bitpos) = enu.bitpos.as_ref() {
            format!("1 << {bitpos}")
        } else if let Some(value) = enu.value.as_ref() {
            value.to_string()
        } else {
            panic!("{enu:?}");
        };

        let text = format!("pub const {name}: {typ} = {value};");
        self.constants.push((name, text));
    }

    fn add_extern_type(&mut self, name: String) {
        let text = format!(
            "\
#[cfg_attr(not(doc), repr(u8))]
pub enum {name} {{
    #[doc(hidden)]
    __variant1,
    #[doc(hidden)]
    __variant2,
}}"
        );
        self.enums.push((name, text));
    }

    fn add_handle_type(&mut self, name: &'a str) {
        self.add_extern_type(format!("{name}_T"));
        self.add_type_alias(name.to_string(), &format!("*mut {name}_T"));
        self.add_type_alias(format!("NonNull{name}"), &format!("NonNull<{name}_T>"));
    }

    fn add_struct_type(&mut self, name: &'a str, typ: &'a Type) {
        let text = Self::rust_struct_or_union_from_registry_type(name, typ);
        self.structs.push((name.to_string(), text));
    }

    fn add_union_type(&mut self, name: &'a str, typ: &'a Type) {
        let text = Self::rust_struct_or_union_from_registry_type(name, typ);
        self.unions.push((name.to_string(), text));
    }

    fn rust_struct_or_union_from_registry_type(name: &'a str, typ: &'a Type) -> String {
        let category = typ.category.as_ref().unwrap();

        let mut s = String::new();
        s += "#[derive(Clone, Copy)]\n";
        s += "#[repr(C)]\n";
        s += &format!("pub {category} {name} {{\n");

        for type_content in &typ.contents {
            let TypeContent::Member(member) = type_content else {
                continue;
            };

            if !Self::api_matches_vulkan(&member.api) {
                continue;
            }

            let mut c_decl = String::new();
            for member_content in &member.contents {
                match member_content {
                    MemberContent::Comment(_) => (),
                    MemberContent::Text(text) => c_decl += text,
                    MemberContent::Type(text) => c_decl += text,
                    MemberContent::Name(text) => c_decl += text,
                    MemberContent::Enum(text) => c_decl += text,
                }
            }

            let c_decl = CDecl::parse(&c_decl);
            let name = match c_decl.name.unwrap() {
                "type" => "typ",
                name => name,
            };

            let typ = Self::rust_type_from_c_type(&c_decl.typ);
            s += &format!("    pub {name}: {typ},\n");
        }

        s += "}";
        s
    }

    fn add_other_type(&mut self, name: &'a str, typ: &'a Type) {
        let mut c_decl = String::new();
        for type_content in &typ.contents {
            match type_content {
                TypeContent::Comment(_) => (),
                TypeContent::Text(text) => c_decl += text,
                TypeContent::Type(text) => c_decl += text,
                TypeContent::Name(text) => c_decl += text,
                TypeContent::Member(_) => panic!("unexpected type member"),
            }
        }

        assert!(c_decl.starts_with("typedef "));
        let c_decl = CDecl::parse(&c_decl["typedef ".len()..]);
        match &c_decl.typ {
            CType::FnPtr {
                return_type,
                params,
            } => {
                let rust_type = Self::rust_type_from_c_fn_ptr(return_type, params);
                self.add_type_alias(name.to_string(), &format!("Option<NonNull{name}>"));
                self.add_type_alias(format!("NonNull{name}"), &rust_type);
            }
            c_type => {
                let rust_type = Self::rust_type_from_c_type(c_type);
                self.add_type_alias(name.to_string(), &rust_type);
            }
        }
    }

    fn rust_type_from_c_fn_ptr(return_type: &CType, params: &[CDecl]) -> String {
        let mut s = "unsafe extern \"system\" fn(".to_string();
        for (i, param) in params.iter().enumerate() {
            if i > 0 {
                s += ", ";
            }

            s += param.name.unwrap_or("_");
            s += ": ";
            s += &Self::rust_type_from_c_type(&param.typ);
        }
        s += ")";

        if !matches!(return_type, CType::Name("void")) {
            s += " -> ";
            s += &Self::rust_type_from_c_type(&return_type);
        }

        s
    }

    fn rust_type_from_c_type_name(name: &str) -> &str {
        match name {
            "void" => "c_void",
            "char" => "c_char",
            "int" => "c_int",
            "float" => "c_float",
            "double" => "c_double",
            "int8_t" => "i8",
            "int16_t" => "i16",
            "int32_t" => "i32",
            "int64_t" => "i64",
            "uint8_t" => "u8",
            "uint16_t" => "u16",
            "uint32_t" => "u32",
            "uint64_t" => "u64",
            "size_t" => "usize",
            name => name,
        }
    }

    fn rust_type_from_c_type(c_type: &CType) -> String {
        match c_type {
            &CType::Name(name) => {
                return Self::rust_type_from_c_type_name(name).to_string();
            }
            CType::Const(non_const_type) => {
                return Self::rust_type_from_c_type(non_const_type);
            }
            CType::Ptr(pointee_type) => {
                let (prefix, pointee_type) = match pointee_type.as_ref() {
                    CType::Const(non_const_type) => ("*const", non_const_type),
                    _ => ("*mut", pointee_type),
                };
                let pointee_type = Self::rust_type_from_c_type(pointee_type);
                format!("{prefix} {pointee_type}")
            }
            CType::FnPtr { .. } => {
                panic!("unexpected c function pointer type");
            }
            CType::Array { elem_type, len } => {
                let elem_type = Self::rust_type_from_c_type(elem_type);
                format!("[{elem_type}; {len} as usize]")
            }
        }
    }

    fn require_enum(&mut self, enu: &'a RequireEnum) {
        if !Self::api_matches_vulkan(&enu.api) {
            return;
        }

        let name = enu.name.as_ref().unwrap().as_str();
        if !self.require_names.insert(name) {
            return;
        }

        if let Some(extends) = enu.extends.as_ref() {
            for group in &self.index.enums[extends.as_str()] {
                self.add_enum_extension(group, enu);
            }
        } else {
            for &enu in &self.index.constants[name] {
                self.add_constant(enu);
            }
        }
    }

    fn add_enum_extension(&mut self, group: &'a Enums, enu: &'a RequireEnum) {
        let name = enu.name.clone().unwrap();
        let typ = group.name.as_ref().unwrap();

        let value = if let Some(alias) = enu.alias.as_ref() {
            alias.to_string()
        } else if let Some(bitpos) = enu.bitpos.as_ref() {
            format!("1 << {bitpos}")
        } else if let Some(offset) = enu.offset.as_ref() {
            let extnumber: u32 = enu.extnumber.as_ref().unwrap().parse().unwrap();
            let offset: u32 = offset.parse().unwrap();
            let value = 1_000_000_000 + 1_000 * (extnumber - 1) + offset;
            match enu.dir.as_ref().map(String::as_str) {
                None => format!("{value}"),
                Some("-") => format!("-{value}"),
                Some(dir) => panic!("unexpected enum dir: {dir:?}"),
            }
        } else if let Some(value) = enu.value.as_ref() {
            value.to_string()
        } else {
            panic!("{enu:?}");
        };

        let text = format!("pub const {name}: {typ} = {value};");
        self.constants.push((name, text));
    }

    fn add_constant(&mut self, enu: &'a Enum) {
        if !Self::api_matches_vulkan(&enu.api) {
            return;
        }

        let name = enu.name.clone().unwrap();

        let typ = enu.typ.as_ref().unwrap();
        let typ = Self::rust_type_from_c_type_name(typ);

        let mut value = enu.value.as_ref().unwrap().as_str();
        if value.starts_with("(") {
            assert!(value.ends_with(")"));
            value = &value[1..value.len() - 1];
        }
        if value.ends_with("L") {
            value = &value[..value.len() - 1];
            if value.ends_with("L") {
                value = &value[..value.len() - 1];
            }
        }
        if value.ends_with("U") {
            value = &value[..value.len() - 1];
        }
        if value.ends_with("F") && !value.contains("0x") {
            value = &value[..value.len() - 1];
        }

        let value = if value.starts_with("~") {
            format!("!{}", &value[1..])
        } else {
            value.to_string()
        };

        let text = format!("pub const {name}: {typ} = {value};");
        self.constants.push((name, text));
    }

    fn require_command(&mut self, command: &'a GeneralRef) {
        let name = command.name.as_ref().unwrap().as_str();
        if self.require_names.insert(name) {
            for &command in &self.index.commands[name] {
                self.add_command(command);
            }
        }
    }

    fn add_command(&mut self, command: &'a Command) {
        if !Self::api_matches_vulkan(&command.api) {
            return;
        }

        // TODO
    }
}
