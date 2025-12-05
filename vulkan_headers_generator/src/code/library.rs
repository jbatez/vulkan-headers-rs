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
    }

    fn write_cargo_toml(&mut self) {
        let mut file = File::create("vulkan_headers/Cargo.toml").unwrap();

        writeln!(file, "[package]").unwrap();
        writeln!(file, "name = \"vulkan_headers\"").unwrap();
        writeln!(file, "edition.workspace = true").unwrap();
        writeln!(file).unwrap();
        writeln!(file, "[features]").unwrap();
        writeln!(file, "exported_prototypes = []").unwrap();
        writeln!(file, "prototypes = [\"exported_prototypes\"]").unwrap();

        for platform in &self.platforms {
            writeln!(file, "{platform}_extensions = []").unwrap();
        }
    }

    fn write_lib_rs(&mut self) {
        let mut file = File::create("vulkan_headers/src/lib.rs").unwrap();

        writeln!(file, "pub use code::*;").unwrap();
        writeln!(file, "mod code {{").unwrap();

        self.sort_and_write_video_modules(&mut file);
        writeln!(file).unwrap();
        self.write_vulkan_module(&mut file);

        writeln!(file, "}}").unwrap();
    }

    fn sort_and_write_video_modules(&mut self, file: &mut File) {
        writeln!(file, "    pub(crate) use vk_video::*;").unwrap();
        writeln!(file, "    pub mod vk_video {{").unwrap();

        self.video_modules.sort();
        for (i, name) in self.video_modules.iter().enumerate() {
            if i > 0 {
                writeln!(file).unwrap();
            }
            writeln!(file, "        pub(crate) use {name}::*;").unwrap();
            writeln!(file, "        pub mod {name};").unwrap();
        }

        writeln!(file, "    }}").unwrap();
    }

    fn write_vulkan_module(&mut self, file: &mut File) {
        writeln!(file, "    pub(crate) use vulkan::*;").unwrap();
        writeln!(file, "    pub mod vulkan {{").unwrap();
        writeln!(file, "        pub mod vulkan_core;").unwrap();

        for platform in &self.platforms {
            writeln!(file).unwrap();
            writeln!(file, "        {}", cfg_platform(platform)).unwrap();
            writeln!(file, "        pub mod vulkan_{platform};").unwrap();
        }

        writeln!(file).unwrap();
        writeln!(file, "        pub(crate) use vulkan::*;").unwrap();
        writeln!(file, "        pub mod vulkan {{").unwrap();
        writeln!(file, "            pub use super::vulkan_core::*;").unwrap();

        for platform in &self.platforms {
            writeln!(file).unwrap();
            writeln!(file, "            {}", cfg_platform(platform)).unwrap();
            writeln!(file, "            pub user super::vulkan_{platform}::*;").unwrap();
        }

        writeln!(file, "        }}").unwrap();
        writeln!(file, "    }}").unwrap();
    }
}

fn cfg_platform(platform: &str) -> String {
    format!("#[cfg(any(doc, feature = \"{platform}_extensions\"))]")
}
