use mods::*;
mod mods {
    pub(crate) use generator::*;
    mod generator;

    pub(crate) use registry_index::*;
    mod registry_index;
}

fn main() {
    Generator::generate();
}
