use std::{fs::File, io::Write};

pub(crate) struct Module {
    parent: String,
    name: String,
    pub(crate) structs: Vec<(String, String)>,
    pub(crate) enums: Vec<(String, String)>,
    pub(crate) constants: Vec<(String, String)>,
    pub(crate) functions: Vec<(String, String)>,
    pub(crate) extern_functions: Vec<(String, String)>,
    pub(crate) type_aliases: Vec<(String, String)>,
    pub(crate) unions: Vec<(String, String)>,
}

impl Module {
    pub(crate) fn new(parent: &str, name: &str) -> Self {
        Self {
            parent: parent.to_string(),
            name: name.to_string(),
            structs: Vec::new(),
            enums: Vec::new(),
            constants: Vec::new(),
            functions: Vec::new(),
            extern_functions: Vec::new(),
            type_aliases: Vec::new(),
            unions: Vec::new(),
        }
    }

    pub(crate) fn write_file(&mut self) {
        let path = format!("vulkan_headers/src/code/{}/{}.rs", self.parent, self.name);
        let mut file = File::create(path).unwrap();

        writeln!(file, "use crate::code::*;").unwrap();
        self.sort_and_write_structs(&mut file);
        self.sort_and_write_enums(&mut file);
        self.sort_and_write_constants(&mut file);
        self.sort_and_write_functions(&mut file);
        self.sort_and_write_extern_functions(&mut file);
        self.sort_and_write_type_aliases(&mut file);
        self.sort_and_write_unions(&mut file);
    }

    fn sort_and_write_structs(&mut self, file: &mut File) {
        self.structs.sort();
        for (_, text) in &self.structs {
            writeln!(file).unwrap();
            writeln!(file, "{text}").unwrap();
        }
    }

    fn sort_and_write_enums(&mut self, file: &mut File) {
        self.enums.sort();
        for (_, text) in &self.enums {
            writeln!(file).unwrap();
            writeln!(file, "{text}").unwrap();
        }
    }

    fn sort_and_write_constants(&mut self, file: &mut File) {
        if self.constants.is_empty() {
            return;
        }

        writeln!(file).unwrap();

        self.constants.sort();
        for (_, text) in &self.constants {
            writeln!(file, "{text}").unwrap();
        }
    }

    fn sort_and_write_functions(&mut self, file: &mut File) {
        self.functions.sort();
        for (_, text) in &self.functions {
            writeln!(file).unwrap();
            writeln!(file, "{text}").unwrap();
        }
    }

    fn sort_and_write_extern_functions(&mut self, file: &mut File) {
        if self.extern_functions.is_empty() {
            return;
        }

        writeln!(file).unwrap();
        writeln!(file, "unsafe extern \"system\" {{").unwrap();

        self.extern_functions.sort();
        for (i, (_, text)) in self.extern_functions.iter().enumerate() {
            if i > 0 {
                writeln!(file).unwrap();
            }
            writeln!(file, "{text}").unwrap();
        }

        writeln!(file, "}}").unwrap();
    }

    fn sort_and_write_type_aliases(&mut self, file: &mut File) {
        if self.type_aliases.is_empty() {
            return;
        }

        writeln!(file).unwrap();

        self.type_aliases.sort();
        for (_, text) in &self.type_aliases {
            writeln!(file, "{text}").unwrap();
        }
    }

    fn sort_and_write_unions(&mut self, file: &mut File) {
        self.unions.sort();
        for (_, text) in &self.unions {
            writeln!(file).unwrap();
            writeln!(file, "{text}").unwrap();
        }
    }
}
