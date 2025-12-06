use crate::code::*;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkViSurfaceCreateInfoNN {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkViSurfaceCreateFlagsNN,
    pub window: *mut c_void,
}

pub const VK_NN_VI_SURFACE_EXTENSION_NAME: &CStr = c"VK_NN_vi_surface";
pub const VK_NN_VI_SURFACE_SPEC_VERSION: u32 = 1;
pub const VK_STRUCTURE_TYPE_VI_SURFACE_CREATE_INFO_NN: VkStructureType = 1000062000;

pub type VkViSurfaceCreateFlagsNN = VkFlags;
