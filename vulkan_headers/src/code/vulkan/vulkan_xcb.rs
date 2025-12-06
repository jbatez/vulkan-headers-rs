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

pub const VK_KHR_XCB_SURFACE_EXTENSION_NAME: &CStr = c"VK_KHR_xcb_surface";
pub const VK_KHR_XCB_SURFACE_SPEC_VERSION: u32 = 6;
pub const VK_STRUCTURE_TYPE_XCB_SURFACE_CREATE_INFO_KHR: VkStructureType = 1000005000;

pub type VkXcbSurfaceCreateFlagsKHR = VkFlags;
