use vulkan_registry::*;

use crate::code::{Module, RegistryIndex};

pub(crate) struct Generator<'a> {
    index: &'a RegistryIndex<'a>,
    module: Module,
}

impl<'a> Generator<'a> {
    pub(crate) fn new(index: &'a RegistryIndex<'a>) -> Self {
        Self {
            index,
            module: Default::default(),
        }
    }

    pub(crate) fn visit_extension(&mut self, extension: &'a Extension) {
        for content in &extension.contents {
            if let ExtensionContent::Require(require) = content {
                self.visit_require(Some(extension), require);
            }
        }
    }

    fn visit_require(&mut self, extension: Option<&'a Extension>, require: &'a Require) {
        if self.index.api_matches(&require.api) {
            for content in &require.contents {
                match content {
                    RequireContent::Comment(_) => (),
                    RequireContent::Type(typ) => self.visit_require_type(typ),
                    RequireContent::Enum(enu) => self.visit_require_enum(extension, enu),
                    RequireContent::Command(command) => self.visit_require_command(command),
                    RequireContent::Feature(_) => (),
                }
            }
        }
    }

    fn visit_require_type(&mut self, _typ: &'a GeneralRef) {
        // TODO
    }

    fn visit_require_enum(&mut self, _extension: Option<&'a Extension>, _enu: &'a RequireEnum) {
        // TODO
    }

    fn visit_require_command(&mut self, _command: &'a GeneralRef) {
        // TODO
    }
}
