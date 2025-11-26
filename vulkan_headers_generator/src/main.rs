fn main() {
    Generator::generate()
}

use code::*;
mod code {
    pub(crate) use generator::*;
    mod generator;

    pub(crate) use registry_index::*;
    mod registry_index;
}
