use std::collections::HashSet;

use vulkan_registry::*;

use crate::code::*;

pub(crate) struct Generator<'a> {
    index: &'a RegistryIndex<'a>,
    require_names: HashSet<&'a str>,
}

impl<'a> Generator<'a> {
    pub(crate) fn generate() {
        let registry = Registry::vk();
        let index = RegistryIndex::new(&registry);
        let mut generator = Generator {
            index: &index,
            require_names: HashSet::new(),
        };

        for features in index.features.values() {
            for &feature in features {
                generator.add_feature(feature);
            }
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
                self.add_type(typ);
            }
        }
    }

    fn add_type(&mut self, typ: &'a Type) {
        if !Self::api_matches_vulkan(&typ.api) {
            return;
        }

        if typ.alias.is_some() {
            self.add_type_alias(typ);
            return;
        }

        match typ.category.as_ref().map(String::as_str) {
            Some("basetype") => self.add_other_type(typ),
            Some("bitmask") => self.add_other_type(typ),
            Some("define") => (),
            Some("enum") => self.add_enum_type(typ),
            Some("funcpointer") => self.add_funcpointer_type(typ),
            Some("handle") => self.add_handle_type(typ),
            Some("include") => (),
            Some("struct") => self.add_struct_or_union_type(typ),
            Some("union") => self.add_struct_or_union_type(typ),
            Some(category) => panic!("unexpected type category: {category:?}"),
            None => self.add_other_type(typ),
        }
    }

    fn add_type_alias(&mut self, _: &'a Type) {
        // TODO
    }

    fn add_enum_type(&mut self, _: &'a Type) {
        // TODO
    }

    fn add_funcpointer_type(&mut self, _: &'a Type) {
        // TODO
    }

    fn add_handle_type(&mut self, _: &'a Type) {
        // TODO
    }

    fn add_struct_or_union_type(&mut self, _: &'a Type) {
        // TODO
    }

    fn add_other_type(&mut self, _: &'a Type) {
        // TODO
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
