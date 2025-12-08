use crate::prelude::*;
use crate::platform::screen::*;
use super::vulkan_core::*;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkExternalFormatQNX {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub externalFormat: u64,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImportScreenBufferInfoQNX {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub buffer: *mut _screen_buffer,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceExternalMemoryScreenBufferFeaturesQNX {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub screenBufferImport: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkScreenBufferFormatPropertiesQNX {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub format: VkFormat,
    pub externalFormat: u64,
    pub screenUsage: u64,
    pub formatFeatures: VkFormatFeatureFlags,
    pub samplerYcbcrConversionComponents: VkComponentMapping,
    pub suggestedYcbcrModel: VkSamplerYcbcrModelConversion,
    pub suggestedYcbcrRange: VkSamplerYcbcrRange,
    pub suggestedXChromaOffset: VkChromaLocation,
    pub suggestedYChromaOffset: VkChromaLocation,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkScreenBufferPropertiesQNX {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub allocationSize: VkDeviceSize,
    pub memoryTypeBits: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkScreenSurfaceCreateInfoQNX {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkScreenSurfaceCreateFlagsQNX,
    pub context: *mut _screen_context,
    pub window: *mut _screen_window,
}

pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_SCREEN_BUFFER_BIT_QNX: VkExternalMemoryHandleTypeFlagBits = 1 << 14;
pub const VK_QNX_EXTERNAL_MEMORY_SCREEN_BUFFER_EXTENSION_NAME: &CStr = c"VK_QNX_external_memory_screen_buffer";
pub const VK_QNX_EXTERNAL_MEMORY_SCREEN_BUFFER_SPEC_VERSION: u32 = 1;
pub const VK_QNX_SCREEN_SURFACE_EXTENSION_NAME: &CStr = c"VK_QNX_screen_surface";
pub const VK_QNX_SCREEN_SURFACE_SPEC_VERSION: u32 = 1;
pub const VK_STRUCTURE_TYPE_EXTERNAL_FORMAT_QNX: VkStructureType = 1000529003;
pub const VK_STRUCTURE_TYPE_IMPORT_SCREEN_BUFFER_INFO_QNX: VkStructureType = 1000529002;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_MEMORY_SCREEN_BUFFER_FEATURES_QNX: VkStructureType = 1000529004;
pub const VK_STRUCTURE_TYPE_SCREEN_BUFFER_FORMAT_PROPERTIES_QNX: VkStructureType = 1000529001;
pub const VK_STRUCTURE_TYPE_SCREEN_BUFFER_PROPERTIES_QNX: VkStructureType = 1000529000;
pub const VK_STRUCTURE_TYPE_SCREEN_SURFACE_CREATE_INFO_QNX: VkStructureType = 1000378000;

unsafe extern "system" {
    /// Available if built with `prototypes`.
    #[cfg(any(doc, feature = "prototypes"))]
    pub fn vkCreateScreenSurfaceQNX(instance: VkInstance, pCreateInfo: *const VkScreenSurfaceCreateInfoQNX, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult;

    /// Available if built with `prototypes`.
    #[cfg(any(doc, feature = "prototypes"))]
    pub fn vkGetPhysicalDeviceScreenPresentationSupportQNX(physicalDevice: VkPhysicalDevice, queueFamilyIndex: u32, window: *mut _screen_window) -> VkBool32;

    /// Available if built with `prototypes`.
    #[cfg(any(doc, feature = "prototypes"))]
    pub fn vkGetScreenBufferPropertiesQNX(device: VkDevice, buffer: *const _screen_buffer, pProperties: *mut VkScreenBufferPropertiesQNX) -> VkResult;
}

pub type NonNullPFN_vkCreateScreenSurfaceQNX = unsafe extern "system" fn(instance: VkInstance, pCreateInfo: *const VkScreenSurfaceCreateInfoQNX, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult;
pub type NonNullPFN_vkGetPhysicalDeviceScreenPresentationSupportQNX = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, queueFamilyIndex: u32, window: *mut _screen_window) -> VkBool32;
pub type NonNullPFN_vkGetScreenBufferPropertiesQNX = unsafe extern "system" fn(device: VkDevice, buffer: *const _screen_buffer, pProperties: *mut VkScreenBufferPropertiesQNX) -> VkResult;
pub type PFN_vkCreateScreenSurfaceQNX = Option<NonNullPFN_vkCreateScreenSurfaceQNX>;
pub type PFN_vkGetPhysicalDeviceScreenPresentationSupportQNX = Option<NonNullPFN_vkGetPhysicalDeviceScreenPresentationSupportQNX>;
pub type PFN_vkGetScreenBufferPropertiesQNX = Option<NonNullPFN_vkGetScreenBufferPropertiesQNX>;
pub type VkScreenSurfaceCreateFlagsQNX = VkFlags;
