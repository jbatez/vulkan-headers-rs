use crate::code::*;

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

pub type VkScreenSurfaceCreateFlagsQNX = VkFlags;
