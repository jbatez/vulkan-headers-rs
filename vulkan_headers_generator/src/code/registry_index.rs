use std::collections::HashMap;

use vulkan_registry::*;

#[derive(Clone, Copy)]
pub(crate) enum Constant<'a> {
    BaseEnum(&'a Enums, &'a Enum),
    ExtensionEnum(Option<&'a Extension>, &'a RequireEnum),
}

pub(crate) struct RegistryIndex<'a> {
    api: &'a str,
    pub(crate) types: HashMap<&'a str, &'a Type>,
    pub(crate) enums: HashMap<&'a str, &'a Enums>,
    pub(crate) constants: HashMap<&'a str, Constant<'a>>,
    pub(crate) commands: HashMap<&'a str, &'a Command>,
}

impl<'a> RegistryIndex<'a> {
    pub(crate) fn new(api: &'a str, registry: &'a Registry) -> Self {
        let mut index = Self {
            api,
            types: HashMap::new(),
            enums: HashMap::new(),
            constants: HashMap::new(),
            commands: HashMap::new(),
        };
        index.visit_registry(registry);
        index
    }

    pub(crate) fn api_matches(&self, attr: &Option<String>) -> bool {
        match attr.as_ref() {
            Some(attr) => attr.split(',').find(|&s| s == self.api).is_some(),
            None => true,
        }
    }

    fn visit_registry(&mut self, registry: &'a Registry) {
        for content in &registry.contents {
            match content {
                RegistryContent::Types(types) => self.visit_types(types),
                RegistryContent::Enums(enums) => self.visit_enums(enums),
                RegistryContent::Commands(commands) => self.visit_commands(commands),
                RegistryContent::Feature(feature) => self.visit_feature(feature),
                RegistryContent::Extensions(extensions) => self.visit_extensions(extensions),
                _ => (),
            }
        }
    }

    fn visit_types(&mut self, types: &'a Types) {
        for content in &types.contents {
            if let TypesContent::Type(typ) = content {
                self.visit_type(typ);
            }
        }
    }

    fn visit_type(&mut self, typ: &'a Type) {
        if self.api_matches(&typ.api) {
            self.add_type(typ);
        }
    }

    fn get_type_name(typ: &'a Type) -> &'a str {
        if let Some(name) = typ.name.as_ref() {
            return name;
        }

        for content in &typ.contents {
            if let TypeContent::Name(name) = content {
                return name;
            }
        }

        panic!("unnamed type");
    }

    fn add_type(&mut self, typ: &'a Type) {
        let name = Self::get_type_name(typ);
        self.types.insert(name, typ);
    }

    fn visit_enums(&mut self, enums: &'a Enums) {
        self.add_enums(enums);
        for content in &enums.contents {
            if let EnumsContent::Enum(enu) = content {
                self.visit_enum(enums, enu);
            }
        }
    }

    fn add_enums(&mut self, enums: &'a Enums) {
        let name = enums.name.as_ref().unwrap().as_str();
        self.enums.insert(name, enums);
    }

    fn visit_enum(&mut self, enums: &'a Enums, enu: &'a Enum) {
        if self.api_matches(&enu.api) {
            self.add_base_enum_constant(enums, enu);
        }
    }

    fn add_base_enum_constant(&mut self, enums: &'a Enums, enu: &'a Enum) {
        let name = enu.name.as_ref().unwrap().as_str();
        let constant = Constant::BaseEnum(enums, enu);
        self.constants.insert(name, constant);
    }

    fn visit_commands(&mut self, commands: &'a Commands) {
        for content in &commands.contents {
            let CommandsContent::Command(command) = content;
            self.visit_command(command);
        }
    }

    fn visit_command(&mut self, command: &'a Command) {
        if self.api_matches(&command.api) {
            self.add_command(command);
        }
    }

    fn get_command_name(command: &'a Command) -> &'a str {
        if let Some(name) = command.name.as_ref() {
            return name;
        }

        for command_content in &command.contents {
            if let CommandContent::Proto(proto) = command_content {
                for proto_content in &proto.contents {
                    if let ProtoContent::Name(name) = proto_content {
                        return name;
                    }
                }
            }
        }

        panic!("unnamed type");
    }

    fn add_command(&mut self, command: &'a Command) {
        let name = Self::get_command_name(command);
        self.commands.insert(name, command);
    }

    fn visit_feature(&mut self, feature: &'a Feature) {
        if self.api_matches(&feature.api) {
            for content in &feature.contents {
                if let FeatureContent::Require(require) = content {
                    self.visit_require(None, require);
                }
            }
        }
    }

    fn visit_extensions(&mut self, extensions: &'a Extensions) {
        for content in &extensions.contents {
            let ExtensionsContent::Extension(extension) = content;
            self.visit_extension(extension);
        }
    }

    fn visit_extension(&mut self, extension: &'a Extension) {
        if self.api_matches(&extension.supported) {
            for content in &extension.contents {
                if let ExtensionContent::Require(require) = content {
                    self.visit_require(None, require);
                }
            }
        }
    }

    fn visit_require(&mut self, extension: Option<&'a Extension>, require: &'a Require) {
        if self.api_matches(&require.api) {
            for content in &require.contents {
                if let RequireContent::Enum(enu) = content {
                    self.visit_require_enum(extension, enu);
                }
            }
        }
    }

    fn visit_require_enum(&mut self, extension: Option<&'a Extension>, enu: &'a RequireEnum) {
        if self.api_matches(&enu.api)
            && (enu.alias.is_some()
                || enu.bitpos.is_some()
                || enu.offset.is_some()
                || enu.value.is_some())
        {
            self.add_extension_enum_constant(extension, enu);
        }
    }

    fn add_extension_enum_constant(
        &mut self,
        extension: Option<&'a Extension>,
        enu: &'a RequireEnum,
    ) {
        let name = enu.name.as_ref().unwrap().as_str();
        let constant = Constant::ExtensionEnum(extension, enu);
        self.constants.insert(name, constant);
    }
}
