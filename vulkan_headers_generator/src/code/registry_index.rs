use std::collections::HashMap;

use vulkan_registry::*;

pub(crate) struct RegistryIndex<'a> {
    pub(crate) types: HashMap<&'a str, &'a Type>,
    pub(crate) enums: HashMap<&'a str, &'a Enums>,
    pub(crate) constants: HashMap<&'a str, &'a Enum>,
    pub(crate) commands: HashMap<&'a str, &'a Command>,
    pub(crate) features: HashMap<&'a str, &'a Feature>,
}

impl<'a> RegistryIndex<'a> {
    pub(crate) fn new(registry: &'a Registry) -> Self {
        let mut index = Self {
            types: HashMap::new(),
            enums: HashMap::new(),
            constants: HashMap::new(),
            commands: HashMap::new(),
            features: HashMap::new(),
        };
        index.add_registry(registry);
        index
    }

    fn api_matches_vulkan(api: &Option<String>) -> bool {
        if let Some(api) = api.as_ref() {
            api.split(',').find(|&s| s == "vulkan").is_some()
        } else {
            true
        }
    }

    fn add_registry(&mut self, registry: &'a Registry) {
        for registry_content in &registry.contents {
            match registry_content {
                RegistryContent::Types(types) => self.add_types(types),
                RegistryContent::Enums(enums) => self.add_enums(enums),
                RegistryContent::Commands(commands) => self.add_commands(commands),
                RegistryContent::Feature(feature) => self.add_feature(feature),
                _ => (),
            }
        }
    }

    fn add_types(&mut self, types: &'a Types) {
        for types_content in &types.contents {
            if let TypesContent::Type(typ) = types_content {
                self.add_type(typ);
            }
        }
    }

    fn get_type_name(typ: &'a Type) -> &'a str {
        if let Some(name) = typ.name.as_ref() {
            return name;
        }

        for type_content in &typ.contents {
            if let TypeContent::Name(name) = type_content {
                return name;
            }
        }

        panic!("unnamed type");
    }

    fn add_type(&mut self, typ: &'a Type) {
        if !Self::api_matches_vulkan(&typ.api) {
            return;
        }

        let name = Self::get_type_name(typ);
        let old = self.types.insert(name, typ);
        assert_eq!(old, None);
    }

    fn add_enums(&mut self, enums: &'a Enums) {
        let typ = enums.typ.as_ref().map(String::as_str);
        if typ == Some("constants") {
            return self.add_constants(enums);
        }

        let name = enums.name.as_ref().unwrap().as_str();
        let old = self.enums.insert(name, enums);
        assert_eq!(old, None);
    }

    fn add_constants(&mut self, enums: &'a Enums) {
        for enums_content in &enums.contents {
            if let EnumsContent::Enum(constant) = enums_content {
                self.add_constant(constant);
            }
        }
    }

    fn add_constant(&mut self, constant: &'a Enum) {
        if !Self::api_matches_vulkan(&constant.api) {
            return;
        }

        let name = constant.name.as_ref().unwrap().as_str();
        let old = self.constants.insert(name, constant);
        assert_eq!(old, None);
    }

    fn add_commands(&mut self, commands: &'a Commands) {
        for commands_content in &commands.contents {
            let CommandsContent::Command(command) = commands_content;
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

        panic!("unnamed command");
    }

    fn add_command(&mut self, command: &'a Command) {
        if !Self::api_matches_vulkan(&command.api) {
            return;
        }

        let name = Self::get_command_name(command);
        let old = self.commands.insert(name, command);
        assert_eq!(old, None);
    }

    fn add_feature(&mut self, feature: &'a Feature) {
        if !Self::api_matches_vulkan(&feature.api) {
            return;
        }

        let name = feature.name.as_ref().unwrap().as_str();
        let old = self.features.insert(name, feature);
        assert_eq!(old, None);
    }
}
