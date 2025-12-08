use crate::prelude::*;
use crate::platform::fuchsia::*;
use super::vulkan_core::*;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkBufferCollectionBufferCreateInfoFUCHSIA {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub collection: VkBufferCollectionFUCHSIA,
    pub index: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkBufferCollectionConstraintsInfoFUCHSIA {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub minBufferCount: u32,
    pub maxBufferCount: u32,
    pub minBufferCountForCamping: u32,
    pub minBufferCountForDedicatedSlack: u32,
    pub minBufferCountForSharedSlack: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkBufferCollectionCreateInfoFUCHSIA {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub collectionToken: zx_handle_t,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkBufferCollectionImageCreateInfoFUCHSIA {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub collection: VkBufferCollectionFUCHSIA,
    pub index: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkBufferCollectionPropertiesFUCHSIA {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub memoryTypeBits: u32,
    pub bufferCount: u32,
    pub createInfoIndex: u32,
    pub sysmemPixelFormat: u64,
    pub formatFeatures: VkFormatFeatureFlags,
    pub sysmemColorSpaceIndex: VkSysmemColorSpaceFUCHSIA,
    pub samplerYcbcrConversionComponents: VkComponentMapping,
    pub suggestedYcbcrModel: VkSamplerYcbcrModelConversion,
    pub suggestedYcbcrRange: VkSamplerYcbcrRange,
    pub suggestedXChromaOffset: VkChromaLocation,
    pub suggestedYChromaOffset: VkChromaLocation,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkBufferConstraintsInfoFUCHSIA {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub createInfo: VkBufferCreateInfo,
    pub requiredFormatFeatures: VkFormatFeatureFlags,
    pub bufferCollectionConstraints: VkBufferCollectionConstraintsInfoFUCHSIA,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImageConstraintsInfoFUCHSIA {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub formatConstraintsCount: u32,
    pub pFormatConstraints: *const VkImageFormatConstraintsInfoFUCHSIA,
    pub bufferCollectionConstraints: VkBufferCollectionConstraintsInfoFUCHSIA,
    pub flags: VkImageConstraintsInfoFlagsFUCHSIA,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImageFormatConstraintsInfoFUCHSIA {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub imageCreateInfo: VkImageCreateInfo,
    pub requiredFormatFeatures: VkFormatFeatureFlags,
    pub flags: VkImageFormatConstraintsFlagsFUCHSIA,
    pub sysmemPixelFormat: u64,
    pub colorSpaceCount: u32,
    pub pColorSpaces: *const VkSysmemColorSpaceFUCHSIA,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImagePipeSurfaceCreateInfoFUCHSIA {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkImagePipeSurfaceCreateFlagsFUCHSIA,
    pub imagePipeHandle: zx_handle_t,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImportMemoryBufferCollectionFUCHSIA {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub collection: VkBufferCollectionFUCHSIA,
    pub index: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImportMemoryZirconHandleInfoFUCHSIA {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub handleType: VkExternalMemoryHandleTypeFlagBits,
    pub handle: zx_handle_t,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImportSemaphoreZirconHandleInfoFUCHSIA {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub semaphore: VkSemaphore,
    pub flags: VkSemaphoreImportFlags,
    pub handleType: VkExternalSemaphoreHandleTypeFlagBits,
    pub zirconHandle: zx_handle_t,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkMemoryGetZirconHandleInfoFUCHSIA {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub memory: VkDeviceMemory,
    pub handleType: VkExternalMemoryHandleTypeFlagBits,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkMemoryZirconHandlePropertiesFUCHSIA {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub memoryTypeBits: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSemaphoreGetZirconHandleInfoFUCHSIA {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub semaphore: VkSemaphore,
    pub handleType: VkExternalSemaphoreHandleTypeFlagBits,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSysmemColorSpaceFUCHSIA {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub colorSpace: u32,
}

#[cfg_attr(not(doc), repr(u8))]
pub enum VkBufferCollectionFUCHSIA_T {
    #[doc(hidden)]
    __variant1,
    #[doc(hidden)]
    __variant2,
}

pub const VK_DEBUG_REPORT_OBJECT_TYPE_BUFFER_COLLECTION_FUCHSIA_EXT: VkDebugReportObjectTypeEXT = 1000366000;
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_ZIRCON_VMO_BIT_FUCHSIA: VkExternalMemoryHandleTypeFlagBits = 1 << 11;
pub const VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_ZIRCON_EVENT_BIT_FUCHSIA: VkExternalSemaphoreHandleTypeFlagBits = 1 << 7;
pub const VK_FUCHSIA_BUFFER_COLLECTION_EXTENSION_NAME: &CStr = c"VK_FUCHSIA_buffer_collection";
pub const VK_FUCHSIA_BUFFER_COLLECTION_SPEC_VERSION: u32 = 2;
pub const VK_FUCHSIA_EXTERNAL_MEMORY_EXTENSION_NAME: &CStr = c"VK_FUCHSIA_external_memory";
pub const VK_FUCHSIA_EXTERNAL_MEMORY_SPEC_VERSION: u32 = 1;
pub const VK_FUCHSIA_EXTERNAL_SEMAPHORE_EXTENSION_NAME: &CStr = c"VK_FUCHSIA_external_semaphore";
pub const VK_FUCHSIA_EXTERNAL_SEMAPHORE_SPEC_VERSION: u32 = 1;
pub const VK_FUCHSIA_IMAGEPIPE_SURFACE_EXTENSION_NAME: &CStr = c"VK_FUCHSIA_imagepipe_surface";
pub const VK_FUCHSIA_IMAGEPIPE_SURFACE_SPEC_VERSION: u32 = 1;
pub const VK_IMAGE_CONSTRAINTS_INFO_CPU_READ_OFTEN_FUCHSIA: VkImageConstraintsInfoFlagBitsFUCHSIA = 1 << 1;
pub const VK_IMAGE_CONSTRAINTS_INFO_CPU_READ_RARELY_FUCHSIA: VkImageConstraintsInfoFlagBitsFUCHSIA = 1 << 0;
pub const VK_IMAGE_CONSTRAINTS_INFO_CPU_WRITE_OFTEN_FUCHSIA: VkImageConstraintsInfoFlagBitsFUCHSIA = 1 << 3;
pub const VK_IMAGE_CONSTRAINTS_INFO_CPU_WRITE_RARELY_FUCHSIA: VkImageConstraintsInfoFlagBitsFUCHSIA = 1 << 2;
pub const VK_IMAGE_CONSTRAINTS_INFO_PROTECTED_OPTIONAL_FUCHSIA: VkImageConstraintsInfoFlagBitsFUCHSIA = 1 << 4;
pub const VK_OBJECT_TYPE_BUFFER_COLLECTION_FUCHSIA: VkObjectType = 1000366000;
pub const VK_STRUCTURE_TYPE_BUFFER_COLLECTION_BUFFER_CREATE_INFO_FUCHSIA: VkStructureType = 1000366005;
pub const VK_STRUCTURE_TYPE_BUFFER_COLLECTION_CONSTRAINTS_INFO_FUCHSIA: VkStructureType = 1000366009;
pub const VK_STRUCTURE_TYPE_BUFFER_COLLECTION_CREATE_INFO_FUCHSIA: VkStructureType = 1000366000;
pub const VK_STRUCTURE_TYPE_BUFFER_COLLECTION_IMAGE_CREATE_INFO_FUCHSIA: VkStructureType = 1000366002;
pub const VK_STRUCTURE_TYPE_BUFFER_COLLECTION_PROPERTIES_FUCHSIA: VkStructureType = 1000366003;
pub const VK_STRUCTURE_TYPE_BUFFER_CONSTRAINTS_INFO_FUCHSIA: VkStructureType = 1000366004;
pub const VK_STRUCTURE_TYPE_IMAGEPIPE_SURFACE_CREATE_INFO_FUCHSIA: VkStructureType = 1000214000;
pub const VK_STRUCTURE_TYPE_IMAGE_CONSTRAINTS_INFO_FUCHSIA: VkStructureType = 1000366006;
pub const VK_STRUCTURE_TYPE_IMAGE_FORMAT_CONSTRAINTS_INFO_FUCHSIA: VkStructureType = 1000366007;
pub const VK_STRUCTURE_TYPE_IMPORT_MEMORY_BUFFER_COLLECTION_FUCHSIA: VkStructureType = 1000366001;
pub const VK_STRUCTURE_TYPE_IMPORT_MEMORY_ZIRCON_HANDLE_INFO_FUCHSIA: VkStructureType = 1000364000;
pub const VK_STRUCTURE_TYPE_IMPORT_SEMAPHORE_ZIRCON_HANDLE_INFO_FUCHSIA: VkStructureType = 1000365000;
pub const VK_STRUCTURE_TYPE_MEMORY_GET_ZIRCON_HANDLE_INFO_FUCHSIA: VkStructureType = 1000364002;
pub const VK_STRUCTURE_TYPE_MEMORY_ZIRCON_HANDLE_PROPERTIES_FUCHSIA: VkStructureType = 1000364001;
pub const VK_STRUCTURE_TYPE_SEMAPHORE_GET_ZIRCON_HANDLE_INFO_FUCHSIA: VkStructureType = 1000365001;
pub const VK_STRUCTURE_TYPE_SYSMEM_COLOR_SPACE_FUCHSIA: VkStructureType = 1000366008;

unsafe extern "system" {
    /// Available if built with `prototypes`.
    #[cfg(any(doc, feature = "prototypes"))]
    pub fn vkCreateBufferCollectionFUCHSIA(device: VkDevice, pCreateInfo: *const VkBufferCollectionCreateInfoFUCHSIA, pAllocator: *const VkAllocationCallbacks, pCollection: *mut VkBufferCollectionFUCHSIA) -> VkResult;

    /// Available if built with `prototypes`.
    #[cfg(any(doc, feature = "prototypes"))]
    pub fn vkCreateImagePipeSurfaceFUCHSIA(instance: VkInstance, pCreateInfo: *const VkImagePipeSurfaceCreateInfoFUCHSIA, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult;

    /// Available if built with `prototypes`.
    #[cfg(any(doc, feature = "prototypes"))]
    pub fn vkDestroyBufferCollectionFUCHSIA(device: VkDevice, collection: VkBufferCollectionFUCHSIA, pAllocator: *const VkAllocationCallbacks);

    /// Available if built with `prototypes`.
    #[cfg(any(doc, feature = "prototypes"))]
    pub fn vkGetBufferCollectionPropertiesFUCHSIA(device: VkDevice, collection: VkBufferCollectionFUCHSIA, pProperties: *mut VkBufferCollectionPropertiesFUCHSIA) -> VkResult;

    /// Available if built with `prototypes`.
    #[cfg(any(doc, feature = "prototypes"))]
    pub fn vkGetMemoryZirconHandleFUCHSIA(device: VkDevice, pGetZirconHandleInfo: *const VkMemoryGetZirconHandleInfoFUCHSIA, pZirconHandle: *mut zx_handle_t) -> VkResult;

    /// Available if built with `prototypes`.
    #[cfg(any(doc, feature = "prototypes"))]
    pub fn vkGetMemoryZirconHandlePropertiesFUCHSIA(device: VkDevice, handleType: VkExternalMemoryHandleTypeFlagBits, zirconHandle: zx_handle_t, pMemoryZirconHandleProperties: *mut VkMemoryZirconHandlePropertiesFUCHSIA) -> VkResult;

    /// Available if built with `prototypes`.
    #[cfg(any(doc, feature = "prototypes"))]
    pub fn vkGetSemaphoreZirconHandleFUCHSIA(device: VkDevice, pGetZirconHandleInfo: *const VkSemaphoreGetZirconHandleInfoFUCHSIA, pZirconHandle: *mut zx_handle_t) -> VkResult;

    /// Available if built with `prototypes`.
    #[cfg(any(doc, feature = "prototypes"))]
    pub fn vkImportSemaphoreZirconHandleFUCHSIA(device: VkDevice, pImportSemaphoreZirconHandleInfo: *const VkImportSemaphoreZirconHandleInfoFUCHSIA) -> VkResult;

    /// Available if built with `prototypes`.
    #[cfg(any(doc, feature = "prototypes"))]
    pub fn vkSetBufferCollectionBufferConstraintsFUCHSIA(device: VkDevice, collection: VkBufferCollectionFUCHSIA, pBufferConstraintsInfo: *const VkBufferConstraintsInfoFUCHSIA) -> VkResult;

    /// Available if built with `prototypes`.
    #[cfg(any(doc, feature = "prototypes"))]
    pub fn vkSetBufferCollectionImageConstraintsFUCHSIA(device: VkDevice, collection: VkBufferCollectionFUCHSIA, pImageConstraintsInfo: *const VkImageConstraintsInfoFUCHSIA) -> VkResult;
}

pub type NonNullPFN_vkCreateBufferCollectionFUCHSIA = unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkBufferCollectionCreateInfoFUCHSIA, pAllocator: *const VkAllocationCallbacks, pCollection: *mut VkBufferCollectionFUCHSIA) -> VkResult;
pub type NonNullPFN_vkCreateImagePipeSurfaceFUCHSIA = unsafe extern "system" fn(instance: VkInstance, pCreateInfo: *const VkImagePipeSurfaceCreateInfoFUCHSIA, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult;
pub type NonNullPFN_vkDestroyBufferCollectionFUCHSIA = unsafe extern "system" fn(device: VkDevice, collection: VkBufferCollectionFUCHSIA, pAllocator: *const VkAllocationCallbacks);
pub type NonNullPFN_vkGetBufferCollectionPropertiesFUCHSIA = unsafe extern "system" fn(device: VkDevice, collection: VkBufferCollectionFUCHSIA, pProperties: *mut VkBufferCollectionPropertiesFUCHSIA) -> VkResult;
pub type NonNullPFN_vkGetMemoryZirconHandleFUCHSIA = unsafe extern "system" fn(device: VkDevice, pGetZirconHandleInfo: *const VkMemoryGetZirconHandleInfoFUCHSIA, pZirconHandle: *mut zx_handle_t) -> VkResult;
pub type NonNullPFN_vkGetMemoryZirconHandlePropertiesFUCHSIA = unsafe extern "system" fn(device: VkDevice, handleType: VkExternalMemoryHandleTypeFlagBits, zirconHandle: zx_handle_t, pMemoryZirconHandleProperties: *mut VkMemoryZirconHandlePropertiesFUCHSIA) -> VkResult;
pub type NonNullPFN_vkGetSemaphoreZirconHandleFUCHSIA = unsafe extern "system" fn(device: VkDevice, pGetZirconHandleInfo: *const VkSemaphoreGetZirconHandleInfoFUCHSIA, pZirconHandle: *mut zx_handle_t) -> VkResult;
pub type NonNullPFN_vkImportSemaphoreZirconHandleFUCHSIA = unsafe extern "system" fn(device: VkDevice, pImportSemaphoreZirconHandleInfo: *const VkImportSemaphoreZirconHandleInfoFUCHSIA) -> VkResult;
pub type NonNullPFN_vkSetBufferCollectionBufferConstraintsFUCHSIA = unsafe extern "system" fn(device: VkDevice, collection: VkBufferCollectionFUCHSIA, pBufferConstraintsInfo: *const VkBufferConstraintsInfoFUCHSIA) -> VkResult;
pub type NonNullPFN_vkSetBufferCollectionImageConstraintsFUCHSIA = unsafe extern "system" fn(device: VkDevice, collection: VkBufferCollectionFUCHSIA, pImageConstraintsInfo: *const VkImageConstraintsInfoFUCHSIA) -> VkResult;
pub type NonNullVkBufferCollectionFUCHSIA = NonNull<VkBufferCollectionFUCHSIA_T>;
pub type PFN_vkCreateBufferCollectionFUCHSIA = Option<NonNullPFN_vkCreateBufferCollectionFUCHSIA>;
pub type PFN_vkCreateImagePipeSurfaceFUCHSIA = Option<NonNullPFN_vkCreateImagePipeSurfaceFUCHSIA>;
pub type PFN_vkDestroyBufferCollectionFUCHSIA = Option<NonNullPFN_vkDestroyBufferCollectionFUCHSIA>;
pub type PFN_vkGetBufferCollectionPropertiesFUCHSIA = Option<NonNullPFN_vkGetBufferCollectionPropertiesFUCHSIA>;
pub type PFN_vkGetMemoryZirconHandleFUCHSIA = Option<NonNullPFN_vkGetMemoryZirconHandleFUCHSIA>;
pub type PFN_vkGetMemoryZirconHandlePropertiesFUCHSIA = Option<NonNullPFN_vkGetMemoryZirconHandlePropertiesFUCHSIA>;
pub type PFN_vkGetSemaphoreZirconHandleFUCHSIA = Option<NonNullPFN_vkGetSemaphoreZirconHandleFUCHSIA>;
pub type PFN_vkImportSemaphoreZirconHandleFUCHSIA = Option<NonNullPFN_vkImportSemaphoreZirconHandleFUCHSIA>;
pub type PFN_vkSetBufferCollectionBufferConstraintsFUCHSIA = Option<NonNullPFN_vkSetBufferCollectionBufferConstraintsFUCHSIA>;
pub type PFN_vkSetBufferCollectionImageConstraintsFUCHSIA = Option<NonNullPFN_vkSetBufferCollectionImageConstraintsFUCHSIA>;
pub type VkBufferCollectionFUCHSIA = *mut VkBufferCollectionFUCHSIA_T;
pub type VkImageConstraintsInfoFlagBitsFUCHSIA = VkFlags;
pub type VkImageConstraintsInfoFlagsFUCHSIA = VkFlags;
pub type VkImageFormatConstraintsFlagsFUCHSIA = VkFlags;
pub type VkImagePipeSurfaceCreateFlagsFUCHSIA = VkFlags;
