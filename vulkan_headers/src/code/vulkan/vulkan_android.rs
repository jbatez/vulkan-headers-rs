use crate::code::*;

#[cfg_attr(not(doc), repr(u8))]
pub enum AHardwareBuffer {
    #[doc(hidden)]
    __variant1,
    #[doc(hidden)]
    __variant2,
}

#[cfg_attr(not(doc), repr(u8))]
pub enum ANativeWindow {
    #[doc(hidden)]
    __variant1,
    #[doc(hidden)]
    __variant2,
}

pub type VkAndroidSurfaceCreateFlagsKHR = VkFlags;
