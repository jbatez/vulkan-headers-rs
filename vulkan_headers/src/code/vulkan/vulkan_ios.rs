use crate::code::*;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkIOSSurfaceCreateInfoMVK {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkIOSSurfaceCreateFlagsMVK,
    pub pView: *const c_void,
}

pub type VkIOSSurfaceCreateFlagsMVK = VkFlags;
