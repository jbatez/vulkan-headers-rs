use crate::prelude::*;
use super::vulkan_core::*;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkIOSSurfaceCreateInfoMVK {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkIOSSurfaceCreateFlagsMVK,
    pub pView: *const c_void,
}

pub const VK_MVK_IOS_SURFACE_EXTENSION_NAME: &CStr = c"VK_MVK_ios_surface";
pub const VK_MVK_IOS_SURFACE_SPEC_VERSION: u32 = 3;
pub const VK_STRUCTURE_TYPE_IOS_SURFACE_CREATE_INFO_MVK: VkStructureType = 1000122000;

unsafe extern "system" {
    /// Available if built with `prototypes`.
    #[cfg(any(doc, feature = "prototypes"))]
    pub fn vkCreateIOSSurfaceMVK(instance: VkInstance, pCreateInfo: *const VkIOSSurfaceCreateInfoMVK, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult;
}

pub type NonNullPFN_vkCreateIOSSurfaceMVK = unsafe extern "system" fn(instance: VkInstance, pCreateInfo: *const VkIOSSurfaceCreateInfoMVK, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult;
pub type PFN_vkCreateIOSSurfaceMVK = Option<NonNullPFN_vkCreateIOSSurfaceMVK>;
pub type VkIOSSurfaceCreateFlagsMVK = VkFlags;
