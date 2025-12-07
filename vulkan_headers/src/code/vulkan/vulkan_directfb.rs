use crate::code::*;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDirectFBSurfaceCreateInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkDirectFBSurfaceCreateFlagsEXT,
    pub dfb: *mut IDirectFB,
    pub surface: *mut IDirectFBSurface,
}

pub const VK_EXT_DIRECTFB_SURFACE_EXTENSION_NAME: &CStr = c"VK_EXT_directfb_surface";
pub const VK_EXT_DIRECTFB_SURFACE_SPEC_VERSION: u32 = 1;
pub const VK_STRUCTURE_TYPE_DIRECTFB_SURFACE_CREATE_INFO_EXT: VkStructureType = 1000346000;

unsafe extern "system" {
    /// Available if built with `prototypes`.
    #[cfg(any(doc, feature = "prototypes"))]
    pub fn vkCreateDirectFBSurfaceEXT(instance: VkInstance, pCreateInfo: *const VkDirectFBSurfaceCreateInfoEXT, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult;

    /// Available if built with `prototypes`.
    #[cfg(any(doc, feature = "prototypes"))]
    pub fn vkGetPhysicalDeviceDirectFBPresentationSupportEXT(physicalDevice: VkPhysicalDevice, queueFamilyIndex: u32, dfb: *mut IDirectFB) -> VkBool32;
}

pub type NonNullPFN_vkCreateDirectFBSurfaceEXT = unsafe extern "system" fn(instance: VkInstance, pCreateInfo: *const VkDirectFBSurfaceCreateInfoEXT, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult;
pub type NonNullPFN_vkGetPhysicalDeviceDirectFBPresentationSupportEXT = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, queueFamilyIndex: u32, dfb: *mut IDirectFB) -> VkBool32;
pub type PFN_vkCreateDirectFBSurfaceEXT = Option<NonNullPFN_vkCreateDirectFBSurfaceEXT>;
pub type PFN_vkGetPhysicalDeviceDirectFBPresentationSupportEXT = Option<NonNullPFN_vkGetPhysicalDeviceDirectFBPresentationSupportEXT>;
pub type VkDirectFBSurfaceCreateFlagsEXT = VkFlags;
