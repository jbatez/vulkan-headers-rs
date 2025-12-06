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

pub const VK_KHR_WAYLAND_SURFACE_EXTENSION_NAME: &CStr = c"VK_KHR_wayland_surface";
pub const VK_KHR_WAYLAND_SURFACE_SPEC_VERSION: u32 = 6;
pub const VK_STRUCTURE_TYPE_WAYLAND_SURFACE_CREATE_INFO_KHR: VkStructureType = 1000006000;

pub type VkWaylandSurfaceCreateFlagsKHR = VkFlags;
