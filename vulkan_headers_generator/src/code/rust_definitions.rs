pub(crate) const VK_API_VERSION_MAJOR: &str = "\
#[inline]
pub const fn VK_API_VERSION_MAJOR(version: u32) -> u32 {
    (version >> 22) & 0x7F
}";

pub(crate) const VK_API_VERSION_MINOR: &str = "\
#[inline]
pub const fn VK_API_VERSION_MINOR(version: u32) -> u32 {
    (version >> 12) & 0x3FF
}";

pub(crate) const VK_API_VERSION_PATCH: &str = "\
#[inline]
pub const fn VK_API_VERSION_PATCH(version: u32) -> u32 {
    version & 0xFFF
}";

pub(crate) const VK_API_VERSION_VARIANT: &str = "\
#[inline]
pub const fn VK_API_VERSION_VARIANT(version: u32) -> u32 {
    version >> 29
}";

pub(crate) const VK_MAKE_API_VERSION: &str = "\
#[inline]
pub const fn VK_MAKE_API_VERSION(variant: u32, major: u32, minor: u32, patch: u32) -> u32 {
    (variant << 29) | (major << 22) | (minor << 12) | patch
}";

pub(crate) const VK_MAKE_VERSION: &str = "\
#[inline]
pub const fn VK_MAKE_VERSION(major: u32, minor: u32, patch: u32) -> u32 {
    (major << 22) | (minor << 12) | patch
}";

pub(crate) const VK_MAKE_VIDEO_STD_VERSION: &str = "\
#[inline]
pub const fn VK_MAKE_VIDEO_STD_VERSION(major: u32, minor: u32, patch: u32) -> u32 {
    (major << 22) | (minor << 12) | patch
}";

pub(crate) const VK_VERSION_MAJOR: &str = "\
#[inline]
pub const fn VK_VERSION_MAJOR(version: u32) -> u32 {
    version >> 22
}";

pub(crate) const VK_VERSION_MINOR: &str = "\
#[inline]
pub const fn VK_VERSION_MINOR(version: u32) -> u32 {
    (version >> 12) & 0x3FF
}";

pub(crate) const VK_VERSION_PATCH: &str = "\
#[inline]
pub const fn VK_VERSION_PATCH(version: u32) -> u32 {
    version & 0xFFF
}";
