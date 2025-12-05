use std::{
    collections::HashSet,
    fs::File,
    ops::{Deref, DerefMut},
};

use vulkan_registry::*;

use crate::code::*;

pub(crate) struct Generator {
    generated_item_names: HashSet<String>,
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
        let mut generator = GeneratorWithIndex::new(self, &registry);
        generator.visit_video_registry(&registry);
    }

    fn generate_vulkan(&mut self) {
        let registry = Registry::vk();
        let mut generator = GeneratorWithIndex::new(self, &registry);
        generator.generate_vulkan_core(&registry);
        generator.generate_vulkan_platforms(&registry);
    }
}

struct GeneratorWithIndex<'a> {
    base: &'a mut Generator,
    index: RegistryIndex<'a>,
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
    fn new(base: &'a mut Generator, registry: &'a Registry) -> Self {
        Self {
            base,
            index: RegistryIndex::new("vulkan", registry),
        }
    }

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
        let mut module = Module::default();
        let mut generator = GeneratorWithIndexAndModule::new(self, &mut module);
        generator.visit_extension(extension);

        let name = extension.name.as_ref().unwrap();
        let path = format!("vulkan_headers/src/code/vk_video/{name}.rs");
        let mut file = File::create(path).unwrap();
        module.write_to_file(&mut file);
    }

    fn generate_vulkan_core(&mut self, registry: &'a Registry) {
        // TODO
    }

    fn generate_vulkan_platforms(&mut self, registry: &'a Registry) {
        // TODO
    }
}

struct GeneratorWithIndexAndModule<'a, 'b> {
    base: &'b mut GeneratorWithIndex<'a>,
    module: &'b mut Module,
}

impl<'a, 'b> Deref for GeneratorWithIndexAndModule<'a, 'b> {
    type Target = GeneratorWithIndex<'a>;
    fn deref(&self) -> &Self::Target {
        self.base
    }
}

impl<'a, 'b> DerefMut for GeneratorWithIndexAndModule<'a, 'b> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.base
    }
}

impl<'a, 'b> GeneratorWithIndexAndModule<'a, 'b> {
    fn new(base: &'b mut GeneratorWithIndex<'a>, module: &'b mut Module) -> Self {
        Self { base, module }
    }

    fn visit_extension(&mut self, extension: &'a Extension) {
        // TODO
    }
}
