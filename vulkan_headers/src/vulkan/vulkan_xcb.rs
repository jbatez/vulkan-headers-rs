use crate::prelude::*;
use crate::platform::xcb::*;
use super::vulkan_core::*;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkXcbSurfaceCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkXcbSurfaceCreateFlagsKHR,
    pub connection: *mut xcb_connection_t,
    pub window: xcb_window_t,
}

pub const VK_KHR_XCB_SURFACE_EXTENSION_NAME: &CStr = c"VK_KHR_xcb_surface";
pub const VK_KHR_XCB_SURFACE_SPEC_VERSION: u32 = 6;
pub const VK_STRUCTURE_TYPE_XCB_SURFACE_CREATE_INFO_KHR: VkStructureType = 1000005000;

unsafe extern "system" {
    /// Available if built with `prototypes`.
    #[cfg(any(doc, feature = "prototypes"))]
    pub fn vkCreateXcbSurfaceKHR(instance: VkInstance, pCreateInfo: *const VkXcbSurfaceCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult;

    /// Available if built with `prototypes`.
    #[cfg(any(doc, feature = "prototypes"))]
    pub fn vkGetPhysicalDeviceXcbPresentationSupportKHR(physicalDevice: VkPhysicalDevice, queueFamilyIndex: u32, connection: *mut xcb_connection_t, visual_id: xcb_visualid_t) -> VkBool32;
}

pub type NonNullPFN_vkCreateXcbSurfaceKHR = unsafe extern "system" fn(instance: VkInstance, pCreateInfo: *const VkXcbSurfaceCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult;
pub type NonNullPFN_vkGetPhysicalDeviceXcbPresentationSupportKHR = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, queueFamilyIndex: u32, connection: *mut xcb_connection_t, visual_id: xcb_visualid_t) -> VkBool32;
pub type PFN_vkCreateXcbSurfaceKHR = Option<NonNullPFN_vkCreateXcbSurfaceKHR>;
pub type PFN_vkGetPhysicalDeviceXcbPresentationSupportKHR = Option<NonNullPFN_vkGetPhysicalDeviceXcbPresentationSupportKHR>;
pub type VkXcbSurfaceCreateFlagsKHR = VkFlags;
