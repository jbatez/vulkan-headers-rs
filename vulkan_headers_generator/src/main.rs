use std::collections::{HashMap, HashSet};

use vulkan_registry::{
    Command, CommandContent, CommandsContent, Enums, Feature, FeatureContent, GeneralRef,
    ProtoContent, Registry, RegistryContent, RequireContent, RequireEnum, Type, TypeContent,
    TypesContent,
};

fn main() {
    Generator::generate();
}

struct RegistryIndex<'a> {
    types: HashMap<&'a str, Vec<&'a Type>>,
    enums: HashMap<&'a str, Vec<&'a Enums>>,
    commands: HashMap<&'a str, Vec<&'a Command>>,
    features: HashMap<&'a str, Vec<&'a Feature>>,
}

impl<'a> RegistryIndex<'a> {
    fn new(registry: &'a Registry) -> Self {
        let mut index = Self {
            types: HashMap::new(),
            enums: HashMap::new(),
            commands: HashMap::new(),
            features: HashMap::new(),
        };

        for registry_content in &registry.contents {
            match registry_content {
                RegistryContent::Types(types) => {
                    for types_content in &types.contents {
                        if let TypesContent::Type(typ) = types_content {
                            index.add_type(typ);
                        }
                    }
                }
                RegistryContent::Enums(enums) => {
                    index.add_enums(enums);
                }
                RegistryContent::Commands(commands) => {
                    for commands_content in &commands.contents {
                        let CommandsContent::Command(command) = commands_content;
                        index.add_command(command);
                    }
                }
                RegistryContent::Feature(feature) => {
                    index.add_feature(feature);
                }
                _ => (),
            }
        }

        index
    }

    fn add_type(&mut self, typ: &'a Type) {
        let name = 'name: {
            if let Some(name) = typ.name.as_ref() {
                break 'name name.as_str();
            }
            for type_content in &typ.contents {
                if let TypeContent::Name(name) = type_content {
                    break 'name name.as_str();
                }
            }
            panic!("unnamed type");
        };

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

        if let Some(vec) = self.enums.get_mut(name) {
            vec.push(enums);
        } else {
            let mut vec = Vec::new();
            vec.push(enums);
            self.enums.insert(name, vec);
        }
    }

    fn add_command(&mut self, command: &'a Command) {
        let name = 'name: {
            if let Some(name) = command.name.as_ref() {
                break 'name name.as_str();
            }
            for command_content in &command.contents {
                if let CommandContent::Proto(proto) = command_content {
                    for proto_content in &proto.contents {
                        if let ProtoContent::Name(name) = proto_content {
                            break 'name name.as_str();
                        }
                    }
                }
            }
            panic!("unnamed command");
        };

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

struct Generator<'a> {
    index: &'a RegistryIndex<'a>,
    type_names: HashSet<&'a str>,
    enum_names: HashSet<&'a str>,
    command_names: HashSet<&'a str>,
}

impl<'a> Generator<'a> {
    fn generate() {
        let registry = Registry::vk();
        let index = RegistryIndex::new(&registry);
        let mut generator = Generator {
            index: &index,
            type_names: HashSet::new(),
            enum_names: HashSet::new(),
            command_names: HashSet::new(),
        };

        for features in index.features.values() {
            for &feature in features {
                generator.include_feature(feature);
            }
        }
    }

    fn api_includes_vulkan(api: &'a Option<String>) -> bool {
        if let Some(api) = api.as_ref() {
            api.split(',').find(|&s| s == "vulkan").is_some()
        } else {
            true
        }
    }

    fn include_feature(&mut self, feature: &'a Feature) {
        if !Self::api_includes_vulkan(&feature.api) {
            return;
        }

        for feature_content in &feature.contents {
            if let FeatureContent::Require(require) = feature_content {
                for require_content in &require.contents {
                    match require_content {
                        RequireContent::Comment(_) => {
                            ();
                        }
                        RequireContent::Type(type_ref) => {
                            self.include_type(type_ref);
                        }
                        RequireContent::Enum(require_enum) => {
                            self.include_enum(require_enum);
                        }
                        RequireContent::Command(command_ref) => {
                            self.include_command(command_ref);
                        }
                        RequireContent::Feature(_) => {
                            ();
                        }
                    }
                }
            }
        }
    }

    fn include_type(&mut self, type_ref: &'a GeneralRef) {
        let name = type_ref.name.as_ref().unwrap().as_str();
        if !self.type_names.insert(name) {
            return;
        }

        let typ = 'typ: {
            for &typ in &self.index.types[name] {
                if Self::api_includes_vulkan(&typ.api) {
                    break 'typ typ;
                }
            }
            panic!("unknown type");
        };

        // TODO
    }

    fn include_enum(&mut self, require_enum: &'a RequireEnum) {
        let name = require_enum.name.as_ref().unwrap().as_str();
        if !self.enum_names.insert(name) {
            return;
        }

        // TODO
    }

    fn include_command(&mut self, command_ref: &'a GeneralRef) {
        let name = command_ref.name.as_ref().unwrap().as_str();
        if !self.command_names.insert(name) {
            return;
        }

        // TODO
    }
}
