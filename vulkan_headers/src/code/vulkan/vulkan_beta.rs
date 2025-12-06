use crate::code::*;

#[cfg_attr(not(doc), repr(u8))]
pub enum VkCudaFunctionNV_T {
    #[doc(hidden)]
    __variant1,
    #[doc(hidden)]
    __variant2,
}

#[cfg_attr(not(doc), repr(u8))]
pub enum VkCudaModuleNV_T {
    #[doc(hidden)]
    __variant1,
    #[doc(hidden)]
    __variant2,
}

pub type NonNullVkCudaFunctionNV = NonNull<VkCudaFunctionNV_T>;
pub type NonNullVkCudaModuleNV = NonNull<VkCudaModuleNV_T>;
pub type VkCompressedTriangleFormatAMDX = i32;
pub type VkCudaFunctionNV = *mut VkCudaFunctionNV_T;
pub type VkCudaModuleNV = *mut VkCudaModuleNV_T;
pub type VkDisplacementMicromapFormatNV = i32;
