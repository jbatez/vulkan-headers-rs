use std::collections::HashMap;

use vulkan_registry::*;

pub(crate) struct RegistryIndex<'a> {
    pub(crate) types: HashMap<&'a str, Vec<&'a Type>>,
    pub(crate) enum_groups: HashMap<&'a str, Vec<&'a Enums>>,
    pub(crate) enums: HashMap<&'a str, Vec<&'a Enum>>,
    pub(crate) commands: HashMap<&'a str, Vec<&'a Command>>,
    pub(crate) features: HashMap<&'a str, Vec<&'a Feature>>,
}

impl<'a> RegistryIndex<'a> {
    pub(crate) fn new(registry: &'a Registry) -> Self {
        let mut index = Self {
            types: HashMap::new(),
            enum_groups: HashMap::new(),
            enums: HashMap::new(),
            commands: HashMap::new(),
            features: HashMap::new(),
        };
        index.add_registry(registry);
        index
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
        let name = Self::get_type_name(typ);
        if let Some(vec) = self.types.get_mut(name) {
            vec.push(typ);
        } else {
            let mut vec = Vec::new();
            vec.push(typ);
            self.types.insert(name, vec);
        }
    }

    fn add_enums(&mut self, enums: &'a Enums) {
        let name = enums.name.as_ref().unwrap().as_str();
        if let Some(vec) = self.enum_groups.get_mut(name) {
            vec.push(enums);
        } else {
            let mut vec = Vec::new();
            vec.push(enums);
            self.enum_groups.insert(name, vec);
        }

        for enums_content in &enums.contents {
            if let EnumsContent::Enum(enu) = enums_content {
                self.add_enum(enu);
            }
        }
    }

    fn add_enum(&mut self, enu: &'a Enum) {
        let name = enu.name.as_ref().unwrap().as_str();
        if let Some(vec) = self.enums.get_mut(name) {
            vec.push(enu);
        } else {
            let mut vec = Vec::new();
            vec.push(enu);
            self.enums.insert(name, vec);
        }
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
        let name = Self::get_command_name(command);
        if let Some(vec) = self.commands.get_mut(name) {
            vec.push(command);
        } else {
            let mut vec = Vec::new();
            vec.push(command);
            self.commands.insert(name, vec);
        }
    }

    fn add_feature(&mut self, feature: &'a Feature) {
        let name = feature.name.as_ref().unwrap().as_str();
        if let Some(vec) = self.features.get_mut(name) {
            vec.push(feature);
        } else {
            let mut vec = Vec::new();
            vec.push(feature);
            self.features.insert(name, vec);
        }
    }
}
