use std::{
    collections::HashSet,
    ops::{Deref, DerefMut},
};

use vulkan_registry::*;

use crate::code::*;

pub(crate) struct Generator {
    pub(crate) generated_item_names: HashSet<String>,
}

impl Generator {
    pub(crate) fn generate() {
        let mut generator = Self {
            generated_item_names: HashSet::new(),
        };
        generator.generate_vk_video();
        generator.generate_vulkan();
    }

    fn generate_vk_video(&mut self) {
        let registry = Registry::video();
        let mut generator = GeneratorWithIndex {
            base: self,
            index: RegistryIndex::new("vulkan", &registry),
        };
        generator.visit_video_registry(&registry);
    }

    fn generate_vulkan(&mut self) {
        let registry = Registry::vk();
        let mut generator = GeneratorWithIndex {
            base: self,
            index: RegistryIndex::new("vulkan", &registry),
        };
        generator.generate_vulkan_core(&registry);
        generator.generate_vulkan_platforms(&registry);
    }
}

pub(crate) struct GeneratorWithIndex<'a> {
    base: &'a mut Generator,
    pub(crate) index: RegistryIndex<'a>,
}

impl<'a> Deref for GeneratorWithIndex<'a> {
    type Target = Generator;
    fn deref(&self) -> &Self::Target {
        self.base
    }
}

impl<'a> DerefMut for GeneratorWithIndex<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.base
    }
}

impl<'a> GeneratorWithIndex<'a> {
    fn visit_video_registry(&mut self, registry: &'a Registry) {
        for content in &registry.contents {
            if let RegistryContent::Extensions(extensions) = content {
                self.visit_video_extensions(extensions);
            }
        }
    }

    fn visit_video_extensions(&mut self, extensions: &'a Extensions) {
        for content in &extensions.contents {
            let ExtensionsContent::Extension(extension) = content;
            self.visit_video_extension(extension);
        }
    }

    fn visit_video_extension(&mut self, extension: &'a Extension) {
        // TODO
    }

    fn generate_vulkan_core(&mut self, registry: &'a Registry) {
        // TODO
    }

    fn generate_vulkan_platforms(&mut self, registry: &'a Registry) {
        // TODO
    }
}
