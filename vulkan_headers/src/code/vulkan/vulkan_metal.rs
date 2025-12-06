use crate::code::*;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkExportMetalBufferInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub memory: VkDeviceMemory,
    pub mtlBuffer: MTLBuffer_id,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkExportMetalCommandQueueInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub queue: VkQueue,
    pub mtlCommandQueue: MTLCommandQueue_id,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkExportMetalDeviceInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub mtlDevice: MTLDevice_id,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkExportMetalIOSurfaceInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub image: VkImage,
    pub ioSurface: IOSurfaceRef,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkExportMetalObjectCreateInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub exportObjectType: VkExportMetalObjectTypeFlagBitsEXT,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkExportMetalObjectsInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkExportMetalSharedEventInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub semaphore: VkSemaphore,
    pub event: VkEvent,
    pub mtlSharedEvent: MTLSharedEvent_id,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkExportMetalTextureInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub image: VkImage,
    pub imageView: VkImageView,
    pub bufferView: VkBufferView,
    pub plane: VkImageAspectFlagBits,
    pub mtlTexture: MTLTexture_id,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImportMemoryMetalHandleInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub handleType: VkExternalMemoryHandleTypeFlagBits,
    pub handle: *mut c_void,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImportMetalBufferInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub mtlBuffer: MTLBuffer_id,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImportMetalIOSurfaceInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub ioSurface: IOSurfaceRef,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImportMetalSharedEventInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub mtlSharedEvent: MTLSharedEvent_id,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImportMetalTextureInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub plane: VkImageAspectFlagBits,
    pub mtlTexture: MTLTexture_id,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkMemoryGetMetalHandleInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub memory: VkDeviceMemory,
    pub handleType: VkExternalMemoryHandleTypeFlagBits,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkMemoryMetalHandlePropertiesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub memoryTypeBits: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkMetalSurfaceCreateInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkMetalSurfaceCreateFlagsEXT,
    pub pLayer: *const CAMetalLayer,
}

pub type CAMetalLayer = c_void;
pub type IOSurfaceRef = *mut __IOSurface;
pub type MTLBuffer_id = *mut c_void;
pub type MTLCommandQueue_id = *mut c_void;
pub type MTLDevice_id = *mut c_void;
pub type MTLSharedEvent_id = *mut c_void;
pub type MTLTexture_id = *mut c_void;
pub type VkExportMetalObjectTypeFlagBitsEXT = VkFlags;
pub type VkExportMetalObjectTypeFlagsEXT = VkFlags;
pub type VkMetalSurfaceCreateFlagsEXT = VkFlags;
