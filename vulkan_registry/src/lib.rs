pub use code::*;
mod code {
    mod parser;

    pub use registry::*;
    mod registry;
}

mod data {
    pub(crate) const VIDEO_XML: &'static str = include_str!("data/video.xml");
    pub(crate) const VK_XML: &'static str = include_str!("data/vk.xml");
}
