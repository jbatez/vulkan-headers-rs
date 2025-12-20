use crate::prelude::*;
use crate::platform::ohos::*;
use super::vulkan_core::*;

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
pub struct VkSurfaceCreateInfoOHOS {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkSurfaceCreateFlagsOHOS,
    pub window: *mut OHNativeWindow,
}

pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_OH_NATIVE_BUFFER_BIT_OHOS: VkExternalMemoryHandleTypeFlagBits = 1 << 15;
pub const VK_OHOS_EXTERNAL_MEMORY_EXTENSION_NAME: &CStr = c"VK_OHOS_external_memory";
pub const VK_OHOS_EXTERNAL_MEMORY_SPEC_VERSION: u32 = 1;
pub const VK_OHOS_SURFACE_EXTENSION_NAME: &CStr = c"VK_OHOS_surface";
pub const VK_OHOS_SURFACE_SPEC_VERSION: u32 = 1;
pub const VK_STRUCTURE_TYPE_EXTERNAL_FORMAT_OHOS: VkStructureType = 1000452005;
pub const VK_STRUCTURE_TYPE_IMPORT_NATIVE_BUFFER_INFO_OHOS: VkStructureType = 1000452003;
pub const VK_STRUCTURE_TYPE_MEMORY_GET_NATIVE_BUFFER_INFO_OHOS: VkStructureType = 1000452004;
pub const VK_STRUCTURE_TYPE_NATIVE_BUFFER_FORMAT_PROPERTIES_OHOS: VkStructureType = 1000452002;
pub const VK_STRUCTURE_TYPE_NATIVE_BUFFER_PROPERTIES_OHOS: VkStructureType = 1000452001;
pub const VK_STRUCTURE_TYPE_NATIVE_BUFFER_USAGE_OHOS: VkStructureType = 1000452000;
pub const VK_STRUCTURE_TYPE_SURFACE_CREATE_INFO_OHOS: VkStructureType = 1000685000;

unsafe extern "system" {
    /// Available if built with `prototypes`.
    #[cfg(any(doc, feature = "prototypes"))]
    pub fn vkCreateSurfaceOHOS(instance: VkInstance, pCreateInfo: *const VkSurfaceCreateInfoOHOS, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult;

    /// Available if built with `prototypes`.
    #[cfg(any(doc, feature = "prototypes"))]
    pub fn vkGetMemoryNativeBufferOHOS(device: VkDevice, pInfo: *const VkMemoryGetNativeBufferInfoOHOS, pBuffer: *mut *mut OH_NativeBuffer) -> VkResult;

    /// Available if built with `prototypes`.
    #[cfg(any(doc, feature = "prototypes"))]
    pub fn vkGetNativeBufferPropertiesOHOS(device: VkDevice, buffer: *const OH_NativeBuffer, pProperties: *mut VkNativeBufferPropertiesOHOS) -> VkResult;
}

pub type NonNullPFN_vkCreateSurfaceOHOS = unsafe extern "system" fn(instance: VkInstance, pCreateInfo: *const VkSurfaceCreateInfoOHOS, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult;
pub type NonNullPFN_vkGetMemoryNativeBufferOHOS = unsafe extern "system" fn(device: VkDevice, pInfo: *const VkMemoryGetNativeBufferInfoOHOS, pBuffer: *mut *mut OH_NativeBuffer) -> VkResult;
pub type NonNullPFN_vkGetNativeBufferPropertiesOHOS = unsafe extern "system" fn(device: VkDevice, buffer: *const OH_NativeBuffer, pProperties: *mut VkNativeBufferPropertiesOHOS) -> VkResult;
pub type PFN_vkCreateSurfaceOHOS = Option<NonNullPFN_vkCreateSurfaceOHOS>;
pub type PFN_vkGetMemoryNativeBufferOHOS = Option<NonNullPFN_vkGetMemoryNativeBufferOHOS>;
pub type PFN_vkGetNativeBufferPropertiesOHOS = Option<NonNullPFN_vkGetNativeBufferPropertiesOHOS>;
pub type VkSurfaceCreateFlagsOHOS = VkFlags;
