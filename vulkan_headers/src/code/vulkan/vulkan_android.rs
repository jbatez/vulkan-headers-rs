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

#[cfg_attr(not(doc), repr(u8))]
pub enum AHardwareBuffer {
    #[doc(hidden)]
    __variant1,
    #[doc(hidden)]
    __variant2,
}

#[cfg_attr(not(doc), repr(u8))]
pub enum ANativeWindow {
    #[doc(hidden)]
    __variant1,
    #[doc(hidden)]
    __variant2,
}

pub type VkAndroidSurfaceCreateFlagsKHR = VkFlags;
