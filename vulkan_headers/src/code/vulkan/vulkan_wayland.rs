use crate::code::*;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkWaylandSurfaceCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkWaylandSurfaceCreateFlagsKHR,
    pub display: *mut wl_display,
    pub surface: *mut wl_surface,
}

pub type VkWaylandSurfaceCreateFlagsKHR = VkFlags;
