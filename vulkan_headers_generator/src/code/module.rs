use std::{fs::File, io::Write};

#[derive(Default)]
pub(crate) struct Module {
    pub(crate) structs: Vec<(String, String)>,
    pub(crate) enums: Vec<(String, String)>,
    pub(crate) constants: Vec<(String, String)>,
    pub(crate) functions: Vec<(String, String)>,
    pub(crate) type_aliases: Vec<(String, String)>,
    pub(crate) unions: Vec<(String, String)>,
}

impl Module {
    pub(crate) fn write_to_file(&mut self, file: &mut File) {
        writeln!(file, "use crate::code::*;").unwrap();
        self.sort_and_write_structs(file);
        self.sort_and_write_enums(file);
        self.sort_and_write_constants(file);
        self.sort_and_write_functions(file);
        self.sort_and_write_type_aliases(file);
        self.sort_and_write_unions(file);
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

        self.constants.sort();

        writeln!(file).unwrap();
        for (_, text) in &self.constants {
            writeln!(file, "{text}").unwrap();
        }
    }

    fn sort_and_write_functions(&mut self, file: &mut File) {
        if self.functions.is_empty() {
            return;
        }

        self.functions.sort();

        writeln!(file).unwrap();
        writeln!(file, "unsafe extern \"system\" {{").unwrap();
        for (i, (_, text)) in self.functions.iter().enumerate() {
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

        self.type_aliases.sort();

        writeln!(file).unwrap();
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
