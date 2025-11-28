use std::{collections::HashSet, fs::File, io::Write};

use vulkan_registry::*;

use crate::code::*;

pub(crate) struct Generator<'a> {
    index: &'a RegistryIndex<'a>,
    require_names: HashSet<&'a str>,
    structs: Vec<(String, String)>,
    enums: Vec<(String, String)>,
    constants: Vec<(String, String)>,
    functions: Vec<(String, String)>,
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
            functions: Vec::new(),
            type_aliases: Vec::new(),
            unions: Vec::new(),
        };

        for feature in index.features.values() {
            generator.add_feature(feature);
        }

        // for extension in index.extensions.values() {
        //     generator.add_extension(extension);
        // }

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
        writeln!(out, "unsafe extern \"system\" {{").unwrap();
        generator.functions.sort_by(|a, b| a.0.cmp(&b.0));
        for (_, text) in &generator.functions {
            writeln!(out, "{text}").unwrap();
        }
        writeln!(out, "}}").unwrap();

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

    fn api_matches_vulkan(api: &Option<String>) -> bool {
        if let Some(api) = api.as_ref() {
            api.split(',').find(|&s| s == "vulkan").is_some()
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
                self.add_require(None, require);
            }
        }
    }

    // fn add_extension(&mut self, extension: &'a Extension) {
    //     if extension.platform.is_some() {
    //         return;
    //     }

    //     for extension_content in &extension.contents {
    //         if let ExtensionContent::Require(require) = extension_content {
    //             self.add_require(Some(extension), require);
    //         }
    //     }
    // }

    fn add_require(&mut self, extension: Option<&Extension>, require: &'a Require) {
        if !Self::api_matches_vulkan(&require.api) {
            return;
        }

        for require_content in &require.contents {
            match require_content {
                RequireContent::Comment(_) => (),
                RequireContent::Type(typ) => self.require_type(typ),
                RequireContent::Enum(enu) => self.require_enum(extension, enu),
                RequireContent::Command(command) => self.require_command(command),
                RequireContent::Feature(_) => (),
            }
        }
    }

    fn require_type(&mut self, typ: &'a GeneralRef) {
        let name = typ.name.as_ref().unwrap().as_str();
        if self.require_names.insert(name) {
            let typ = &self.index.types[name];
            self.add_type(name, typ);
        }
    }

    fn add_type(&mut self, name: &str, typ: &Type) {
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

    fn add_enum_type(&mut self, name: &str) {
        let enums = &self.index.enums[name];

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

    fn add_enum(&mut self, enums: &Enums, enu: &Enum) {
        if !Self::api_matches_vulkan(&enu.api) {
            return;
        }

        let name = enu.name.clone().unwrap();
        let typ = enums.name.as_ref().unwrap();

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

    fn add_handle_type(&mut self, name: &str) {
        self.add_extern_type(format!("{name}_T"));
        self.add_type_alias(name.to_string(), &format!("*mut {name}_T"));
        self.add_type_alias(format!("NonNull{name}"), &format!("NonNull<{name}_T>"));
    }

    fn add_pfn_type(&mut self, name: &str, signature: &str) {
        let rust_type = format!("unsafe extern \"system\" fn{signature}");
        self.add_type_alias(format!("NonNull{name}"), &rust_type);
        self.add_type_alias(name.to_string(), &format!("Option<NonNull{name}>"));
    }

    fn add_struct_type(&mut self, name: &str, typ: &Type) {
        let text = Self::rust_struct_or_union_from_registry_type(name, typ);
        self.structs.push((name.to_string(), text));
    }

    fn add_union_type(&mut self, name: &str, typ: &Type) {
        let text = Self::rust_struct_or_union_from_registry_type(name, typ);
        self.unions.push((name.to_string(), text));
    }

    fn rust_struct_or_union_from_registry_type(name: &str, typ: &Type) -> String {
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
            let name = match c_decl.name.as_ref().unwrap().as_str() {
                "type" => "typ",
                name => name,
            };

            let typ = Self::rust_type_from_c_type(&c_decl.typ);
            s += &format!("    pub {name}: {typ},\n");
        }

        s += "}";
        s
    }

    fn add_other_type(&mut self, name: &str, typ: &Type) {
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
            CType::Pfn {
                return_type,
                params,
            } => {
                let signature = Self::rust_signature_from_c_fn(return_type, params);
                self.add_pfn_type(name, &signature);
            }
            c_type => {
                let rust_type = Self::rust_type_from_c_type(c_type);
                self.add_type_alias(name.to_string(), &rust_type);
            }
        }
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
            CType::Name(name) => {
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
            CType::Pfn { .. } => {
                panic!("unexpected c function pointer type");
            }
            CType::Array { elem_type, len } => {
                let elem_type = Self::rust_type_from_c_type(elem_type);
                format!("[{elem_type}; {len} as usize]")
            }
        }
    }

    fn require_enum(&mut self, extension: Option<&Extension>, enu: &'a RequireEnum) {
        if !Self::api_matches_vulkan(&enu.api) {
            return;
        }

        let name = enu.name.as_ref().unwrap().as_str();
        if !self.require_names.insert(name) {
            return;
        }

        if let Some(extends) = enu.extends.as_ref() {
            let enums = &self.index.enums[extends.as_str()];
            self.add_extension_enum(extension, enu, enums);
        } else if let Some(_value) = enu.value.as_ref() {
            self.add_extension_constant(enu);
        } else if let Some(_alias) = enu.alias.as_ref() {
            self.add_extension_constant_alias(enu);
        } else if let Some(&constant) = self.index.constants.get(name) {
            self.add_constant(constant);
        }
    }

    fn add_extension_enum(
        &mut self,
        extension: Option<&Extension>,
        enu: &RequireEnum,
        enums: &Enums,
    ) {
        let name = enu.name.clone().unwrap();
        let typ = enums.name.as_ref().unwrap();

        let value = if let Some(alias) = enu.alias.as_ref() {
            alias.to_string()
        } else if let Some(bitpos) = enu.bitpos.as_ref() {
            format!("1 << {bitpos}")
        } else if let Some(offset) = enu.offset.as_ref() {
            let extnumber = match enu.extnumber.as_ref() {
                Some(extnumber) => extnumber,
                None => extension.unwrap().number.as_ref().unwrap(),
            };
            let extnumber: u32 = extnumber.parse().unwrap();
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

    fn add_extension_constant(&mut self, _constant: &RequireEnum) {
        // TODO
    }

    fn add_extension_constant_alias(&mut self, _constant: &RequireEnum) {
        // TODO
    }

    fn add_constant(&mut self, constant: &Enum) {
        if !Self::api_matches_vulkan(&constant.api) {
            return;
        }

        let name = constant.name.clone().unwrap();

        let typ = constant.typ.as_ref().unwrap();
        let typ = Self::rust_type_from_c_type_name(typ);

        let mut value = constant.value.as_ref().unwrap().as_str();
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
            let command = &self.index.commands[name];
            self.add_command(command);
        }
    }

    fn add_command(&mut self, command: &Command) {
        if !Self::api_matches_vulkan(&command.api) {
            return;
        }

        let actual_command = match command.alias.as_ref() {
            Some(alias) => &self.index.commands[alias.as_str()],
            None => command,
        };

        let mut proto_decl = None;
        let mut params = Vec::new();
        for command_content in &actual_command.contents {
            match command_content {
                CommandContent::Proto(proto) => {
                    let mut c_decl = String::new();
                    for proto_content in &proto.contents {
                        match proto_content {
                            ProtoContent::Text(text) => c_decl += text,
                            ProtoContent::Type(text) => c_decl += text,
                            ProtoContent::Name(text) => c_decl += text,
                        }
                    }
                    assert!(proto_decl.is_none());
                    proto_decl = Some(CDecl::parse(&c_decl));
                }
                CommandContent::Param(param) => {
                    if !Self::api_matches_vulkan(&param.api) {
                        continue;
                    }
                    let mut c_decl = String::new();
                    for proto_content in &param.contents {
                        match proto_content {
                            ParamContent::Text(text) => c_decl += text,
                            ParamContent::Type(text) => c_decl += text,
                            ParamContent::Name(text) => c_decl += text,
                        }
                    }
                    params.push(CDecl::parse(&c_decl));
                }
                CommandContent::ImplicitExternSyncParams(_) => (),
            }
        }

        let proto = proto_decl.unwrap();
        let name = match command.name.as_ref() {
            Some(name) => name,
            None => proto.name.as_ref().unwrap(),
        };

        let signature = Self::rust_signature_from_c_fn(&proto.typ, &params);
        self.add_pfn_type(&format!("PFN_{name}"), &signature);

        let feature = if let Some(export) = command.export.as_ref()
            && export.split(',').find(|&s| s == "vulkan").is_some()
        {
            "exported_prototypes"
        } else {
            "prototypes"
        };

        let text = format!("    #[cfg(feature = \"{feature}\")]\n    pub fn {name}{signature};");
        self.functions.push((name.to_string(), text));
    }

    fn rust_signature_from_c_fn(return_type: &CType, params: &[CDecl]) -> String {
        let mut s = "(".to_string();
        for (i, param) in params.iter().enumerate() {
            if i > 0 {
                s += ", ";
            }

            s += match param.name.as_ref().unwrap().as_str() {
                "type" => "typ",
                name => name,
            };

            s += ": ";
            match &param.typ {
                CType::Array { elem_type, .. } => {
                    let (prefix, pointee_type) = match elem_type.as_ref() {
                        CType::Const(non_const_type) => ("*const", non_const_type),
                        _ => ("*mut", elem_type),
                    };
                    let pointee_type = Self::rust_type_from_c_type(pointee_type);
                    s += &format!("{prefix} {pointee_type}");
                }
                param_typ => {
                    s += &Self::rust_type_from_c_type(param_typ);
                }
            }
        }
        s += ")";

        if let CType::Name(typ_name) = return_type
            && typ_name == "void"
        {
            return s;
        }

        s += " -> ";
        s += &Self::rust_type_from_c_type(&return_type);
        s
    }
}
