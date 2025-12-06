use crate::code::*;

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
