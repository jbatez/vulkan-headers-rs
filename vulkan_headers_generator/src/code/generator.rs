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
        self.visit_core_registry(registry, index, &mut module);
        module.write_file();
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
                "basetype" => Self::add_base_type(typ, name, module),
                "bitmask" => Self::add_bitmask_type(typ, name, module),
                "define" => Self::add_define_type(name, module),
                "enum" => self.add_enum_type(typ, index, module),
                "funcpointer" => self.add_funcpointer_type(typ, index, module),
                "handle" => self.add_handle_type(typ, index, module),
                "include" => (),
                "struct" => self.add_struct_type(typ, index, module),
                "union" => self.add_union_type(typ, index, module),
                category => panic!("unexpected type category: {category:?}"),
            }
        }
    }

    fn add_type_alias(name: &str, alias: &str, module: &mut Module) {
        let text = format!("pub type {name} = {alias};");
        module.type_aliases.push((name.to_string(), text));
    }

    fn get_type_text(typ: &Type) -> String {
        let mut ret = String::new();
        for content in &typ.contents {
            match content {
                TypeContent::Comment(_) => (),
                TypeContent::Text(text) => ret += text,
                TypeContent::Type(text) => ret += text,
                TypeContent::Name(text) => ret += text,
                TypeContent::Member(_) => panic!("unexpected type member"),
            }
        }
        ret
    }

    fn add_typedef(name: &str, text: &str, module: &mut Module) {
        let c_decl = CDecl::parse_typedef(&text);
        let alias = rust_type_from_c_type(&c_decl.typ);
        Self::add_type_alias(name, &alias, module);
    }

    fn add_base_type(typ: &Type, name: &str, module: &mut Module) {
        let text = Self::get_type_text(typ);
        if text.starts_with("struct ") {
            // TODO
            return;
        } else if text.starts_with("typedef ") {
            return Self::add_typedef(name, &text, module);
        }

        match name {
            "CAMetalLayer" => (),       // TODO
            "MTLBuffer_id" => (),       // TODO
            "MTLCommandQueue_id" => (), // TODO
            "MTLDevice_id" => (),       // TODO
            "MTLSharedEvent_id" => (),  // TODO
            "MTLTexture_id" => (),      // TODO
            name => panic!("unexpected basetype: {name:?}"),
        }
    }

    fn add_bitmask_type(typ: &Type, name: &str, module: &mut Module) {
        let text = Self::get_type_text(typ);
        Self::add_typedef(name, &text, module);
    }

    fn add_define_type(name: &str, module: &mut Module) {
        match name {
            "VK_API_VERSION_1_0" => (),                                      // TODO
            "VK_API_VERSION_1_1" => (),                                      // TODO
            "VK_API_VERSION_1_2" => (),                                      // TODO
            "VK_API_VERSION_1_3" => (),                                      // TODO
            "VK_API_VERSION_1_4" => (),                                      // TODO
            "VK_API_VERSION_MAJOR" => (),                                    // TODO
            "VK_API_VERSION_MINOR" => (),                                    // TODO
            "VK_API_VERSION_PATCH" => (),                                    // TODO
            "VK_API_VERSION_VARIANT" => (),                                  // TODO
            "VK_API_VERSION" => (),                                          // TODO
            "VK_DEFINE_HANDLE" => (),                                        // TODO
            "VK_DEFINE_NON_DISPATCHABLE_HANDLE" => (),                       // TODO
            "VK_HEADER_VERSION_COMPLETE" => (),                              // TODO
            "VK_HEADER_VERSION" => (),                                       // TODO
            "VK_MAKE_API_VERSION" => (),                                     // TODO
            "VK_MAKE_VERSION" => (),                                         // TODO
            "VK_MAKE_VIDEO_STD_VERSION" => (),                               // TODO
            "VK_NULL_HANDLE" => (),                                          // TODO
            "VK_STD_VULKAN_VIDEO_CODEC_AV1_DECODE_API_VERSION_1_0_0" => (),  // TODO
            "VK_STD_VULKAN_VIDEO_CODEC_AV1_ENCODE_API_VERSION_1_0_0" => (),  // TODO
            "VK_STD_VULKAN_VIDEO_CODEC_H264_DECODE_API_VERSION_1_0_0" => (), // TODO
            "VK_STD_VULKAN_VIDEO_CODEC_H264_ENCODE_API_VERSION_1_0_0" => (), // TODO
            "VK_STD_VULKAN_VIDEO_CODEC_H265_DECODE_API_VERSION_1_0_0" => (), // TODO
            "VK_STD_VULKAN_VIDEO_CODEC_H265_ENCODE_API_VERSION_1_0_0" => (), // TODO
            "VK_STD_VULKAN_VIDEO_CODEC_VP9_DECODE_API_VERSION_1_0_0" => (),  // TODO
            "VK_USE_64_BIT_PTR_DEFINES" => (),                               // TODO
            "VK_VERSION_MAJOR" => (),                                        // TODO
            "VK_VERSION_MINOR" => (),                                        // TODO
            "VK_VERSION_PATCH" => (),                                        // TODO
            name => panic!("unexpected define type: {name:?}"),
        }
    }

    fn add_enum_type(&mut self, typ: &Type, index: &RegistryIndex, module: &mut Module) {
        // TODO
    }

    fn add_funcpointer_type(&mut self, typ: &Type, index: &RegistryIndex, module: &mut Module) {
        // TODO
    }

    fn add_handle_type(&mut self, typ: &Type, index: &RegistryIndex, module: &mut Module) {
        // TODO
    }

    fn add_struct_type(&mut self, typ: &Type, index: &RegistryIndex, module: &mut Module) {
        // TODO
    }

    fn add_union_type(&mut self, typ: &Type, index: &RegistryIndex, module: &mut Module) {
        // TODO
    }

    fn require_enum(&mut self, enu: &RequireEnum, index: &RegistryIndex, module: &mut Module) {
        if index.api_matches(&enu.api) && self.items.insert(enu.name.clone().unwrap()) {
            // TODO
        }
    }

    fn require_command(&mut self, cmd: &GeneralRef, index: &RegistryIndex, module: &mut Module) {
        if self.items.insert(cmd.name.clone().unwrap()) {
            // TODO
        }
    }
}
