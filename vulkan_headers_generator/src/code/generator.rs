use std::{collections::HashSet, fs::File, io::Write};

use vulkan_registry::*;

use crate::code::*;

pub(crate) struct Generator<'a> {
    index: &'a RegistryIndex<'a>,
    require_names: HashSet<&'a str>,
    enums: Vec<(String, String)>,
    structs: Vec<(String, String)>,
    unions: Vec<(String, String)>,
    type_aliases: Vec<(String, String)>,
}

impl<'a> Generator<'a> {
    pub(crate) fn generate() {
        let registry = Registry::vk();
        let index = RegistryIndex::new(&registry);

        let mut generator = Generator {
            index: &index,
            require_names: HashSet::new(),
            enums: Vec::new(),
            structs: Vec::new(),
            unions: Vec::new(),
            type_aliases: Vec::new(),
        };

        for features in index.features.values() {
            for &feature in features {
                generator.add_feature(feature);
            }
        }

        let mut out = File::create("vulkan_headers/src/lib.rs").unwrap();

        generator.enums.sort_by(|a, b| a.0.cmp(&b.0));
        for (_, text) in &generator.enums {
            writeln!(out).unwrap();
            writeln!(out, "{text}").unwrap();
        }

        generator.structs.sort_by(|a, b| a.0.cmp(&b.0));
        for (_, text) in &generator.structs {
            writeln!(out).unwrap();
            writeln!(out, "{text}").unwrap();
        }

        generator.unions.sort_by(|a, b| a.0.cmp(&b.0));
        for (_, text) in &generator.unions {
            writeln!(out).unwrap();
            writeln!(out, "{text}").unwrap();
        }

        writeln!(out).unwrap();
        generator.type_aliases.sort_by(|a, b| a.0.cmp(&b.0));
        for (_, text) in &generator.type_aliases {
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
        for &enums in &self.index.enum_groups[name] {
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
        }
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
        let mut s = String::new();
        s += "#[derive(Clone, Copy, Debug, Default]\n";
        s += "#[repr(C)]\n";
        s += &format!("pub {} {} {{\n", typ.category.as_ref().unwrap(), name);

        for type_content in &typ.contents {
            match type_content {
                TypeContent::Comment(_) => (),
                TypeContent::Text(text) => assert!(text.trim().is_empty()),
                TypeContent::Type(_) => panic!(),
                TypeContent::Name(_) => panic!(),
                TypeContent::Member(member) => {
                    if !Self::api_matches_vulkan(&member.api) {
                        continue;
                    }

                    let mut contents = String::new();
                    for member_content in &member.contents {
                        match member_content {
                            MemberContent::Comment(_) => (),
                            MemberContent::Text(text) => contents += text,
                            MemberContent::Type(text) => contents += text,
                            MemberContent::Name(text) => contents += text,
                            MemberContent::Enum(text) => contents += text,
                        }
                    }

                    let c_decl = CDecl::parse(&contents);
                    let name = c_decl.name.unwrap();
                    let rust_type = Self::rust_type_from_c_type(&c_decl.typ);
                    s += &format!("    pub {}: {},\n", name, rust_type);
                }
            }
        }

        s += "}";
        s
    }

    fn add_other_type(&mut self, name: &'a str, typ: &'a Type) {
        let mut contents = String::new();
        for type_content in &typ.contents {
            match type_content {
                TypeContent::Comment(_) => (),
                TypeContent::Text(text) => contents += text,
                TypeContent::Type(text) => contents += text,
                TypeContent::Name(text) => contents += text,
                TypeContent::Member(_) => panic!("unexpected type member"),
            }
        }

        assert!(contents.starts_with("typedef "));
        let c_decl = CDecl::parse(&contents["typedef ".len()..]);
        match &c_decl.typ {
            CType::FuncPtr {
                return_type,
                params,
            } => {
                let rust_type = Self::rust_type_from_c_func_ptr(return_type, params);
                self.add_type_alias(name.to_string(), &format!("Option<NonNull{name}>"));
                self.add_type_alias(format!("NonNull{name}"), &rust_type);
            }
            c_type => {
                let rust_type = Self::rust_type_from_c_type(c_type);
                self.add_type_alias(name.to_string(), &rust_type);
            }
        }
    }

    fn rust_type_from_c_func_ptr(return_type: &CType, params: &[CDecl]) -> String {
        let mut s = "unsafe extern \"system\" fn(".to_string();
        for (i, param) in params.iter().enumerate() {
            if i > 0 {
                s += ", ";
            }
            match param.name {
                Some(name) => s += name,
                None => s += "_",
            }
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

    fn rust_type_from_c_type(c_type: &CType) -> String {
        match c_type {
            &CType::Name(name) => match name {
                "void" => "c_void".to_string(),
                "char" => "c_char".to_string(),
                "int" => "c_int".to_string(),
                "float" => "c_float".to_string(),
                "double" => "c_double".to_string(),
                "int8_t" => "i8".to_string(),
                "int16_t" => "i16".to_string(),
                "int32_t" => "i32".to_string(),
                "int64_t" => "i64".to_string(),
                "uint8_t" => "u8".to_string(),
                "uint16_t" => "u16".to_string(),
                "uint32_t" => "u32".to_string(),
                "uint64_t" => "u64".to_string(),
                "size_t" => "usize".to_string(),
                name => name.to_string(),
            },
            CType::Const(non_const_type) => {
                return Self::rust_type_from_c_type(non_const_type);
            }
            CType::Ptr(pointee_type) => {
                let (prefix, pointee_type) = match pointee_type.as_ref() {
                    CType::Const(non_const_type) => ("*const ", non_const_type),
                    _ => ("*mut ", pointee_type),
                };
                format!("{}{}", prefix, Self::rust_type_from_c_type(pointee_type))
            }
            CType::FuncPtr { .. } => {
                panic!("unexpected c function pointer type");
            }
            CType::Array { elem_type, len } => {
                format!("[{}; {}]", Self::rust_type_from_c_type(elem_type), len)
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

        // TODO
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
