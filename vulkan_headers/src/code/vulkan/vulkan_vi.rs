use crate::code::*;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkViSurfaceCreateInfoNN {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkViSurfaceCreateFlagsNN,
    pub window: *mut c_void,
}

pub type VkViSurfaceCreateFlagsNN = VkFlags;
