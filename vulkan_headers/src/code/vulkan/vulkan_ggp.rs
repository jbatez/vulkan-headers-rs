use crate::code::*;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPresentFrameTokenGGP {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub frameToken: GgpFrameToken,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkStreamDescriptorSurfaceCreateInfoGGP {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkStreamDescriptorSurfaceCreateFlagsGGP,
    pub streamDescriptor: GgpStreamDescriptor,
}

pub type VkStreamDescriptorSurfaceCreateFlagsGGP = VkFlags;
