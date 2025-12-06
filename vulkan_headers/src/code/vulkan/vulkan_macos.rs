use crate::code::*;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkMacOSSurfaceCreateInfoMVK {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkMacOSSurfaceCreateFlagsMVK,
    pub pView: *const c_void,
}

pub type VkMacOSSurfaceCreateFlagsMVK = VkFlags;
