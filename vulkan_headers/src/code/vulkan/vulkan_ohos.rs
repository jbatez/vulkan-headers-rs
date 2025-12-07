use crate::code::*;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkExternalFormatOHOS {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub externalFormat: u64,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImportNativeBufferInfoOHOS {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub buffer: *mut OH_NativeBuffer,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkMemoryGetNativeBufferInfoOHOS {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub memory: VkDeviceMemory,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkNativeBufferFormatPropertiesOHOS {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub format: VkFormat,
    pub externalFormat: u64,
    pub formatFeatures: VkFormatFeatureFlags,
    pub samplerYcbcrConversionComponents: VkComponentMapping,
    pub suggestedYcbcrModel: VkSamplerYcbcrModelConversion,
    pub suggestedYcbcrRange: VkSamplerYcbcrRange,
    pub suggestedXChromaOffset: VkChromaLocation,
    pub suggestedYChromaOffset: VkChromaLocation,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkNativeBufferOHOS {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub handle: *mut OHBufferHandle,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkNativeBufferPropertiesOHOS {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub allocationSize: VkDeviceSize,
    pub memoryTypeBits: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkNativeBufferUsageOHOS {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub OHOSNativeBufferUsage: u64,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDevicePresentationPropertiesOHOS {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub sharedImage: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSurfaceCreateInfoOHOS {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkSurfaceCreateFlagsOHOS,
    pub window: *mut OHNativeWindow,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSwapchainImageCreateInfoOHOS {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub usage: VkSwapchainImageUsageFlagsOHOS,
}

pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_OH_NATIVE_BUFFER_BIT_OHOS: VkExternalMemoryHandleTypeFlagBits = 1 << 15;
pub const VK_OHOS_EXTERNAL_MEMORY_EXTENSION_NAME: &CStr = c"VK_OHOS_external_memory";
pub const VK_OHOS_EXTERNAL_MEMORY_SPEC_VERSION: u32 = 1;
pub const VK_OHOS_NATIVE_BUFFER_EXTENSION_NAME: &CStr = c"VK_OHOS_native_buffer";
pub const VK_OHOS_NATIVE_BUFFER_SPEC_VERSION: u32 = 1;
pub const VK_OHOS_SURFACE_EXTENSION_NAME: &CStr = c"VK_OHOS_surface";
pub const VK_OHOS_SURFACE_SPEC_VERSION: u32 = 1;
pub const VK_STRUCTURE_TYPE_EXTERNAL_FORMAT_OHOS: VkStructureType = 1000452005;
pub const VK_STRUCTURE_TYPE_IMPORT_NATIVE_BUFFER_INFO_OHOS: VkStructureType = 1000452003;
pub const VK_STRUCTURE_TYPE_MEMORY_GET_NATIVE_BUFFER_INFO_OHOS: VkStructureType = 1000452004;
pub const VK_STRUCTURE_TYPE_NATIVE_BUFFER_FORMAT_PROPERTIES_OHOS: VkStructureType = 1000452002;
pub const VK_STRUCTURE_TYPE_NATIVE_BUFFER_OHOS: VkStructureType = 1000453001;
pub const VK_STRUCTURE_TYPE_NATIVE_BUFFER_PROPERTIES_OHOS: VkStructureType = 1000452001;
pub const VK_STRUCTURE_TYPE_NATIVE_BUFFER_USAGE_OHOS: VkStructureType = 1000452000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PRESENTATION_PROPERTIES_OHOS: VkStructureType = 1000453003;
pub const VK_STRUCTURE_TYPE_SURFACE_CREATE_INFO_OHOS: VkStructureType = 1000685000;
pub const VK_STRUCTURE_TYPE_SWAPCHAIN_IMAGE_CREATE_INFO_OHOS: VkStructureType = 1000453002;
pub const VK_SWAPCHAIN_IMAGE_USAGE_SHARED_BIT_OHOS: VkSwapchainImageUsageFlagBitsOHOS = 1 << 0;

pub type VkSurfaceCreateFlagsOHOS = VkFlags;
pub type VkSwapchainImageUsageFlagBitsOHOS = VkFlags;
pub type VkSwapchainImageUsageFlagsOHOS = VkFlags;
