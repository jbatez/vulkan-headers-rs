pub use mods::*;
mod mods {
    mod parser;

    pub use registry::*;
    mod registry;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        Registry::vk();
    }
}
