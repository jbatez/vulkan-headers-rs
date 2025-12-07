use std::collections::{HashMap, HashSet};

use vulkan_registry::*;

use crate::code::*;

pub(crate) struct Generator {
    library: Library,
    items: HashSet<String>,
}

impl Generator {
    pub(crate) fn generate() {
        let mut generator = Self {
            library: Library::new(),
            items: HashSet::new(),
        };
        generator.generate_vk_video();
        generator.generate_vulkan();
        generator.library.write_files();
    }

    fn generate_vk_video(&mut self) {
        let registry = Registry::video();
        let index = RegistryIndex::new("vulkan", &registry);
        self.visit_video_registry(&registry, &index);
    }

    fn visit_video_registry(&mut self, registry: &Registry, index: &RegistryIndex) {
        for content in &registry.contents {
            if let RegistryContent::Extensions(extensions) = content {
                self.visit_video_extensions(extensions, index);
            }
        }
    }

    fn visit_video_extensions(&mut self, extensions: &Extensions, index: &RegistryIndex) {
        for ExtensionsContent::Extension(extension) in &extensions.contents {
            self.visit_video_extension(extension, index);
        }
    }

    fn visit_video_extension(&mut self, extension: &Extension, index: &RegistryIndex) {
        if index.api_matches(&extension.supported) {
            let name = extension.name.as_ref().unwrap();
            self.library.video_modules.push(name.to_string());

            let mut module = Module::new("vk_video", name);
            self.visit_extension(extension, index, &mut module);
            module.write_file();
        }
    }

    fn generate_vulkan(&mut self) {
        let registry = Registry::vk();
        let index = RegistryIndex::new("vulkan", &registry);
        self.generate_vulkan_core(&registry, &index);
        self.generate_vulkan_platforms(&registry, &index);
    }

    fn generate_vulkan_core(&mut self, registry: &Registry, index: &RegistryIndex) {
        let mut module = Module::new("vulkan", "vulkan_core");
        self.add_vulkan_core_re_exports(&mut module);
        self.visit_core_registry(registry, index, &mut module);
        module.write_file();
    }

    fn add_vulkan_core_re_exports(&mut self, module: &mut Module) {
        for name in &self.library.video_modules {
            let text = format!("#[doc(no_inline)]\npub use crate::vk_video::{name}::*;");
            module.re_exports.push((name.clone(), text));
        }
    }

    fn visit_core_registry(
        &mut self,
        registry: &Registry,
        index: &RegistryIndex,
        module: &mut Module,
    ) {
        for content in &registry.contents {
            match content {
                RegistryContent::Feature(feature) => {
                    self.visit_core_feature(feature, index, module);
                }
                RegistryContent::Extensions(extensions) => {
                    self.visit_core_extensions(extensions, index, module);
                }
                _ => (),
            }
        }
    }

    fn visit_core_feature(
        &mut self,
        feature: &Feature,
        index: &RegistryIndex,
        module: &mut Module,
    ) {
        if index.api_matches(&feature.api) {
            for content in &feature.contents {
                if let FeatureContent::Require(require) = content {
                    self.visit_require(require, index, module);
                }
            }
        }
    }

    fn visit_core_extensions(
        &mut self,
        extensions: &Extensions,
        index: &RegistryIndex,
        module: &mut Module,
    ) {
        for ExtensionsContent::Extension(extension) in &extensions.contents {
            self.visit_core_extension(extension, index, module);
        }
    }

    fn visit_core_extension(
        &mut self,
        extension: &Extension,
        index: &RegistryIndex,
        module: &mut Module,
    ) {
        if extension.platform.is_none() && index.api_matches(&extension.supported) {
            self.visit_extension(extension, index, module);
        }
    }

    fn generate_vulkan_platforms(&mut self, registry: &Registry, index: &RegistryIndex) {
        let mut modules = HashMap::new();
        self.visit_platform_registry(registry, index, &mut modules);
        for module in modules.values_mut() {
            module.write_file();
        }
    }

    fn visit_platform_registry(
        &mut self,
        registry: &Registry,
        index: &RegistryIndex,
        modules: &mut HashMap<String, Module>,
    ) {
        for content in &registry.contents {
            if let RegistryContent::Extensions(extensions) = content {
                self.visit_platform_extensions(extensions, index, modules);
            }
        }
    }

    fn visit_platform_extensions(
        &mut self,
        extensions: &Extensions,
        index: &RegistryIndex,
        modules: &mut HashMap<String, Module>,
    ) {
        for ExtensionsContent::Extension(extension) in &extensions.contents {
            self.visit_platform_extension(extension, index, modules);
        }
    }

    fn visit_platform_extension(
        &mut self,
        extension: &Extension,
        index: &RegistryIndex,
        modules: &mut HashMap<String, Module>,
    ) {
        if extension.platform.is_some() && index.api_matches(&extension.supported) {
            let platform = match extension.platform.as_ref().unwrap().as_str() {
                "provisional" => "beta",
                platform => platform,
            };

            if !modules.contains_key(platform) {
                self.library.platforms.push(platform.to_string());
                let module = Module::new("vulkan", &format!("vulkan_{platform}"));
                modules.insert(platform.to_string(), module);
            }

            let module = modules.get_mut(platform).unwrap();
            self.visit_extension(extension, index, module);
        }
    }

    fn visit_extension(
        &mut self,
        extension: &Extension,
        index: &RegistryIndex,
        module: &mut Module,
    ) {
        for content in &extension.contents {
            if let ExtensionContent::Require(require) = content {
                self.visit_require(require, index, module);
            }
        }
    }

    fn visit_require(&mut self, require: &Require, index: &RegistryIndex, module: &mut Module) {
        if index.api_matches(&require.api) {
            for content in &require.contents {
                match content {
                    RequireContent::Comment(_) => (),
                    RequireContent::Type(typ) => self.require_type(typ, index, module),
                    RequireContent::Enum(enu) => self.require_enum(enu, index, module),
                    RequireContent::Command(cmd) => self.require_command(cmd, index, module),
                    RequireContent::Feature(_) => (),
                }
            }
        }
    }

    fn require_type(&mut self, typ: &GeneralRef, index: &RegistryIndex, module: &mut Module) {
        let name = typ.name.as_ref().unwrap().as_str();
        if self.items.insert(name.to_string()) {
            let typ = &index.types[name];
            if let Some(alias) = typ.alias.as_ref() {
                Self::add_type_alias(name, alias, module);
                return;
            }

            match typ.category.as_ref().unwrap().as_str() {
                "basetype" => Self::add_base_type(name, typ, module),
                "bitmask" => Self::add_bitmask_type(name, typ, module),
                "define" => Self::add_define_type(name, typ, module),
                "enum" => Self::add_enum_type(name, index, module),
                "funcpointer" => Self::add_funcpointer_type(name, typ, module),
                "handle" => Self::add_handle_type(name, module),
                "include" => (),
                "struct" => Self::add_struct_type(name, typ, index, module),
                "union" => Self::add_union_type(name, typ, index, module),
                category => panic!("unexpected type category: {category:?}"),
            }
        }
    }

    fn add_type_alias(name: &str, alias: &str, module: &mut Module) {
        let text = format!("pub type {name} = {alias};");
        module.type_aliases.push((name.to_string(), text));
    }

    fn collect_type_text(typ: &Type) -> String {
        let mut s = String::new();
        for content in &typ.contents {
            match content {
                TypeContent::Comment(_) => (),
                TypeContent::Text(text) => s += text,
                TypeContent::Type(text) => s += text,
                TypeContent::Name(text) => s += text,
                TypeContent::Member(_) => panic!("unexpected type member"),
            }
        }
        s
    }

    fn add_extern_type(name: &str, module: &mut Module) {
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
        module.enums.push((name.to_string(), text));
    }

    fn add_typedef(name: &str, text: &str, module: &mut Module) {
        let c_decl = CDecl::parse_typedef_decl(&text);

        assert_eq!(c_decl.ident.unwrap(), name);
        let alias = rust_type_from_c_type(&c_decl.typ, false);
        Self::add_type_alias(name, &alias, module);
    }

    fn add_base_type(name: &str, typ: &Type, module: &mut Module) {
        let text = Self::collect_type_text(typ);
        if text.starts_with("typedef ") && !text.starts_with("typedef struct ") {
            return Self::add_typedef(name, &text, module);
        }

        match name {
            "AHardwareBuffer" | "ANativeWindow" | "CAMetalLayer" | "IOSurfaceRef"
            | "MTLBuffer_id" | "MTLCommandQueue_id" | "MTLDevice_id" | "MTLSharedEvent_id"
            | "MTLTexture_id" | "OHBufferHandle" | "OHNativeWindow" | "OH_NativeBuffer" => (),
            name => panic!("unexpected basetype: {name:?}"),
        }
    }

    fn add_bitmask_type(name: &str, typ: &Type, module: &mut Module) {
        let text = Self::collect_type_text(typ);
        Self::add_typedef(name, &text, module);
    }

    fn add_define_type(name: &str, typ: &Type, module: &mut Module) {
        let text = Self::collect_type_text(typ);
        if let Some(value) = CDecl::parse_define_constant(&text, name) {
            return Self::add_constant(name, "u32", value, module);
        }

        match name {
            "VK_API_VERSION_MAJOR" => {
                Self::add_function(name, VK_API_VERSION_MAJOR, module);
            }
            "VK_API_VERSION_MINOR" => {
                Self::add_function(name, VK_API_VERSION_MINOR, module);
            }
            "VK_API_VERSION_PATCH" => {
                Self::add_function(name, VK_API_VERSION_PATCH, module);
            }
            "VK_API_VERSION_VARIANT" => {
                Self::add_function(name, VK_API_VERSION_VARIANT, module);
            }
            "VK_API_VERSION" => {
                ();
            }
            "VK_DEFINE_HANDLE" => {
                ();
            }
            "VK_DEFINE_NON_DISPATCHABLE_HANDLE" => {
                ();
            }
            "VK_MAKE_API_VERSION" => {
                Self::add_function(name, VK_MAKE_API_VERSION, module);
            }
            "VK_MAKE_VERSION" => {
                Self::add_function(name, VK_MAKE_VERSION, module);
            }
            "VK_MAKE_VIDEO_STD_VERSION" => {
                Self::add_function(name, VK_MAKE_VIDEO_STD_VERSION, module);
            }
            "VK_NULL_HANDLE" => {
                ();
            }
            "VK_USE_64_BIT_PTR_DEFINES" => {
                Self::add_constant(name, "bool", "true", module);
            }
            "VK_VERSION_MAJOR" => {
                Self::add_function(name, VK_VERSION_MAJOR, module);
            }
            "VK_VERSION_MINOR" => {
                Self::add_function(name, VK_VERSION_MINOR, module);
            }
            "VK_VERSION_PATCH" => {
                Self::add_function(name, VK_VERSION_PATCH, module);
            }
            name => {
                panic!("unexpected define: {name:?}");
            }
        }
    }

    fn add_function(name: &str, text: &str, module: &mut Module) {
        module.functions.push((name.to_string(), text.to_string()));
    }

    fn add_enum_type(name: &str, index: &RegistryIndex, module: &mut Module) {
        let enums = index.enums[name];
        Self::add_enum_type_alias(name, enums, module);
        Self::add_enum_constants(name, enums, index, module);
    }

    fn add_enum_type_alias(name: &str, enums: &Enums, module: &mut Module) {
        let alias = match enums.typ.as_ref().unwrap().as_str() {
            "enum" => "i32",
            "bitmask" => match enums.bitwidth.as_ref().map(String::as_str) {
                None => "VkFlags",
                Some("64") => "VkFlags64",
                Some(bitwidth) => panic!("unexpected enums bitwidth: {bitwidth:?}"),
            },
            typ => panic!("unexpected enums type: {typ:?}"),
        };
        Self::add_type_alias(name, alias, module);
    }

    fn add_enum_constants(typ: &str, enums: &Enums, index: &RegistryIndex, module: &mut Module) {
        for content in &enums.contents {
            if let EnumsContent::Enum(enu) = content
                && index.api_matches(&enu.api)
            {
                let name = enu.name.as_ref().unwrap();
                if let Some(alias) = enu.alias.as_ref() {
                    Self::add_constant(name, typ, alias, module);
                } else if let Some(bitpos) = enu.bitpos.as_ref() {
                    Self::add_constant(name, typ, &format!("1 << {bitpos}"), module);
                } else if let Some(value) = enu.value.as_ref() {
                    Self::add_constant(name, typ, value, module);
                } else {
                    panic!("unable to determine value for {name:?}");
                }
            }
        }
    }

    fn add_funcpointer_type(name: &str, typ: &Type, module: &mut Module) {
        let text = Self::collect_type_text(typ);
        let c_decl = CDecl::parse_typedef_decl(&text);

        assert_eq!(c_decl.ident.unwrap(), name);
        let signature = match &c_decl.typ {
            CType::Pfn {
                return_type,
                params,
            } => rust_fn_signature_from_c(return_type, params),
            _ => panic!("expected a C function pointer type"),
        };

        Self::add_pfn_type_aliases(name, &signature, module);
    }

    fn add_pfn_type_aliases(name: &str, signature: &str, module: &mut Module) {
        Self::add_type_alias(name, &format!("Option<NonNull{name}>"), module);

        let non_null_name = format!("NonNull{name}");
        let non_null_alias = format!("unsafe extern \"system\" fn{signature}");
        Self::add_type_alias(&non_null_name, &non_null_alias, module);
    }

    fn add_handle_type(name: &str, module: &mut Module) {
        Self::add_extern_type(&format!("{name}_T"), module);
        Self::add_type_alias(name, &format!("*mut {name}_T"), module);

        let non_null_name = format!("NonNull{name}");
        let non_null_alias = format!("NonNull<{name}_T>");
        Self::add_type_alias(&non_null_name, &non_null_alias, module);
    }

    fn add_struct_type(name: &str, typ: &Type, index: &RegistryIndex, module: &mut Module) {
        let text = Self::generate_struct_or_union_text(name, typ, index);
        module.structs.push((name.to_string(), text));
    }

    fn add_union_type(name: &str, typ: &Type, index: &RegistryIndex, module: &mut Module) {
        let text = Self::generate_struct_or_union_text(name, typ, index);
        module.unions.push((name.to_string(), text));
    }

    fn collect_member_text(member: &Member) -> String {
        let mut s = String::new();
        for content in &member.contents {
            match content {
                MemberContent::Comment(_) => (),
                MemberContent::Text(text) => s += text,
                MemberContent::Type(text) => s += text,
                MemberContent::Name(text) => s += text,
                MemberContent::Enum(text) => s += text,
            }
        }
        s
    }

    fn generate_struct_or_union_text(name: &str, typ: &Type, index: &RegistryIndex) -> String {
        let category = typ.category.as_ref().unwrap();
        let mut s = format!("#[derive(Clone, Copy)]\n#[repr(C)]\npub {category} {name} {{\n");

        for content in &typ.contents {
            if let TypeContent::Member(member) = content
                && index.api_matches(&member.api)
            {
                let text = Self::collect_member_text(member);
                let c_decl = CDecl::parse_member_decl(&text);
                if c_decl.bit_field_width.is_some() {
                    // TODO
                }

                s += "    pub ";
                rust_decl_from_c_decl(&mut s, &c_decl, false);
                s += ",\n";
            }
        }

        s += "}";
        s
    }

    fn require_enum(&mut self, enu: &RequireEnum, index: &RegistryIndex, module: &mut Module) {
        let name = enu.name.as_ref().unwrap().as_str();
        if index.api_matches(&enu.api) && self.items.insert(name.to_string()) {
            let constant = index.constants[name];
            let typ = Self::get_constant_type(name, constant, index);
            let value = Self::get_constant_value(name, constant);
            Self::add_constant(name, typ, &value, module);
        }
    }

    fn get_constant_type<'a>(
        name: &str,
        constant: Constant<'a>,
        index: &'a RegistryIndex,
    ) -> &'a str {
        match constant {
            Constant::BaseEnum(enums, enu) => {
                assert_eq!(enums.typ.as_ref().unwrap(), "constants");
                rust_type_from_c_type_name(enu.typ.as_ref().unwrap())
            }
            Constant::ExtensionEnum(_, enu) => {
                if let Some(extends) = enu.extends.as_ref() {
                    extends
                } else if let Some(typ) = enu.typ.as_ref() {
                    rust_type_from_c_type_name(typ)
                } else if let Some(value) = enu.value.as_ref() {
                    rust_type_from_c_value(value)
                } else if let Some(alias) = enu.alias.as_ref() {
                    let alias_constant = index.constants[alias.as_str()];
                    Self::get_constant_type(alias, alias_constant, index)
                } else {
                    panic!("unable to determine type for {name:?}");
                }
            }
        }
    }

    fn get_constant_value(name: &str, constant: Constant) -> String {
        match constant {
            Constant::BaseEnum(enums, enu) => {
                assert_eq!(enums.typ.as_ref().unwrap(), "constants");
                rust_value_from_c_value(enu.value.as_ref().unwrap())
            }
            Constant::ExtensionEnum(extension, enu) => {
                if let Some(alias) = enu.alias.as_ref() {
                    alias.clone()
                } else if let Some(bitpos) = enu.bitpos.as_ref() {
                    format!("1 << {bitpos}")
                } else if let Some(value) = enu.value.as_ref() {
                    rust_value_from_c_value(value)
                } else if let Some(offset) = enu.offset.as_ref() {
                    let dir = enu.dir.as_ref().map(String::as_str).unwrap_or("");
                    let extnumber = match enu.extnumber.as_ref() {
                        Some(extnumber) => extnumber,
                        None => extension.unwrap().number.as_ref().unwrap(),
                    };
                    let extnumber: u32 = extnumber.parse().unwrap();
                    let base = 1_000_000_000 + 1_000 * extnumber.checked_sub(1).unwrap();
                    let magnitude = base + offset.parse::<u32>().unwrap();
                    format!("{dir}{magnitude}")
                } else {
                    panic!("unable to determine value for {name:?}");
                }
            }
        }
    }

    fn add_constant(name: &str, typ: &str, value: &str, module: &mut Module) {
        let text = format!("pub const {name}: {typ} = {value};");
        module.constants.push((name.to_string(), text));
    }

    fn require_command(&mut self, cmd: &GeneralRef, index: &RegistryIndex, module: &mut Module) {
        let name = cmd.name.as_ref().unwrap().as_str();
        if self.items.insert(name.to_string()) {
            let cmd = index.commands[name];
            let signature = if let Some(alias) = cmd.alias.as_ref() {
                let alias_cmd = index.commands[alias.as_str()];
                Self::collect_command_signature(alias_cmd, index)
            } else {
                Self::collect_command_signature(cmd, index)
            };

            Self::add_pfn_type_aliases(&format!("PFN_{name}"), &signature, module);
            Self::add_extern_function(name, cmd, &signature, index, module);
        }
    }

    fn collect_proto_text(proto: &Proto) -> String {
        let mut s = String::new();
        for content in &proto.contents {
            match content {
                ProtoContent::Text(text) => s += text,
                ProtoContent::Type(text) => s += text,
                ProtoContent::Name(text) => s += text,
            }
        }
        s
    }

    fn collect_param_text(param: &Param) -> String {
        let mut s = String::new();
        for content in &param.contents {
            match content {
                ParamContent::Text(text) => s += text,
                ParamContent::Type(text) => s += text,
                ParamContent::Name(text) => s += text,
            }
        }
        s
    }

    fn collect_command_signature(cmd: &Command, index: &RegistryIndex) -> String {
        let mut return_type = None;
        let mut params = Vec::new();

        for content in &cmd.contents {
            match content {
                CommandContent::Proto(proto) => {
                    let text = Self::collect_proto_text(proto);
                    let c_decl = CDecl::parse_cmd_decl(&text);
                    assert!(return_type.is_none());
                    return_type = Some(c_decl.typ);
                }
                CommandContent::Param(param) => {
                    if index.api_matches(&param.api) {
                        let text = Self::collect_param_text(param);
                        params.push(CDecl::parse_cmd_decl(&text));
                    }
                }
                CommandContent::ImplicitExternSyncParams(_) => (),
            }
        }

        rust_fn_signature_from_c(&return_type.unwrap(), &params)
    }

    fn add_extern_function(
        name: &str,
        cmd: &Command,
        signature: &str,
        index: &RegistryIndex,
        module: &mut Module,
    ) {
        let feature = if cmd.export.is_some() && index.api_matches(&cmd.export) {
            "exported_prototypes"
        } else {
            "prototypes"
        };

        let mut text = String::new();
        text += &format!("    /// Available if built with `{feature}`.\n");
        text += &format!("    #[cfg(any(doc, feature = \"{feature}\"))]\n");
        text += &format!("    pub fn {name}{signature};");

        module.extern_functions.push((name.to_string(), text));
    }
}
