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

pub const VK_EXPORT_METAL_OBJECT_TYPE_METAL_BUFFER_BIT_EXT: VkExportMetalObjectTypeFlagBitsEXT = 1 << 2;
pub const VK_EXPORT_METAL_OBJECT_TYPE_METAL_COMMAND_QUEUE_BIT_EXT: VkExportMetalObjectTypeFlagBitsEXT = 1 << 1;
pub const VK_EXPORT_METAL_OBJECT_TYPE_METAL_DEVICE_BIT_EXT: VkExportMetalObjectTypeFlagBitsEXT = 1 << 0;
pub const VK_EXPORT_METAL_OBJECT_TYPE_METAL_IOSURFACE_BIT_EXT: VkExportMetalObjectTypeFlagBitsEXT = 1 << 4;
pub const VK_EXPORT_METAL_OBJECT_TYPE_METAL_SHARED_EVENT_BIT_EXT: VkExportMetalObjectTypeFlagBitsEXT = 1 << 5;
pub const VK_EXPORT_METAL_OBJECT_TYPE_METAL_TEXTURE_BIT_EXT: VkExportMetalObjectTypeFlagBitsEXT = 1 << 3;
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_MTLBUFFER_BIT_EXT: VkExternalMemoryHandleTypeFlagBits = 1 << 16;
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_MTLHEAP_BIT_EXT: VkExternalMemoryHandleTypeFlagBits = 1 << 18;
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_MTLTEXTURE_BIT_EXT: VkExternalMemoryHandleTypeFlagBits = 1 << 17;
pub const VK_EXT_EXTERNAL_MEMORY_METAL_EXTENSION_NAME: &CStr = c"VK_EXT_external_memory_metal";
pub const VK_EXT_EXTERNAL_MEMORY_METAL_SPEC_VERSION: u32 = 1;
pub const VK_EXT_METAL_OBJECTS_EXTENSION_NAME: &CStr = c"VK_EXT_metal_objects";
pub const VK_EXT_METAL_OBJECTS_SPEC_VERSION: u32 = 2;
pub const VK_EXT_METAL_SURFACE_EXTENSION_NAME: &CStr = c"VK_EXT_metal_surface";
pub const VK_EXT_METAL_SURFACE_SPEC_VERSION: u32 = 1;
pub const VK_STRUCTURE_TYPE_EXPORT_METAL_BUFFER_INFO_EXT: VkStructureType = 1000311004;
pub const VK_STRUCTURE_TYPE_EXPORT_METAL_COMMAND_QUEUE_INFO_EXT: VkStructureType = 1000311003;
pub const VK_STRUCTURE_TYPE_EXPORT_METAL_DEVICE_INFO_EXT: VkStructureType = 1000311002;
pub const VK_STRUCTURE_TYPE_EXPORT_METAL_IO_SURFACE_INFO_EXT: VkStructureType = 1000311008;
pub const VK_STRUCTURE_TYPE_EXPORT_METAL_OBJECTS_INFO_EXT: VkStructureType = 1000311001;
pub const VK_STRUCTURE_TYPE_EXPORT_METAL_OBJECT_CREATE_INFO_EXT: VkStructureType = 1000311000;
pub const VK_STRUCTURE_TYPE_EXPORT_METAL_SHARED_EVENT_INFO_EXT: VkStructureType = 1000311010;
pub const VK_STRUCTURE_TYPE_EXPORT_METAL_TEXTURE_INFO_EXT: VkStructureType = 1000311006;
pub const VK_STRUCTURE_TYPE_IMPORT_MEMORY_METAL_HANDLE_INFO_EXT: VkStructureType = 1000602000;
pub const VK_STRUCTURE_TYPE_IMPORT_METAL_BUFFER_INFO_EXT: VkStructureType = 1000311005;
pub const VK_STRUCTURE_TYPE_IMPORT_METAL_IO_SURFACE_INFO_EXT: VkStructureType = 1000311009;
pub const VK_STRUCTURE_TYPE_IMPORT_METAL_SHARED_EVENT_INFO_EXT: VkStructureType = 1000311011;
pub const VK_STRUCTURE_TYPE_IMPORT_METAL_TEXTURE_INFO_EXT: VkStructureType = 1000311007;
pub const VK_STRUCTURE_TYPE_MEMORY_GET_METAL_HANDLE_INFO_EXT: VkStructureType = 1000602002;
pub const VK_STRUCTURE_TYPE_MEMORY_METAL_HANDLE_PROPERTIES_EXT: VkStructureType = 1000602001;
pub const VK_STRUCTURE_TYPE_METAL_SURFACE_CREATE_INFO_EXT: VkStructureType = 1000217000;

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
