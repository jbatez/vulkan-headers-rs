use std::collections::HashSet;

use vulkan_registry::*;

use mods::*;
mod mods {
    pub use registry_index::*;
    mod registry_index;
}

fn main() {
    Generator::generate();
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
