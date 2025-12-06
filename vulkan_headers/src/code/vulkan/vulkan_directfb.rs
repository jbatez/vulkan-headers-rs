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

pub type VkDirectFBSurfaceCreateFlagsEXT = VkFlags;
