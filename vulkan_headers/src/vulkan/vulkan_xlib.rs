use crate::prelude::*;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkXlibSurfaceCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkXlibSurfaceCreateFlagsKHR,
    pub dpy: *mut Display,
    pub window: Window,
}

pub const VK_KHR_XLIB_SURFACE_EXTENSION_NAME: &CStr = c"VK_KHR_xlib_surface";
pub const VK_KHR_XLIB_SURFACE_SPEC_VERSION: u32 = 6;
pub const VK_STRUCTURE_TYPE_XLIB_SURFACE_CREATE_INFO_KHR: VkStructureType = 1000004000;

unsafe extern "system" {
    /// Available if built with `prototypes`.
    #[cfg(any(doc, feature = "prototypes"))]
    pub fn vkCreateXlibSurfaceKHR(instance: VkInstance, pCreateInfo: *const VkXlibSurfaceCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult;

    /// Available if built with `prototypes`.
    #[cfg(any(doc, feature = "prototypes"))]
    pub fn vkGetPhysicalDeviceXlibPresentationSupportKHR(physicalDevice: VkPhysicalDevice, queueFamilyIndex: u32, dpy: *mut Display, visualID: VisualID) -> VkBool32;
}

pub type NonNullPFN_vkCreateXlibSurfaceKHR = unsafe extern "system" fn(instance: VkInstance, pCreateInfo: *const VkXlibSurfaceCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult;
pub type NonNullPFN_vkGetPhysicalDeviceXlibPresentationSupportKHR = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, queueFamilyIndex: u32, dpy: *mut Display, visualID: VisualID) -> VkBool32;
pub type PFN_vkCreateXlibSurfaceKHR = Option<NonNullPFN_vkCreateXlibSurfaceKHR>;
pub type PFN_vkGetPhysicalDeviceXlibPresentationSupportKHR = Option<NonNullPFN_vkGetPhysicalDeviceXlibPresentationSupportKHR>;
pub type VkXlibSurfaceCreateFlagsKHR = VkFlags;
