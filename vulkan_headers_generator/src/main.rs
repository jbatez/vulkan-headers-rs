fn main() {
    todo!();
}

use code::*;
mod code {
    pub(crate) use module::*;
    mod module;

    pub(crate) use registry_index::*;
    mod registry_index;
}
