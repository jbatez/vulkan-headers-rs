use crate::code::*;

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

pub type VkIOSSurfaceCreateFlagsMVK = VkFlags;
