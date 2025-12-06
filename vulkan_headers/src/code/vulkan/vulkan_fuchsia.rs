use crate::code::*;

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
