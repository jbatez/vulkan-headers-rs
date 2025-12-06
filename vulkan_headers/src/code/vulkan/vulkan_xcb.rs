use crate::code::*;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkXcbSurfaceCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkXcbSurfaceCreateFlagsKHR,
    pub connection: *mut xcb_connection_t,
    pub window: xcb_window_t,
}

pub type VkXcbSurfaceCreateFlagsKHR = VkFlags;
