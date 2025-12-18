use std::{fs::File, io::Write};

pub(crate) struct Library {
    pub(crate) video_modules: Vec<String>,
    pub(crate) platforms: Vec<String>,
}

impl Library {
    pub(crate) fn new() -> Self {
        Self {
            video_modules: Vec::new(),
            platforms: Vec::new(),
        }
    }

    pub(crate) fn write_files(&mut self) {
        self.platforms.sort();
        self.write_cargo_toml();
        self.write_lib_rs();
        self.write_vulkan_rs();
    }

    fn write_cargo_toml(&mut self) {
        let mut file = File::create("vulkan-headers/Cargo.toml").unwrap();

        write!(
            file,
            "{}",
            "\
[package]
name = \"vulkan-headers\"
version.workspace = true
edition.workspace = true
description = \"Minimalist Rust FFI bindings for Vulkan\"
repository.workspace = true
license = \"MIT-0\"
keywords.workspace = true
categories.workspace = true

[features]
exported_prototypes = []
prototypes = [\"exported_prototypes\"]
"
        )
        .unwrap();

        writeln!(file).unwrap();
        for platform in &self.platforms {
            writeln!(file, "{platform}_extensions = []").unwrap();
        }
    }

    fn write_lib_rs(&mut self) {
        let mut file = File::create("vulkan-headers/src/lib.rs").unwrap();

        write!(
            file,
            "{}",
            "\
#![allow(nonstandard_style, unused_imports)]
#![doc = include_str!(\"../README.md\")]
#![no_std]

const _: () = assert!(cfg!(target_pointer_width = \"64\"));

pub mod platform;

mod prelude;
"
        )
        .unwrap();

        self.sort_and_write_video_modules(&mut file);
        self.write_vulkan_module(&mut file);
    }

    fn sort_and_write_video_modules(&mut self, file: &mut File) {
        writeln!(file).unwrap();
        writeln!(file, "pub mod vk_video {{").unwrap();

        self.video_modules.sort();
        for name in &self.video_modules {
            writeln!(file, "    pub mod {name};").unwrap();
        }

        writeln!(file, "}}").unwrap();
    }

    fn write_vulkan_module(&mut self, file: &mut File) {
        writeln!(file).unwrap();
        writeln!(file, "pub mod vulkan {{").unwrap();
        writeln!(file, "    pub mod vulkan;").unwrap();
        writeln!(file, "    pub mod vulkan_core;").unwrap();

        for platform in &self.platforms {
            let doc_comment = format!("/// Available if built with `{platform}_extensions`.");
            let cfg_attr = format!("#[cfg(any(doc, feature = \"{platform}_extensions\"))]");
            let mod_decl = format!("pub mod vulkan_{platform};");

            writeln!(file).unwrap();
            writeln!(file, "    {doc_comment}").unwrap();
            writeln!(file, "    {cfg_attr}").unwrap();
            writeln!(file, "    {mod_decl}").unwrap();
        }

        writeln!(file, "}}").unwrap();
    }

    fn write_vulkan_rs(&mut self) {
        let mut file = File::create("vulkan-headers/src/vulkan/vulkan.rs").unwrap();

        writeln!(file, "#[doc(no_inline)]").unwrap();
        writeln!(file, "pub use super::vulkan_core::*;").unwrap();

        for platform in &self.platforms {
            let cfg_attr = format!("#[cfg(any(doc, feature = \"{platform}_extensions\"))]");
            let doc_attr = format!("#[doc(no_inline)]");
            let re_export = format!("pub use super::vulkan_{platform}::*;");

            writeln!(file).unwrap();
            writeln!(file, "{cfg_attr}").unwrap();
            writeln!(file, "{doc_attr}").unwrap();
            writeln!(file, "{re_export}").unwrap();
        }
    }
}
