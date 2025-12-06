use crate::code::*;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkMacOSSurfaceCreateInfoMVK {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkMacOSSurfaceCreateFlagsMVK,
    pub pView: *const c_void,
}

pub const VK_MVK_MACOS_SURFACE_EXTENSION_NAME: &CStr = c"VK_MVK_macos_surface";
pub const VK_MVK_MACOS_SURFACE_SPEC_VERSION: u32 = 3;
pub const VK_STRUCTURE_TYPE_MACOS_SURFACE_CREATE_INFO_MVK: VkStructureType = 1000123000;

pub type VkMacOSSurfaceCreateFlagsMVK = VkFlags;
