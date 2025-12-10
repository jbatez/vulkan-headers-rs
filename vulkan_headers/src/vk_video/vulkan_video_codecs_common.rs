use crate::prelude::*;

pub const fn VK_MAKE_VIDEO_STD_VERSION(major: u32, minor: u32, patch: u32) -> u32 {
    (major << 22) | (minor << 12) | patch
}
