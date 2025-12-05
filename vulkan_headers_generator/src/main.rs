fn main() {
    Generator::generate();
}

use code::*;
mod code {
    pub(crate) use c_decl::*;
    mod c_decl;

    pub(crate) use generator::*;
    mod generator;

    pub(crate) use module::*;
    mod module;

    pub(crate) use registry_index::*;
    mod registry_index;
}
