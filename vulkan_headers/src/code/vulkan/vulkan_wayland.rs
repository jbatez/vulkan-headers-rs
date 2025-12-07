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

unsafe extern "system" {
    /// Available if built with `prototypes`.
    #[cfg(any(doc, feature = "prototypes"))]
    pub fn vkCreateWaylandSurfaceKHR(instance: VkInstance, pCreateInfo: *const VkWaylandSurfaceCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult;

    /// Available if built with `prototypes`.
    #[cfg(any(doc, feature = "prototypes"))]
    pub fn vkGetPhysicalDeviceWaylandPresentationSupportKHR(physicalDevice: VkPhysicalDevice, queueFamilyIndex: u32, display: *mut wl_display) -> VkBool32;
}

pub type NonNullPFN_vkCreateWaylandSurfaceKHR = unsafe extern "system" fn(instance: VkInstance, pCreateInfo: *const VkWaylandSurfaceCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult;
pub type NonNullPFN_vkGetPhysicalDeviceWaylandPresentationSupportKHR = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, queueFamilyIndex: u32, display: *mut wl_display) -> VkBool32;
pub type PFN_vkCreateWaylandSurfaceKHR = Option<NonNullPFN_vkCreateWaylandSurfaceKHR>;
pub type PFN_vkGetPhysicalDeviceWaylandPresentationSupportKHR = Option<NonNullPFN_vkGetPhysicalDeviceWaylandPresentationSupportKHR>;
pub type VkWaylandSurfaceCreateFlagsKHR = VkFlags;
