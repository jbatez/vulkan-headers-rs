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

#[cfg_attr(not(doc), repr(u8))]
pub enum OHBufferHandle {
    #[doc(hidden)]
    __variant1,
    #[doc(hidden)]
    __variant2,
}

#[cfg_attr(not(doc), repr(u8))]
pub enum OH_NativeBuffer {
    #[doc(hidden)]
    __variant1,
    #[doc(hidden)]
    __variant2,
}

pub type OHNativeWindow = NativeWindow;
pub type VkSurfaceCreateFlagsOHOS = VkFlags;
pub type VkSwapchainImageUsageFlagBitsOHOS = VkFlags;
pub type VkSwapchainImageUsageFlagsOHOS = VkFlags;
