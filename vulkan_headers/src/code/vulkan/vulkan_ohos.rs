use crate::code::*;

#[cfg_attr(not(doc), repr(u8))]
pub enum OHBufferHandle {
    #[doc(hidden)]
    __variant1,
    #[doc(hidden)]
    __variant2,
}

#[cfg_attr(not(doc), repr(u8))]
pub enum OH_NativeBuffer {
    #[doc(hidden)]
    __variant1,
    #[doc(hidden)]
    __variant2,
}

pub type OHNativeWindow = NativeWindow;
pub type VkSurfaceCreateFlagsOHOS = VkFlags;
pub type VkSwapchainImageUsageFlagsOHOS = VkFlags;
