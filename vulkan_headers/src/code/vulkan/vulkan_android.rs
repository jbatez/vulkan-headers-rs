use crate::code::*;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkAndroidHardwareBufferFormatProperties2ANDROID {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub format: VkFormat,
    pub externalFormat: u64,
    pub formatFeatures: VkFormatFeatureFlags2,
    pub samplerYcbcrConversionComponents: VkComponentMapping,
    pub suggestedYcbcrModel: VkSamplerYcbcrModelConversion,
    pub suggestedYcbcrRange: VkSamplerYcbcrRange,
    pub suggestedXChromaOffset: VkChromaLocation,
    pub suggestedYChromaOffset: VkChromaLocation,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkAndroidHardwareBufferFormatPropertiesANDROID {
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
pub struct VkAndroidHardwareBufferFormatResolvePropertiesANDROID {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub colorAttachmentFormat: VkFormat,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkAndroidHardwareBufferPropertiesANDROID {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub allocationSize: VkDeviceSize,
    pub memoryTypeBits: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkAndroidHardwareBufferUsageANDROID {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub androidHardwareBufferUsage: u64,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkAndroidSurfaceCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkAndroidSurfaceCreateFlagsKHR,
    pub window: *mut ANativeWindow,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkExternalFormatANDROID {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub externalFormat: u64,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImportAndroidHardwareBufferInfoANDROID {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub buffer: *mut AHardwareBuffer,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkMemoryGetAndroidHardwareBufferInfoANDROID {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub memory: VkDeviceMemory,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceExternalFormatResolveFeaturesANDROID {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub externalFormatResolve: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceExternalFormatResolvePropertiesANDROID {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub nullColorAttachmentWithExternalFormatResolve: VkBool32,
    pub externalFormatResolveChromaOffsetX: VkChromaLocation,
    pub externalFormatResolveChromaOffsetY: VkChromaLocation,
}

pub const VK_ANDROID_EXTERNAL_FORMAT_RESOLVE_EXTENSION_NAME: &CStr = c"VK_ANDROID_external_format_resolve";
pub const VK_ANDROID_EXTERNAL_FORMAT_RESOLVE_SPEC_VERSION: u32 = 1;
pub const VK_ANDROID_EXTERNAL_MEMORY_ANDROID_HARDWARE_BUFFER_EXTENSION_NAME: &CStr = c"VK_ANDROID_external_memory_android_hardware_buffer";
pub const VK_ANDROID_EXTERNAL_MEMORY_ANDROID_HARDWARE_BUFFER_SPEC_VERSION: u32 = 5;
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_ANDROID_HARDWARE_BUFFER_BIT_ANDROID: VkExternalMemoryHandleTypeFlagBits = 1 << 10;
pub const VK_KHR_ANDROID_SURFACE_EXTENSION_NAME: &CStr = c"VK_KHR_android_surface";
pub const VK_KHR_ANDROID_SURFACE_SPEC_VERSION: u32 = 6;
pub const VK_RESOLVE_MODE_EXTERNAL_FORMAT_DOWNSAMPLE_ANDROID: VkResolveModeFlagBits = VK_RESOLVE_MODE_EXTERNAL_FORMAT_DOWNSAMPLE_BIT_ANDROID;
pub const VK_RESOLVE_MODE_EXTERNAL_FORMAT_DOWNSAMPLE_BIT_ANDROID: VkResolveModeFlagBits = 1 << 4;
pub const VK_STRUCTURE_TYPE_ANDROID_HARDWARE_BUFFER_FORMAT_PROPERTIES_2_ANDROID: VkStructureType = 1000129006;
pub const VK_STRUCTURE_TYPE_ANDROID_HARDWARE_BUFFER_FORMAT_PROPERTIES_ANDROID: VkStructureType = 1000129002;
pub const VK_STRUCTURE_TYPE_ANDROID_HARDWARE_BUFFER_FORMAT_RESOLVE_PROPERTIES_ANDROID: VkStructureType = 1000468002;
pub const VK_STRUCTURE_TYPE_ANDROID_HARDWARE_BUFFER_PROPERTIES_ANDROID: VkStructureType = 1000129001;
pub const VK_STRUCTURE_TYPE_ANDROID_HARDWARE_BUFFER_USAGE_ANDROID: VkStructureType = 1000129000;
pub const VK_STRUCTURE_TYPE_ANDROID_SURFACE_CREATE_INFO_KHR: VkStructureType = 1000008000;
pub const VK_STRUCTURE_TYPE_EXTERNAL_FORMAT_ANDROID: VkStructureType = 1000129005;
pub const VK_STRUCTURE_TYPE_IMPORT_ANDROID_HARDWARE_BUFFER_INFO_ANDROID: VkStructureType = 1000129003;
pub const VK_STRUCTURE_TYPE_MEMORY_GET_ANDROID_HARDWARE_BUFFER_INFO_ANDROID: VkStructureType = 1000129004;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_FORMAT_RESOLVE_FEATURES_ANDROID: VkStructureType = 1000468000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_FORMAT_RESOLVE_PROPERTIES_ANDROID: VkStructureType = 1000468001;

pub type VkAndroidSurfaceCreateFlagsKHR = VkFlags;
