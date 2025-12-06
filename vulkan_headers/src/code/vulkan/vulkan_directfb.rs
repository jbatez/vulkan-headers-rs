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

pub type VkDirectFBSurfaceCreateFlagsEXT = VkFlags;
