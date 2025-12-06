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

pub type VkXlibSurfaceCreateFlagsKHR = VkFlags;
