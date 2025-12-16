mod c_decl;
mod generator;
mod library;
mod module;
mod registry_index;
mod rust_definitions;
mod rust_from_c;

fn main() {
    generator::Generator::generate();
}
