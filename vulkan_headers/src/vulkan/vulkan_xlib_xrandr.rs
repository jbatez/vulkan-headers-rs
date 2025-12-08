use crate::prelude::*;
use crate::platform::xlib::*;
use crate::platform::xlib_xrandr::*;
use super::vulkan_core::*;

pub const VK_EXT_ACQUIRE_XLIB_DISPLAY_EXTENSION_NAME: &CStr = c"VK_EXT_acquire_xlib_display";
pub const VK_EXT_ACQUIRE_XLIB_DISPLAY_SPEC_VERSION: u32 = 1;

unsafe extern "system" {
    /// Available if built with `prototypes`.
    #[cfg(any(doc, feature = "prototypes"))]
    pub fn vkAcquireXlibDisplayEXT(physicalDevice: VkPhysicalDevice, dpy: *mut Display, display: VkDisplayKHR) -> VkResult;

    /// Available if built with `prototypes`.
    #[cfg(any(doc, feature = "prototypes"))]
    pub fn vkGetRandROutputDisplayEXT(physicalDevice: VkPhysicalDevice, dpy: *mut Display, rrOutput: RROutput, pDisplay: *mut VkDisplayKHR) -> VkResult;
}

pub type NonNullPFN_vkAcquireXlibDisplayEXT = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, dpy: *mut Display, display: VkDisplayKHR) -> VkResult;
pub type NonNullPFN_vkGetRandROutputDisplayEXT = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, dpy: *mut Display, rrOutput: RROutput, pDisplay: *mut VkDisplayKHR) -> VkResult;
pub type PFN_vkAcquireXlibDisplayEXT = Option<NonNullPFN_vkAcquireXlibDisplayEXT>;
pub type PFN_vkGetRandROutputDisplayEXT = Option<NonNullPFN_vkGetRandROutputDisplayEXT>;
