use crate::prelude::*;
use crate::platform::ggp::*;
use super::vulkan_core::*;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPresentFrameTokenGGP {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub frameToken: GgpFrameToken,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkStreamDescriptorSurfaceCreateInfoGGP {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkStreamDescriptorSurfaceCreateFlagsGGP,
    pub streamDescriptor: GgpStreamDescriptor,
}

pub const VK_GGP_FRAME_TOKEN_EXTENSION_NAME: &CStr = c"VK_GGP_frame_token";
pub const VK_GGP_FRAME_TOKEN_SPEC_VERSION: u32 = 1;
pub const VK_GGP_STREAM_DESCRIPTOR_SURFACE_EXTENSION_NAME: &CStr = c"VK_GGP_stream_descriptor_surface";
pub const VK_GGP_STREAM_DESCRIPTOR_SURFACE_SPEC_VERSION: u32 = 1;
pub const VK_STRUCTURE_TYPE_PRESENT_FRAME_TOKEN_GGP: VkStructureType = 1000191000;
pub const VK_STRUCTURE_TYPE_STREAM_DESCRIPTOR_SURFACE_CREATE_INFO_GGP: VkStructureType = 1000049000;

unsafe extern "system" {
    /// Available if built with `prototypes`.
    #[cfg(any(doc, feature = "prototypes"))]
    pub fn vkCreateStreamDescriptorSurfaceGGP(instance: VkInstance, pCreateInfo: *const VkStreamDescriptorSurfaceCreateInfoGGP, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult;
}

pub type NonNullPFN_vkCreateStreamDescriptorSurfaceGGP = unsafe extern "system" fn(instance: VkInstance, pCreateInfo: *const VkStreamDescriptorSurfaceCreateInfoGGP, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult;
pub type PFN_vkCreateStreamDescriptorSurfaceGGP = Option<NonNullPFN_vkCreateStreamDescriptorSurfaceGGP>;
pub type VkStreamDescriptorSurfaceCreateFlagsGGP = VkFlags;
