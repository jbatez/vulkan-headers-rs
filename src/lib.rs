pub use mods::*;
mod mods {
    mod parser;

    pub use types::*;
    mod types;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        Registry::vk();
    }
}
