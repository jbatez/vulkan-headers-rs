use crate::code::*;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkXlibSurfaceCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkXlibSurfaceCreateFlagsKHR,
    pub dpy: *mut Display,
    pub window: Window,
}

pub const VK_KHR_XLIB_SURFACE_EXTENSION_NAME: &CStr = c"VK_KHR_xlib_surface";
pub const VK_KHR_XLIB_SURFACE_SPEC_VERSION: u32 = 6;
pub const VK_STRUCTURE_TYPE_XLIB_SURFACE_CREATE_INFO_KHR: VkStructureType = 1000004000;

pub type VkXlibSurfaceCreateFlagsKHR = VkFlags;
