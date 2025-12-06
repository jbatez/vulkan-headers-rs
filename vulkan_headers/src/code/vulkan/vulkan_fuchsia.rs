use crate::code::*;

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

pub type NonNullVkBufferCollectionFUCHSIA = NonNull<VkBufferCollectionFUCHSIA_T>;
pub type VkBufferCollectionFUCHSIA = *mut VkBufferCollectionFUCHSIA_T;
pub type VkImageConstraintsInfoFlagBitsFUCHSIA = VkFlags;
pub type VkImageConstraintsInfoFlagsFUCHSIA = VkFlags;
pub type VkImageFormatConstraintsFlagsFUCHSIA = VkFlags;
pub type VkImagePipeSurfaceCreateFlagsFUCHSIA = VkFlags;
