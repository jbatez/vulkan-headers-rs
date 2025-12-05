use std::{collections::HashSet, fs::File, io::Write};

pub(crate) struct Library {
    pub(crate) vk_video_modules: Vec<String>,
    pub(crate) vulkan_modules: Vec<String>,
    pub(crate) items: HashSet<String>,
}

impl Library {
    pub(crate) fn new() -> Self {
        Self {
            vk_video_modules: Vec::new(),
            vulkan_modules: Vec::new(),
            items: HashSet::new(),
        }
    }

    pub(crate) fn write(&mut self) {
        let mut file = File::create("vulkan_headers/src/lib.rs").unwrap();

        writeln!(file, "pub use code::*;").unwrap();
        writeln!(file, "mod code {{").unwrap();
        writeln!(file, "    pub(crate) use vk_video::*;").unwrap();
        writeln!(file, "    pub mod vk_video {{").unwrap();

        Self::sort_and_write_modules(&mut self.vk_video_modules, &mut file);

        writeln!(file, "    }}").unwrap();
        writeln!(file).unwrap();
        writeln!(file, "    pub(crate) use vulkan::*;").unwrap();
        writeln!(file, "    pub mod vulkan {{").unwrap();

        Self::sort_and_write_modules(&mut self.vulkan_modules, &mut file);

        writeln!(file, "    }}").unwrap();
        writeln!(file, "}}").unwrap();
    }

    fn sort_and_write_modules(modules: &mut Vec<String>, file: &mut File) {
        modules.sort();

        for (i, name) in modules.iter().enumerate() {
            if i > 0 {
                writeln!(file).unwrap();
            }
            writeln!(file, "        pub(crate) use {name}::*;").unwrap();
            writeln!(file, "        pub mod {name};").unwrap();
        }
    }
}
