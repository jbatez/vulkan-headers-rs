#![allow(nonstandard_style)]
#![no_std]

use core::{ffi::{c_char, c_float, c_void}, ptr::NonNull};
        
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkAllocationCallbacks {
    pub pUserData: *mut c_void,
    pub pfnAllocation: PFN_vkAllocationFunction,
    pub pfnReallocation: PFN_vkReallocationFunction,
    pub pfnFree: PFN_vkFreeFunction,
    pub pfnInternalAllocation: PFN_vkInternalAllocationNotification,
    pub pfnInternalFree: PFN_vkInternalFreeNotification,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkApplicationInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub pApplicationName: *const c_char,
    pub applicationVersion: u32,
    pub pEngineName: *const c_char,
    pub engineVersion: u32,
    pub apiVersion: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkAttachmentDescription {
    pub flags: VkAttachmentDescriptionFlags,
    pub format: VkFormat,
    pub samples: VkSampleCountFlagBits,
    pub loadOp: VkAttachmentLoadOp,
    pub storeOp: VkAttachmentStoreOp,
    pub stencilLoadOp: VkAttachmentLoadOp,
    pub stencilStoreOp: VkAttachmentStoreOp,
    pub initialLayout: VkImageLayout,
    pub finalLayout: VkImageLayout,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkAttachmentDescription2 {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkAttachmentDescriptionFlags,
    pub format: VkFormat,
    pub samples: VkSampleCountFlagBits,
    pub loadOp: VkAttachmentLoadOp,
    pub storeOp: VkAttachmentStoreOp,
    pub stencilLoadOp: VkAttachmentLoadOp,
    pub stencilStoreOp: VkAttachmentStoreOp,
    pub initialLayout: VkImageLayout,
    pub finalLayout: VkImageLayout,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkAttachmentDescriptionStencilLayout {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub stencilInitialLayout: VkImageLayout,
    pub stencilFinalLayout: VkImageLayout,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkAttachmentReference {
    pub attachment: u32,
    pub layout: VkImageLayout,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkAttachmentReference2 {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub attachment: u32,
    pub layout: VkImageLayout,
    pub aspectMask: VkImageAspectFlags,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkAttachmentReferenceStencilLayout {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub stencilLayout: VkImageLayout,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkBaseInStructure {
    pub sType: VkStructureType,
    pub pNext: *const VkBaseInStructure,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkBaseOutStructure {
    pub sType: VkStructureType,
    pub pNext: *mut VkBaseOutStructure,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkBindBufferMemoryDeviceGroupInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub deviceIndexCount: u32,
    pub pDeviceIndices: *const u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkBindBufferMemoryInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub buffer: VkBuffer,
    pub memory: VkDeviceMemory,
    pub memoryOffset: VkDeviceSize,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkBindDescriptorSetsInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub stageFlags: VkShaderStageFlags,
    pub layout: VkPipelineLayout,
    pub firstSet: u32,
    pub descriptorSetCount: u32,
    pub pDescriptorSets: *const VkDescriptorSet,
    pub dynamicOffsetCount: u32,
    pub pDynamicOffsets: *const u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkBindImageMemoryDeviceGroupInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub deviceIndexCount: u32,
    pub pDeviceIndices: *const u32,
    pub splitInstanceBindRegionCount: u32,
    pub pSplitInstanceBindRegions: *const VkRect2D,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkBindImageMemoryInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub image: VkImage,
    pub memory: VkDeviceMemory,
    pub memoryOffset: VkDeviceSize,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkBindImagePlaneMemoryInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub planeAspect: VkImageAspectFlagBits,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkBindMemoryStatus {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub pResult: *mut VkResult,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkBindSparseInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub waitSemaphoreCount: u32,
    pub pWaitSemaphores: *const VkSemaphore,
    pub bufferBindCount: u32,
    pub pBufferBinds: *const VkSparseBufferMemoryBindInfo,
    pub imageOpaqueBindCount: u32,
    pub pImageOpaqueBinds: *const VkSparseImageOpaqueMemoryBindInfo,
    pub imageBindCount: u32,
    pub pImageBinds: *const VkSparseImageMemoryBindInfo,
    pub signalSemaphoreCount: u32,
    pub pSignalSemaphores: *const VkSemaphore,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkBlitImageInfo2 {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub srcImage: VkImage,
    pub srcImageLayout: VkImageLayout,
    pub dstImage: VkImage,
    pub dstImageLayout: VkImageLayout,
    pub regionCount: u32,
    pub pRegions: *const VkImageBlit2,
    pub filter: VkFilter,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkBufferCopy {
    pub srcOffset: VkDeviceSize,
    pub dstOffset: VkDeviceSize,
    pub size: VkDeviceSize,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkBufferCopy2 {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub srcOffset: VkDeviceSize,
    pub dstOffset: VkDeviceSize,
    pub size: VkDeviceSize,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkBufferCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkBufferCreateFlags,
    pub size: VkDeviceSize,
    pub usage: VkBufferUsageFlags,
    pub sharingMode: VkSharingMode,
    pub queueFamilyIndexCount: u32,
    pub pQueueFamilyIndices: *const u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkBufferDeviceAddressInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub buffer: VkBuffer,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkBufferImageCopy {
    pub bufferOffset: VkDeviceSize,
    pub bufferRowLength: u32,
    pub bufferImageHeight: u32,
    pub imageSubresource: VkImageSubresourceLayers,
    pub imageOffset: VkOffset3D,
    pub imageExtent: VkExtent3D,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkBufferImageCopy2 {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub bufferOffset: VkDeviceSize,
    pub bufferRowLength: u32,
    pub bufferImageHeight: u32,
    pub imageSubresource: VkImageSubresourceLayers,
    pub imageOffset: VkOffset3D,
    pub imageExtent: VkExtent3D,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkBufferMemoryBarrier {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub srcAccessMask: VkAccessFlags,
    pub dstAccessMask: VkAccessFlags,
    pub srcQueueFamilyIndex: u32,
    pub dstQueueFamilyIndex: u32,
    pub buffer: VkBuffer,
    pub offset: VkDeviceSize,
    pub size: VkDeviceSize,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkBufferMemoryBarrier2 {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub srcStageMask: VkPipelineStageFlags2,
    pub srcAccessMask: VkAccessFlags2,
    pub dstStageMask: VkPipelineStageFlags2,
    pub dstAccessMask: VkAccessFlags2,
    pub srcQueueFamilyIndex: u32,
    pub dstQueueFamilyIndex: u32,
    pub buffer: VkBuffer,
    pub offset: VkDeviceSize,
    pub size: VkDeviceSize,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkBufferMemoryRequirementsInfo2 {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub buffer: VkBuffer,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkBufferOpaqueCaptureAddressCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub opaqueCaptureAddress: u64,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkBufferUsageFlags2CreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub usage: VkBufferUsageFlags2,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkBufferViewCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkBufferViewCreateFlags,
    pub buffer: VkBuffer,
    pub format: VkFormat,
    pub offset: VkDeviceSize,
    pub range: VkDeviceSize,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkClearAttachment {
    pub aspectMask: VkImageAspectFlags,
    pub colorAttachment: u32,
    pub clearValue: VkClearValue,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkClearDepthStencilValue {
    pub depth: c_float,
    pub stencil: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkClearRect {
    pub rect: VkRect2D,
    pub baseArrayLayer: u32,
    pub layerCount: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkCommandBufferAllocateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub commandPool: VkCommandPool,
    pub level: VkCommandBufferLevel,
    pub commandBufferCount: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkCommandBufferBeginInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkCommandBufferUsageFlags,
    pub pInheritanceInfo: *const VkCommandBufferInheritanceInfo,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkCommandBufferInheritanceInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub renderPass: VkRenderPass,
    pub subpass: u32,
    pub framebuffer: VkFramebuffer,
    pub occlusionQueryEnable: VkBool32,
    pub queryFlags: VkQueryControlFlags,
    pub pipelineStatistics: VkQueryPipelineStatisticFlags,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkCommandBufferInheritanceRenderingInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkRenderingFlags,
    pub viewMask: u32,
    pub colorAttachmentCount: u32,
    pub pColorAttachmentFormats: *const VkFormat,
    pub depthAttachmentFormat: VkFormat,
    pub stencilAttachmentFormat: VkFormat,
    pub rasterizationSamples: VkSampleCountFlagBits,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkCommandBufferSubmitInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub commandBuffer: VkCommandBuffer,
    pub deviceMask: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkCommandPoolCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkCommandPoolCreateFlags,
    pub queueFamilyIndex: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkComponentMapping {
    pub r: VkComponentSwizzle,
    pub g: VkComponentSwizzle,
    pub b: VkComponentSwizzle,
    pub a: VkComponentSwizzle,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkComputePipelineCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineCreateFlags,
    pub stage: VkPipelineShaderStageCreateInfo,
    pub layout: VkPipelineLayout,
    pub basePipelineHandle: VkPipeline,
    pub basePipelineIndex: i32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkConformanceVersion {
    pub major: u8,
    pub minor: u8,
    pub subminor: u8,
    pub patch: u8,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkCopyBufferInfo2 {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub srcBuffer: VkBuffer,
    pub dstBuffer: VkBuffer,
    pub regionCount: u32,
    pub pRegions: *const VkBufferCopy2,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkCopyBufferToImageInfo2 {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub srcBuffer: VkBuffer,
    pub dstImage: VkImage,
    pub dstImageLayout: VkImageLayout,
    pub regionCount: u32,
    pub pRegions: *const VkBufferImageCopy2,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkCopyDescriptorSet {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub srcSet: VkDescriptorSet,
    pub srcBinding: u32,
    pub srcArrayElement: u32,
    pub dstSet: VkDescriptorSet,
    pub dstBinding: u32,
    pub dstArrayElement: u32,
    pub descriptorCount: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkCopyImageInfo2 {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub srcImage: VkImage,
    pub srcImageLayout: VkImageLayout,
    pub dstImage: VkImage,
    pub dstImageLayout: VkImageLayout,
    pub regionCount: u32,
    pub pRegions: *const VkImageCopy2,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkCopyImageToBufferInfo2 {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub srcImage: VkImage,
    pub srcImageLayout: VkImageLayout,
    pub dstBuffer: VkBuffer,
    pub regionCount: u32,
    pub pRegions: *const VkBufferImageCopy2,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkCopyImageToImageInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkHostImageCopyFlags,
    pub srcImage: VkImage,
    pub srcImageLayout: VkImageLayout,
    pub dstImage: VkImage,
    pub dstImageLayout: VkImageLayout,
    pub regionCount: u32,
    pub pRegions: *const VkImageCopy2,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkCopyImageToMemoryInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkHostImageCopyFlags,
    pub srcImage: VkImage,
    pub srcImageLayout: VkImageLayout,
    pub regionCount: u32,
    pub pRegions: *const VkImageToMemoryCopy,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkCopyMemoryToImageInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkHostImageCopyFlags,
    pub dstImage: VkImage,
    pub dstImageLayout: VkImageLayout,
    pub regionCount: u32,
    pub pRegions: *const VkMemoryToImageCopy,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDependencyInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub dependencyFlags: VkDependencyFlags,
    pub memoryBarrierCount: u32,
    pub pMemoryBarriers: *const VkMemoryBarrier2,
    pub bufferMemoryBarrierCount: u32,
    pub pBufferMemoryBarriers: *const VkBufferMemoryBarrier2,
    pub imageMemoryBarrierCount: u32,
    pub pImageMemoryBarriers: *const VkImageMemoryBarrier2,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDescriptorBufferInfo {
    pub buffer: VkBuffer,
    pub offset: VkDeviceSize,
    pub range: VkDeviceSize,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDescriptorImageInfo {
    pub sampler: VkSampler,
    pub imageView: VkImageView,
    pub imageLayout: VkImageLayout,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDescriptorPoolCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkDescriptorPoolCreateFlags,
    pub maxSets: u32,
    pub poolSizeCount: u32,
    pub pPoolSizes: *const VkDescriptorPoolSize,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDescriptorPoolInlineUniformBlockCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub maxInlineUniformBlockBindings: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDescriptorPoolSize {
    pub typ: VkDescriptorType,
    pub descriptorCount: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDescriptorSetAllocateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub descriptorPool: VkDescriptorPool,
    pub descriptorSetCount: u32,
    pub pSetLayouts: *const VkDescriptorSetLayout,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDescriptorSetLayoutBinding {
    pub binding: u32,
    pub descriptorType: VkDescriptorType,
    pub descriptorCount: u32,
    pub stageFlags: VkShaderStageFlags,
    pub pImmutableSamplers: *const VkSampler,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDescriptorSetLayoutBindingFlagsCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub bindingCount: u32,
    pub pBindingFlags: *const VkDescriptorBindingFlags,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDescriptorSetLayoutCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkDescriptorSetLayoutCreateFlags,
    pub bindingCount: u32,
    pub pBindings: *const VkDescriptorSetLayoutBinding,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDescriptorSetLayoutSupport {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub supported: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDescriptorSetVariableDescriptorCountAllocateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub descriptorSetCount: u32,
    pub pDescriptorCounts: *const u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDescriptorSetVariableDescriptorCountLayoutSupport {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub maxVariableDescriptorCount: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDescriptorUpdateTemplateCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkDescriptorUpdateTemplateCreateFlags,
    pub descriptorUpdateEntryCount: u32,
    pub pDescriptorUpdateEntries: *const VkDescriptorUpdateTemplateEntry,
    pub templateType: VkDescriptorUpdateTemplateType,
    pub descriptorSetLayout: VkDescriptorSetLayout,
    pub pipelineBindPoint: VkPipelineBindPoint,
    pub pipelineLayout: VkPipelineLayout,
    pub set: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDescriptorUpdateTemplateEntry {
    pub dstBinding: u32,
    pub dstArrayElement: u32,
    pub descriptorCount: u32,
    pub descriptorType: VkDescriptorType,
    pub offset: usize,
    pub stride: usize,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDeviceBufferMemoryRequirements {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub pCreateInfo: *const VkBufferCreateInfo,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDeviceCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkDeviceCreateFlags,
    pub queueCreateInfoCount: u32,
    pub pQueueCreateInfos: *const VkDeviceQueueCreateInfo,
    pub enabledLayerCount: u32,
    pub ppEnabledLayerNames: *const *const c_char,
    pub enabledExtensionCount: u32,
    pub ppEnabledExtensionNames: *const *const c_char,
    pub pEnabledFeatures: *const VkPhysicalDeviceFeatures,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDeviceGroupBindSparseInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub resourceDeviceIndex: u32,
    pub memoryDeviceIndex: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDeviceGroupCommandBufferBeginInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub deviceMask: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDeviceGroupDeviceCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub physicalDeviceCount: u32,
    pub pPhysicalDevices: *const VkPhysicalDevice,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDeviceGroupRenderPassBeginInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub deviceMask: u32,
    pub deviceRenderAreaCount: u32,
    pub pDeviceRenderAreas: *const VkRect2D,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDeviceGroupSubmitInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub waitSemaphoreCount: u32,
    pub pWaitSemaphoreDeviceIndices: *const u32,
    pub commandBufferCount: u32,
    pub pCommandBufferDeviceMasks: *const u32,
    pub signalSemaphoreCount: u32,
    pub pSignalSemaphoreDeviceIndices: *const u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDeviceImageMemoryRequirements {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub pCreateInfo: *const VkImageCreateInfo,
    pub planeAspect: VkImageAspectFlagBits,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDeviceImageSubresourceInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub pCreateInfo: *const VkImageCreateInfo,
    pub pSubresource: *const VkImageSubresource2,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDeviceMemoryOpaqueCaptureAddressInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub memory: VkDeviceMemory,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDevicePrivateDataCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub privateDataSlotRequestCount: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDeviceQueueCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkDeviceQueueCreateFlags,
    pub queueFamilyIndex: u32,
    pub queueCount: u32,
    pub pQueuePriorities: *const c_float,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDeviceQueueGlobalPriorityCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub globalPriority: VkQueueGlobalPriority,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDeviceQueueInfo2 {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkDeviceQueueCreateFlags,
    pub queueFamilyIndex: u32,
    pub queueIndex: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDispatchIndirectCommand {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDrawIndexedIndirectCommand {
    pub indexCount: u32,
    pub instanceCount: u32,
    pub firstIndex: u32,
    pub vertexOffset: i32,
    pub firstInstance: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDrawIndirectCommand {
    pub vertexCount: u32,
    pub instanceCount: u32,
    pub firstVertex: u32,
    pub firstInstance: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkEventCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkEventCreateFlags,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkExportFenceCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub handleTypes: VkExternalFenceHandleTypeFlags,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkExportMemoryAllocateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub handleTypes: VkExternalMemoryHandleTypeFlags,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkExportSemaphoreCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub handleTypes: VkExternalSemaphoreHandleTypeFlags,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkExtensionProperties {
    pub extensionName: [c_char; VK_MAX_EXTENSION_NAME_SIZE as usize],
    pub specVersion: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkExtent2D {
    pub width: u32,
    pub height: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkExtent3D {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkExternalBufferProperties {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub externalMemoryProperties: VkExternalMemoryProperties,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkExternalFenceProperties {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub exportFromImportedHandleTypes: VkExternalFenceHandleTypeFlags,
    pub compatibleHandleTypes: VkExternalFenceHandleTypeFlags,
    pub externalFenceFeatures: VkExternalFenceFeatureFlags,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkExternalImageFormatProperties {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub externalMemoryProperties: VkExternalMemoryProperties,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkExternalMemoryBufferCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub handleTypes: VkExternalMemoryHandleTypeFlags,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkExternalMemoryImageCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub handleTypes: VkExternalMemoryHandleTypeFlags,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkExternalMemoryProperties {
    pub externalMemoryFeatures: VkExternalMemoryFeatureFlags,
    pub exportFromImportedHandleTypes: VkExternalMemoryHandleTypeFlags,
    pub compatibleHandleTypes: VkExternalMemoryHandleTypeFlags,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkExternalSemaphoreProperties {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub exportFromImportedHandleTypes: VkExternalSemaphoreHandleTypeFlags,
    pub compatibleHandleTypes: VkExternalSemaphoreHandleTypeFlags,
    pub externalSemaphoreFeatures: VkExternalSemaphoreFeatureFlags,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkFenceCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkFenceCreateFlags,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkFormatProperties {
    pub linearTilingFeatures: VkFormatFeatureFlags,
    pub optimalTilingFeatures: VkFormatFeatureFlags,
    pub bufferFeatures: VkFormatFeatureFlags,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkFormatProperties2 {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub formatProperties: VkFormatProperties,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkFormatProperties3 {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub linearTilingFeatures: VkFormatFeatureFlags2,
    pub optimalTilingFeatures: VkFormatFeatureFlags2,
    pub bufferFeatures: VkFormatFeatureFlags2,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkFramebufferAttachmentImageInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkImageCreateFlags,
    pub usage: VkImageUsageFlags,
    pub width: u32,
    pub height: u32,
    pub layerCount: u32,
    pub viewFormatCount: u32,
    pub pViewFormats: *const VkFormat,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkFramebufferAttachmentsCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub attachmentImageInfoCount: u32,
    pub pAttachmentImageInfos: *const VkFramebufferAttachmentImageInfo,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkFramebufferCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkFramebufferCreateFlags,
    pub renderPass: VkRenderPass,
    pub attachmentCount: u32,
    pub pAttachments: *const VkImageView,
    pub width: u32,
    pub height: u32,
    pub layers: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkGraphicsPipelineCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineCreateFlags,
    pub stageCount: u32,
    pub pStages: *const VkPipelineShaderStageCreateInfo,
    pub pVertexInputState: *const VkPipelineVertexInputStateCreateInfo,
    pub pInputAssemblyState: *const VkPipelineInputAssemblyStateCreateInfo,
    pub pTessellationState: *const VkPipelineTessellationStateCreateInfo,
    pub pViewportState: *const VkPipelineViewportStateCreateInfo,
    pub pRasterizationState: *const VkPipelineRasterizationStateCreateInfo,
    pub pMultisampleState: *const VkPipelineMultisampleStateCreateInfo,
    pub pDepthStencilState: *const VkPipelineDepthStencilStateCreateInfo,
    pub pColorBlendState: *const VkPipelineColorBlendStateCreateInfo,
    pub pDynamicState: *const VkPipelineDynamicStateCreateInfo,
    pub layout: VkPipelineLayout,
    pub renderPass: VkRenderPass,
    pub subpass: u32,
    pub basePipelineHandle: VkPipeline,
    pub basePipelineIndex: i32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkHostImageCopyDevicePerformanceQuery {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub optimalDeviceAccess: VkBool32,
    pub identicalMemoryLayout: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkHostImageLayoutTransitionInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub image: VkImage,
    pub oldLayout: VkImageLayout,
    pub newLayout: VkImageLayout,
    pub subresourceRange: VkImageSubresourceRange,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImageBlit {
    pub srcSubresource: VkImageSubresourceLayers,
    pub srcOffsets: [VkOffset3D; 2 as usize],
    pub dstSubresource: VkImageSubresourceLayers,
    pub dstOffsets: [VkOffset3D; 2 as usize],
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImageBlit2 {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub srcSubresource: VkImageSubresourceLayers,
    pub srcOffsets: [VkOffset3D; 2 as usize],
    pub dstSubresource: VkImageSubresourceLayers,
    pub dstOffsets: [VkOffset3D; 2 as usize],
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImageCopy {
    pub srcSubresource: VkImageSubresourceLayers,
    pub srcOffset: VkOffset3D,
    pub dstSubresource: VkImageSubresourceLayers,
    pub dstOffset: VkOffset3D,
    pub extent: VkExtent3D,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImageCopy2 {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub srcSubresource: VkImageSubresourceLayers,
    pub srcOffset: VkOffset3D,
    pub dstSubresource: VkImageSubresourceLayers,
    pub dstOffset: VkOffset3D,
    pub extent: VkExtent3D,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImageCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkImageCreateFlags,
    pub imageType: VkImageType,
    pub format: VkFormat,
    pub extent: VkExtent3D,
    pub mipLevels: u32,
    pub arrayLayers: u32,
    pub samples: VkSampleCountFlagBits,
    pub tiling: VkImageTiling,
    pub usage: VkImageUsageFlags,
    pub sharingMode: VkSharingMode,
    pub queueFamilyIndexCount: u32,
    pub pQueueFamilyIndices: *const u32,
    pub initialLayout: VkImageLayout,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImageFormatListCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub viewFormatCount: u32,
    pub pViewFormats: *const VkFormat,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImageFormatProperties {
    pub maxExtent: VkExtent3D,
    pub maxMipLevels: u32,
    pub maxArrayLayers: u32,
    pub sampleCounts: VkSampleCountFlags,
    pub maxResourceSize: VkDeviceSize,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImageFormatProperties2 {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub imageFormatProperties: VkImageFormatProperties,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImageMemoryBarrier {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub srcAccessMask: VkAccessFlags,
    pub dstAccessMask: VkAccessFlags,
    pub oldLayout: VkImageLayout,
    pub newLayout: VkImageLayout,
    pub srcQueueFamilyIndex: u32,
    pub dstQueueFamilyIndex: u32,
    pub image: VkImage,
    pub subresourceRange: VkImageSubresourceRange,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImageMemoryBarrier2 {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub srcStageMask: VkPipelineStageFlags2,
    pub srcAccessMask: VkAccessFlags2,
    pub dstStageMask: VkPipelineStageFlags2,
    pub dstAccessMask: VkAccessFlags2,
    pub oldLayout: VkImageLayout,
    pub newLayout: VkImageLayout,
    pub srcQueueFamilyIndex: u32,
    pub dstQueueFamilyIndex: u32,
    pub image: VkImage,
    pub subresourceRange: VkImageSubresourceRange,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImageMemoryRequirementsInfo2 {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub image: VkImage,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImagePlaneMemoryRequirementsInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub planeAspect: VkImageAspectFlagBits,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImageResolve {
    pub srcSubresource: VkImageSubresourceLayers,
    pub srcOffset: VkOffset3D,
    pub dstSubresource: VkImageSubresourceLayers,
    pub dstOffset: VkOffset3D,
    pub extent: VkExtent3D,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImageResolve2 {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub srcSubresource: VkImageSubresourceLayers,
    pub srcOffset: VkOffset3D,
    pub dstSubresource: VkImageSubresourceLayers,
    pub dstOffset: VkOffset3D,
    pub extent: VkExtent3D,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImageSparseMemoryRequirementsInfo2 {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub image: VkImage,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImageStencilUsageCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub stencilUsage: VkImageUsageFlags,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImageSubresource {
    pub aspectMask: VkImageAspectFlags,
    pub mipLevel: u32,
    pub arrayLayer: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImageSubresource2 {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub imageSubresource: VkImageSubresource,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImageSubresourceLayers {
    pub aspectMask: VkImageAspectFlags,
    pub mipLevel: u32,
    pub baseArrayLayer: u32,
    pub layerCount: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImageSubresourceRange {
    pub aspectMask: VkImageAspectFlags,
    pub baseMipLevel: u32,
    pub levelCount: u32,
    pub baseArrayLayer: u32,
    pub layerCount: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImageToMemoryCopy {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub pHostPointer: *mut c_void,
    pub memoryRowLength: u32,
    pub memoryImageHeight: u32,
    pub imageSubresource: VkImageSubresourceLayers,
    pub imageOffset: VkOffset3D,
    pub imageExtent: VkExtent3D,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImageViewCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkImageViewCreateFlags,
    pub image: VkImage,
    pub viewType: VkImageViewType,
    pub format: VkFormat,
    pub components: VkComponentMapping,
    pub subresourceRange: VkImageSubresourceRange,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImageViewUsageCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub usage: VkImageUsageFlags,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkInputAttachmentAspectReference {
    pub subpass: u32,
    pub inputAttachmentIndex: u32,
    pub aspectMask: VkImageAspectFlags,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkInstanceCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkInstanceCreateFlags,
    pub pApplicationInfo: *const VkApplicationInfo,
    pub enabledLayerCount: u32,
    pub ppEnabledLayerNames: *const *const c_char,
    pub enabledExtensionCount: u32,
    pub ppEnabledExtensionNames: *const *const c_char,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkLayerProperties {
    pub layerName: [c_char; VK_MAX_EXTENSION_NAME_SIZE as usize],
    pub specVersion: u32,
    pub implementationVersion: u32,
    pub description: [c_char; VK_MAX_DESCRIPTION_SIZE as usize],
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkMappedMemoryRange {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub memory: VkDeviceMemory,
    pub offset: VkDeviceSize,
    pub size: VkDeviceSize,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkMemoryAllocateFlagsInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkMemoryAllocateFlags,
    pub deviceMask: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkMemoryAllocateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub allocationSize: VkDeviceSize,
    pub memoryTypeIndex: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkMemoryBarrier {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub srcAccessMask: VkAccessFlags,
    pub dstAccessMask: VkAccessFlags,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkMemoryBarrier2 {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub srcStageMask: VkPipelineStageFlags2,
    pub srcAccessMask: VkAccessFlags2,
    pub dstStageMask: VkPipelineStageFlags2,
    pub dstAccessMask: VkAccessFlags2,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkMemoryDedicatedAllocateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub image: VkImage,
    pub buffer: VkBuffer,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkMemoryDedicatedRequirements {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub prefersDedicatedAllocation: VkBool32,
    pub requiresDedicatedAllocation: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkMemoryHeap {
    pub size: VkDeviceSize,
    pub flags: VkMemoryHeapFlags,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkMemoryMapInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkMemoryMapFlags,
    pub memory: VkDeviceMemory,
    pub offset: VkDeviceSize,
    pub size: VkDeviceSize,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkMemoryOpaqueCaptureAddressAllocateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub opaqueCaptureAddress: u64,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkMemoryRequirements {
    pub size: VkDeviceSize,
    pub alignment: VkDeviceSize,
    pub memoryTypeBits: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkMemoryRequirements2 {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub memoryRequirements: VkMemoryRequirements,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkMemoryToImageCopy {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub pHostPointer: *const c_void,
    pub memoryRowLength: u32,
    pub memoryImageHeight: u32,
    pub imageSubresource: VkImageSubresourceLayers,
    pub imageOffset: VkOffset3D,
    pub imageExtent: VkExtent3D,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkMemoryType {
    pub propertyFlags: VkMemoryPropertyFlags,
    pub heapIndex: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkMemoryUnmapInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkMemoryUnmapFlags,
    pub memory: VkDeviceMemory,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkOffset2D {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkOffset3D {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDevice16BitStorageFeatures {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub storageBuffer16BitAccess: VkBool32,
    pub uniformAndStorageBuffer16BitAccess: VkBool32,
    pub storagePushConstant16: VkBool32,
    pub storageInputOutput16: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDevice8BitStorageFeatures {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub storageBuffer8BitAccess: VkBool32,
    pub uniformAndStorageBuffer8BitAccess: VkBool32,
    pub storagePushConstant8: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceBufferDeviceAddressFeatures {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub bufferDeviceAddress: VkBool32,
    pub bufferDeviceAddressCaptureReplay: VkBool32,
    pub bufferDeviceAddressMultiDevice: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceDepthStencilResolveProperties {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub supportedDepthResolveModes: VkResolveModeFlags,
    pub supportedStencilResolveModes: VkResolveModeFlags,
    pub independentResolveNone: VkBool32,
    pub independentResolve: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceDescriptorIndexingFeatures {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub shaderInputAttachmentArrayDynamicIndexing: VkBool32,
    pub shaderUniformTexelBufferArrayDynamicIndexing: VkBool32,
    pub shaderStorageTexelBufferArrayDynamicIndexing: VkBool32,
    pub shaderUniformBufferArrayNonUniformIndexing: VkBool32,
    pub shaderSampledImageArrayNonUniformIndexing: VkBool32,
    pub shaderStorageBufferArrayNonUniformIndexing: VkBool32,
    pub shaderStorageImageArrayNonUniformIndexing: VkBool32,
    pub shaderInputAttachmentArrayNonUniformIndexing: VkBool32,
    pub shaderUniformTexelBufferArrayNonUniformIndexing: VkBool32,
    pub shaderStorageTexelBufferArrayNonUniformIndexing: VkBool32,
    pub descriptorBindingUniformBufferUpdateAfterBind: VkBool32,
    pub descriptorBindingSampledImageUpdateAfterBind: VkBool32,
    pub descriptorBindingStorageImageUpdateAfterBind: VkBool32,
    pub descriptorBindingStorageBufferUpdateAfterBind: VkBool32,
    pub descriptorBindingUniformTexelBufferUpdateAfterBind: VkBool32,
    pub descriptorBindingStorageTexelBufferUpdateAfterBind: VkBool32,
    pub descriptorBindingUpdateUnusedWhilePending: VkBool32,
    pub descriptorBindingPartiallyBound: VkBool32,
    pub descriptorBindingVariableDescriptorCount: VkBool32,
    pub runtimeDescriptorArray: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceDescriptorIndexingProperties {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub maxUpdateAfterBindDescriptorsInAllPools: u32,
    pub shaderUniformBufferArrayNonUniformIndexingNative: VkBool32,
    pub shaderSampledImageArrayNonUniformIndexingNative: VkBool32,
    pub shaderStorageBufferArrayNonUniformIndexingNative: VkBool32,
    pub shaderStorageImageArrayNonUniformIndexingNative: VkBool32,
    pub shaderInputAttachmentArrayNonUniformIndexingNative: VkBool32,
    pub robustBufferAccessUpdateAfterBind: VkBool32,
    pub quadDivergentImplicitLod: VkBool32,
    pub maxPerStageDescriptorUpdateAfterBindSamplers: u32,
    pub maxPerStageDescriptorUpdateAfterBindUniformBuffers: u32,
    pub maxPerStageDescriptorUpdateAfterBindStorageBuffers: u32,
    pub maxPerStageDescriptorUpdateAfterBindSampledImages: u32,
    pub maxPerStageDescriptorUpdateAfterBindStorageImages: u32,
    pub maxPerStageDescriptorUpdateAfterBindInputAttachments: u32,
    pub maxPerStageUpdateAfterBindResources: u32,
    pub maxDescriptorSetUpdateAfterBindSamplers: u32,
    pub maxDescriptorSetUpdateAfterBindUniformBuffers: u32,
    pub maxDescriptorSetUpdateAfterBindUniformBuffersDynamic: u32,
    pub maxDescriptorSetUpdateAfterBindStorageBuffers: u32,
    pub maxDescriptorSetUpdateAfterBindStorageBuffersDynamic: u32,
    pub maxDescriptorSetUpdateAfterBindSampledImages: u32,
    pub maxDescriptorSetUpdateAfterBindStorageImages: u32,
    pub maxDescriptorSetUpdateAfterBindInputAttachments: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceDriverProperties {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub driverID: VkDriverId,
    pub driverName: [c_char; VK_MAX_DRIVER_NAME_SIZE as usize],
    pub driverInfo: [c_char; VK_MAX_DRIVER_INFO_SIZE as usize],
    pub conformanceVersion: VkConformanceVersion,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceDynamicRenderingFeatures {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub dynamicRendering: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceDynamicRenderingLocalReadFeatures {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub dynamicRenderingLocalRead: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceExternalBufferInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkBufferCreateFlags,
    pub usage: VkBufferUsageFlags,
    pub handleType: VkExternalMemoryHandleTypeFlagBits,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceExternalFenceInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub handleType: VkExternalFenceHandleTypeFlagBits,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceExternalImageFormatInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub handleType: VkExternalMemoryHandleTypeFlagBits,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceExternalSemaphoreInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub handleType: VkExternalSemaphoreHandleTypeFlagBits,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceFeatures {
    pub robustBufferAccess: VkBool32,
    pub fullDrawIndexUint32: VkBool32,
    pub imageCubeArray: VkBool32,
    pub independentBlend: VkBool32,
    pub geometryShader: VkBool32,
    pub tessellationShader: VkBool32,
    pub sampleRateShading: VkBool32,
    pub dualSrcBlend: VkBool32,
    pub logicOp: VkBool32,
    pub multiDrawIndirect: VkBool32,
    pub drawIndirectFirstInstance: VkBool32,
    pub depthClamp: VkBool32,
    pub depthBiasClamp: VkBool32,
    pub fillModeNonSolid: VkBool32,
    pub depthBounds: VkBool32,
    pub wideLines: VkBool32,
    pub largePoints: VkBool32,
    pub alphaToOne: VkBool32,
    pub multiViewport: VkBool32,
    pub samplerAnisotropy: VkBool32,
    pub textureCompressionETC2: VkBool32,
    pub textureCompressionASTC_LDR: VkBool32,
    pub textureCompressionBC: VkBool32,
    pub occlusionQueryPrecise: VkBool32,
    pub pipelineStatisticsQuery: VkBool32,
    pub vertexPipelineStoresAndAtomics: VkBool32,
    pub fragmentStoresAndAtomics: VkBool32,
    pub shaderTessellationAndGeometryPointSize: VkBool32,
    pub shaderImageGatherExtended: VkBool32,
    pub shaderStorageImageExtendedFormats: VkBool32,
    pub shaderStorageImageMultisample: VkBool32,
    pub shaderStorageImageReadWithoutFormat: VkBool32,
    pub shaderStorageImageWriteWithoutFormat: VkBool32,
    pub shaderUniformBufferArrayDynamicIndexing: VkBool32,
    pub shaderSampledImageArrayDynamicIndexing: VkBool32,
    pub shaderStorageBufferArrayDynamicIndexing: VkBool32,
    pub shaderStorageImageArrayDynamicIndexing: VkBool32,
    pub shaderClipDistance: VkBool32,
    pub shaderCullDistance: VkBool32,
    pub shaderFloat64: VkBool32,
    pub shaderInt64: VkBool32,
    pub shaderInt16: VkBool32,
    pub shaderResourceResidency: VkBool32,
    pub shaderResourceMinLod: VkBool32,
    pub sparseBinding: VkBool32,
    pub sparseResidencyBuffer: VkBool32,
    pub sparseResidencyImage2D: VkBool32,
    pub sparseResidencyImage3D: VkBool32,
    pub sparseResidency2Samples: VkBool32,
    pub sparseResidency4Samples: VkBool32,
    pub sparseResidency8Samples: VkBool32,
    pub sparseResidency16Samples: VkBool32,
    pub sparseResidencyAliased: VkBool32,
    pub variableMultisampleRate: VkBool32,
    pub inheritedQueries: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceFeatures2 {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub features: VkPhysicalDeviceFeatures,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceFloatControlsProperties {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub denormBehaviorIndependence: VkShaderFloatControlsIndependence,
    pub roundingModeIndependence: VkShaderFloatControlsIndependence,
    pub shaderSignedZeroInfNanPreserveFloat16: VkBool32,
    pub shaderSignedZeroInfNanPreserveFloat32: VkBool32,
    pub shaderSignedZeroInfNanPreserveFloat64: VkBool32,
    pub shaderDenormPreserveFloat16: VkBool32,
    pub shaderDenormPreserveFloat32: VkBool32,
    pub shaderDenormPreserveFloat64: VkBool32,
    pub shaderDenormFlushToZeroFloat16: VkBool32,
    pub shaderDenormFlushToZeroFloat32: VkBool32,
    pub shaderDenormFlushToZeroFloat64: VkBool32,
    pub shaderRoundingModeRTEFloat16: VkBool32,
    pub shaderRoundingModeRTEFloat32: VkBool32,
    pub shaderRoundingModeRTEFloat64: VkBool32,
    pub shaderRoundingModeRTZFloat16: VkBool32,
    pub shaderRoundingModeRTZFloat32: VkBool32,
    pub shaderRoundingModeRTZFloat64: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceGlobalPriorityQueryFeatures {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub globalPriorityQuery: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceGroupProperties {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub physicalDeviceCount: u32,
    pub physicalDevices: [VkPhysicalDevice; VK_MAX_DEVICE_GROUP_SIZE as usize],
    pub subsetAllocation: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceHostImageCopyFeatures {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub hostImageCopy: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceHostImageCopyProperties {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub copySrcLayoutCount: u32,
    pub pCopySrcLayouts: *mut VkImageLayout,
    pub copyDstLayoutCount: u32,
    pub pCopyDstLayouts: *mut VkImageLayout,
    pub optimalTilingLayoutUUID: [u8; VK_UUID_SIZE as usize],
    pub identicalMemoryTypeRequirements: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceHostQueryResetFeatures {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub hostQueryReset: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceIDProperties {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub deviceUUID: [u8; VK_UUID_SIZE as usize],
    pub driverUUID: [u8; VK_UUID_SIZE as usize],
    pub deviceLUID: [u8; VK_LUID_SIZE as usize],
    pub deviceNodeMask: u32,
    pub deviceLUIDValid: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceImageFormatInfo2 {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub format: VkFormat,
    pub typ: VkImageType,
    pub tiling: VkImageTiling,
    pub usage: VkImageUsageFlags,
    pub flags: VkImageCreateFlags,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceImageRobustnessFeatures {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub robustImageAccess: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceImagelessFramebufferFeatures {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub imagelessFramebuffer: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceIndexTypeUint8Features {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub indexTypeUint8: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceInlineUniformBlockFeatures {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub inlineUniformBlock: VkBool32,
    pub descriptorBindingInlineUniformBlockUpdateAfterBind: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceInlineUniformBlockProperties {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub maxInlineUniformBlockSize: u32,
    pub maxPerStageDescriptorInlineUniformBlocks: u32,
    pub maxPerStageDescriptorUpdateAfterBindInlineUniformBlocks: u32,
    pub maxDescriptorSetInlineUniformBlocks: u32,
    pub maxDescriptorSetUpdateAfterBindInlineUniformBlocks: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceLimits {
    pub maxImageDimension1D: u32,
    pub maxImageDimension2D: u32,
    pub maxImageDimension3D: u32,
    pub maxImageDimensionCube: u32,
    pub maxImageArrayLayers: u32,
    pub maxTexelBufferElements: u32,
    pub maxUniformBufferRange: u32,
    pub maxStorageBufferRange: u32,
    pub maxPushConstantsSize: u32,
    pub maxMemoryAllocationCount: u32,
    pub maxSamplerAllocationCount: u32,
    pub bufferImageGranularity: VkDeviceSize,
    pub sparseAddressSpaceSize: VkDeviceSize,
    pub maxBoundDescriptorSets: u32,
    pub maxPerStageDescriptorSamplers: u32,
    pub maxPerStageDescriptorUniformBuffers: u32,
    pub maxPerStageDescriptorStorageBuffers: u32,
    pub maxPerStageDescriptorSampledImages: u32,
    pub maxPerStageDescriptorStorageImages: u32,
    pub maxPerStageDescriptorInputAttachments: u32,
    pub maxPerStageResources: u32,
    pub maxDescriptorSetSamplers: u32,
    pub maxDescriptorSetUniformBuffers: u32,
    pub maxDescriptorSetUniformBuffersDynamic: u32,
    pub maxDescriptorSetStorageBuffers: u32,
    pub maxDescriptorSetStorageBuffersDynamic: u32,
    pub maxDescriptorSetSampledImages: u32,
    pub maxDescriptorSetStorageImages: u32,
    pub maxDescriptorSetInputAttachments: u32,
    pub maxVertexInputAttributes: u32,
    pub maxVertexInputBindings: u32,
    pub maxVertexInputAttributeOffset: u32,
    pub maxVertexInputBindingStride: u32,
    pub maxVertexOutputComponents: u32,
    pub maxTessellationGenerationLevel: u32,
    pub maxTessellationPatchSize: u32,
    pub maxTessellationControlPerVertexInputComponents: u32,
    pub maxTessellationControlPerVertexOutputComponents: u32,
    pub maxTessellationControlPerPatchOutputComponents: u32,
    pub maxTessellationControlTotalOutputComponents: u32,
    pub maxTessellationEvaluationInputComponents: u32,
    pub maxTessellationEvaluationOutputComponents: u32,
    pub maxGeometryShaderInvocations: u32,
    pub maxGeometryInputComponents: u32,
    pub maxGeometryOutputComponents: u32,
    pub maxGeometryOutputVertices: u32,
    pub maxGeometryTotalOutputComponents: u32,
    pub maxFragmentInputComponents: u32,
    pub maxFragmentOutputAttachments: u32,
    pub maxFragmentDualSrcAttachments: u32,
    pub maxFragmentCombinedOutputResources: u32,
    pub maxComputeSharedMemorySize: u32,
    pub maxComputeWorkGroupCount: [u32; 3 as usize],
    pub maxComputeWorkGroupInvocations: u32,
    pub maxComputeWorkGroupSize: [u32; 3 as usize],
    pub subPixelPrecisionBits: u32,
    pub subTexelPrecisionBits: u32,
    pub mipmapPrecisionBits: u32,
    pub maxDrawIndexedIndexValue: u32,
    pub maxDrawIndirectCount: u32,
    pub maxSamplerLodBias: c_float,
    pub maxSamplerAnisotropy: c_float,
    pub maxViewports: u32,
    pub maxViewportDimensions: [u32; 2 as usize],
    pub viewportBoundsRange: [c_float; 2 as usize],
    pub viewportSubPixelBits: u32,
    pub minMemoryMapAlignment: usize,
    pub minTexelBufferOffsetAlignment: VkDeviceSize,
    pub minUniformBufferOffsetAlignment: VkDeviceSize,
    pub minStorageBufferOffsetAlignment: VkDeviceSize,
    pub minTexelOffset: i32,
    pub maxTexelOffset: u32,
    pub minTexelGatherOffset: i32,
    pub maxTexelGatherOffset: u32,
    pub minInterpolationOffset: c_float,
    pub maxInterpolationOffset: c_float,
    pub subPixelInterpolationOffsetBits: u32,
    pub maxFramebufferWidth: u32,
    pub maxFramebufferHeight: u32,
    pub maxFramebufferLayers: u32,
    pub framebufferColorSampleCounts: VkSampleCountFlags,
    pub framebufferDepthSampleCounts: VkSampleCountFlags,
    pub framebufferStencilSampleCounts: VkSampleCountFlags,
    pub framebufferNoAttachmentsSampleCounts: VkSampleCountFlags,
    pub maxColorAttachments: u32,
    pub sampledImageColorSampleCounts: VkSampleCountFlags,
    pub sampledImageIntegerSampleCounts: VkSampleCountFlags,
    pub sampledImageDepthSampleCounts: VkSampleCountFlags,
    pub sampledImageStencilSampleCounts: VkSampleCountFlags,
    pub storageImageSampleCounts: VkSampleCountFlags,
    pub maxSampleMaskWords: u32,
    pub timestampComputeAndGraphics: VkBool32,
    pub timestampPeriod: c_float,
    pub maxClipDistances: u32,
    pub maxCullDistances: u32,
    pub maxCombinedClipAndCullDistances: u32,
    pub discreteQueuePriorities: u32,
    pub pointSizeRange: [c_float; 2 as usize],
    pub lineWidthRange: [c_float; 2 as usize],
    pub pointSizeGranularity: c_float,
    pub lineWidthGranularity: c_float,
    pub strictLines: VkBool32,
    pub standardSampleLocations: VkBool32,
    pub optimalBufferCopyOffsetAlignment: VkDeviceSize,
    pub optimalBufferCopyRowPitchAlignment: VkDeviceSize,
    pub nonCoherentAtomSize: VkDeviceSize,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceLineRasterizationFeatures {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub rectangularLines: VkBool32,
    pub bresenhamLines: VkBool32,
    pub smoothLines: VkBool32,
    pub stippledRectangularLines: VkBool32,
    pub stippledBresenhamLines: VkBool32,
    pub stippledSmoothLines: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceLineRasterizationProperties {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub lineSubPixelPrecisionBits: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceMaintenance3Properties {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub maxPerSetDescriptors: u32,
    pub maxMemoryAllocationSize: VkDeviceSize,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceMaintenance4Features {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub maintenance4: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceMaintenance4Properties {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub maxBufferSize: VkDeviceSize,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceMaintenance5Features {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub maintenance5: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceMaintenance5Properties {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub earlyFragmentMultisampleCoverageAfterSampleCounting: VkBool32,
    pub earlyFragmentSampleMaskTestBeforeSampleCounting: VkBool32,
    pub depthStencilSwizzleOneSupport: VkBool32,
    pub polygonModePointSize: VkBool32,
    pub nonStrictSinglePixelWideLinesUseParallelogram: VkBool32,
    pub nonStrictWideLinesUseParallelogram: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceMaintenance6Features {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub maintenance6: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceMaintenance6Properties {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub blockTexelViewCompatibleMultipleLayers: VkBool32,
    pub maxCombinedImageSamplerDescriptorCount: u32,
    pub fragmentShadingRateClampCombinerInputs: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceMemoryProperties {
    pub memoryTypeCount: u32,
    pub memoryTypes: [VkMemoryType; VK_MAX_MEMORY_TYPES as usize],
    pub memoryHeapCount: u32,
    pub memoryHeaps: [VkMemoryHeap; VK_MAX_MEMORY_HEAPS as usize],
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceMemoryProperties2 {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub memoryProperties: VkPhysicalDeviceMemoryProperties,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceMultiviewFeatures {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub multiview: VkBool32,
    pub multiviewGeometryShader: VkBool32,
    pub multiviewTessellationShader: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceMultiviewProperties {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub maxMultiviewViewCount: u32,
    pub maxMultiviewInstanceIndex: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDevicePipelineCreationCacheControlFeatures {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub pipelineCreationCacheControl: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDevicePipelineProtectedAccessFeatures {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub pipelineProtectedAccess: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDevicePipelineRobustnessFeatures {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub pipelineRobustness: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDevicePipelineRobustnessProperties {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub defaultRobustnessStorageBuffers: VkPipelineRobustnessBufferBehavior,
    pub defaultRobustnessUniformBuffers: VkPipelineRobustnessBufferBehavior,
    pub defaultRobustnessVertexInputs: VkPipelineRobustnessBufferBehavior,
    pub defaultRobustnessImages: VkPipelineRobustnessImageBehavior,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDevicePointClippingProperties {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub pointClippingBehavior: VkPointClippingBehavior,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDevicePrivateDataFeatures {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub privateData: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceProperties {
    pub apiVersion: u32,
    pub driverVersion: u32,
    pub vendorID: u32,
    pub deviceID: u32,
    pub deviceType: VkPhysicalDeviceType,
    pub deviceName: [c_char; VK_MAX_PHYSICAL_DEVICE_NAME_SIZE as usize],
    pub pipelineCacheUUID: [u8; VK_UUID_SIZE as usize],
    pub limits: VkPhysicalDeviceLimits,
    pub sparseProperties: VkPhysicalDeviceSparseProperties,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceProperties2 {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub properties: VkPhysicalDeviceProperties,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceProtectedMemoryFeatures {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub protectedMemory: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceProtectedMemoryProperties {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub protectedNoFault: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDevicePushDescriptorProperties {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub maxPushDescriptors: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceSamplerFilterMinmaxProperties {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub filterMinmaxSingleComponentFormats: VkBool32,
    pub filterMinmaxImageComponentMapping: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceSamplerYcbcrConversionFeatures {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub samplerYcbcrConversion: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceScalarBlockLayoutFeatures {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub scalarBlockLayout: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceSeparateDepthStencilLayoutsFeatures {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub separateDepthStencilLayouts: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceShaderAtomicInt64Features {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub shaderBufferInt64Atomics: VkBool32,
    pub shaderSharedInt64Atomics: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceShaderDemoteToHelperInvocationFeatures {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub shaderDemoteToHelperInvocation: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceShaderDrawParametersFeatures {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub shaderDrawParameters: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceShaderExpectAssumeFeatures {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub shaderExpectAssume: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceShaderFloat16Int8Features {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub shaderFloat16: VkBool32,
    pub shaderInt8: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceShaderFloatControls2Features {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub shaderFloatControls2: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceShaderIntegerDotProductFeatures {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub shaderIntegerDotProduct: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceShaderIntegerDotProductProperties {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub integerDotProduct8BitUnsignedAccelerated: VkBool32,
    pub integerDotProduct8BitSignedAccelerated: VkBool32,
    pub integerDotProduct8BitMixedSignednessAccelerated: VkBool32,
    pub integerDotProduct4x8BitPackedUnsignedAccelerated: VkBool32,
    pub integerDotProduct4x8BitPackedSignedAccelerated: VkBool32,
    pub integerDotProduct4x8BitPackedMixedSignednessAccelerated: VkBool32,
    pub integerDotProduct16BitUnsignedAccelerated: VkBool32,
    pub integerDotProduct16BitSignedAccelerated: VkBool32,
    pub integerDotProduct16BitMixedSignednessAccelerated: VkBool32,
    pub integerDotProduct32BitUnsignedAccelerated: VkBool32,
    pub integerDotProduct32BitSignedAccelerated: VkBool32,
    pub integerDotProduct32BitMixedSignednessAccelerated: VkBool32,
    pub integerDotProduct64BitUnsignedAccelerated: VkBool32,
    pub integerDotProduct64BitSignedAccelerated: VkBool32,
    pub integerDotProduct64BitMixedSignednessAccelerated: VkBool32,
    pub integerDotProductAccumulatingSaturating8BitUnsignedAccelerated: VkBool32,
    pub integerDotProductAccumulatingSaturating8BitSignedAccelerated: VkBool32,
    pub integerDotProductAccumulatingSaturating8BitMixedSignednessAccelerated: VkBool32,
    pub integerDotProductAccumulatingSaturating4x8BitPackedUnsignedAccelerated: VkBool32,
    pub integerDotProductAccumulatingSaturating4x8BitPackedSignedAccelerated: VkBool32,
    pub integerDotProductAccumulatingSaturating4x8BitPackedMixedSignednessAccelerated: VkBool32,
    pub integerDotProductAccumulatingSaturating16BitUnsignedAccelerated: VkBool32,
    pub integerDotProductAccumulatingSaturating16BitSignedAccelerated: VkBool32,
    pub integerDotProductAccumulatingSaturating16BitMixedSignednessAccelerated: VkBool32,
    pub integerDotProductAccumulatingSaturating32BitUnsignedAccelerated: VkBool32,
    pub integerDotProductAccumulatingSaturating32BitSignedAccelerated: VkBool32,
    pub integerDotProductAccumulatingSaturating32BitMixedSignednessAccelerated: VkBool32,
    pub integerDotProductAccumulatingSaturating64BitUnsignedAccelerated: VkBool32,
    pub integerDotProductAccumulatingSaturating64BitSignedAccelerated: VkBool32,
    pub integerDotProductAccumulatingSaturating64BitMixedSignednessAccelerated: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceShaderSubgroupExtendedTypesFeatures {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub shaderSubgroupExtendedTypes: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceShaderSubgroupRotateFeatures {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub shaderSubgroupRotate: VkBool32,
    pub shaderSubgroupRotateClustered: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceShaderTerminateInvocationFeatures {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub shaderTerminateInvocation: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceSparseImageFormatInfo2 {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub format: VkFormat,
    pub typ: VkImageType,
    pub samples: VkSampleCountFlagBits,
    pub usage: VkImageUsageFlags,
    pub tiling: VkImageTiling,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceSparseProperties {
    pub residencyStandard2DBlockShape: VkBool32,
    pub residencyStandard2DMultisampleBlockShape: VkBool32,
    pub residencyStandard3DBlockShape: VkBool32,
    pub residencyAlignedMipSize: VkBool32,
    pub residencyNonResidentStrict: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceSubgroupProperties {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub subgroupSize: u32,
    pub supportedStages: VkShaderStageFlags,
    pub supportedOperations: VkSubgroupFeatureFlags,
    pub quadOperationsInAllStages: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceSubgroupSizeControlFeatures {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub subgroupSizeControl: VkBool32,
    pub computeFullSubgroups: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceSubgroupSizeControlProperties {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub minSubgroupSize: u32,
    pub maxSubgroupSize: u32,
    pub maxComputeWorkgroupSubgroups: u32,
    pub requiredSubgroupSizeStages: VkShaderStageFlags,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceSynchronization2Features {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub synchronization2: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceTexelBufferAlignmentProperties {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub storageTexelBufferOffsetAlignmentBytes: VkDeviceSize,
    pub storageTexelBufferOffsetSingleTexelAlignment: VkBool32,
    pub uniformTexelBufferOffsetAlignmentBytes: VkDeviceSize,
    pub uniformTexelBufferOffsetSingleTexelAlignment: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceTextureCompressionASTCHDRFeatures {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub textureCompressionASTC_HDR: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceTimelineSemaphoreFeatures {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub timelineSemaphore: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceTimelineSemaphoreProperties {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub maxTimelineSemaphoreValueDifference: u64,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceToolProperties {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub name: [c_char; VK_MAX_EXTENSION_NAME_SIZE as usize],
    pub version: [c_char; VK_MAX_EXTENSION_NAME_SIZE as usize],
    pub purposes: VkToolPurposeFlags,
    pub description: [c_char; VK_MAX_DESCRIPTION_SIZE as usize],
    pub layer: [c_char; VK_MAX_EXTENSION_NAME_SIZE as usize],
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceUniformBufferStandardLayoutFeatures {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub uniformBufferStandardLayout: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceVariablePointersFeatures {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub variablePointersStorageBuffer: VkBool32,
    pub variablePointers: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceVertexAttributeDivisorFeatures {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub vertexAttributeInstanceRateDivisor: VkBool32,
    pub vertexAttributeInstanceRateZeroDivisor: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceVertexAttributeDivisorProperties {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub maxVertexAttribDivisor: u32,
    pub supportsNonZeroFirstInstance: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceVulkan11Features {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub storageBuffer16BitAccess: VkBool32,
    pub uniformAndStorageBuffer16BitAccess: VkBool32,
    pub storagePushConstant16: VkBool32,
    pub storageInputOutput16: VkBool32,
    pub multiview: VkBool32,
    pub multiviewGeometryShader: VkBool32,
    pub multiviewTessellationShader: VkBool32,
    pub variablePointersStorageBuffer: VkBool32,
    pub variablePointers: VkBool32,
    pub protectedMemory: VkBool32,
    pub samplerYcbcrConversion: VkBool32,
    pub shaderDrawParameters: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceVulkan11Properties {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub deviceUUID: [u8; VK_UUID_SIZE as usize],
    pub driverUUID: [u8; VK_UUID_SIZE as usize],
    pub deviceLUID: [u8; VK_LUID_SIZE as usize],
    pub deviceNodeMask: u32,
    pub deviceLUIDValid: VkBool32,
    pub subgroupSize: u32,
    pub subgroupSupportedStages: VkShaderStageFlags,
    pub subgroupSupportedOperations: VkSubgroupFeatureFlags,
    pub subgroupQuadOperationsInAllStages: VkBool32,
    pub pointClippingBehavior: VkPointClippingBehavior,
    pub maxMultiviewViewCount: u32,
    pub maxMultiviewInstanceIndex: u32,
    pub protectedNoFault: VkBool32,
    pub maxPerSetDescriptors: u32,
    pub maxMemoryAllocationSize: VkDeviceSize,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceVulkan12Features {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub samplerMirrorClampToEdge: VkBool32,
    pub drawIndirectCount: VkBool32,
    pub storageBuffer8BitAccess: VkBool32,
    pub uniformAndStorageBuffer8BitAccess: VkBool32,
    pub storagePushConstant8: VkBool32,
    pub shaderBufferInt64Atomics: VkBool32,
    pub shaderSharedInt64Atomics: VkBool32,
    pub shaderFloat16: VkBool32,
    pub shaderInt8: VkBool32,
    pub descriptorIndexing: VkBool32,
    pub shaderInputAttachmentArrayDynamicIndexing: VkBool32,
    pub shaderUniformTexelBufferArrayDynamicIndexing: VkBool32,
    pub shaderStorageTexelBufferArrayDynamicIndexing: VkBool32,
    pub shaderUniformBufferArrayNonUniformIndexing: VkBool32,
    pub shaderSampledImageArrayNonUniformIndexing: VkBool32,
    pub shaderStorageBufferArrayNonUniformIndexing: VkBool32,
    pub shaderStorageImageArrayNonUniformIndexing: VkBool32,
    pub shaderInputAttachmentArrayNonUniformIndexing: VkBool32,
    pub shaderUniformTexelBufferArrayNonUniformIndexing: VkBool32,
    pub shaderStorageTexelBufferArrayNonUniformIndexing: VkBool32,
    pub descriptorBindingUniformBufferUpdateAfterBind: VkBool32,
    pub descriptorBindingSampledImageUpdateAfterBind: VkBool32,
    pub descriptorBindingStorageImageUpdateAfterBind: VkBool32,
    pub descriptorBindingStorageBufferUpdateAfterBind: VkBool32,
    pub descriptorBindingUniformTexelBufferUpdateAfterBind: VkBool32,
    pub descriptorBindingStorageTexelBufferUpdateAfterBind: VkBool32,
    pub descriptorBindingUpdateUnusedWhilePending: VkBool32,
    pub descriptorBindingPartiallyBound: VkBool32,
    pub descriptorBindingVariableDescriptorCount: VkBool32,
    pub runtimeDescriptorArray: VkBool32,
    pub samplerFilterMinmax: VkBool32,
    pub scalarBlockLayout: VkBool32,
    pub imagelessFramebuffer: VkBool32,
    pub uniformBufferStandardLayout: VkBool32,
    pub shaderSubgroupExtendedTypes: VkBool32,
    pub separateDepthStencilLayouts: VkBool32,
    pub hostQueryReset: VkBool32,
    pub timelineSemaphore: VkBool32,
    pub bufferDeviceAddress: VkBool32,
    pub bufferDeviceAddressCaptureReplay: VkBool32,
    pub bufferDeviceAddressMultiDevice: VkBool32,
    pub vulkanMemoryModel: VkBool32,
    pub vulkanMemoryModelDeviceScope: VkBool32,
    pub vulkanMemoryModelAvailabilityVisibilityChains: VkBool32,
    pub shaderOutputViewportIndex: VkBool32,
    pub shaderOutputLayer: VkBool32,
    pub subgroupBroadcastDynamicId: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceVulkan12Properties {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub driverID: VkDriverId,
    pub driverName: [c_char; VK_MAX_DRIVER_NAME_SIZE as usize],
    pub driverInfo: [c_char; VK_MAX_DRIVER_INFO_SIZE as usize],
    pub conformanceVersion: VkConformanceVersion,
    pub denormBehaviorIndependence: VkShaderFloatControlsIndependence,
    pub roundingModeIndependence: VkShaderFloatControlsIndependence,
    pub shaderSignedZeroInfNanPreserveFloat16: VkBool32,
    pub shaderSignedZeroInfNanPreserveFloat32: VkBool32,
    pub shaderSignedZeroInfNanPreserveFloat64: VkBool32,
    pub shaderDenormPreserveFloat16: VkBool32,
    pub shaderDenormPreserveFloat32: VkBool32,
    pub shaderDenormPreserveFloat64: VkBool32,
    pub shaderDenormFlushToZeroFloat16: VkBool32,
    pub shaderDenormFlushToZeroFloat32: VkBool32,
    pub shaderDenormFlushToZeroFloat64: VkBool32,
    pub shaderRoundingModeRTEFloat16: VkBool32,
    pub shaderRoundingModeRTEFloat32: VkBool32,
    pub shaderRoundingModeRTEFloat64: VkBool32,
    pub shaderRoundingModeRTZFloat16: VkBool32,
    pub shaderRoundingModeRTZFloat32: VkBool32,
    pub shaderRoundingModeRTZFloat64: VkBool32,
    pub maxUpdateAfterBindDescriptorsInAllPools: u32,
    pub shaderUniformBufferArrayNonUniformIndexingNative: VkBool32,
    pub shaderSampledImageArrayNonUniformIndexingNative: VkBool32,
    pub shaderStorageBufferArrayNonUniformIndexingNative: VkBool32,
    pub shaderStorageImageArrayNonUniformIndexingNative: VkBool32,
    pub shaderInputAttachmentArrayNonUniformIndexingNative: VkBool32,
    pub robustBufferAccessUpdateAfterBind: VkBool32,
    pub quadDivergentImplicitLod: VkBool32,
    pub maxPerStageDescriptorUpdateAfterBindSamplers: u32,
    pub maxPerStageDescriptorUpdateAfterBindUniformBuffers: u32,
    pub maxPerStageDescriptorUpdateAfterBindStorageBuffers: u32,
    pub maxPerStageDescriptorUpdateAfterBindSampledImages: u32,
    pub maxPerStageDescriptorUpdateAfterBindStorageImages: u32,
    pub maxPerStageDescriptorUpdateAfterBindInputAttachments: u32,
    pub maxPerStageUpdateAfterBindResources: u32,
    pub maxDescriptorSetUpdateAfterBindSamplers: u32,
    pub maxDescriptorSetUpdateAfterBindUniformBuffers: u32,
    pub maxDescriptorSetUpdateAfterBindUniformBuffersDynamic: u32,
    pub maxDescriptorSetUpdateAfterBindStorageBuffers: u32,
    pub maxDescriptorSetUpdateAfterBindStorageBuffersDynamic: u32,
    pub maxDescriptorSetUpdateAfterBindSampledImages: u32,
    pub maxDescriptorSetUpdateAfterBindStorageImages: u32,
    pub maxDescriptorSetUpdateAfterBindInputAttachments: u32,
    pub supportedDepthResolveModes: VkResolveModeFlags,
    pub supportedStencilResolveModes: VkResolveModeFlags,
    pub independentResolveNone: VkBool32,
    pub independentResolve: VkBool32,
    pub filterMinmaxSingleComponentFormats: VkBool32,
    pub filterMinmaxImageComponentMapping: VkBool32,
    pub maxTimelineSemaphoreValueDifference: u64,
    pub framebufferIntegerColorSampleCounts: VkSampleCountFlags,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceVulkan13Features {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub robustImageAccess: VkBool32,
    pub inlineUniformBlock: VkBool32,
    pub descriptorBindingInlineUniformBlockUpdateAfterBind: VkBool32,
    pub pipelineCreationCacheControl: VkBool32,
    pub privateData: VkBool32,
    pub shaderDemoteToHelperInvocation: VkBool32,
    pub shaderTerminateInvocation: VkBool32,
    pub subgroupSizeControl: VkBool32,
    pub computeFullSubgroups: VkBool32,
    pub synchronization2: VkBool32,
    pub textureCompressionASTC_HDR: VkBool32,
    pub shaderZeroInitializeWorkgroupMemory: VkBool32,
    pub dynamicRendering: VkBool32,
    pub shaderIntegerDotProduct: VkBool32,
    pub maintenance4: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceVulkan13Properties {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub minSubgroupSize: u32,
    pub maxSubgroupSize: u32,
    pub maxComputeWorkgroupSubgroups: u32,
    pub requiredSubgroupSizeStages: VkShaderStageFlags,
    pub maxInlineUniformBlockSize: u32,
    pub maxPerStageDescriptorInlineUniformBlocks: u32,
    pub maxPerStageDescriptorUpdateAfterBindInlineUniformBlocks: u32,
    pub maxDescriptorSetInlineUniformBlocks: u32,
    pub maxDescriptorSetUpdateAfterBindInlineUniformBlocks: u32,
    pub maxInlineUniformTotalSize: u32,
    pub integerDotProduct8BitUnsignedAccelerated: VkBool32,
    pub integerDotProduct8BitSignedAccelerated: VkBool32,
    pub integerDotProduct8BitMixedSignednessAccelerated: VkBool32,
    pub integerDotProduct4x8BitPackedUnsignedAccelerated: VkBool32,
    pub integerDotProduct4x8BitPackedSignedAccelerated: VkBool32,
    pub integerDotProduct4x8BitPackedMixedSignednessAccelerated: VkBool32,
    pub integerDotProduct16BitUnsignedAccelerated: VkBool32,
    pub integerDotProduct16BitSignedAccelerated: VkBool32,
    pub integerDotProduct16BitMixedSignednessAccelerated: VkBool32,
    pub integerDotProduct32BitUnsignedAccelerated: VkBool32,
    pub integerDotProduct32BitSignedAccelerated: VkBool32,
    pub integerDotProduct32BitMixedSignednessAccelerated: VkBool32,
    pub integerDotProduct64BitUnsignedAccelerated: VkBool32,
    pub integerDotProduct64BitSignedAccelerated: VkBool32,
    pub integerDotProduct64BitMixedSignednessAccelerated: VkBool32,
    pub integerDotProductAccumulatingSaturating8BitUnsignedAccelerated: VkBool32,
    pub integerDotProductAccumulatingSaturating8BitSignedAccelerated: VkBool32,
    pub integerDotProductAccumulatingSaturating8BitMixedSignednessAccelerated: VkBool32,
    pub integerDotProductAccumulatingSaturating4x8BitPackedUnsignedAccelerated: VkBool32,
    pub integerDotProductAccumulatingSaturating4x8BitPackedSignedAccelerated: VkBool32,
    pub integerDotProductAccumulatingSaturating4x8BitPackedMixedSignednessAccelerated: VkBool32,
    pub integerDotProductAccumulatingSaturating16BitUnsignedAccelerated: VkBool32,
    pub integerDotProductAccumulatingSaturating16BitSignedAccelerated: VkBool32,
    pub integerDotProductAccumulatingSaturating16BitMixedSignednessAccelerated: VkBool32,
    pub integerDotProductAccumulatingSaturating32BitUnsignedAccelerated: VkBool32,
    pub integerDotProductAccumulatingSaturating32BitSignedAccelerated: VkBool32,
    pub integerDotProductAccumulatingSaturating32BitMixedSignednessAccelerated: VkBool32,
    pub integerDotProductAccumulatingSaturating64BitUnsignedAccelerated: VkBool32,
    pub integerDotProductAccumulatingSaturating64BitSignedAccelerated: VkBool32,
    pub integerDotProductAccumulatingSaturating64BitMixedSignednessAccelerated: VkBool32,
    pub storageTexelBufferOffsetAlignmentBytes: VkDeviceSize,
    pub storageTexelBufferOffsetSingleTexelAlignment: VkBool32,
    pub uniformTexelBufferOffsetAlignmentBytes: VkDeviceSize,
    pub uniformTexelBufferOffsetSingleTexelAlignment: VkBool32,
    pub maxBufferSize: VkDeviceSize,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceVulkan14Features {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub globalPriorityQuery: VkBool32,
    pub shaderSubgroupRotate: VkBool32,
    pub shaderSubgroupRotateClustered: VkBool32,
    pub shaderFloatControls2: VkBool32,
    pub shaderExpectAssume: VkBool32,
    pub rectangularLines: VkBool32,
    pub bresenhamLines: VkBool32,
    pub smoothLines: VkBool32,
    pub stippledRectangularLines: VkBool32,
    pub stippledBresenhamLines: VkBool32,
    pub stippledSmoothLines: VkBool32,
    pub vertexAttributeInstanceRateDivisor: VkBool32,
    pub vertexAttributeInstanceRateZeroDivisor: VkBool32,
    pub indexTypeUint8: VkBool32,
    pub dynamicRenderingLocalRead: VkBool32,
    pub maintenance5: VkBool32,
    pub maintenance6: VkBool32,
    pub pipelineProtectedAccess: VkBool32,
    pub pipelineRobustness: VkBool32,
    pub hostImageCopy: VkBool32,
    pub pushDescriptor: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceVulkan14Properties {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub lineSubPixelPrecisionBits: u32,
    pub maxVertexAttribDivisor: u32,
    pub supportsNonZeroFirstInstance: VkBool32,
    pub maxPushDescriptors: u32,
    pub dynamicRenderingLocalReadDepthStencilAttachments: VkBool32,
    pub dynamicRenderingLocalReadMultisampledAttachments: VkBool32,
    pub earlyFragmentMultisampleCoverageAfterSampleCounting: VkBool32,
    pub earlyFragmentSampleMaskTestBeforeSampleCounting: VkBool32,
    pub depthStencilSwizzleOneSupport: VkBool32,
    pub polygonModePointSize: VkBool32,
    pub nonStrictSinglePixelWideLinesUseParallelogram: VkBool32,
    pub nonStrictWideLinesUseParallelogram: VkBool32,
    pub blockTexelViewCompatibleMultipleLayers: VkBool32,
    pub maxCombinedImageSamplerDescriptorCount: u32,
    pub fragmentShadingRateClampCombinerInputs: VkBool32,
    pub defaultRobustnessStorageBuffers: VkPipelineRobustnessBufferBehavior,
    pub defaultRobustnessUniformBuffers: VkPipelineRobustnessBufferBehavior,
    pub defaultRobustnessVertexInputs: VkPipelineRobustnessBufferBehavior,
    pub defaultRobustnessImages: VkPipelineRobustnessImageBehavior,
    pub copySrcLayoutCount: u32,
    pub pCopySrcLayouts: *mut VkImageLayout,
    pub copyDstLayoutCount: u32,
    pub pCopyDstLayouts: *mut VkImageLayout,
    pub optimalTilingLayoutUUID: [u8; VK_UUID_SIZE as usize],
    pub identicalMemoryTypeRequirements: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceVulkanMemoryModelFeatures {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub vulkanMemoryModel: VkBool32,
    pub vulkanMemoryModelDeviceScope: VkBool32,
    pub vulkanMemoryModelAvailabilityVisibilityChains: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceZeroInitializeWorkgroupMemoryFeatures {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub shaderZeroInitializeWorkgroupMemory: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineCacheCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineCacheCreateFlags,
    pub initialDataSize: usize,
    pub pInitialData: *const c_void,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineCacheHeaderVersionOne {
    pub headerSize: u32,
    pub headerVersion: VkPipelineCacheHeaderVersion,
    pub vendorID: u32,
    pub deviceID: u32,
    pub pipelineCacheUUID: [u8; VK_UUID_SIZE as usize],
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineColorBlendAttachmentState {
    pub blendEnable: VkBool32,
    pub srcColorBlendFactor: VkBlendFactor,
    pub dstColorBlendFactor: VkBlendFactor,
    pub colorBlendOp: VkBlendOp,
    pub srcAlphaBlendFactor: VkBlendFactor,
    pub dstAlphaBlendFactor: VkBlendFactor,
    pub alphaBlendOp: VkBlendOp,
    pub colorWriteMask: VkColorComponentFlags,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineColorBlendStateCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineColorBlendStateCreateFlags,
    pub logicOpEnable: VkBool32,
    pub logicOp: VkLogicOp,
    pub attachmentCount: u32,
    pub pAttachments: *const VkPipelineColorBlendAttachmentState,
    pub blendConstants: [c_float; 4 as usize],
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineCreateFlags2CreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineCreateFlags2,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineCreationFeedback {
    pub flags: VkPipelineCreationFeedbackFlags,
    pub duration: u64,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineCreationFeedbackCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub pPipelineCreationFeedback: *mut VkPipelineCreationFeedback,
    pub pipelineStageCreationFeedbackCount: u32,
    pub pPipelineStageCreationFeedbacks: *mut VkPipelineCreationFeedback,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineDepthStencilStateCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineDepthStencilStateCreateFlags,
    pub depthTestEnable: VkBool32,
    pub depthWriteEnable: VkBool32,
    pub depthCompareOp: VkCompareOp,
    pub depthBoundsTestEnable: VkBool32,
    pub stencilTestEnable: VkBool32,
    pub front: VkStencilOpState,
    pub back: VkStencilOpState,
    pub minDepthBounds: c_float,
    pub maxDepthBounds: c_float,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineDynamicStateCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineDynamicStateCreateFlags,
    pub dynamicStateCount: u32,
    pub pDynamicStates: *const VkDynamicState,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineInputAssemblyStateCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineInputAssemblyStateCreateFlags,
    pub topology: VkPrimitiveTopology,
    pub primitiveRestartEnable: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineLayoutCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineLayoutCreateFlags,
    pub setLayoutCount: u32,
    pub pSetLayouts: *const VkDescriptorSetLayout,
    pub pushConstantRangeCount: u32,
    pub pPushConstantRanges: *const VkPushConstantRange,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineMultisampleStateCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineMultisampleStateCreateFlags,
    pub rasterizationSamples: VkSampleCountFlagBits,
    pub sampleShadingEnable: VkBool32,
    pub minSampleShading: c_float,
    pub pSampleMask: *const VkSampleMask,
    pub alphaToCoverageEnable: VkBool32,
    pub alphaToOneEnable: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineRasterizationLineStateCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub lineRasterizationMode: VkLineRasterizationMode,
    pub stippledLineEnable: VkBool32,
    pub lineStippleFactor: u32,
    pub lineStipplePattern: u16,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineRasterizationStateCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineRasterizationStateCreateFlags,
    pub depthClampEnable: VkBool32,
    pub rasterizerDiscardEnable: VkBool32,
    pub polygonMode: VkPolygonMode,
    pub cullMode: VkCullModeFlags,
    pub frontFace: VkFrontFace,
    pub depthBiasEnable: VkBool32,
    pub depthBiasConstantFactor: c_float,
    pub depthBiasClamp: c_float,
    pub depthBiasSlopeFactor: c_float,
    pub lineWidth: c_float,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineRenderingCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub viewMask: u32,
    pub colorAttachmentCount: u32,
    pub pColorAttachmentFormats: *const VkFormat,
    pub depthAttachmentFormat: VkFormat,
    pub stencilAttachmentFormat: VkFormat,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineRobustnessCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub storageBuffers: VkPipelineRobustnessBufferBehavior,
    pub uniformBuffers: VkPipelineRobustnessBufferBehavior,
    pub vertexInputs: VkPipelineRobustnessBufferBehavior,
    pub images: VkPipelineRobustnessImageBehavior,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineShaderStageCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineShaderStageCreateFlags,
    pub stage: VkShaderStageFlagBits,
    pub module: VkShaderModule,
    pub pName: *const c_char,
    pub pSpecializationInfo: *const VkSpecializationInfo,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineShaderStageRequiredSubgroupSizeCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub requiredSubgroupSize: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineTessellationDomainOriginStateCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub domainOrigin: VkTessellationDomainOrigin,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineTessellationStateCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineTessellationStateCreateFlags,
    pub patchControlPoints: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineVertexInputDivisorStateCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub vertexBindingDivisorCount: u32,
    pub pVertexBindingDivisors: *const VkVertexInputBindingDivisorDescription,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineVertexInputStateCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineVertexInputStateCreateFlags,
    pub vertexBindingDescriptionCount: u32,
    pub pVertexBindingDescriptions: *const VkVertexInputBindingDescription,
    pub vertexAttributeDescriptionCount: u32,
    pub pVertexAttributeDescriptions: *const VkVertexInputAttributeDescription,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineViewportStateCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineViewportStateCreateFlags,
    pub viewportCount: u32,
    pub pViewports: *const VkViewport,
    pub scissorCount: u32,
    pub pScissors: *const VkRect2D,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPrivateDataSlotCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPrivateDataSlotCreateFlags,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkProtectedSubmitInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub protectedSubmit: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPushConstantRange {
    pub stageFlags: VkShaderStageFlags,
    pub offset: u32,
    pub size: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPushConstantsInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub layout: VkPipelineLayout,
    pub stageFlags: VkShaderStageFlags,
    pub offset: u32,
    pub size: u32,
    pub pValues: *const c_void,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPushDescriptorSetInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub stageFlags: VkShaderStageFlags,
    pub layout: VkPipelineLayout,
    pub set: u32,
    pub descriptorWriteCount: u32,
    pub pDescriptorWrites: *const VkWriteDescriptorSet,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPushDescriptorSetWithTemplateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub descriptorUpdateTemplate: VkDescriptorUpdateTemplate,
    pub layout: VkPipelineLayout,
    pub set: u32,
    pub pData: *const c_void,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkQueryPoolCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkQueryPoolCreateFlags,
    pub queryType: VkQueryType,
    pub queryCount: u32,
    pub pipelineStatistics: VkQueryPipelineStatisticFlags,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkQueueFamilyGlobalPriorityProperties {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub priorityCount: u32,
    pub priorities: [VkQueueGlobalPriority; VK_MAX_GLOBAL_PRIORITY_SIZE as usize],
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkQueueFamilyProperties {
    pub queueFlags: VkQueueFlags,
    pub queueCount: u32,
    pub timestampValidBits: u32,
    pub minImageTransferGranularity: VkExtent3D,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkQueueFamilyProperties2 {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub queueFamilyProperties: VkQueueFamilyProperties,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkRect2D {
    pub offset: VkOffset2D,
    pub extent: VkExtent2D,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkRenderPassAttachmentBeginInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub attachmentCount: u32,
    pub pAttachments: *const VkImageView,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkRenderPassBeginInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub renderPass: VkRenderPass,
    pub framebuffer: VkFramebuffer,
    pub renderArea: VkRect2D,
    pub clearValueCount: u32,
    pub pClearValues: *const VkClearValue,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkRenderPassCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkRenderPassCreateFlags,
    pub attachmentCount: u32,
    pub pAttachments: *const VkAttachmentDescription,
    pub subpassCount: u32,
    pub pSubpasses: *const VkSubpassDescription,
    pub dependencyCount: u32,
    pub pDependencies: *const VkSubpassDependency,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkRenderPassCreateInfo2 {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkRenderPassCreateFlags,
    pub attachmentCount: u32,
    pub pAttachments: *const VkAttachmentDescription2,
    pub subpassCount: u32,
    pub pSubpasses: *const VkSubpassDescription2,
    pub dependencyCount: u32,
    pub pDependencies: *const VkSubpassDependency2,
    pub correlatedViewMaskCount: u32,
    pub pCorrelatedViewMasks: *const u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkRenderPassInputAttachmentAspectCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub aspectReferenceCount: u32,
    pub pAspectReferences: *const VkInputAttachmentAspectReference,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkRenderPassMultiviewCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub subpassCount: u32,
    pub pViewMasks: *const u32,
    pub dependencyCount: u32,
    pub pViewOffsets: *const i32,
    pub correlationMaskCount: u32,
    pub pCorrelationMasks: *const u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkRenderingAreaInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub viewMask: u32,
    pub colorAttachmentCount: u32,
    pub pColorAttachmentFormats: *const VkFormat,
    pub depthAttachmentFormat: VkFormat,
    pub stencilAttachmentFormat: VkFormat,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkRenderingAttachmentInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub imageView: VkImageView,
    pub imageLayout: VkImageLayout,
    pub resolveMode: VkResolveModeFlagBits,
    pub resolveImageView: VkImageView,
    pub resolveImageLayout: VkImageLayout,
    pub loadOp: VkAttachmentLoadOp,
    pub storeOp: VkAttachmentStoreOp,
    pub clearValue: VkClearValue,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkRenderingAttachmentLocationInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub colorAttachmentCount: u32,
    pub pColorAttachmentLocations: *const u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkRenderingInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkRenderingFlags,
    pub renderArea: VkRect2D,
    pub layerCount: u32,
    pub viewMask: u32,
    pub colorAttachmentCount: u32,
    pub pColorAttachments: *const VkRenderingAttachmentInfo,
    pub pDepthAttachment: *const VkRenderingAttachmentInfo,
    pub pStencilAttachment: *const VkRenderingAttachmentInfo,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkRenderingInputAttachmentIndexInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub colorAttachmentCount: u32,
    pub pColorAttachmentInputIndices: *const u32,
    pub pDepthInputAttachmentIndex: *const u32,
    pub pStencilInputAttachmentIndex: *const u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkResolveImageInfo2 {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub srcImage: VkImage,
    pub srcImageLayout: VkImageLayout,
    pub dstImage: VkImage,
    pub dstImageLayout: VkImageLayout,
    pub regionCount: u32,
    pub pRegions: *const VkImageResolve2,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSamplerCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkSamplerCreateFlags,
    pub magFilter: VkFilter,
    pub minFilter: VkFilter,
    pub mipmapMode: VkSamplerMipmapMode,
    pub addressModeU: VkSamplerAddressMode,
    pub addressModeV: VkSamplerAddressMode,
    pub addressModeW: VkSamplerAddressMode,
    pub mipLodBias: c_float,
    pub anisotropyEnable: VkBool32,
    pub maxAnisotropy: c_float,
    pub compareEnable: VkBool32,
    pub compareOp: VkCompareOp,
    pub minLod: c_float,
    pub maxLod: c_float,
    pub borderColor: VkBorderColor,
    pub unnormalizedCoordinates: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSamplerReductionModeCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub reductionMode: VkSamplerReductionMode,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSamplerYcbcrConversionCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub format: VkFormat,
    pub ycbcrModel: VkSamplerYcbcrModelConversion,
    pub ycbcrRange: VkSamplerYcbcrRange,
    pub components: VkComponentMapping,
    pub xChromaOffset: VkChromaLocation,
    pub yChromaOffset: VkChromaLocation,
    pub chromaFilter: VkFilter,
    pub forceExplicitReconstruction: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSamplerYcbcrConversionImageFormatProperties {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub combinedImageSamplerDescriptorCount: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSamplerYcbcrConversionInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub conversion: VkSamplerYcbcrConversion,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSemaphoreCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkSemaphoreCreateFlags,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSemaphoreSignalInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub semaphore: VkSemaphore,
    pub value: u64,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSemaphoreSubmitInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub semaphore: VkSemaphore,
    pub value: u64,
    pub stageMask: VkPipelineStageFlags2,
    pub deviceIndex: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSemaphoreTypeCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub semaphoreType: VkSemaphoreType,
    pub initialValue: u64,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSemaphoreWaitInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkSemaphoreWaitFlags,
    pub semaphoreCount: u32,
    pub pSemaphores: *const VkSemaphore,
    pub pValues: *const u64,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkShaderModuleCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkShaderModuleCreateFlags,
    pub codeSize: usize,
    pub pCode: *const u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSparseBufferMemoryBindInfo {
    pub buffer: VkBuffer,
    pub bindCount: u32,
    pub pBinds: *const VkSparseMemoryBind,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSparseImageFormatProperties {
    pub aspectMask: VkImageAspectFlags,
    pub imageGranularity: VkExtent3D,
    pub flags: VkSparseImageFormatFlags,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSparseImageFormatProperties2 {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub properties: VkSparseImageFormatProperties,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSparseImageMemoryBind {
    pub subresource: VkImageSubresource,
    pub offset: VkOffset3D,
    pub extent: VkExtent3D,
    pub memory: VkDeviceMemory,
    pub memoryOffset: VkDeviceSize,
    pub flags: VkSparseMemoryBindFlags,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSparseImageMemoryBindInfo {
    pub image: VkImage,
    pub bindCount: u32,
    pub pBinds: *const VkSparseImageMemoryBind,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSparseImageMemoryRequirements {
    pub formatProperties: VkSparseImageFormatProperties,
    pub imageMipTailFirstLod: u32,
    pub imageMipTailSize: VkDeviceSize,
    pub imageMipTailOffset: VkDeviceSize,
    pub imageMipTailStride: VkDeviceSize,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSparseImageMemoryRequirements2 {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub memoryRequirements: VkSparseImageMemoryRequirements,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSparseImageOpaqueMemoryBindInfo {
    pub image: VkImage,
    pub bindCount: u32,
    pub pBinds: *const VkSparseMemoryBind,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSparseMemoryBind {
    pub resourceOffset: VkDeviceSize,
    pub size: VkDeviceSize,
    pub memory: VkDeviceMemory,
    pub memoryOffset: VkDeviceSize,
    pub flags: VkSparseMemoryBindFlags,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSpecializationInfo {
    pub mapEntryCount: u32,
    pub pMapEntries: *const VkSpecializationMapEntry,
    pub dataSize: usize,
    pub pData: *const c_void,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSpecializationMapEntry {
    pub constantID: u32,
    pub offset: u32,
    pub size: usize,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkStencilOpState {
    pub failOp: VkStencilOp,
    pub passOp: VkStencilOp,
    pub depthFailOp: VkStencilOp,
    pub compareOp: VkCompareOp,
    pub compareMask: u32,
    pub writeMask: u32,
    pub reference: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSubmitInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub waitSemaphoreCount: u32,
    pub pWaitSemaphores: *const VkSemaphore,
    pub pWaitDstStageMask: *const VkPipelineStageFlags,
    pub commandBufferCount: u32,
    pub pCommandBuffers: *const VkCommandBuffer,
    pub signalSemaphoreCount: u32,
    pub pSignalSemaphores: *const VkSemaphore,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSubmitInfo2 {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkSubmitFlags,
    pub waitSemaphoreInfoCount: u32,
    pub pWaitSemaphoreInfos: *const VkSemaphoreSubmitInfo,
    pub commandBufferInfoCount: u32,
    pub pCommandBufferInfos: *const VkCommandBufferSubmitInfo,
    pub signalSemaphoreInfoCount: u32,
    pub pSignalSemaphoreInfos: *const VkSemaphoreSubmitInfo,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSubpassBeginInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub contents: VkSubpassContents,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSubpassDependency {
    pub srcSubpass: u32,
    pub dstSubpass: u32,
    pub srcStageMask: VkPipelineStageFlags,
    pub dstStageMask: VkPipelineStageFlags,
    pub srcAccessMask: VkAccessFlags,
    pub dstAccessMask: VkAccessFlags,
    pub dependencyFlags: VkDependencyFlags,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSubpassDependency2 {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub srcSubpass: u32,
    pub dstSubpass: u32,
    pub srcStageMask: VkPipelineStageFlags,
    pub dstStageMask: VkPipelineStageFlags,
    pub srcAccessMask: VkAccessFlags,
    pub dstAccessMask: VkAccessFlags,
    pub dependencyFlags: VkDependencyFlags,
    pub viewOffset: i32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSubpassDescription {
    pub flags: VkSubpassDescriptionFlags,
    pub pipelineBindPoint: VkPipelineBindPoint,
    pub inputAttachmentCount: u32,
    pub pInputAttachments: *const VkAttachmentReference,
    pub colorAttachmentCount: u32,
    pub pColorAttachments: *const VkAttachmentReference,
    pub pResolveAttachments: *const VkAttachmentReference,
    pub pDepthStencilAttachment: *const VkAttachmentReference,
    pub preserveAttachmentCount: u32,
    pub pPreserveAttachments: *const u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSubpassDescription2 {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkSubpassDescriptionFlags,
    pub pipelineBindPoint: VkPipelineBindPoint,
    pub viewMask: u32,
    pub inputAttachmentCount: u32,
    pub pInputAttachments: *const VkAttachmentReference2,
    pub colorAttachmentCount: u32,
    pub pColorAttachments: *const VkAttachmentReference2,
    pub pResolveAttachments: *const VkAttachmentReference2,
    pub pDepthStencilAttachment: *const VkAttachmentReference2,
    pub preserveAttachmentCount: u32,
    pub pPreserveAttachments: *const u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSubpassDescriptionDepthStencilResolve {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub depthResolveMode: VkResolveModeFlagBits,
    pub stencilResolveMode: VkResolveModeFlagBits,
    pub pDepthStencilResolveAttachment: *const VkAttachmentReference2,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSubpassEndInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSubresourceHostMemcpySize {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub size: VkDeviceSize,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSubresourceLayout {
    pub offset: VkDeviceSize,
    pub size: VkDeviceSize,
    pub rowPitch: VkDeviceSize,
    pub arrayPitch: VkDeviceSize,
    pub depthPitch: VkDeviceSize,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSubresourceLayout2 {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub subresourceLayout: VkSubresourceLayout,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkTimelineSemaphoreSubmitInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub waitSemaphoreValueCount: u32,
    pub pWaitSemaphoreValues: *const u64,
    pub signalSemaphoreValueCount: u32,
    pub pSignalSemaphoreValues: *const u64,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVertexInputAttributeDescription {
    pub location: u32,
    pub binding: u32,
    pub format: VkFormat,
    pub offset: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVertexInputBindingDescription {
    pub binding: u32,
    pub stride: u32,
    pub inputRate: VkVertexInputRate,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVertexInputBindingDivisorDescription {
    pub binding: u32,
    pub divisor: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkViewport {
    pub x: c_float,
    pub y: c_float,
    pub width: c_float,
    pub height: c_float,
    pub minDepth: c_float,
    pub maxDepth: c_float,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkWriteDescriptorSet {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub dstSet: VkDescriptorSet,
    pub dstBinding: u32,
    pub dstArrayElement: u32,
    pub descriptorCount: u32,
    pub descriptorType: VkDescriptorType,
    pub pImageInfo: *const VkDescriptorImageInfo,
    pub pBufferInfo: *const VkDescriptorBufferInfo,
    pub pTexelBufferView: *const VkBufferView,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkWriteDescriptorSetInlineUniformBlock {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub dataSize: u32,
    pub pData: *const c_void,
}

#[cfg_attr(not(doc), repr(u8))]
pub enum VkBufferView_T {
    #[doc(hidden)]
    __variant1,
    #[doc(hidden)]
    __variant2,
}

#[cfg_attr(not(doc), repr(u8))]
pub enum VkBuffer_T {
    #[doc(hidden)]
    __variant1,
    #[doc(hidden)]
    __variant2,
}

#[cfg_attr(not(doc), repr(u8))]
pub enum VkCommandBuffer_T {
    #[doc(hidden)]
    __variant1,
    #[doc(hidden)]
    __variant2,
}

#[cfg_attr(not(doc), repr(u8))]
pub enum VkCommandPool_T {
    #[doc(hidden)]
    __variant1,
    #[doc(hidden)]
    __variant2,
}

#[cfg_attr(not(doc), repr(u8))]
pub enum VkDescriptorPool_T {
    #[doc(hidden)]
    __variant1,
    #[doc(hidden)]
    __variant2,
}

#[cfg_attr(not(doc), repr(u8))]
pub enum VkDescriptorSetLayout_T {
    #[doc(hidden)]
    __variant1,
    #[doc(hidden)]
    __variant2,
}

#[cfg_attr(not(doc), repr(u8))]
pub enum VkDescriptorSet_T {
    #[doc(hidden)]
    __variant1,
    #[doc(hidden)]
    __variant2,
}

#[cfg_attr(not(doc), repr(u8))]
pub enum VkDescriptorUpdateTemplate_T {
    #[doc(hidden)]
    __variant1,
    #[doc(hidden)]
    __variant2,
}

#[cfg_attr(not(doc), repr(u8))]
pub enum VkDeviceMemory_T {
    #[doc(hidden)]
    __variant1,
    #[doc(hidden)]
    __variant2,
}

#[cfg_attr(not(doc), repr(u8))]
pub enum VkDevice_T {
    #[doc(hidden)]
    __variant1,
    #[doc(hidden)]
    __variant2,
}

#[cfg_attr(not(doc), repr(u8))]
pub enum VkEvent_T {
    #[doc(hidden)]
    __variant1,
    #[doc(hidden)]
    __variant2,
}

#[cfg_attr(not(doc), repr(u8))]
pub enum VkFence_T {
    #[doc(hidden)]
    __variant1,
    #[doc(hidden)]
    __variant2,
}

#[cfg_attr(not(doc), repr(u8))]
pub enum VkFramebuffer_T {
    #[doc(hidden)]
    __variant1,
    #[doc(hidden)]
    __variant2,
}

#[cfg_attr(not(doc), repr(u8))]
pub enum VkImageView_T {
    #[doc(hidden)]
    __variant1,
    #[doc(hidden)]
    __variant2,
}

#[cfg_attr(not(doc), repr(u8))]
pub enum VkImage_T {
    #[doc(hidden)]
    __variant1,
    #[doc(hidden)]
    __variant2,
}

#[cfg_attr(not(doc), repr(u8))]
pub enum VkInstance_T {
    #[doc(hidden)]
    __variant1,
    #[doc(hidden)]
    __variant2,
}

#[cfg_attr(not(doc), repr(u8))]
pub enum VkPhysicalDevice_T {
    #[doc(hidden)]
    __variant1,
    #[doc(hidden)]
    __variant2,
}

#[cfg_attr(not(doc), repr(u8))]
pub enum VkPipelineCache_T {
    #[doc(hidden)]
    __variant1,
    #[doc(hidden)]
    __variant2,
}

#[cfg_attr(not(doc), repr(u8))]
pub enum VkPipelineLayout_T {
    #[doc(hidden)]
    __variant1,
    #[doc(hidden)]
    __variant2,
}

#[cfg_attr(not(doc), repr(u8))]
pub enum VkPipeline_T {
    #[doc(hidden)]
    __variant1,
    #[doc(hidden)]
    __variant2,
}

#[cfg_attr(not(doc), repr(u8))]
pub enum VkPrivateDataSlot_T {
    #[doc(hidden)]
    __variant1,
    #[doc(hidden)]
    __variant2,
}

#[cfg_attr(not(doc), repr(u8))]
pub enum VkQueryPool_T {
    #[doc(hidden)]
    __variant1,
    #[doc(hidden)]
    __variant2,
}

#[cfg_attr(not(doc), repr(u8))]
pub enum VkQueue_T {
    #[doc(hidden)]
    __variant1,
    #[doc(hidden)]
    __variant2,
}

#[cfg_attr(not(doc), repr(u8))]
pub enum VkRenderPass_T {
    #[doc(hidden)]
    __variant1,
    #[doc(hidden)]
    __variant2,
}

#[cfg_attr(not(doc), repr(u8))]
pub enum VkSamplerYcbcrConversion_T {
    #[doc(hidden)]
    __variant1,
    #[doc(hidden)]
    __variant2,
}

#[cfg_attr(not(doc), repr(u8))]
pub enum VkSampler_T {
    #[doc(hidden)]
    __variant1,
    #[doc(hidden)]
    __variant2,
}

#[cfg_attr(not(doc), repr(u8))]
pub enum VkSemaphore_T {
    #[doc(hidden)]
    __variant1,
    #[doc(hidden)]
    __variant2,
}

#[cfg_attr(not(doc), repr(u8))]
pub enum VkShaderModule_T {
    #[doc(hidden)]
    __variant1,
    #[doc(hidden)]
    __variant2,
}

pub const VK_ACCESS_2_COLOR_ATTACHMENT_READ_BIT: VkAccessFlagBits2 = 1 << 7;
pub const VK_ACCESS_2_COLOR_ATTACHMENT_WRITE_BIT: VkAccessFlagBits2 = 1 << 8;
pub const VK_ACCESS_2_DEPTH_STENCIL_ATTACHMENT_READ_BIT: VkAccessFlagBits2 = 1 << 9;
pub const VK_ACCESS_2_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT: VkAccessFlagBits2 = 1 << 10;
pub const VK_ACCESS_2_HOST_READ_BIT: VkAccessFlagBits2 = 1 << 13;
pub const VK_ACCESS_2_HOST_WRITE_BIT: VkAccessFlagBits2 = 1 << 14;
pub const VK_ACCESS_2_INDEX_READ_BIT: VkAccessFlagBits2 = 1 << 1;
pub const VK_ACCESS_2_INDIRECT_COMMAND_READ_BIT: VkAccessFlagBits2 = 1 << 0;
pub const VK_ACCESS_2_INPUT_ATTACHMENT_READ_BIT: VkAccessFlagBits2 = 1 << 4;
pub const VK_ACCESS_2_MEMORY_READ_BIT: VkAccessFlagBits2 = 1 << 15;
pub const VK_ACCESS_2_MEMORY_WRITE_BIT: VkAccessFlagBits2 = 1 << 16;
pub const VK_ACCESS_2_NONE: VkAccessFlagBits2 = 0;
pub const VK_ACCESS_2_SHADER_READ_BIT: VkAccessFlagBits2 = 1 << 5;
pub const VK_ACCESS_2_SHADER_SAMPLED_READ_BIT: VkAccessFlagBits2 = 1 << 32;
pub const VK_ACCESS_2_SHADER_STORAGE_READ_BIT: VkAccessFlagBits2 = 1 << 33;
pub const VK_ACCESS_2_SHADER_STORAGE_WRITE_BIT: VkAccessFlagBits2 = 1 << 34;
pub const VK_ACCESS_2_SHADER_WRITE_BIT: VkAccessFlagBits2 = 1 << 6;
pub const VK_ACCESS_2_TRANSFER_READ_BIT: VkAccessFlagBits2 = 1 << 11;
pub const VK_ACCESS_2_TRANSFER_WRITE_BIT: VkAccessFlagBits2 = 1 << 12;
pub const VK_ACCESS_2_UNIFORM_READ_BIT: VkAccessFlagBits2 = 1 << 3;
pub const VK_ACCESS_2_VERTEX_ATTRIBUTE_READ_BIT: VkAccessFlagBits2 = 1 << 2;
pub const VK_ACCESS_COLOR_ATTACHMENT_READ_BIT: VkAccessFlagBits = 1 << 7;
pub const VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT: VkAccessFlagBits = 1 << 8;
pub const VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_READ_BIT: VkAccessFlagBits = 1 << 9;
pub const VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT: VkAccessFlagBits = 1 << 10;
pub const VK_ACCESS_HOST_READ_BIT: VkAccessFlagBits = 1 << 13;
pub const VK_ACCESS_HOST_WRITE_BIT: VkAccessFlagBits = 1 << 14;
pub const VK_ACCESS_INDEX_READ_BIT: VkAccessFlagBits = 1 << 1;
pub const VK_ACCESS_INDIRECT_COMMAND_READ_BIT: VkAccessFlagBits = 1 << 0;
pub const VK_ACCESS_INPUT_ATTACHMENT_READ_BIT: VkAccessFlagBits = 1 << 4;
pub const VK_ACCESS_MEMORY_READ_BIT: VkAccessFlagBits = 1 << 15;
pub const VK_ACCESS_MEMORY_WRITE_BIT: VkAccessFlagBits = 1 << 16;
pub const VK_ACCESS_NONE: VkAccessFlagBits = 0;
pub const VK_ACCESS_SHADER_READ_BIT: VkAccessFlagBits = 1 << 5;
pub const VK_ACCESS_SHADER_WRITE_BIT: VkAccessFlagBits = 1 << 6;
pub const VK_ACCESS_TRANSFER_READ_BIT: VkAccessFlagBits = 1 << 11;
pub const VK_ACCESS_TRANSFER_WRITE_BIT: VkAccessFlagBits = 1 << 12;
pub const VK_ACCESS_UNIFORM_READ_BIT: VkAccessFlagBits = 1 << 3;
pub const VK_ACCESS_VERTEX_ATTRIBUTE_READ_BIT: VkAccessFlagBits = 1 << 2;
pub const VK_ATTACHMENT_DESCRIPTION_MAY_ALIAS_BIT: VkAttachmentDescriptionFlagBits = 1 << 0;
pub const VK_ATTACHMENT_LOAD_OP_CLEAR: VkAttachmentLoadOp = 1;
pub const VK_ATTACHMENT_LOAD_OP_DONT_CARE: VkAttachmentLoadOp = 2;
pub const VK_ATTACHMENT_LOAD_OP_LOAD: VkAttachmentLoadOp = 0;
pub const VK_ATTACHMENT_LOAD_OP_NONE: VkAttachmentLoadOp = 1000400000;
pub const VK_ATTACHMENT_STORE_OP_DONT_CARE: VkAttachmentStoreOp = 1;
pub const VK_ATTACHMENT_STORE_OP_NONE: VkAttachmentStoreOp = 1000301000;
pub const VK_ATTACHMENT_STORE_OP_STORE: VkAttachmentStoreOp = 0;
pub const VK_ATTACHMENT_UNUSED: u32 = !0;
pub const VK_BLEND_FACTOR_CONSTANT_ALPHA: VkBlendFactor = 12;
pub const VK_BLEND_FACTOR_CONSTANT_COLOR: VkBlendFactor = 10;
pub const VK_BLEND_FACTOR_DST_ALPHA: VkBlendFactor = 8;
pub const VK_BLEND_FACTOR_DST_COLOR: VkBlendFactor = 4;
pub const VK_BLEND_FACTOR_ONE: VkBlendFactor = 1;
pub const VK_BLEND_FACTOR_ONE_MINUS_CONSTANT_ALPHA: VkBlendFactor = 13;
pub const VK_BLEND_FACTOR_ONE_MINUS_CONSTANT_COLOR: VkBlendFactor = 11;
pub const VK_BLEND_FACTOR_ONE_MINUS_DST_ALPHA: VkBlendFactor = 9;
pub const VK_BLEND_FACTOR_ONE_MINUS_DST_COLOR: VkBlendFactor = 5;
pub const VK_BLEND_FACTOR_ONE_MINUS_SRC1_ALPHA: VkBlendFactor = 18;
pub const VK_BLEND_FACTOR_ONE_MINUS_SRC1_COLOR: VkBlendFactor = 16;
pub const VK_BLEND_FACTOR_ONE_MINUS_SRC_ALPHA: VkBlendFactor = 7;
pub const VK_BLEND_FACTOR_ONE_MINUS_SRC_COLOR: VkBlendFactor = 3;
pub const VK_BLEND_FACTOR_SRC1_ALPHA: VkBlendFactor = 17;
pub const VK_BLEND_FACTOR_SRC1_COLOR: VkBlendFactor = 15;
pub const VK_BLEND_FACTOR_SRC_ALPHA: VkBlendFactor = 6;
pub const VK_BLEND_FACTOR_SRC_ALPHA_SATURATE: VkBlendFactor = 14;
pub const VK_BLEND_FACTOR_SRC_COLOR: VkBlendFactor = 2;
pub const VK_BLEND_FACTOR_ZERO: VkBlendFactor = 0;
pub const VK_BLEND_OP_ADD: VkBlendOp = 0;
pub const VK_BLEND_OP_MAX: VkBlendOp = 4;
pub const VK_BLEND_OP_MIN: VkBlendOp = 3;
pub const VK_BLEND_OP_REVERSE_SUBTRACT: VkBlendOp = 2;
pub const VK_BLEND_OP_SUBTRACT: VkBlendOp = 1;
pub const VK_BORDER_COLOR_FLOAT_OPAQUE_BLACK: VkBorderColor = 2;
pub const VK_BORDER_COLOR_FLOAT_OPAQUE_WHITE: VkBorderColor = 4;
pub const VK_BORDER_COLOR_FLOAT_TRANSPARENT_BLACK: VkBorderColor = 0;
pub const VK_BORDER_COLOR_INT_OPAQUE_BLACK: VkBorderColor = 3;
pub const VK_BORDER_COLOR_INT_OPAQUE_WHITE: VkBorderColor = 5;
pub const VK_BORDER_COLOR_INT_TRANSPARENT_BLACK: VkBorderColor = 1;
pub const VK_BUFFER_CREATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT: VkBufferCreateFlagBits = 1 << 4;
pub const VK_BUFFER_CREATE_PROTECTED_BIT: VkBufferCreateFlagBits = 1 << 3;
pub const VK_BUFFER_CREATE_SPARSE_ALIASED_BIT: VkBufferCreateFlagBits = 1 << 2;
pub const VK_BUFFER_CREATE_SPARSE_BINDING_BIT: VkBufferCreateFlagBits = 1 << 0;
pub const VK_BUFFER_CREATE_SPARSE_RESIDENCY_BIT: VkBufferCreateFlagBits = 1 << 1;
pub const VK_BUFFER_USAGE_2_INDEX_BUFFER_BIT: VkBufferUsageFlagBits2 = 1 << 6;
pub const VK_BUFFER_USAGE_2_INDIRECT_BUFFER_BIT: VkBufferUsageFlagBits2 = 1 << 8;
pub const VK_BUFFER_USAGE_2_SHADER_DEVICE_ADDRESS_BIT: VkBufferUsageFlagBits2 = 1 << 17;
pub const VK_BUFFER_USAGE_2_STORAGE_BUFFER_BIT: VkBufferUsageFlagBits2 = 1 << 5;
pub const VK_BUFFER_USAGE_2_STORAGE_TEXEL_BUFFER_BIT: VkBufferUsageFlagBits2 = 1 << 3;
pub const VK_BUFFER_USAGE_2_TRANSFER_DST_BIT: VkBufferUsageFlagBits2 = 1 << 1;
pub const VK_BUFFER_USAGE_2_TRANSFER_SRC_BIT: VkBufferUsageFlagBits2 = 1 << 0;
pub const VK_BUFFER_USAGE_2_UNIFORM_BUFFER_BIT: VkBufferUsageFlagBits2 = 1 << 4;
pub const VK_BUFFER_USAGE_2_UNIFORM_TEXEL_BUFFER_BIT: VkBufferUsageFlagBits2 = 1 << 2;
pub const VK_BUFFER_USAGE_2_VERTEX_BUFFER_BIT: VkBufferUsageFlagBits2 = 1 << 7;
pub const VK_BUFFER_USAGE_INDEX_BUFFER_BIT: VkBufferUsageFlagBits = 1 << 6;
pub const VK_BUFFER_USAGE_INDIRECT_BUFFER_BIT: VkBufferUsageFlagBits = 1 << 8;
pub const VK_BUFFER_USAGE_SHADER_DEVICE_ADDRESS_BIT: VkBufferUsageFlagBits = 1 << 17;
pub const VK_BUFFER_USAGE_STORAGE_BUFFER_BIT: VkBufferUsageFlagBits = 1 << 5;
pub const VK_BUFFER_USAGE_STORAGE_TEXEL_BUFFER_BIT: VkBufferUsageFlagBits = 1 << 3;
pub const VK_BUFFER_USAGE_TRANSFER_DST_BIT: VkBufferUsageFlagBits = 1 << 1;
pub const VK_BUFFER_USAGE_TRANSFER_SRC_BIT: VkBufferUsageFlagBits = 1 << 0;
pub const VK_BUFFER_USAGE_UNIFORM_BUFFER_BIT: VkBufferUsageFlagBits = 1 << 4;
pub const VK_BUFFER_USAGE_UNIFORM_TEXEL_BUFFER_BIT: VkBufferUsageFlagBits = 1 << 2;
pub const VK_BUFFER_USAGE_VERTEX_BUFFER_BIT: VkBufferUsageFlagBits = 1 << 7;
pub const VK_CHROMA_LOCATION_COSITED_EVEN: VkChromaLocation = 0;
pub const VK_CHROMA_LOCATION_MIDPOINT: VkChromaLocation = 1;
pub const VK_COLOR_COMPONENT_A_BIT: VkColorComponentFlagBits = 1 << 3;
pub const VK_COLOR_COMPONENT_B_BIT: VkColorComponentFlagBits = 1 << 2;
pub const VK_COLOR_COMPONENT_G_BIT: VkColorComponentFlagBits = 1 << 1;
pub const VK_COLOR_COMPONENT_R_BIT: VkColorComponentFlagBits = 1 << 0;
pub const VK_COMMAND_BUFFER_LEVEL_PRIMARY: VkCommandBufferLevel = 0;
pub const VK_COMMAND_BUFFER_LEVEL_SECONDARY: VkCommandBufferLevel = 1;
pub const VK_COMMAND_BUFFER_RESET_RELEASE_RESOURCES_BIT: VkCommandBufferResetFlagBits = 1 << 0;
pub const VK_COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT: VkCommandBufferUsageFlagBits = 1 << 0;
pub const VK_COMMAND_BUFFER_USAGE_RENDER_PASS_CONTINUE_BIT: VkCommandBufferUsageFlagBits = 1 << 1;
pub const VK_COMMAND_BUFFER_USAGE_SIMULTANEOUS_USE_BIT: VkCommandBufferUsageFlagBits = 1 << 2;
pub const VK_COMMAND_POOL_CREATE_PROTECTED_BIT: VkCommandPoolCreateFlagBits = 1 << 2;
pub const VK_COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT: VkCommandPoolCreateFlagBits = 1 << 1;
pub const VK_COMMAND_POOL_CREATE_TRANSIENT_BIT: VkCommandPoolCreateFlagBits = 1 << 0;
pub const VK_COMMAND_POOL_RESET_RELEASE_RESOURCES_BIT: VkCommandPoolResetFlagBits = 1 << 0;
pub const VK_COMPARE_OP_ALWAYS: VkCompareOp = 7;
pub const VK_COMPARE_OP_EQUAL: VkCompareOp = 2;
pub const VK_COMPARE_OP_GREATER: VkCompareOp = 4;
pub const VK_COMPARE_OP_GREATER_OR_EQUAL: VkCompareOp = 6;
pub const VK_COMPARE_OP_LESS: VkCompareOp = 1;
pub const VK_COMPARE_OP_LESS_OR_EQUAL: VkCompareOp = 3;
pub const VK_COMPARE_OP_NEVER: VkCompareOp = 0;
pub const VK_COMPARE_OP_NOT_EQUAL: VkCompareOp = 5;
pub const VK_COMPONENT_SWIZZLE_A: VkComponentSwizzle = 6;
pub const VK_COMPONENT_SWIZZLE_B: VkComponentSwizzle = 5;
pub const VK_COMPONENT_SWIZZLE_G: VkComponentSwizzle = 4;
pub const VK_COMPONENT_SWIZZLE_IDENTITY: VkComponentSwizzle = 0;
pub const VK_COMPONENT_SWIZZLE_ONE: VkComponentSwizzle = 2;
pub const VK_COMPONENT_SWIZZLE_R: VkComponentSwizzle = 3;
pub const VK_COMPONENT_SWIZZLE_ZERO: VkComponentSwizzle = 1;
pub const VK_CULL_MODE_BACK_BIT: VkCullModeFlagBits = 1 << 1;
pub const VK_CULL_MODE_FRONT_AND_BACK: VkCullModeFlagBits = 0x00000003;
pub const VK_CULL_MODE_FRONT_BIT: VkCullModeFlagBits = 1 << 0;
pub const VK_CULL_MODE_NONE: VkCullModeFlagBits = 0;
pub const VK_DEPENDENCY_BY_REGION_BIT: VkDependencyFlagBits = 1 << 0;
pub const VK_DEPENDENCY_DEVICE_GROUP_BIT: VkDependencyFlagBits = 1 << 2;
pub const VK_DEPENDENCY_VIEW_LOCAL_BIT: VkDependencyFlagBits = 1 << 1;
pub const VK_DESCRIPTOR_BINDING_PARTIALLY_BOUND_BIT: VkDescriptorBindingFlagBits = 1 << 2;
pub const VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT: VkDescriptorBindingFlagBits = 1 << 0;
pub const VK_DESCRIPTOR_BINDING_UPDATE_UNUSED_WHILE_PENDING_BIT: VkDescriptorBindingFlagBits = 1 << 1;
pub const VK_DESCRIPTOR_BINDING_VARIABLE_DESCRIPTOR_COUNT_BIT: VkDescriptorBindingFlagBits = 1 << 3;
pub const VK_DESCRIPTOR_POOL_CREATE_FREE_DESCRIPTOR_SET_BIT: VkDescriptorPoolCreateFlagBits = 1 << 0;
pub const VK_DESCRIPTOR_POOL_CREATE_UPDATE_AFTER_BIND_BIT: VkDescriptorPoolCreateFlagBits = 1 << 1;
pub const VK_DESCRIPTOR_SET_LAYOUT_CREATE_PUSH_DESCRIPTOR_BIT: VkDescriptorSetLayoutCreateFlagBits = 1 << 0;
pub const VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT: VkDescriptorSetLayoutCreateFlagBits = 1 << 1;
pub const VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER: VkDescriptorType = 1;
pub const VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK: VkDescriptorType = 1000138000;
pub const VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT: VkDescriptorType = 10;
pub const VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE: VkDescriptorType = 2;
pub const VK_DESCRIPTOR_TYPE_SAMPLER: VkDescriptorType = 0;
pub const VK_DESCRIPTOR_TYPE_STORAGE_BUFFER: VkDescriptorType = 7;
pub const VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC: VkDescriptorType = 9;
pub const VK_DESCRIPTOR_TYPE_STORAGE_IMAGE: VkDescriptorType = 3;
pub const VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER: VkDescriptorType = 5;
pub const VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER: VkDescriptorType = 6;
pub const VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC: VkDescriptorType = 8;
pub const VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER: VkDescriptorType = 4;
pub const VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_DESCRIPTOR_SET: VkDescriptorUpdateTemplateType = 0;
pub const VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_PUSH_DESCRIPTORS: VkDescriptorUpdateTemplateType = 1;
pub const VK_DEVICE_QUEUE_CREATE_PROTECTED_BIT: VkDeviceQueueCreateFlagBits = 1 << 0;
pub const VK_DRIVER_ID_AMD_OPEN_SOURCE: VkDriverId = 2;
pub const VK_DRIVER_ID_AMD_PROPRIETARY: VkDriverId = 1;
pub const VK_DRIVER_ID_ARM_PROPRIETARY: VkDriverId = 9;
pub const VK_DRIVER_ID_BROADCOM_PROPRIETARY: VkDriverId = 12;
pub const VK_DRIVER_ID_COREAVI_PROPRIETARY: VkDriverId = 15;
pub const VK_DRIVER_ID_GGP_PROPRIETARY: VkDriverId = 11;
pub const VK_DRIVER_ID_GOOGLE_SWIFTSHADER: VkDriverId = 10;
pub const VK_DRIVER_ID_IMAGINATION_OPEN_SOURCE_MESA: VkDriverId = 25;
pub const VK_DRIVER_ID_IMAGINATION_PROPRIETARY: VkDriverId = 7;
pub const VK_DRIVER_ID_INTEL_OPEN_SOURCE_MESA: VkDriverId = 6;
pub const VK_DRIVER_ID_INTEL_PROPRIETARY_WINDOWS: VkDriverId = 5;
pub const VK_DRIVER_ID_JUICE_PROPRIETARY: VkDriverId = 16;
pub const VK_DRIVER_ID_MESA_DOZEN: VkDriverId = 23;
pub const VK_DRIVER_ID_MESA_HONEYKRISP: VkDriverId = 26;
pub const VK_DRIVER_ID_MESA_KOSMICKRISP: VkDriverId = 28;
pub const VK_DRIVER_ID_MESA_LLVMPIPE: VkDriverId = 13;
pub const VK_DRIVER_ID_MESA_NVK: VkDriverId = 24;
pub const VK_DRIVER_ID_MESA_PANVK: VkDriverId = 20;
pub const VK_DRIVER_ID_MESA_RADV: VkDriverId = 3;
pub const VK_DRIVER_ID_MESA_TURNIP: VkDriverId = 18;
pub const VK_DRIVER_ID_MESA_V3DV: VkDriverId = 19;
pub const VK_DRIVER_ID_MESA_VENUS: VkDriverId = 22;
pub const VK_DRIVER_ID_MOLTENVK: VkDriverId = 14;
pub const VK_DRIVER_ID_NVIDIA_PROPRIETARY: VkDriverId = 4;
pub const VK_DRIVER_ID_QUALCOMM_PROPRIETARY: VkDriverId = 8;
pub const VK_DRIVER_ID_SAMSUNG_PROPRIETARY: VkDriverId = 21;
pub const VK_DRIVER_ID_VERISILICON_PROPRIETARY: VkDriverId = 17;
pub const VK_DRIVER_ID_VULKAN_SC_EMULATION_ON_VULKAN: VkDriverId = 27;
pub const VK_DYNAMIC_STATE_BLEND_CONSTANTS: VkDynamicState = 4;
pub const VK_DYNAMIC_STATE_CULL_MODE: VkDynamicState = 1000267000;
pub const VK_DYNAMIC_STATE_DEPTH_BIAS: VkDynamicState = 3;
pub const VK_DYNAMIC_STATE_DEPTH_BIAS_ENABLE: VkDynamicState = 1000377002;
pub const VK_DYNAMIC_STATE_DEPTH_BOUNDS: VkDynamicState = 5;
pub const VK_DYNAMIC_STATE_DEPTH_BOUNDS_TEST_ENABLE: VkDynamicState = 1000267009;
pub const VK_DYNAMIC_STATE_DEPTH_COMPARE_OP: VkDynamicState = 1000267008;
pub const VK_DYNAMIC_STATE_DEPTH_TEST_ENABLE: VkDynamicState = 1000267006;
pub const VK_DYNAMIC_STATE_DEPTH_WRITE_ENABLE: VkDynamicState = 1000267007;
pub const VK_DYNAMIC_STATE_FRONT_FACE: VkDynamicState = 1000267001;
pub const VK_DYNAMIC_STATE_LINE_STIPPLE: VkDynamicState = 1000259000;
pub const VK_DYNAMIC_STATE_LINE_WIDTH: VkDynamicState = 2;
pub const VK_DYNAMIC_STATE_PRIMITIVE_RESTART_ENABLE: VkDynamicState = 1000377004;
pub const VK_DYNAMIC_STATE_PRIMITIVE_TOPOLOGY: VkDynamicState = 1000267002;
pub const VK_DYNAMIC_STATE_RASTERIZER_DISCARD_ENABLE: VkDynamicState = 1000377001;
pub const VK_DYNAMIC_STATE_SCISSOR: VkDynamicState = 1;
pub const VK_DYNAMIC_STATE_SCISSOR_WITH_COUNT: VkDynamicState = 1000267004;
pub const VK_DYNAMIC_STATE_STENCIL_COMPARE_MASK: VkDynamicState = 6;
pub const VK_DYNAMIC_STATE_STENCIL_OP: VkDynamicState = 1000267011;
pub const VK_DYNAMIC_STATE_STENCIL_REFERENCE: VkDynamicState = 8;
pub const VK_DYNAMIC_STATE_STENCIL_TEST_ENABLE: VkDynamicState = 1000267010;
pub const VK_DYNAMIC_STATE_STENCIL_WRITE_MASK: VkDynamicState = 7;
pub const VK_DYNAMIC_STATE_VERTEX_INPUT_BINDING_STRIDE: VkDynamicState = 1000267005;
pub const VK_DYNAMIC_STATE_VIEWPORT: VkDynamicState = 0;
pub const VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT: VkDynamicState = 1000267003;
pub const VK_ERROR_DEVICE_LOST: VkResult = -4;
pub const VK_ERROR_EXTENSION_NOT_PRESENT: VkResult = -7;
pub const VK_ERROR_FEATURE_NOT_PRESENT: VkResult = -8;
pub const VK_ERROR_FORMAT_NOT_SUPPORTED: VkResult = -11;
pub const VK_ERROR_FRAGMENTATION: VkResult = -1000161000;
pub const VK_ERROR_FRAGMENTED_POOL: VkResult = -12;
pub const VK_ERROR_INCOMPATIBLE_DRIVER: VkResult = -9;
pub const VK_ERROR_INITIALIZATION_FAILED: VkResult = -3;
pub const VK_ERROR_INVALID_EXTERNAL_HANDLE: VkResult = -1000072003;
pub const VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS: VkResult = -1000257000;
pub const VK_ERROR_LAYER_NOT_PRESENT: VkResult = -6;
pub const VK_ERROR_MEMORY_MAP_FAILED: VkResult = -5;
pub const VK_ERROR_NOT_PERMITTED: VkResult = -1000174001;
pub const VK_ERROR_OUT_OF_DEVICE_MEMORY: VkResult = -2;
pub const VK_ERROR_OUT_OF_HOST_MEMORY: VkResult = -1;
pub const VK_ERROR_OUT_OF_POOL_MEMORY: VkResult = -1000069000;
pub const VK_ERROR_TOO_MANY_OBJECTS: VkResult = -10;
pub const VK_ERROR_UNKNOWN: VkResult = -13;
pub const VK_ERROR_VALIDATION_FAILED: VkResult = -1000011001;
pub const VK_EVENT_CREATE_DEVICE_ONLY_BIT: VkEventCreateFlagBits = 1 << 0;
pub const VK_EVENT_RESET: VkResult = 4;
pub const VK_EVENT_SET: VkResult = 3;
pub const VK_EXTERNAL_FENCE_FEATURE_EXPORTABLE_BIT: VkExternalFenceFeatureFlagBits = 1 << 0;
pub const VK_EXTERNAL_FENCE_FEATURE_IMPORTABLE_BIT: VkExternalFenceFeatureFlagBits = 1 << 1;
pub const VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_FD_BIT: VkExternalFenceHandleTypeFlagBits = 1 << 0;
pub const VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_BIT: VkExternalFenceHandleTypeFlagBits = 1 << 1;
pub const VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT: VkExternalFenceHandleTypeFlagBits = 1 << 2;
pub const VK_EXTERNAL_FENCE_HANDLE_TYPE_SYNC_FD_BIT: VkExternalFenceHandleTypeFlagBits = 1 << 3;
pub const VK_EXTERNAL_MEMORY_FEATURE_DEDICATED_ONLY_BIT: VkExternalMemoryFeatureFlagBits = 1 << 0;
pub const VK_EXTERNAL_MEMORY_FEATURE_EXPORTABLE_BIT: VkExternalMemoryFeatureFlagBits = 1 << 1;
pub const VK_EXTERNAL_MEMORY_FEATURE_IMPORTABLE_BIT: VkExternalMemoryFeatureFlagBits = 1 << 2;
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_BIT: VkExternalMemoryHandleTypeFlagBits = 1 << 3;
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_KMT_BIT: VkExternalMemoryHandleTypeFlagBits = 1 << 4;
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_HEAP_BIT: VkExternalMemoryHandleTypeFlagBits = 1 << 5;
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_RESOURCE_BIT: VkExternalMemoryHandleTypeFlagBits = 1 << 6;
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_FD_BIT: VkExternalMemoryHandleTypeFlagBits = 1 << 0;
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT: VkExternalMemoryHandleTypeFlagBits = 1 << 1;
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT: VkExternalMemoryHandleTypeFlagBits = 1 << 2;
pub const VK_EXTERNAL_SEMAPHORE_FEATURE_EXPORTABLE_BIT: VkExternalSemaphoreFeatureFlagBits = 1 << 0;
pub const VK_EXTERNAL_SEMAPHORE_FEATURE_IMPORTABLE_BIT: VkExternalSemaphoreFeatureFlagBits = 1 << 1;
pub const VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D11_FENCE_BIT: VkExternalSemaphoreHandleTypeFlagBits = VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D12_FENCE_BIT;
pub const VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D12_FENCE_BIT: VkExternalSemaphoreHandleTypeFlagBits = 1 << 3;
pub const VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_FD_BIT: VkExternalSemaphoreHandleTypeFlagBits = 1 << 0;
pub const VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_BIT: VkExternalSemaphoreHandleTypeFlagBits = 1 << 1;
pub const VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT: VkExternalSemaphoreHandleTypeFlagBits = 1 << 2;
pub const VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_SYNC_FD_BIT: VkExternalSemaphoreHandleTypeFlagBits = 1 << 4;
pub const VK_FALSE: u32 = 0;
pub const VK_FENCE_CREATE_SIGNALED_BIT: VkFenceCreateFlagBits = 1 << 0;
pub const VK_FENCE_IMPORT_TEMPORARY_BIT: VkFenceImportFlagBits = 1 << 0;
pub const VK_FILTER_LINEAR: VkFilter = 1;
pub const VK_FILTER_NEAREST: VkFilter = 0;
pub const VK_FORMAT_A1B5G5R5_UNORM_PACK16: VkFormat = 1000470000;
pub const VK_FORMAT_A1R5G5B5_UNORM_PACK16: VkFormat = 8;
pub const VK_FORMAT_A2B10G10R10_SINT_PACK32: VkFormat = 69;
pub const VK_FORMAT_A2B10G10R10_SNORM_PACK32: VkFormat = 65;
pub const VK_FORMAT_A2B10G10R10_SSCALED_PACK32: VkFormat = 67;
pub const VK_FORMAT_A2B10G10R10_UINT_PACK32: VkFormat = 68;
pub const VK_FORMAT_A2B10G10R10_UNORM_PACK32: VkFormat = 64;
pub const VK_FORMAT_A2B10G10R10_USCALED_PACK32: VkFormat = 66;
pub const VK_FORMAT_A2R10G10B10_SINT_PACK32: VkFormat = 63;
pub const VK_FORMAT_A2R10G10B10_SNORM_PACK32: VkFormat = 59;
pub const VK_FORMAT_A2R10G10B10_SSCALED_PACK32: VkFormat = 61;
pub const VK_FORMAT_A2R10G10B10_UINT_PACK32: VkFormat = 62;
pub const VK_FORMAT_A2R10G10B10_UNORM_PACK32: VkFormat = 58;
pub const VK_FORMAT_A2R10G10B10_USCALED_PACK32: VkFormat = 60;
pub const VK_FORMAT_A4B4G4R4_UNORM_PACK16: VkFormat = 1000340001;
pub const VK_FORMAT_A4R4G4B4_UNORM_PACK16: VkFormat = 1000340000;
pub const VK_FORMAT_A8B8G8R8_SINT_PACK32: VkFormat = 56;
pub const VK_FORMAT_A8B8G8R8_SNORM_PACK32: VkFormat = 52;
pub const VK_FORMAT_A8B8G8R8_SRGB_PACK32: VkFormat = 57;
pub const VK_FORMAT_A8B8G8R8_SSCALED_PACK32: VkFormat = 54;
pub const VK_FORMAT_A8B8G8R8_UINT_PACK32: VkFormat = 55;
pub const VK_FORMAT_A8B8G8R8_UNORM_PACK32: VkFormat = 51;
pub const VK_FORMAT_A8B8G8R8_USCALED_PACK32: VkFormat = 53;
pub const VK_FORMAT_A8_UNORM: VkFormat = 1000470001;
pub const VK_FORMAT_ASTC_10x10_SFLOAT_BLOCK: VkFormat = 1000066011;
pub const VK_FORMAT_ASTC_10x10_SRGB_BLOCK: VkFormat = 180;
pub const VK_FORMAT_ASTC_10x10_UNORM_BLOCK: VkFormat = 179;
pub const VK_FORMAT_ASTC_10x5_SFLOAT_BLOCK: VkFormat = 1000066008;
pub const VK_FORMAT_ASTC_10x5_SRGB_BLOCK: VkFormat = 174;
pub const VK_FORMAT_ASTC_10x5_UNORM_BLOCK: VkFormat = 173;
pub const VK_FORMAT_ASTC_10x6_SFLOAT_BLOCK: VkFormat = 1000066009;
pub const VK_FORMAT_ASTC_10x6_SRGB_BLOCK: VkFormat = 176;
pub const VK_FORMAT_ASTC_10x6_UNORM_BLOCK: VkFormat = 175;
pub const VK_FORMAT_ASTC_10x8_SFLOAT_BLOCK: VkFormat = 1000066010;
pub const VK_FORMAT_ASTC_10x8_SRGB_BLOCK: VkFormat = 178;
pub const VK_FORMAT_ASTC_10x8_UNORM_BLOCK: VkFormat = 177;
pub const VK_FORMAT_ASTC_12x10_SFLOAT_BLOCK: VkFormat = 1000066012;
pub const VK_FORMAT_ASTC_12x10_SRGB_BLOCK: VkFormat = 182;
pub const VK_FORMAT_ASTC_12x10_UNORM_BLOCK: VkFormat = 181;
pub const VK_FORMAT_ASTC_12x12_SFLOAT_BLOCK: VkFormat = 1000066013;
pub const VK_FORMAT_ASTC_12x12_SRGB_BLOCK: VkFormat = 184;
pub const VK_FORMAT_ASTC_12x12_UNORM_BLOCK: VkFormat = 183;
pub const VK_FORMAT_ASTC_4x4_SFLOAT_BLOCK: VkFormat = 1000066000;
pub const VK_FORMAT_ASTC_4x4_SRGB_BLOCK: VkFormat = 158;
pub const VK_FORMAT_ASTC_4x4_UNORM_BLOCK: VkFormat = 157;
pub const VK_FORMAT_ASTC_5x4_SFLOAT_BLOCK: VkFormat = 1000066001;
pub const VK_FORMAT_ASTC_5x4_SRGB_BLOCK: VkFormat = 160;
pub const VK_FORMAT_ASTC_5x4_UNORM_BLOCK: VkFormat = 159;
pub const VK_FORMAT_ASTC_5x5_SFLOAT_BLOCK: VkFormat = 1000066002;
pub const VK_FORMAT_ASTC_5x5_SRGB_BLOCK: VkFormat = 162;
pub const VK_FORMAT_ASTC_5x5_UNORM_BLOCK: VkFormat = 161;
pub const VK_FORMAT_ASTC_6x5_SFLOAT_BLOCK: VkFormat = 1000066003;
pub const VK_FORMAT_ASTC_6x5_SRGB_BLOCK: VkFormat = 164;
pub const VK_FORMAT_ASTC_6x5_UNORM_BLOCK: VkFormat = 163;
pub const VK_FORMAT_ASTC_6x6_SFLOAT_BLOCK: VkFormat = 1000066004;
pub const VK_FORMAT_ASTC_6x6_SRGB_BLOCK: VkFormat = 166;
pub const VK_FORMAT_ASTC_6x6_UNORM_BLOCK: VkFormat = 165;
pub const VK_FORMAT_ASTC_8x5_SFLOAT_BLOCK: VkFormat = 1000066005;
pub const VK_FORMAT_ASTC_8x5_SRGB_BLOCK: VkFormat = 168;
pub const VK_FORMAT_ASTC_8x5_UNORM_BLOCK: VkFormat = 167;
pub const VK_FORMAT_ASTC_8x6_SFLOAT_BLOCK: VkFormat = 1000066006;
pub const VK_FORMAT_ASTC_8x6_SRGB_BLOCK: VkFormat = 170;
pub const VK_FORMAT_ASTC_8x6_UNORM_BLOCK: VkFormat = 169;
pub const VK_FORMAT_ASTC_8x8_SFLOAT_BLOCK: VkFormat = 1000066007;
pub const VK_FORMAT_ASTC_8x8_SRGB_BLOCK: VkFormat = 172;
pub const VK_FORMAT_ASTC_8x8_UNORM_BLOCK: VkFormat = 171;
pub const VK_FORMAT_B10G11R11_UFLOAT_PACK32: VkFormat = 122;
pub const VK_FORMAT_B10X6G10X6R10X6G10X6_422_UNORM_4PACK16: VkFormat = 1000156011;
pub const VK_FORMAT_B12X4G12X4R12X4G12X4_422_UNORM_4PACK16: VkFormat = 1000156021;
pub const VK_FORMAT_B16G16R16G16_422_UNORM: VkFormat = 1000156028;
pub const VK_FORMAT_B4G4R4A4_UNORM_PACK16: VkFormat = 3;
pub const VK_FORMAT_B5G5R5A1_UNORM_PACK16: VkFormat = 7;
pub const VK_FORMAT_B5G6R5_UNORM_PACK16: VkFormat = 5;
pub const VK_FORMAT_B8G8R8A8_SINT: VkFormat = 49;
pub const VK_FORMAT_B8G8R8A8_SNORM: VkFormat = 45;
pub const VK_FORMAT_B8G8R8A8_SRGB: VkFormat = 50;
pub const VK_FORMAT_B8G8R8A8_SSCALED: VkFormat = 47;
pub const VK_FORMAT_B8G8R8A8_UINT: VkFormat = 48;
pub const VK_FORMAT_B8G8R8A8_UNORM: VkFormat = 44;
pub const VK_FORMAT_B8G8R8A8_USCALED: VkFormat = 46;
pub const VK_FORMAT_B8G8R8G8_422_UNORM: VkFormat = 1000156001;
pub const VK_FORMAT_B8G8R8_SINT: VkFormat = 35;
pub const VK_FORMAT_B8G8R8_SNORM: VkFormat = 31;
pub const VK_FORMAT_B8G8R8_SRGB: VkFormat = 36;
pub const VK_FORMAT_B8G8R8_SSCALED: VkFormat = 33;
pub const VK_FORMAT_B8G8R8_UINT: VkFormat = 34;
pub const VK_FORMAT_B8G8R8_UNORM: VkFormat = 30;
pub const VK_FORMAT_B8G8R8_USCALED: VkFormat = 32;
pub const VK_FORMAT_BC1_RGBA_SRGB_BLOCK: VkFormat = 134;
pub const VK_FORMAT_BC1_RGBA_UNORM_BLOCK: VkFormat = 133;
pub const VK_FORMAT_BC1_RGB_SRGB_BLOCK: VkFormat = 132;
pub const VK_FORMAT_BC1_RGB_UNORM_BLOCK: VkFormat = 131;
pub const VK_FORMAT_BC2_SRGB_BLOCK: VkFormat = 136;
pub const VK_FORMAT_BC2_UNORM_BLOCK: VkFormat = 135;
pub const VK_FORMAT_BC3_SRGB_BLOCK: VkFormat = 138;
pub const VK_FORMAT_BC3_UNORM_BLOCK: VkFormat = 137;
pub const VK_FORMAT_BC4_SNORM_BLOCK: VkFormat = 140;
pub const VK_FORMAT_BC4_UNORM_BLOCK: VkFormat = 139;
pub const VK_FORMAT_BC5_SNORM_BLOCK: VkFormat = 142;
pub const VK_FORMAT_BC5_UNORM_BLOCK: VkFormat = 141;
pub const VK_FORMAT_BC6H_SFLOAT_BLOCK: VkFormat = 144;
pub const VK_FORMAT_BC6H_UFLOAT_BLOCK: VkFormat = 143;
pub const VK_FORMAT_BC7_SRGB_BLOCK: VkFormat = 146;
pub const VK_FORMAT_BC7_UNORM_BLOCK: VkFormat = 145;
pub const VK_FORMAT_D16_UNORM: VkFormat = 124;
pub const VK_FORMAT_D16_UNORM_S8_UINT: VkFormat = 128;
pub const VK_FORMAT_D24_UNORM_S8_UINT: VkFormat = 129;
pub const VK_FORMAT_D32_SFLOAT: VkFormat = 126;
pub const VK_FORMAT_D32_SFLOAT_S8_UINT: VkFormat = 130;
pub const VK_FORMAT_E5B9G9R9_UFLOAT_PACK32: VkFormat = 123;
pub const VK_FORMAT_EAC_R11G11_SNORM_BLOCK: VkFormat = 156;
pub const VK_FORMAT_EAC_R11G11_UNORM_BLOCK: VkFormat = 155;
pub const VK_FORMAT_EAC_R11_SNORM_BLOCK: VkFormat = 154;
pub const VK_FORMAT_EAC_R11_UNORM_BLOCK: VkFormat = 153;
pub const VK_FORMAT_ETC2_R8G8B8A1_SRGB_BLOCK: VkFormat = 150;
pub const VK_FORMAT_ETC2_R8G8B8A1_UNORM_BLOCK: VkFormat = 149;
pub const VK_FORMAT_ETC2_R8G8B8A8_SRGB_BLOCK: VkFormat = 152;
pub const VK_FORMAT_ETC2_R8G8B8A8_UNORM_BLOCK: VkFormat = 151;
pub const VK_FORMAT_ETC2_R8G8B8_SRGB_BLOCK: VkFormat = 148;
pub const VK_FORMAT_ETC2_R8G8B8_UNORM_BLOCK: VkFormat = 147;
pub const VK_FORMAT_FEATURE_2_BLIT_DST_BIT: VkFormatFeatureFlagBits2 = 1 << 11;
pub const VK_FORMAT_FEATURE_2_BLIT_SRC_BIT: VkFormatFeatureFlagBits2 = 1 << 10;
pub const VK_FORMAT_FEATURE_2_COLOR_ATTACHMENT_BIT: VkFormatFeatureFlagBits2 = 1 << 7;
pub const VK_FORMAT_FEATURE_2_COLOR_ATTACHMENT_BLEND_BIT: VkFormatFeatureFlagBits2 = 1 << 8;
pub const VK_FORMAT_FEATURE_2_COSITED_CHROMA_SAMPLES_BIT: VkFormatFeatureFlagBits2 = 1 << 23;
pub const VK_FORMAT_FEATURE_2_DEPTH_STENCIL_ATTACHMENT_BIT: VkFormatFeatureFlagBits2 = 1 << 9;
pub const VK_FORMAT_FEATURE_2_DISJOINT_BIT: VkFormatFeatureFlagBits2 = 1 << 22;
pub const VK_FORMAT_FEATURE_2_HOST_IMAGE_TRANSFER_BIT: VkFormatFeatureFlagBits2 = 1 << 46;
pub const VK_FORMAT_FEATURE_2_MIDPOINT_CHROMA_SAMPLES_BIT: VkFormatFeatureFlagBits2 = 1 << 17;
pub const VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_BIT: VkFormatFeatureFlagBits2 = 1 << 0;
pub const VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_DEPTH_COMPARISON_BIT: VkFormatFeatureFlagBits2 = 1 << 33;
pub const VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_FILTER_CUBIC_BIT: VkFormatFeatureFlagBits2 = 1 << 13;
pub const VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_FILTER_LINEAR_BIT: VkFormatFeatureFlagBits2 = 1 << 12;
pub const VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_FILTER_MINMAX_BIT: VkFormatFeatureFlagBits2 = 1 << 16;
pub const VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_BIT: VkFormatFeatureFlagBits2 = 1 << 20;
pub const VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_BIT: VkFormatFeatureFlagBits2 = 1 << 21;
pub const VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_BIT: VkFormatFeatureFlagBits2 = 1 << 18;
pub const VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_BIT: VkFormatFeatureFlagBits2 = 1 << 19;
pub const VK_FORMAT_FEATURE_2_STORAGE_IMAGE_ATOMIC_BIT: VkFormatFeatureFlagBits2 = 1 << 2;
pub const VK_FORMAT_FEATURE_2_STORAGE_IMAGE_BIT: VkFormatFeatureFlagBits2 = 1 << 1;
pub const VK_FORMAT_FEATURE_2_STORAGE_READ_WITHOUT_FORMAT_BIT: VkFormatFeatureFlagBits2 = 1 << 31;
pub const VK_FORMAT_FEATURE_2_STORAGE_TEXEL_BUFFER_ATOMIC_BIT: VkFormatFeatureFlagBits2 = 1 << 5;
pub const VK_FORMAT_FEATURE_2_STORAGE_TEXEL_BUFFER_BIT: VkFormatFeatureFlagBits2 = 1 << 4;
pub const VK_FORMAT_FEATURE_2_STORAGE_WRITE_WITHOUT_FORMAT_BIT: VkFormatFeatureFlagBits2 = 1 << 32;
pub const VK_FORMAT_FEATURE_2_TRANSFER_DST_BIT: VkFormatFeatureFlagBits2 = 1 << 15;
pub const VK_FORMAT_FEATURE_2_TRANSFER_SRC_BIT: VkFormatFeatureFlagBits2 = 1 << 14;
pub const VK_FORMAT_FEATURE_2_UNIFORM_TEXEL_BUFFER_BIT: VkFormatFeatureFlagBits2 = 1 << 3;
pub const VK_FORMAT_FEATURE_2_VERTEX_BUFFER_BIT: VkFormatFeatureFlagBits2 = 1 << 6;
pub const VK_FORMAT_FEATURE_BLIT_DST_BIT: VkFormatFeatureFlagBits = 1 << 11;
pub const VK_FORMAT_FEATURE_BLIT_SRC_BIT: VkFormatFeatureFlagBits = 1 << 10;
pub const VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BIT: VkFormatFeatureFlagBits = 1 << 7;
pub const VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BLEND_BIT: VkFormatFeatureFlagBits = 1 << 8;
pub const VK_FORMAT_FEATURE_COSITED_CHROMA_SAMPLES_BIT: VkFormatFeatureFlagBits = 1 << 23;
pub const VK_FORMAT_FEATURE_DEPTH_STENCIL_ATTACHMENT_BIT: VkFormatFeatureFlagBits = 1 << 9;
pub const VK_FORMAT_FEATURE_DISJOINT_BIT: VkFormatFeatureFlagBits = 1 << 22;
pub const VK_FORMAT_FEATURE_MIDPOINT_CHROMA_SAMPLES_BIT: VkFormatFeatureFlagBits = 1 << 17;
pub const VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT: VkFormatFeatureFlagBits = 1 << 0;
pub const VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT: VkFormatFeatureFlagBits = 1 << 12;
pub const VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_MINMAX_BIT: VkFormatFeatureFlagBits = 1 << 16;
pub const VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_BIT: VkFormatFeatureFlagBits = 1 << 20;
pub const VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_BIT: VkFormatFeatureFlagBits = 1 << 21;
pub const VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_BIT: VkFormatFeatureFlagBits = 1 << 18;
pub const VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_BIT: VkFormatFeatureFlagBits = 1 << 19;
pub const VK_FORMAT_FEATURE_STORAGE_IMAGE_ATOMIC_BIT: VkFormatFeatureFlagBits = 1 << 2;
pub const VK_FORMAT_FEATURE_STORAGE_IMAGE_BIT: VkFormatFeatureFlagBits = 1 << 1;
pub const VK_FORMAT_FEATURE_STORAGE_TEXEL_BUFFER_ATOMIC_BIT: VkFormatFeatureFlagBits = 1 << 5;
pub const VK_FORMAT_FEATURE_STORAGE_TEXEL_BUFFER_BIT: VkFormatFeatureFlagBits = 1 << 4;
pub const VK_FORMAT_FEATURE_TRANSFER_DST_BIT: VkFormatFeatureFlagBits = 1 << 15;
pub const VK_FORMAT_FEATURE_TRANSFER_SRC_BIT: VkFormatFeatureFlagBits = 1 << 14;
pub const VK_FORMAT_FEATURE_UNIFORM_TEXEL_BUFFER_BIT: VkFormatFeatureFlagBits = 1 << 3;
pub const VK_FORMAT_FEATURE_VERTEX_BUFFER_BIT: VkFormatFeatureFlagBits = 1 << 6;
pub const VK_FORMAT_G10X6B10X6G10X6R10X6_422_UNORM_4PACK16: VkFormat = 1000156010;
pub const VK_FORMAT_G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16: VkFormat = 1000156013;
pub const VK_FORMAT_G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16: VkFormat = 1000156015;
pub const VK_FORMAT_G10X6_B10X6R10X6_2PLANE_444_UNORM_3PACK16: VkFormat = 1000330001;
pub const VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16: VkFormat = 1000156012;
pub const VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16: VkFormat = 1000156014;
pub const VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16: VkFormat = 1000156016;
pub const VK_FORMAT_G12X4B12X4G12X4R12X4_422_UNORM_4PACK16: VkFormat = 1000156020;
pub const VK_FORMAT_G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16: VkFormat = 1000156023;
pub const VK_FORMAT_G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16: VkFormat = 1000156025;
pub const VK_FORMAT_G12X4_B12X4R12X4_2PLANE_444_UNORM_3PACK16: VkFormat = 1000330002;
pub const VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16: VkFormat = 1000156022;
pub const VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16: VkFormat = 1000156024;
pub const VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16: VkFormat = 1000156026;
pub const VK_FORMAT_G16B16G16R16_422_UNORM: VkFormat = 1000156027;
pub const VK_FORMAT_G16_B16R16_2PLANE_420_UNORM: VkFormat = 1000156030;
pub const VK_FORMAT_G16_B16R16_2PLANE_422_UNORM: VkFormat = 1000156032;
pub const VK_FORMAT_G16_B16R16_2PLANE_444_UNORM: VkFormat = 1000330003;
pub const VK_FORMAT_G16_B16_R16_3PLANE_420_UNORM: VkFormat = 1000156029;
pub const VK_FORMAT_G16_B16_R16_3PLANE_422_UNORM: VkFormat = 1000156031;
pub const VK_FORMAT_G16_B16_R16_3PLANE_444_UNORM: VkFormat = 1000156033;
pub const VK_FORMAT_G8B8G8R8_422_UNORM: VkFormat = 1000156000;
pub const VK_FORMAT_G8_B8R8_2PLANE_420_UNORM: VkFormat = 1000156003;
pub const VK_FORMAT_G8_B8R8_2PLANE_422_UNORM: VkFormat = 1000156005;
pub const VK_FORMAT_G8_B8R8_2PLANE_444_UNORM: VkFormat = 1000330000;
pub const VK_FORMAT_G8_B8_R8_3PLANE_420_UNORM: VkFormat = 1000156002;
pub const VK_FORMAT_G8_B8_R8_3PLANE_422_UNORM: VkFormat = 1000156004;
pub const VK_FORMAT_G8_B8_R8_3PLANE_444_UNORM: VkFormat = 1000156006;
pub const VK_FORMAT_R10X6G10X6B10X6A10X6_UNORM_4PACK16: VkFormat = 1000156009;
pub const VK_FORMAT_R10X6G10X6_UNORM_2PACK16: VkFormat = 1000156008;
pub const VK_FORMAT_R10X6_UNORM_PACK16: VkFormat = 1000156007;
pub const VK_FORMAT_R12X4G12X4B12X4A12X4_UNORM_4PACK16: VkFormat = 1000156019;
pub const VK_FORMAT_R12X4G12X4_UNORM_2PACK16: VkFormat = 1000156018;
pub const VK_FORMAT_R12X4_UNORM_PACK16: VkFormat = 1000156017;
pub const VK_FORMAT_R16G16B16A16_SFLOAT: VkFormat = 97;
pub const VK_FORMAT_R16G16B16A16_SINT: VkFormat = 96;
pub const VK_FORMAT_R16G16B16A16_SNORM: VkFormat = 92;
pub const VK_FORMAT_R16G16B16A16_SSCALED: VkFormat = 94;
pub const VK_FORMAT_R16G16B16A16_UINT: VkFormat = 95;
pub const VK_FORMAT_R16G16B16A16_UNORM: VkFormat = 91;
pub const VK_FORMAT_R16G16B16A16_USCALED: VkFormat = 93;
pub const VK_FORMAT_R16G16B16_SFLOAT: VkFormat = 90;
pub const VK_FORMAT_R16G16B16_SINT: VkFormat = 89;
pub const VK_FORMAT_R16G16B16_SNORM: VkFormat = 85;
pub const VK_FORMAT_R16G16B16_SSCALED: VkFormat = 87;
pub const VK_FORMAT_R16G16B16_UINT: VkFormat = 88;
pub const VK_FORMAT_R16G16B16_UNORM: VkFormat = 84;
pub const VK_FORMAT_R16G16B16_USCALED: VkFormat = 86;
pub const VK_FORMAT_R16G16_SFLOAT: VkFormat = 83;
pub const VK_FORMAT_R16G16_SINT: VkFormat = 82;
pub const VK_FORMAT_R16G16_SNORM: VkFormat = 78;
pub const VK_FORMAT_R16G16_SSCALED: VkFormat = 80;
pub const VK_FORMAT_R16G16_UINT: VkFormat = 81;
pub const VK_FORMAT_R16G16_UNORM: VkFormat = 77;
pub const VK_FORMAT_R16G16_USCALED: VkFormat = 79;
pub const VK_FORMAT_R16_SFLOAT: VkFormat = 76;
pub const VK_FORMAT_R16_SINT: VkFormat = 75;
pub const VK_FORMAT_R16_SNORM: VkFormat = 71;
pub const VK_FORMAT_R16_SSCALED: VkFormat = 73;
pub const VK_FORMAT_R16_UINT: VkFormat = 74;
pub const VK_FORMAT_R16_UNORM: VkFormat = 70;
pub const VK_FORMAT_R16_USCALED: VkFormat = 72;
pub const VK_FORMAT_R32G32B32A32_SFLOAT: VkFormat = 109;
pub const VK_FORMAT_R32G32B32A32_SINT: VkFormat = 108;
pub const VK_FORMAT_R32G32B32A32_UINT: VkFormat = 107;
pub const VK_FORMAT_R32G32B32_SFLOAT: VkFormat = 106;
pub const VK_FORMAT_R32G32B32_SINT: VkFormat = 105;
pub const VK_FORMAT_R32G32B32_UINT: VkFormat = 104;
pub const VK_FORMAT_R32G32_SFLOAT: VkFormat = 103;
pub const VK_FORMAT_R32G32_SINT: VkFormat = 102;
pub const VK_FORMAT_R32G32_UINT: VkFormat = 101;
pub const VK_FORMAT_R32_SFLOAT: VkFormat = 100;
pub const VK_FORMAT_R32_SINT: VkFormat = 99;
pub const VK_FORMAT_R32_UINT: VkFormat = 98;
pub const VK_FORMAT_R4G4B4A4_UNORM_PACK16: VkFormat = 2;
pub const VK_FORMAT_R4G4_UNORM_PACK8: VkFormat = 1;
pub const VK_FORMAT_R5G5B5A1_UNORM_PACK16: VkFormat = 6;
pub const VK_FORMAT_R5G6B5_UNORM_PACK16: VkFormat = 4;
pub const VK_FORMAT_R64G64B64A64_SFLOAT: VkFormat = 121;
pub const VK_FORMAT_R64G64B64A64_SINT: VkFormat = 120;
pub const VK_FORMAT_R64G64B64A64_UINT: VkFormat = 119;
pub const VK_FORMAT_R64G64B64_SFLOAT: VkFormat = 118;
pub const VK_FORMAT_R64G64B64_SINT: VkFormat = 117;
pub const VK_FORMAT_R64G64B64_UINT: VkFormat = 116;
pub const VK_FORMAT_R64G64_SFLOAT: VkFormat = 115;
pub const VK_FORMAT_R64G64_SINT: VkFormat = 114;
pub const VK_FORMAT_R64G64_UINT: VkFormat = 113;
pub const VK_FORMAT_R64_SFLOAT: VkFormat = 112;
pub const VK_FORMAT_R64_SINT: VkFormat = 111;
pub const VK_FORMAT_R64_UINT: VkFormat = 110;
pub const VK_FORMAT_R8G8B8A8_SINT: VkFormat = 42;
pub const VK_FORMAT_R8G8B8A8_SNORM: VkFormat = 38;
pub const VK_FORMAT_R8G8B8A8_SRGB: VkFormat = 43;
pub const VK_FORMAT_R8G8B8A8_SSCALED: VkFormat = 40;
pub const VK_FORMAT_R8G8B8A8_UINT: VkFormat = 41;
pub const VK_FORMAT_R8G8B8A8_UNORM: VkFormat = 37;
pub const VK_FORMAT_R8G8B8A8_USCALED: VkFormat = 39;
pub const VK_FORMAT_R8G8B8_SINT: VkFormat = 28;
pub const VK_FORMAT_R8G8B8_SNORM: VkFormat = 24;
pub const VK_FORMAT_R8G8B8_SRGB: VkFormat = 29;
pub const VK_FORMAT_R8G8B8_SSCALED: VkFormat = 26;
pub const VK_FORMAT_R8G8B8_UINT: VkFormat = 27;
pub const VK_FORMAT_R8G8B8_UNORM: VkFormat = 23;
pub const VK_FORMAT_R8G8B8_USCALED: VkFormat = 25;
pub const VK_FORMAT_R8G8_SINT: VkFormat = 21;
pub const VK_FORMAT_R8G8_SNORM: VkFormat = 17;
pub const VK_FORMAT_R8G8_SRGB: VkFormat = 22;
pub const VK_FORMAT_R8G8_SSCALED: VkFormat = 19;
pub const VK_FORMAT_R8G8_UINT: VkFormat = 20;
pub const VK_FORMAT_R8G8_UNORM: VkFormat = 16;
pub const VK_FORMAT_R8G8_USCALED: VkFormat = 18;
pub const VK_FORMAT_R8_SINT: VkFormat = 14;
pub const VK_FORMAT_R8_SNORM: VkFormat = 10;
pub const VK_FORMAT_R8_SRGB: VkFormat = 15;
pub const VK_FORMAT_R8_SSCALED: VkFormat = 12;
pub const VK_FORMAT_R8_UINT: VkFormat = 13;
pub const VK_FORMAT_R8_UNORM: VkFormat = 9;
pub const VK_FORMAT_R8_USCALED: VkFormat = 11;
pub const VK_FORMAT_S8_UINT: VkFormat = 127;
pub const VK_FORMAT_UNDEFINED: VkFormat = 0;
pub const VK_FORMAT_X8_D24_UNORM_PACK32: VkFormat = 125;
pub const VK_FRAMEBUFFER_CREATE_IMAGELESS_BIT: VkFramebufferCreateFlagBits = 1 << 0;
pub const VK_FRONT_FACE_CLOCKWISE: VkFrontFace = 1;
pub const VK_FRONT_FACE_COUNTER_CLOCKWISE: VkFrontFace = 0;
pub const VK_HOST_IMAGE_COPY_MEMCPY: VkHostImageCopyFlagBits = VK_HOST_IMAGE_COPY_MEMCPY_BIT;
pub const VK_HOST_IMAGE_COPY_MEMCPY_BIT: VkHostImageCopyFlagBits = 1 << 0;
pub const VK_IMAGE_ASPECT_COLOR_BIT: VkImageAspectFlagBits = 1 << 0;
pub const VK_IMAGE_ASPECT_DEPTH_BIT: VkImageAspectFlagBits = 1 << 1;
pub const VK_IMAGE_ASPECT_METADATA_BIT: VkImageAspectFlagBits = 1 << 3;
pub const VK_IMAGE_ASPECT_NONE: VkImageAspectFlagBits = 0;
pub const VK_IMAGE_ASPECT_PLANE_0_BIT: VkImageAspectFlagBits = 1 << 4;
pub const VK_IMAGE_ASPECT_PLANE_1_BIT: VkImageAspectFlagBits = 1 << 5;
pub const VK_IMAGE_ASPECT_PLANE_2_BIT: VkImageAspectFlagBits = 1 << 6;
pub const VK_IMAGE_ASPECT_STENCIL_BIT: VkImageAspectFlagBits = 1 << 2;
pub const VK_IMAGE_CREATE_2D_ARRAY_COMPATIBLE_BIT: VkImageCreateFlagBits = 1 << 5;
pub const VK_IMAGE_CREATE_ALIAS_BIT: VkImageCreateFlagBits = 1 << 10;
pub const VK_IMAGE_CREATE_BLOCK_TEXEL_VIEW_COMPATIBLE_BIT: VkImageCreateFlagBits = 1 << 7;
pub const VK_IMAGE_CREATE_CUBE_COMPATIBLE_BIT: VkImageCreateFlagBits = 1 << 4;
pub const VK_IMAGE_CREATE_DISJOINT_BIT: VkImageCreateFlagBits = 1 << 9;
pub const VK_IMAGE_CREATE_EXTENDED_USAGE_BIT: VkImageCreateFlagBits = 1 << 8;
pub const VK_IMAGE_CREATE_MUTABLE_FORMAT_BIT: VkImageCreateFlagBits = 1 << 3;
pub const VK_IMAGE_CREATE_PROTECTED_BIT: VkImageCreateFlagBits = 1 << 11;
pub const VK_IMAGE_CREATE_SPARSE_ALIASED_BIT: VkImageCreateFlagBits = 1 << 2;
pub const VK_IMAGE_CREATE_SPARSE_BINDING_BIT: VkImageCreateFlagBits = 1 << 0;
pub const VK_IMAGE_CREATE_SPARSE_RESIDENCY_BIT: VkImageCreateFlagBits = 1 << 1;
pub const VK_IMAGE_CREATE_SPLIT_INSTANCE_BIND_REGIONS_BIT: VkImageCreateFlagBits = 1 << 6;
pub const VK_IMAGE_LAYOUT_ATTACHMENT_OPTIMAL: VkImageLayout = 1000314001;
pub const VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL: VkImageLayout = 2;
pub const VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL: VkImageLayout = 1000241000;
pub const VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL: VkImageLayout = 1000117001;
pub const VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL: VkImageLayout = 1000241001;
pub const VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL: VkImageLayout = 1000117000;
pub const VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL: VkImageLayout = 3;
pub const VK_IMAGE_LAYOUT_DEPTH_STENCIL_READ_ONLY_OPTIMAL: VkImageLayout = 4;
pub const VK_IMAGE_LAYOUT_GENERAL: VkImageLayout = 1;
pub const VK_IMAGE_LAYOUT_PREINITIALIZED: VkImageLayout = 8;
pub const VK_IMAGE_LAYOUT_READ_ONLY_OPTIMAL: VkImageLayout = 1000314000;
pub const VK_IMAGE_LAYOUT_RENDERING_LOCAL_READ: VkImageLayout = 1000232000;
pub const VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL: VkImageLayout = 5;
pub const VK_IMAGE_LAYOUT_STENCIL_ATTACHMENT_OPTIMAL: VkImageLayout = 1000241002;
pub const VK_IMAGE_LAYOUT_STENCIL_READ_ONLY_OPTIMAL: VkImageLayout = 1000241003;
pub const VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL: VkImageLayout = 7;
pub const VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL: VkImageLayout = 6;
pub const VK_IMAGE_LAYOUT_UNDEFINED: VkImageLayout = 0;
pub const VK_IMAGE_TILING_LINEAR: VkImageTiling = 1;
pub const VK_IMAGE_TILING_OPTIMAL: VkImageTiling = 0;
pub const VK_IMAGE_TYPE_1D: VkImageType = 0;
pub const VK_IMAGE_TYPE_2D: VkImageType = 1;
pub const VK_IMAGE_TYPE_3D: VkImageType = 2;
pub const VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT: VkImageUsageFlagBits = 1 << 4;
pub const VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT: VkImageUsageFlagBits = 1 << 5;
pub const VK_IMAGE_USAGE_HOST_TRANSFER_BIT: VkImageUsageFlagBits = 1 << 22;
pub const VK_IMAGE_USAGE_INPUT_ATTACHMENT_BIT: VkImageUsageFlagBits = 1 << 7;
pub const VK_IMAGE_USAGE_SAMPLED_BIT: VkImageUsageFlagBits = 1 << 2;
pub const VK_IMAGE_USAGE_STORAGE_BIT: VkImageUsageFlagBits = 1 << 3;
pub const VK_IMAGE_USAGE_TRANSFER_DST_BIT: VkImageUsageFlagBits = 1 << 1;
pub const VK_IMAGE_USAGE_TRANSFER_SRC_BIT: VkImageUsageFlagBits = 1 << 0;
pub const VK_IMAGE_USAGE_TRANSIENT_ATTACHMENT_BIT: VkImageUsageFlagBits = 1 << 6;
pub const VK_IMAGE_VIEW_TYPE_1D: VkImageViewType = 0;
pub const VK_IMAGE_VIEW_TYPE_1D_ARRAY: VkImageViewType = 4;
pub const VK_IMAGE_VIEW_TYPE_2D: VkImageViewType = 1;
pub const VK_IMAGE_VIEW_TYPE_2D_ARRAY: VkImageViewType = 5;
pub const VK_IMAGE_VIEW_TYPE_3D: VkImageViewType = 2;
pub const VK_IMAGE_VIEW_TYPE_CUBE: VkImageViewType = 3;
pub const VK_IMAGE_VIEW_TYPE_CUBE_ARRAY: VkImageViewType = 6;
pub const VK_INCOMPLETE: VkResult = 5;
pub const VK_INDEX_TYPE_UINT16: VkIndexType = 0;
pub const VK_INDEX_TYPE_UINT32: VkIndexType = 1;
pub const VK_INDEX_TYPE_UINT8: VkIndexType = 1000265000;
pub const VK_INTERNAL_ALLOCATION_TYPE_EXECUTABLE: VkInternalAllocationType = 0;
pub const VK_LINE_RASTERIZATION_MODE_BRESENHAM: VkLineRasterizationMode = 2;
pub const VK_LINE_RASTERIZATION_MODE_DEFAULT: VkLineRasterizationMode = 0;
pub const VK_LINE_RASTERIZATION_MODE_RECTANGULAR: VkLineRasterizationMode = 1;
pub const VK_LINE_RASTERIZATION_MODE_RECTANGULAR_SMOOTH: VkLineRasterizationMode = 3;
pub const VK_LOD_CLAMP_NONE: c_float = 1000.0;
pub const VK_LOGIC_OP_AND: VkLogicOp = 1;
pub const VK_LOGIC_OP_AND_INVERTED: VkLogicOp = 4;
pub const VK_LOGIC_OP_AND_REVERSE: VkLogicOp = 2;
pub const VK_LOGIC_OP_CLEAR: VkLogicOp = 0;
pub const VK_LOGIC_OP_COPY: VkLogicOp = 3;
pub const VK_LOGIC_OP_COPY_INVERTED: VkLogicOp = 12;
pub const VK_LOGIC_OP_EQUIVALENT: VkLogicOp = 9;
pub const VK_LOGIC_OP_INVERT: VkLogicOp = 10;
pub const VK_LOGIC_OP_NAND: VkLogicOp = 14;
pub const VK_LOGIC_OP_NOR: VkLogicOp = 8;
pub const VK_LOGIC_OP_NO_OP: VkLogicOp = 5;
pub const VK_LOGIC_OP_OR: VkLogicOp = 7;
pub const VK_LOGIC_OP_OR_INVERTED: VkLogicOp = 13;
pub const VK_LOGIC_OP_OR_REVERSE: VkLogicOp = 11;
pub const VK_LOGIC_OP_SET: VkLogicOp = 15;
pub const VK_LOGIC_OP_XOR: VkLogicOp = 6;
pub const VK_LUID_SIZE: u32 = 8;
pub const VK_MAX_DESCRIPTION_SIZE: u32 = 256;
pub const VK_MAX_DEVICE_GROUP_SIZE: u32 = 32;
pub const VK_MAX_DRIVER_INFO_SIZE: u32 = 256;
pub const VK_MAX_DRIVER_NAME_SIZE: u32 = 256;
pub const VK_MAX_EXTENSION_NAME_SIZE: u32 = 256;
pub const VK_MAX_GLOBAL_PRIORITY_SIZE: u32 = 16;
pub const VK_MAX_MEMORY_HEAPS: u32 = 16;
pub const VK_MAX_MEMORY_TYPES: u32 = 32;
pub const VK_MAX_PHYSICAL_DEVICE_NAME_SIZE: u32 = 256;
pub const VK_MEMORY_ALLOCATE_DEVICE_ADDRESS_BIT: VkMemoryAllocateFlagBits = 1 << 1;
pub const VK_MEMORY_ALLOCATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT: VkMemoryAllocateFlagBits = 1 << 2;
pub const VK_MEMORY_ALLOCATE_DEVICE_MASK_BIT: VkMemoryAllocateFlagBits = 1 << 0;
pub const VK_MEMORY_HEAP_DEVICE_LOCAL_BIT: VkMemoryHeapFlagBits = 1 << 0;
pub const VK_MEMORY_HEAP_MULTI_INSTANCE_BIT: VkMemoryHeapFlagBits = 1 << 1;
pub const VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT: VkMemoryPropertyFlagBits = 1 << 0;
pub const VK_MEMORY_PROPERTY_HOST_CACHED_BIT: VkMemoryPropertyFlagBits = 1 << 3;
pub const VK_MEMORY_PROPERTY_HOST_COHERENT_BIT: VkMemoryPropertyFlagBits = 1 << 2;
pub const VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT: VkMemoryPropertyFlagBits = 1 << 1;
pub const VK_MEMORY_PROPERTY_LAZILY_ALLOCATED_BIT: VkMemoryPropertyFlagBits = 1 << 4;
pub const VK_MEMORY_PROPERTY_PROTECTED_BIT: VkMemoryPropertyFlagBits = 1 << 5;
pub const VK_NOT_READY: VkResult = 1;
pub const VK_OBJECT_TYPE_BUFFER: VkObjectType = 9;
pub const VK_OBJECT_TYPE_BUFFER_VIEW: VkObjectType = 13;
pub const VK_OBJECT_TYPE_COMMAND_BUFFER: VkObjectType = 6;
pub const VK_OBJECT_TYPE_COMMAND_POOL: VkObjectType = 25;
pub const VK_OBJECT_TYPE_DESCRIPTOR_POOL: VkObjectType = 22;
pub const VK_OBJECT_TYPE_DESCRIPTOR_SET: VkObjectType = 23;
pub const VK_OBJECT_TYPE_DESCRIPTOR_SET_LAYOUT: VkObjectType = 20;
pub const VK_OBJECT_TYPE_DESCRIPTOR_UPDATE_TEMPLATE: VkObjectType = 1000085000;
pub const VK_OBJECT_TYPE_DEVICE: VkObjectType = 3;
pub const VK_OBJECT_TYPE_DEVICE_MEMORY: VkObjectType = 8;
pub const VK_OBJECT_TYPE_EVENT: VkObjectType = 11;
pub const VK_OBJECT_TYPE_FENCE: VkObjectType = 7;
pub const VK_OBJECT_TYPE_FRAMEBUFFER: VkObjectType = 24;
pub const VK_OBJECT_TYPE_IMAGE: VkObjectType = 10;
pub const VK_OBJECT_TYPE_IMAGE_VIEW: VkObjectType = 14;
pub const VK_OBJECT_TYPE_INSTANCE: VkObjectType = 1;
pub const VK_OBJECT_TYPE_PHYSICAL_DEVICE: VkObjectType = 2;
pub const VK_OBJECT_TYPE_PIPELINE: VkObjectType = 19;
pub const VK_OBJECT_TYPE_PIPELINE_CACHE: VkObjectType = 16;
pub const VK_OBJECT_TYPE_PIPELINE_LAYOUT: VkObjectType = 17;
pub const VK_OBJECT_TYPE_PRIVATE_DATA_SLOT: VkObjectType = 1000295000;
pub const VK_OBJECT_TYPE_QUERY_POOL: VkObjectType = 12;
pub const VK_OBJECT_TYPE_QUEUE: VkObjectType = 4;
pub const VK_OBJECT_TYPE_RENDER_PASS: VkObjectType = 18;
pub const VK_OBJECT_TYPE_SAMPLER: VkObjectType = 21;
pub const VK_OBJECT_TYPE_SAMPLER_YCBCR_CONVERSION: VkObjectType = 1000156000;
pub const VK_OBJECT_TYPE_SEMAPHORE: VkObjectType = 5;
pub const VK_OBJECT_TYPE_SHADER_MODULE: VkObjectType = 15;
pub const VK_OBJECT_TYPE_UNKNOWN: VkObjectType = 0;
pub const VK_PEER_MEMORY_FEATURE_COPY_DST_BIT: VkPeerMemoryFeatureFlagBits = 1 << 1;
pub const VK_PEER_MEMORY_FEATURE_COPY_SRC_BIT: VkPeerMemoryFeatureFlagBits = 1 << 0;
pub const VK_PEER_MEMORY_FEATURE_GENERIC_DST_BIT: VkPeerMemoryFeatureFlagBits = 1 << 3;
pub const VK_PEER_MEMORY_FEATURE_GENERIC_SRC_BIT: VkPeerMemoryFeatureFlagBits = 1 << 2;
pub const VK_PHYSICAL_DEVICE_TYPE_CPU: VkPhysicalDeviceType = 4;
pub const VK_PHYSICAL_DEVICE_TYPE_DISCRETE_GPU: VkPhysicalDeviceType = 2;
pub const VK_PHYSICAL_DEVICE_TYPE_INTEGRATED_GPU: VkPhysicalDeviceType = 1;
pub const VK_PHYSICAL_DEVICE_TYPE_OTHER: VkPhysicalDeviceType = 0;
pub const VK_PHYSICAL_DEVICE_TYPE_VIRTUAL_GPU: VkPhysicalDeviceType = 3;
pub const VK_PIPELINE_BIND_POINT_COMPUTE: VkPipelineBindPoint = 1;
pub const VK_PIPELINE_BIND_POINT_GRAPHICS: VkPipelineBindPoint = 0;
pub const VK_PIPELINE_CACHE_CREATE_EXTERNALLY_SYNCHRONIZED_BIT: VkPipelineCacheCreateFlagBits = 1 << 0;
pub const VK_PIPELINE_CACHE_HEADER_VERSION_ONE: VkPipelineCacheHeaderVersion = 1;
pub const VK_PIPELINE_COMPILE_REQUIRED: VkResult = 1000297000;
pub const VK_PIPELINE_CREATE_2_ALLOW_DERIVATIVES_BIT: VkPipelineCreateFlagBits2 = 1 << 1;
pub const VK_PIPELINE_CREATE_2_DERIVATIVE_BIT: VkPipelineCreateFlagBits2 = 1 << 2;
pub const VK_PIPELINE_CREATE_2_DISABLE_OPTIMIZATION_BIT: VkPipelineCreateFlagBits2 = 1 << 0;
pub const VK_PIPELINE_CREATE_2_DISPATCH_BASE_BIT: VkPipelineCreateFlagBits2 = 1 << 4;
pub const VK_PIPELINE_CREATE_2_EARLY_RETURN_ON_FAILURE_BIT: VkPipelineCreateFlagBits2 = 1 << 9;
pub const VK_PIPELINE_CREATE_2_FAIL_ON_PIPELINE_COMPILE_REQUIRED_BIT: VkPipelineCreateFlagBits2 = 1 << 8;
pub const VK_PIPELINE_CREATE_2_NO_PROTECTED_ACCESS_BIT: VkPipelineCreateFlagBits2 = 1 << 27;
pub const VK_PIPELINE_CREATE_2_PROTECTED_ACCESS_ONLY_BIT: VkPipelineCreateFlagBits2 = 1 << 30;
pub const VK_PIPELINE_CREATE_2_VIEW_INDEX_FROM_DEVICE_INDEX_BIT: VkPipelineCreateFlagBits2 = 1 << 3;
pub const VK_PIPELINE_CREATE_ALLOW_DERIVATIVES_BIT: VkPipelineCreateFlagBits = 1 << 1;
pub const VK_PIPELINE_CREATE_DERIVATIVE_BIT: VkPipelineCreateFlagBits = 1 << 2;
pub const VK_PIPELINE_CREATE_DISABLE_OPTIMIZATION_BIT: VkPipelineCreateFlagBits = 1 << 0;
pub const VK_PIPELINE_CREATE_DISPATCH_BASE: VkPipelineCreateFlagBits = VK_PIPELINE_CREATE_DISPATCH_BASE_BIT;
pub const VK_PIPELINE_CREATE_DISPATCH_BASE_BIT: VkPipelineCreateFlagBits = 1 << 4;
pub const VK_PIPELINE_CREATE_EARLY_RETURN_ON_FAILURE_BIT: VkPipelineCreateFlagBits = 1 << 9;
pub const VK_PIPELINE_CREATE_FAIL_ON_PIPELINE_COMPILE_REQUIRED_BIT: VkPipelineCreateFlagBits = 1 << 8;
pub const VK_PIPELINE_CREATE_NO_PROTECTED_ACCESS_BIT: VkPipelineCreateFlagBits = 1 << 27;
pub const VK_PIPELINE_CREATE_PROTECTED_ACCESS_ONLY_BIT: VkPipelineCreateFlagBits = 1 << 30;
pub const VK_PIPELINE_CREATE_VIEW_INDEX_FROM_DEVICE_INDEX_BIT: VkPipelineCreateFlagBits = 1 << 3;
pub const VK_PIPELINE_CREATION_FEEDBACK_APPLICATION_PIPELINE_CACHE_HIT_BIT: VkPipelineCreationFeedbackFlagBits = 1 << 1;
pub const VK_PIPELINE_CREATION_FEEDBACK_BASE_PIPELINE_ACCELERATION_BIT: VkPipelineCreationFeedbackFlagBits = 1 << 2;
pub const VK_PIPELINE_CREATION_FEEDBACK_VALID_BIT: VkPipelineCreationFeedbackFlagBits = 1 << 0;
pub const VK_PIPELINE_ROBUSTNESS_BUFFER_BEHAVIOR_DEVICE_DEFAULT: VkPipelineRobustnessBufferBehavior = 0;
pub const VK_PIPELINE_ROBUSTNESS_BUFFER_BEHAVIOR_DISABLED: VkPipelineRobustnessBufferBehavior = 1;
pub const VK_PIPELINE_ROBUSTNESS_BUFFER_BEHAVIOR_ROBUST_BUFFER_ACCESS: VkPipelineRobustnessBufferBehavior = 2;
pub const VK_PIPELINE_ROBUSTNESS_BUFFER_BEHAVIOR_ROBUST_BUFFER_ACCESS_2: VkPipelineRobustnessBufferBehavior = 3;
pub const VK_PIPELINE_ROBUSTNESS_IMAGE_BEHAVIOR_DEVICE_DEFAULT: VkPipelineRobustnessImageBehavior = 0;
pub const VK_PIPELINE_ROBUSTNESS_IMAGE_BEHAVIOR_DISABLED: VkPipelineRobustnessImageBehavior = 1;
pub const VK_PIPELINE_ROBUSTNESS_IMAGE_BEHAVIOR_ROBUST_IMAGE_ACCESS: VkPipelineRobustnessImageBehavior = 2;
pub const VK_PIPELINE_ROBUSTNESS_IMAGE_BEHAVIOR_ROBUST_IMAGE_ACCESS_2: VkPipelineRobustnessImageBehavior = 3;
pub const VK_PIPELINE_SHADER_STAGE_CREATE_ALLOW_VARYING_SUBGROUP_SIZE_BIT: VkPipelineShaderStageCreateFlagBits = 1 << 0;
pub const VK_PIPELINE_SHADER_STAGE_CREATE_REQUIRE_FULL_SUBGROUPS_BIT: VkPipelineShaderStageCreateFlagBits = 1 << 1;
pub const VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT: VkPipelineStageFlagBits2 = 1 << 16;
pub const VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT: VkPipelineStageFlagBits2 = 1 << 15;
pub const VK_PIPELINE_STAGE_2_ALL_TRANSFER_BIT: VkPipelineStageFlagBits2 = 1 << 12;
pub const VK_PIPELINE_STAGE_2_BLIT_BIT: VkPipelineStageFlagBits2 = 1 << 34;
pub const VK_PIPELINE_STAGE_2_BOTTOM_OF_PIPE_BIT: VkPipelineStageFlagBits2 = 1 << 13;
pub const VK_PIPELINE_STAGE_2_CLEAR_BIT: VkPipelineStageFlagBits2 = 1 << 35;
pub const VK_PIPELINE_STAGE_2_COLOR_ATTACHMENT_OUTPUT_BIT: VkPipelineStageFlagBits2 = 1 << 10;
pub const VK_PIPELINE_STAGE_2_COMPUTE_SHADER_BIT: VkPipelineStageFlagBits2 = 1 << 11;
pub const VK_PIPELINE_STAGE_2_COPY_BIT: VkPipelineStageFlagBits2 = 1 << 32;
pub const VK_PIPELINE_STAGE_2_DRAW_INDIRECT_BIT: VkPipelineStageFlagBits2 = 1 << 1;
pub const VK_PIPELINE_STAGE_2_EARLY_FRAGMENT_TESTS_BIT: VkPipelineStageFlagBits2 = 1 << 8;
pub const VK_PIPELINE_STAGE_2_FRAGMENT_SHADER_BIT: VkPipelineStageFlagBits2 = 1 << 7;
pub const VK_PIPELINE_STAGE_2_GEOMETRY_SHADER_BIT: VkPipelineStageFlagBits2 = 1 << 6;
pub const VK_PIPELINE_STAGE_2_HOST_BIT: VkPipelineStageFlagBits2 = 1 << 14;
pub const VK_PIPELINE_STAGE_2_INDEX_INPUT_BIT: VkPipelineStageFlagBits2 = 1 << 36;
pub const VK_PIPELINE_STAGE_2_LATE_FRAGMENT_TESTS_BIT: VkPipelineStageFlagBits2 = 1 << 9;
pub const VK_PIPELINE_STAGE_2_NONE: VkPipelineStageFlagBits2 = 0;
pub const VK_PIPELINE_STAGE_2_PRE_RASTERIZATION_SHADERS_BIT: VkPipelineStageFlagBits2 = 1 << 38;
pub const VK_PIPELINE_STAGE_2_RESOLVE_BIT: VkPipelineStageFlagBits2 = 1 << 33;
pub const VK_PIPELINE_STAGE_2_TESSELLATION_CONTROL_SHADER_BIT: VkPipelineStageFlagBits2 = 1 << 4;
pub const VK_PIPELINE_STAGE_2_TESSELLATION_EVALUATION_SHADER_BIT: VkPipelineStageFlagBits2 = 1 << 5;
pub const VK_PIPELINE_STAGE_2_TOP_OF_PIPE_BIT: VkPipelineStageFlagBits2 = 1 << 0;
pub const VK_PIPELINE_STAGE_2_TRANSFER_BIT: VkPipelineStageFlagBits2 = VK_PIPELINE_STAGE_2_ALL_TRANSFER_BIT;
pub const VK_PIPELINE_STAGE_2_VERTEX_ATTRIBUTE_INPUT_BIT: VkPipelineStageFlagBits2 = 1 << 37;
pub const VK_PIPELINE_STAGE_2_VERTEX_INPUT_BIT: VkPipelineStageFlagBits2 = 1 << 2;
pub const VK_PIPELINE_STAGE_2_VERTEX_SHADER_BIT: VkPipelineStageFlagBits2 = 1 << 3;
pub const VK_PIPELINE_STAGE_ALL_COMMANDS_BIT: VkPipelineStageFlagBits = 1 << 16;
pub const VK_PIPELINE_STAGE_ALL_GRAPHICS_BIT: VkPipelineStageFlagBits = 1 << 15;
pub const VK_PIPELINE_STAGE_BOTTOM_OF_PIPE_BIT: VkPipelineStageFlagBits = 1 << 13;
pub const VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT: VkPipelineStageFlagBits = 1 << 10;
pub const VK_PIPELINE_STAGE_COMPUTE_SHADER_BIT: VkPipelineStageFlagBits = 1 << 11;
pub const VK_PIPELINE_STAGE_DRAW_INDIRECT_BIT: VkPipelineStageFlagBits = 1 << 1;
pub const VK_PIPELINE_STAGE_EARLY_FRAGMENT_TESTS_BIT: VkPipelineStageFlagBits = 1 << 8;
pub const VK_PIPELINE_STAGE_FRAGMENT_SHADER_BIT: VkPipelineStageFlagBits = 1 << 7;
pub const VK_PIPELINE_STAGE_GEOMETRY_SHADER_BIT: VkPipelineStageFlagBits = 1 << 6;
pub const VK_PIPELINE_STAGE_HOST_BIT: VkPipelineStageFlagBits = 1 << 14;
pub const VK_PIPELINE_STAGE_LATE_FRAGMENT_TESTS_BIT: VkPipelineStageFlagBits = 1 << 9;
pub const VK_PIPELINE_STAGE_NONE: VkPipelineStageFlagBits = 0;
pub const VK_PIPELINE_STAGE_TESSELLATION_CONTROL_SHADER_BIT: VkPipelineStageFlagBits = 1 << 4;
pub const VK_PIPELINE_STAGE_TESSELLATION_EVALUATION_SHADER_BIT: VkPipelineStageFlagBits = 1 << 5;
pub const VK_PIPELINE_STAGE_TOP_OF_PIPE_BIT: VkPipelineStageFlagBits = 1 << 0;
pub const VK_PIPELINE_STAGE_TRANSFER_BIT: VkPipelineStageFlagBits = 1 << 12;
pub const VK_PIPELINE_STAGE_VERTEX_INPUT_BIT: VkPipelineStageFlagBits = 1 << 2;
pub const VK_PIPELINE_STAGE_VERTEX_SHADER_BIT: VkPipelineStageFlagBits = 1 << 3;
pub const VK_POINT_CLIPPING_BEHAVIOR_ALL_CLIP_PLANES: VkPointClippingBehavior = 0;
pub const VK_POINT_CLIPPING_BEHAVIOR_USER_CLIP_PLANES_ONLY: VkPointClippingBehavior = 1;
pub const VK_POLYGON_MODE_FILL: VkPolygonMode = 0;
pub const VK_POLYGON_MODE_LINE: VkPolygonMode = 1;
pub const VK_POLYGON_MODE_POINT: VkPolygonMode = 2;
pub const VK_PRIMITIVE_TOPOLOGY_LINE_LIST: VkPrimitiveTopology = 1;
pub const VK_PRIMITIVE_TOPOLOGY_LINE_LIST_WITH_ADJACENCY: VkPrimitiveTopology = 6;
pub const VK_PRIMITIVE_TOPOLOGY_LINE_STRIP: VkPrimitiveTopology = 2;
pub const VK_PRIMITIVE_TOPOLOGY_LINE_STRIP_WITH_ADJACENCY: VkPrimitiveTopology = 7;
pub const VK_PRIMITIVE_TOPOLOGY_PATCH_LIST: VkPrimitiveTopology = 10;
pub const VK_PRIMITIVE_TOPOLOGY_POINT_LIST: VkPrimitiveTopology = 0;
pub const VK_PRIMITIVE_TOPOLOGY_TRIANGLE_FAN: VkPrimitiveTopology = 5;
pub const VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST: VkPrimitiveTopology = 3;
pub const VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST_WITH_ADJACENCY: VkPrimitiveTopology = 8;
pub const VK_PRIMITIVE_TOPOLOGY_TRIANGLE_STRIP: VkPrimitiveTopology = 4;
pub const VK_PRIMITIVE_TOPOLOGY_TRIANGLE_STRIP_WITH_ADJACENCY: VkPrimitiveTopology = 9;
pub const VK_QUERY_CONTROL_PRECISE_BIT: VkQueryControlFlagBits = 1 << 0;
pub const VK_QUERY_PIPELINE_STATISTIC_CLIPPING_INVOCATIONS_BIT: VkQueryPipelineStatisticFlagBits = 1 << 5;
pub const VK_QUERY_PIPELINE_STATISTIC_CLIPPING_PRIMITIVES_BIT: VkQueryPipelineStatisticFlagBits = 1 << 6;
pub const VK_QUERY_PIPELINE_STATISTIC_COMPUTE_SHADER_INVOCATIONS_BIT: VkQueryPipelineStatisticFlagBits = 1 << 10;
pub const VK_QUERY_PIPELINE_STATISTIC_FRAGMENT_SHADER_INVOCATIONS_BIT: VkQueryPipelineStatisticFlagBits = 1 << 7;
pub const VK_QUERY_PIPELINE_STATISTIC_GEOMETRY_SHADER_INVOCATIONS_BIT: VkQueryPipelineStatisticFlagBits = 1 << 3;
pub const VK_QUERY_PIPELINE_STATISTIC_GEOMETRY_SHADER_PRIMITIVES_BIT: VkQueryPipelineStatisticFlagBits = 1 << 4;
pub const VK_QUERY_PIPELINE_STATISTIC_INPUT_ASSEMBLY_PRIMITIVES_BIT: VkQueryPipelineStatisticFlagBits = 1 << 1;
pub const VK_QUERY_PIPELINE_STATISTIC_INPUT_ASSEMBLY_VERTICES_BIT: VkQueryPipelineStatisticFlagBits = 1 << 0;
pub const VK_QUERY_PIPELINE_STATISTIC_TESSELLATION_CONTROL_SHADER_PATCHES_BIT: VkQueryPipelineStatisticFlagBits = 1 << 8;
pub const VK_QUERY_PIPELINE_STATISTIC_TESSELLATION_EVALUATION_SHADER_INVOCATIONS_BIT: VkQueryPipelineStatisticFlagBits = 1 << 9;
pub const VK_QUERY_PIPELINE_STATISTIC_VERTEX_SHADER_INVOCATIONS_BIT: VkQueryPipelineStatisticFlagBits = 1 << 2;
pub const VK_QUERY_RESULT_64_BIT: VkQueryResultFlagBits = 1 << 0;
pub const VK_QUERY_RESULT_PARTIAL_BIT: VkQueryResultFlagBits = 1 << 3;
pub const VK_QUERY_RESULT_WAIT_BIT: VkQueryResultFlagBits = 1 << 1;
pub const VK_QUERY_RESULT_WITH_AVAILABILITY_BIT: VkQueryResultFlagBits = 1 << 2;
pub const VK_QUERY_TYPE_OCCLUSION: VkQueryType = 0;
pub const VK_QUERY_TYPE_PIPELINE_STATISTICS: VkQueryType = 1;
pub const VK_QUERY_TYPE_TIMESTAMP: VkQueryType = 2;
pub const VK_QUEUE_COMPUTE_BIT: VkQueueFlagBits = 1 << 1;
pub const VK_QUEUE_FAMILY_EXTERNAL: u32 = !1;
pub const VK_QUEUE_FAMILY_IGNORED: u32 = !0;
pub const VK_QUEUE_GLOBAL_PRIORITY_HIGH: VkQueueGlobalPriority = 512;
pub const VK_QUEUE_GLOBAL_PRIORITY_LOW: VkQueueGlobalPriority = 128;
pub const VK_QUEUE_GLOBAL_PRIORITY_MEDIUM: VkQueueGlobalPriority = 256;
pub const VK_QUEUE_GLOBAL_PRIORITY_REALTIME: VkQueueGlobalPriority = 1024;
pub const VK_QUEUE_GRAPHICS_BIT: VkQueueFlagBits = 1 << 0;
pub const VK_QUEUE_PROTECTED_BIT: VkQueueFlagBits = 1 << 4;
pub const VK_QUEUE_SPARSE_BINDING_BIT: VkQueueFlagBits = 1 << 3;
pub const VK_QUEUE_TRANSFER_BIT: VkQueueFlagBits = 1 << 2;
pub const VK_REMAINING_ARRAY_LAYERS: u32 = !0;
pub const VK_REMAINING_MIP_LEVELS: u32 = !0;
pub const VK_RENDERING_CONTENTS_SECONDARY_COMMAND_BUFFERS_BIT: VkRenderingFlagBits = 1 << 0;
pub const VK_RENDERING_RESUMING_BIT: VkRenderingFlagBits = 1 << 2;
pub const VK_RENDERING_SUSPENDING_BIT: VkRenderingFlagBits = 1 << 1;
pub const VK_RESOLVE_MODE_AVERAGE_BIT: VkResolveModeFlagBits = 1 << 1;
pub const VK_RESOLVE_MODE_MAX_BIT: VkResolveModeFlagBits = 1 << 3;
pub const VK_RESOLVE_MODE_MIN_BIT: VkResolveModeFlagBits = 1 << 2;
pub const VK_RESOLVE_MODE_NONE: VkResolveModeFlagBits = 0;
pub const VK_RESOLVE_MODE_SAMPLE_ZERO_BIT: VkResolveModeFlagBits = 1 << 0;
pub const VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_BORDER: VkSamplerAddressMode = 3;
pub const VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_EDGE: VkSamplerAddressMode = 2;
pub const VK_SAMPLER_ADDRESS_MODE_MIRRORED_REPEAT: VkSamplerAddressMode = 1;
pub const VK_SAMPLER_ADDRESS_MODE_MIRROR_CLAMP_TO_EDGE: VkSamplerAddressMode = 4;
pub const VK_SAMPLER_ADDRESS_MODE_REPEAT: VkSamplerAddressMode = 0;
pub const VK_SAMPLER_MIPMAP_MODE_LINEAR: VkSamplerMipmapMode = 1;
pub const VK_SAMPLER_MIPMAP_MODE_NEAREST: VkSamplerMipmapMode = 0;
pub const VK_SAMPLER_REDUCTION_MODE_MAX: VkSamplerReductionMode = 2;
pub const VK_SAMPLER_REDUCTION_MODE_MIN: VkSamplerReductionMode = 1;
pub const VK_SAMPLER_REDUCTION_MODE_WEIGHTED_AVERAGE: VkSamplerReductionMode = 0;
pub const VK_SAMPLER_YCBCR_MODEL_CONVERSION_RGB_IDENTITY: VkSamplerYcbcrModelConversion = 0;
pub const VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_2020: VkSamplerYcbcrModelConversion = 4;
pub const VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_601: VkSamplerYcbcrModelConversion = 3;
pub const VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_709: VkSamplerYcbcrModelConversion = 2;
pub const VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_IDENTITY: VkSamplerYcbcrModelConversion = 1;
pub const VK_SAMPLER_YCBCR_RANGE_ITU_FULL: VkSamplerYcbcrRange = 0;
pub const VK_SAMPLER_YCBCR_RANGE_ITU_NARROW: VkSamplerYcbcrRange = 1;
pub const VK_SAMPLE_COUNT_16_BIT: VkSampleCountFlagBits = 1 << 4;
pub const VK_SAMPLE_COUNT_1_BIT: VkSampleCountFlagBits = 1 << 0;
pub const VK_SAMPLE_COUNT_2_BIT: VkSampleCountFlagBits = 1 << 1;
pub const VK_SAMPLE_COUNT_32_BIT: VkSampleCountFlagBits = 1 << 5;
pub const VK_SAMPLE_COUNT_4_BIT: VkSampleCountFlagBits = 1 << 2;
pub const VK_SAMPLE_COUNT_64_BIT: VkSampleCountFlagBits = 1 << 6;
pub const VK_SAMPLE_COUNT_8_BIT: VkSampleCountFlagBits = 1 << 3;
pub const VK_SEMAPHORE_IMPORT_TEMPORARY_BIT: VkSemaphoreImportFlagBits = 1 << 0;
pub const VK_SEMAPHORE_TYPE_BINARY: VkSemaphoreType = 0;
pub const VK_SEMAPHORE_TYPE_TIMELINE: VkSemaphoreType = 1;
pub const VK_SEMAPHORE_WAIT_ANY_BIT: VkSemaphoreWaitFlagBits = 1 << 0;
pub const VK_SHADER_FLOAT_CONTROLS_INDEPENDENCE_32_BIT_ONLY: VkShaderFloatControlsIndependence = 0;
pub const VK_SHADER_FLOAT_CONTROLS_INDEPENDENCE_ALL: VkShaderFloatControlsIndependence = 1;
pub const VK_SHADER_FLOAT_CONTROLS_INDEPENDENCE_NONE: VkShaderFloatControlsIndependence = 2;
pub const VK_SHADER_STAGE_ALL: VkShaderStageFlagBits = 0x7FFFFFFF;
pub const VK_SHADER_STAGE_ALL_GRAPHICS: VkShaderStageFlagBits = 0x0000001F;
pub const VK_SHADER_STAGE_COMPUTE_BIT: VkShaderStageFlagBits = 1 << 5;
pub const VK_SHADER_STAGE_FRAGMENT_BIT: VkShaderStageFlagBits = 1 << 4;
pub const VK_SHADER_STAGE_GEOMETRY_BIT: VkShaderStageFlagBits = 1 << 3;
pub const VK_SHADER_STAGE_TESSELLATION_CONTROL_BIT: VkShaderStageFlagBits = 1 << 1;
pub const VK_SHADER_STAGE_TESSELLATION_EVALUATION_BIT: VkShaderStageFlagBits = 1 << 2;
pub const VK_SHADER_STAGE_VERTEX_BIT: VkShaderStageFlagBits = 1 << 0;
pub const VK_SHARING_MODE_CONCURRENT: VkSharingMode = 1;
pub const VK_SHARING_MODE_EXCLUSIVE: VkSharingMode = 0;
pub const VK_SPARSE_IMAGE_FORMAT_ALIGNED_MIP_SIZE_BIT: VkSparseImageFormatFlagBits = 1 << 1;
pub const VK_SPARSE_IMAGE_FORMAT_NONSTANDARD_BLOCK_SIZE_BIT: VkSparseImageFormatFlagBits = 1 << 2;
pub const VK_SPARSE_IMAGE_FORMAT_SINGLE_MIPTAIL_BIT: VkSparseImageFormatFlagBits = 1 << 0;
pub const VK_SPARSE_MEMORY_BIND_METADATA_BIT: VkSparseMemoryBindFlagBits = 1 << 0;
pub const VK_STENCIL_FACE_BACK_BIT: VkStencilFaceFlagBits = 1 << 1;
pub const VK_STENCIL_FACE_FRONT_AND_BACK: VkStencilFaceFlagBits = 0x00000003;
pub const VK_STENCIL_FACE_FRONT_BIT: VkStencilFaceFlagBits = 1 << 0;
pub const VK_STENCIL_FRONT_AND_BACK: VkStencilFaceFlagBits = VK_STENCIL_FACE_FRONT_AND_BACK;
pub const VK_STENCIL_OP_DECREMENT_AND_CLAMP: VkStencilOp = 4;
pub const VK_STENCIL_OP_DECREMENT_AND_WRAP: VkStencilOp = 7;
pub const VK_STENCIL_OP_INCREMENT_AND_CLAMP: VkStencilOp = 3;
pub const VK_STENCIL_OP_INCREMENT_AND_WRAP: VkStencilOp = 6;
pub const VK_STENCIL_OP_INVERT: VkStencilOp = 5;
pub const VK_STENCIL_OP_KEEP: VkStencilOp = 0;
pub const VK_STENCIL_OP_REPLACE: VkStencilOp = 2;
pub const VK_STENCIL_OP_ZERO: VkStencilOp = 1;
pub const VK_STRUCTURE_TYPE_APPLICATION_INFO: VkStructureType = 0;
pub const VK_STRUCTURE_TYPE_ATTACHMENT_DESCRIPTION_2: VkStructureType = 1000109000;
pub const VK_STRUCTURE_TYPE_ATTACHMENT_DESCRIPTION_STENCIL_LAYOUT: VkStructureType = 1000241002;
pub const VK_STRUCTURE_TYPE_ATTACHMENT_REFERENCE_2: VkStructureType = 1000109001;
pub const VK_STRUCTURE_TYPE_ATTACHMENT_REFERENCE_STENCIL_LAYOUT: VkStructureType = 1000241001;
pub const VK_STRUCTURE_TYPE_BIND_BUFFER_MEMORY_DEVICE_GROUP_INFO: VkStructureType = 1000060013;
pub const VK_STRUCTURE_TYPE_BIND_BUFFER_MEMORY_INFO: VkStructureType = 1000157000;
pub const VK_STRUCTURE_TYPE_BIND_DESCRIPTOR_SETS_INFO: VkStructureType = 1000545003;
pub const VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_DEVICE_GROUP_INFO: VkStructureType = 1000060014;
pub const VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_INFO: VkStructureType = 1000157001;
pub const VK_STRUCTURE_TYPE_BIND_IMAGE_PLANE_MEMORY_INFO: VkStructureType = 1000156002;
pub const VK_STRUCTURE_TYPE_BIND_MEMORY_STATUS: VkStructureType = 1000545002;
pub const VK_STRUCTURE_TYPE_BIND_SPARSE_INFO: VkStructureType = 7;
pub const VK_STRUCTURE_TYPE_BLIT_IMAGE_INFO_2: VkStructureType = 1000337004;
pub const VK_STRUCTURE_TYPE_BUFFER_COPY_2: VkStructureType = 1000337006;
pub const VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO: VkStructureType = 12;
pub const VK_STRUCTURE_TYPE_BUFFER_DEVICE_ADDRESS_INFO: VkStructureType = 1000244001;
pub const VK_STRUCTURE_TYPE_BUFFER_IMAGE_COPY_2: VkStructureType = 1000337009;
pub const VK_STRUCTURE_TYPE_BUFFER_MEMORY_BARRIER: VkStructureType = 44;
pub const VK_STRUCTURE_TYPE_BUFFER_MEMORY_BARRIER_2: VkStructureType = 1000314001;
pub const VK_STRUCTURE_TYPE_BUFFER_MEMORY_REQUIREMENTS_INFO_2: VkStructureType = 1000146000;
pub const VK_STRUCTURE_TYPE_BUFFER_OPAQUE_CAPTURE_ADDRESS_CREATE_INFO: VkStructureType = 1000257002;
pub const VK_STRUCTURE_TYPE_BUFFER_USAGE_FLAGS_2_CREATE_INFO: VkStructureType = 1000470006;
pub const VK_STRUCTURE_TYPE_BUFFER_VIEW_CREATE_INFO: VkStructureType = 13;
pub const VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO: VkStructureType = 40;
pub const VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO: VkStructureType = 42;
pub const VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_INFO: VkStructureType = 41;
pub const VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_RENDERING_INFO: VkStructureType = 1000044004;
pub const VK_STRUCTURE_TYPE_COMMAND_BUFFER_SUBMIT_INFO: VkStructureType = 1000314006;
pub const VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO: VkStructureType = 39;
pub const VK_STRUCTURE_TYPE_COMPUTE_PIPELINE_CREATE_INFO: VkStructureType = 29;
pub const VK_STRUCTURE_TYPE_COPY_BUFFER_INFO_2: VkStructureType = 1000337000;
pub const VK_STRUCTURE_TYPE_COPY_BUFFER_TO_IMAGE_INFO_2: VkStructureType = 1000337002;
pub const VK_STRUCTURE_TYPE_COPY_DESCRIPTOR_SET: VkStructureType = 36;
pub const VK_STRUCTURE_TYPE_COPY_IMAGE_INFO_2: VkStructureType = 1000337001;
pub const VK_STRUCTURE_TYPE_COPY_IMAGE_TO_BUFFER_INFO_2: VkStructureType = 1000337003;
pub const VK_STRUCTURE_TYPE_COPY_IMAGE_TO_IMAGE_INFO: VkStructureType = 1000270007;
pub const VK_STRUCTURE_TYPE_COPY_IMAGE_TO_MEMORY_INFO: VkStructureType = 1000270004;
pub const VK_STRUCTURE_TYPE_COPY_MEMORY_TO_IMAGE_INFO: VkStructureType = 1000270005;
pub const VK_STRUCTURE_TYPE_DEPENDENCY_INFO: VkStructureType = 1000314003;
pub const VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO: VkStructureType = 33;
pub const VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_INLINE_UNIFORM_BLOCK_CREATE_INFO: VkStructureType = 1000138003;
pub const VK_STRUCTURE_TYPE_DESCRIPTOR_SET_ALLOCATE_INFO: VkStructureType = 34;
pub const VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_BINDING_FLAGS_CREATE_INFO: VkStructureType = 1000161000;
pub const VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO: VkStructureType = 32;
pub const VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_SUPPORT: VkStructureType = 1000168001;
pub const VK_STRUCTURE_TYPE_DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_ALLOCATE_INFO: VkStructureType = 1000161003;
pub const VK_STRUCTURE_TYPE_DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_LAYOUT_SUPPORT: VkStructureType = 1000161004;
pub const VK_STRUCTURE_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO: VkStructureType = 1000085000;
pub const VK_STRUCTURE_TYPE_DEVICE_BUFFER_MEMORY_REQUIREMENTS: VkStructureType = 1000413002;
pub const VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO: VkStructureType = 3;
pub const VK_STRUCTURE_TYPE_DEVICE_GROUP_BIND_SPARSE_INFO: VkStructureType = 1000060006;
pub const VK_STRUCTURE_TYPE_DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO: VkStructureType = 1000060004;
pub const VK_STRUCTURE_TYPE_DEVICE_GROUP_DEVICE_CREATE_INFO: VkStructureType = 1000070001;
pub const VK_STRUCTURE_TYPE_DEVICE_GROUP_RENDER_PASS_BEGIN_INFO: VkStructureType = 1000060003;
pub const VK_STRUCTURE_TYPE_DEVICE_GROUP_SUBMIT_INFO: VkStructureType = 1000060005;
pub const VK_STRUCTURE_TYPE_DEVICE_IMAGE_MEMORY_REQUIREMENTS: VkStructureType = 1000413003;
pub const VK_STRUCTURE_TYPE_DEVICE_IMAGE_SUBRESOURCE_INFO: VkStructureType = 1000470004;
pub const VK_STRUCTURE_TYPE_DEVICE_MEMORY_OPAQUE_CAPTURE_ADDRESS_INFO: VkStructureType = 1000257004;
pub const VK_STRUCTURE_TYPE_DEVICE_PRIVATE_DATA_CREATE_INFO: VkStructureType = 1000295001;
pub const VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO: VkStructureType = 2;
pub const VK_STRUCTURE_TYPE_DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO: VkStructureType = 1000174000;
pub const VK_STRUCTURE_TYPE_DEVICE_QUEUE_INFO_2: VkStructureType = 1000145003;
pub const VK_STRUCTURE_TYPE_EVENT_CREATE_INFO: VkStructureType = 10;
pub const VK_STRUCTURE_TYPE_EXPORT_FENCE_CREATE_INFO: VkStructureType = 1000113000;
pub const VK_STRUCTURE_TYPE_EXPORT_MEMORY_ALLOCATE_INFO: VkStructureType = 1000072002;
pub const VK_STRUCTURE_TYPE_EXPORT_SEMAPHORE_CREATE_INFO: VkStructureType = 1000077000;
pub const VK_STRUCTURE_TYPE_EXTERNAL_BUFFER_PROPERTIES: VkStructureType = 1000071003;
pub const VK_STRUCTURE_TYPE_EXTERNAL_FENCE_PROPERTIES: VkStructureType = 1000112001;
pub const VK_STRUCTURE_TYPE_EXTERNAL_IMAGE_FORMAT_PROPERTIES: VkStructureType = 1000071001;
pub const VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_BUFFER_CREATE_INFO: VkStructureType = 1000072000;
pub const VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_IMAGE_CREATE_INFO: VkStructureType = 1000072001;
pub const VK_STRUCTURE_TYPE_EXTERNAL_SEMAPHORE_PROPERTIES: VkStructureType = 1000076001;
pub const VK_STRUCTURE_TYPE_FENCE_CREATE_INFO: VkStructureType = 8;
pub const VK_STRUCTURE_TYPE_FORMAT_PROPERTIES_2: VkStructureType = 1000059002;
pub const VK_STRUCTURE_TYPE_FORMAT_PROPERTIES_3: VkStructureType = 1000360000;
pub const VK_STRUCTURE_TYPE_FRAMEBUFFER_ATTACHMENTS_CREATE_INFO: VkStructureType = 1000108001;
pub const VK_STRUCTURE_TYPE_FRAMEBUFFER_ATTACHMENT_IMAGE_INFO: VkStructureType = 1000108002;
pub const VK_STRUCTURE_TYPE_FRAMEBUFFER_CREATE_INFO: VkStructureType = 37;
pub const VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_CREATE_INFO: VkStructureType = 28;
pub const VK_STRUCTURE_TYPE_HOST_IMAGE_COPY_DEVICE_PERFORMANCE_QUERY: VkStructureType = 1000270009;
pub const VK_STRUCTURE_TYPE_HOST_IMAGE_LAYOUT_TRANSITION_INFO: VkStructureType = 1000270006;
pub const VK_STRUCTURE_TYPE_IMAGE_BLIT_2: VkStructureType = 1000337008;
pub const VK_STRUCTURE_TYPE_IMAGE_COPY_2: VkStructureType = 1000337007;
pub const VK_STRUCTURE_TYPE_IMAGE_CREATE_INFO: VkStructureType = 14;
pub const VK_STRUCTURE_TYPE_IMAGE_FORMAT_LIST_CREATE_INFO: VkStructureType = 1000147000;
pub const VK_STRUCTURE_TYPE_IMAGE_FORMAT_PROPERTIES_2: VkStructureType = 1000059003;
pub const VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER: VkStructureType = 45;
pub const VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER_2: VkStructureType = 1000314002;
pub const VK_STRUCTURE_TYPE_IMAGE_MEMORY_REQUIREMENTS_INFO_2: VkStructureType = 1000146001;
pub const VK_STRUCTURE_TYPE_IMAGE_PLANE_MEMORY_REQUIREMENTS_INFO: VkStructureType = 1000156003;
pub const VK_STRUCTURE_TYPE_IMAGE_RESOLVE_2: VkStructureType = 1000337010;
pub const VK_STRUCTURE_TYPE_IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2: VkStructureType = 1000146002;
pub const VK_STRUCTURE_TYPE_IMAGE_STENCIL_USAGE_CREATE_INFO: VkStructureType = 1000246000;
pub const VK_STRUCTURE_TYPE_IMAGE_SUBRESOURCE_2: VkStructureType = 1000338003;
pub const VK_STRUCTURE_TYPE_IMAGE_TO_MEMORY_COPY: VkStructureType = 1000270003;
pub const VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO: VkStructureType = 15;
pub const VK_STRUCTURE_TYPE_IMAGE_VIEW_USAGE_CREATE_INFO: VkStructureType = 1000117002;
pub const VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO: VkStructureType = 1;
pub const VK_STRUCTURE_TYPE_LOADER_DEVICE_CREATE_INFO: VkStructureType = 48;
pub const VK_STRUCTURE_TYPE_LOADER_INSTANCE_CREATE_INFO: VkStructureType = 47;
pub const VK_STRUCTURE_TYPE_MAPPED_MEMORY_RANGE: VkStructureType = 6;
pub const VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_FLAGS_INFO: VkStructureType = 1000060000;
pub const VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO: VkStructureType = 5;
pub const VK_STRUCTURE_TYPE_MEMORY_BARRIER: VkStructureType = 46;
pub const VK_STRUCTURE_TYPE_MEMORY_BARRIER_2: VkStructureType = 1000314000;
pub const VK_STRUCTURE_TYPE_MEMORY_DEDICATED_ALLOCATE_INFO: VkStructureType = 1000127001;
pub const VK_STRUCTURE_TYPE_MEMORY_DEDICATED_REQUIREMENTS: VkStructureType = 1000127000;
pub const VK_STRUCTURE_TYPE_MEMORY_MAP_INFO: VkStructureType = 1000271000;
pub const VK_STRUCTURE_TYPE_MEMORY_OPAQUE_CAPTURE_ADDRESS_ALLOCATE_INFO: VkStructureType = 1000257003;
pub const VK_STRUCTURE_TYPE_MEMORY_REQUIREMENTS_2: VkStructureType = 1000146003;
pub const VK_STRUCTURE_TYPE_MEMORY_TO_IMAGE_COPY: VkStructureType = 1000270002;
pub const VK_STRUCTURE_TYPE_MEMORY_UNMAP_INFO: VkStructureType = 1000271001;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES: VkStructureType = 1000083000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_8BIT_STORAGE_FEATURES: VkStructureType = 1000177000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES: VkStructureType = 1000257000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEPTH_STENCIL_RESOLVE_PROPERTIES: VkStructureType = 1000199000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_FEATURES: VkStructureType = 1000161001;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_PROPERTIES: VkStructureType = 1000161002;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DRIVER_PROPERTIES: VkStructureType = 1000196000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DYNAMIC_RENDERING_FEATURES: VkStructureType = 1000044003;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DYNAMIC_RENDERING_LOCAL_READ_FEATURES: VkStructureType = 1000232000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO: VkStructureType = 1000071002;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO: VkStructureType = 1000112000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO: VkStructureType = 1000071000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO: VkStructureType = 1000076000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FEATURES_2: VkStructureType = 1000059000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FLOAT_CONTROLS_PROPERTIES: VkStructureType = 1000197000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GLOBAL_PRIORITY_QUERY_FEATURES: VkStructureType = 1000388000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GROUP_PROPERTIES: VkStructureType = 1000070000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_HOST_IMAGE_COPY_FEATURES: VkStructureType = 1000270000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_HOST_IMAGE_COPY_PROPERTIES: VkStructureType = 1000270001;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_HOST_QUERY_RESET_FEATURES: VkStructureType = 1000261000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ID_PROPERTIES: VkStructureType = 1000071004;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGELESS_FRAMEBUFFER_FEATURES: VkStructureType = 1000108000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2: VkStructureType = 1000059004;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_ROBUSTNESS_FEATURES: VkStructureType = 1000335000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_INDEX_TYPE_UINT8_FEATURES: VkStructureType = 1000265000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_FEATURES: VkStructureType = 1000138000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_PROPERTIES: VkStructureType = 1000138001;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_LINE_RASTERIZATION_FEATURES: VkStructureType = 1000259000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_LINE_RASTERIZATION_PROPERTIES: VkStructureType = 1000259002;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_3_PROPERTIES: VkStructureType = 1000168000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_4_FEATURES: VkStructureType = 1000413000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_4_PROPERTIES: VkStructureType = 1000413001;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_5_FEATURES: VkStructureType = 1000470000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_5_PROPERTIES: VkStructureType = 1000470001;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_6_FEATURES: VkStructureType = 1000545000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_6_PROPERTIES: VkStructureType = 1000545001;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MEMORY_PROPERTIES_2: VkStructureType = 1000059006;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_FEATURES: VkStructureType = 1000053001;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES: VkStructureType = 1000053002;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_CREATION_CACHE_CONTROL_FEATURES: VkStructureType = 1000297000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_PROTECTED_ACCESS_FEATURES: VkStructureType = 1000466000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_ROBUSTNESS_FEATURES: VkStructureType = 1000068001;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_ROBUSTNESS_PROPERTIES: VkStructureType = 1000068002;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_POINT_CLIPPING_PROPERTIES: VkStructureType = 1000117000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PRIVATE_DATA_FEATURES: VkStructureType = 1000295000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROPERTIES_2: VkStructureType = 1000059001;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROTECTED_MEMORY_FEATURES: VkStructureType = 1000145001;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROTECTED_MEMORY_PROPERTIES: VkStructureType = 1000145002;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PUSH_DESCRIPTOR_PROPERTIES: VkStructureType = 1000080000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SAMPLER_FILTER_MINMAX_PROPERTIES: VkStructureType = 1000130000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES: VkStructureType = 1000156004;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SCALAR_BLOCK_LAYOUT_FEATURES: VkStructureType = 1000221000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SEPARATE_DEPTH_STENCIL_LAYOUTS_FEATURES: VkStructureType = 1000241000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_ATOMIC_INT64_FEATURES: VkStructureType = 1000180000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_DEMOTE_TO_HELPER_INVOCATION_FEATURES: VkStructureType = 1000276000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_DRAW_PARAMETERS_FEATURES: VkStructureType = 1000063000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_DRAW_PARAMETER_FEATURES: VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_DRAW_PARAMETERS_FEATURES;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_EXPECT_ASSUME_FEATURES: VkStructureType = 1000544000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_FLOAT16_INT8_FEATURES: VkStructureType = 1000082000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_FLOAT_CONTROLS_2_FEATURES: VkStructureType = 1000528000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_FEATURES: VkStructureType = 1000280000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_PROPERTIES: VkStructureType = 1000280001;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_SUBGROUP_EXTENDED_TYPES_FEATURES: VkStructureType = 1000175000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_SUBGROUP_ROTATE_FEATURES: VkStructureType = 1000416000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_TERMINATE_INVOCATION_FEATURES: VkStructureType = 1000215000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2: VkStructureType = 1000059008;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBGROUP_PROPERTIES: VkStructureType = 1000094000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_FEATURES: VkStructureType = 1000225002;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_PROPERTIES: VkStructureType = 1000225000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SYNCHRONIZATION_2_FEATURES: VkStructureType = 1000314007;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_PROPERTIES: VkStructureType = 1000281001;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TEXTURE_COMPRESSION_ASTC_HDR_FEATURES: VkStructureType = 1000066000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_FEATURES: VkStructureType = 1000207000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_PROPERTIES: VkStructureType = 1000207001;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TOOL_PROPERTIES: VkStructureType = 1000245000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_UNIFORM_BUFFER_STANDARD_LAYOUT_FEATURES: VkStructureType = 1000253000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES: VkStructureType = 1000120000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VARIABLE_POINTER_FEATURES: VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_FEATURES: VkStructureType = 1000190002;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_PROPERTIES: VkStructureType = 1000525000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_1_FEATURES: VkStructureType = 49;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_1_PROPERTIES: VkStructureType = 50;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_2_FEATURES: VkStructureType = 51;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_2_PROPERTIES: VkStructureType = 52;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_3_FEATURES: VkStructureType = 53;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_3_PROPERTIES: VkStructureType = 54;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_4_FEATURES: VkStructureType = 55;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_4_PROPERTIES: VkStructureType = 56;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_MEMORY_MODEL_FEATURES: VkStructureType = 1000211000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ZERO_INITIALIZE_WORKGROUP_MEMORY_FEATURES: VkStructureType = 1000325000;
pub const VK_STRUCTURE_TYPE_PIPELINE_CACHE_CREATE_INFO: VkStructureType = 17;
pub const VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_STATE_CREATE_INFO: VkStructureType = 26;
pub const VK_STRUCTURE_TYPE_PIPELINE_CREATE_FLAGS_2_CREATE_INFO: VkStructureType = 1000470005;
pub const VK_STRUCTURE_TYPE_PIPELINE_CREATION_FEEDBACK_CREATE_INFO: VkStructureType = 1000192000;
pub const VK_STRUCTURE_TYPE_PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO: VkStructureType = 25;
pub const VK_STRUCTURE_TYPE_PIPELINE_DYNAMIC_STATE_CREATE_INFO: VkStructureType = 27;
pub const VK_STRUCTURE_TYPE_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO: VkStructureType = 20;
pub const VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO: VkStructureType = 30;
pub const VK_STRUCTURE_TYPE_PIPELINE_MULTISAMPLE_STATE_CREATE_INFO: VkStructureType = 24;
pub const VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_LINE_STATE_CREATE_INFO: VkStructureType = 1000259001;
pub const VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_CREATE_INFO: VkStructureType = 23;
pub const VK_STRUCTURE_TYPE_PIPELINE_RENDERING_CREATE_INFO: VkStructureType = 1000044002;
pub const VK_STRUCTURE_TYPE_PIPELINE_ROBUSTNESS_CREATE_INFO: VkStructureType = 1000068000;
pub const VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO: VkStructureType = 18;
pub const VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_REQUIRED_SUBGROUP_SIZE_CREATE_INFO: VkStructureType = 1000225001;
pub const VK_STRUCTURE_TYPE_PIPELINE_TESSELLATION_DOMAIN_ORIGIN_STATE_CREATE_INFO: VkStructureType = 1000117003;
pub const VK_STRUCTURE_TYPE_PIPELINE_TESSELLATION_STATE_CREATE_INFO: VkStructureType = 21;
pub const VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_DIVISOR_STATE_CREATE_INFO: VkStructureType = 1000190001;
pub const VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO: VkStructureType = 19;
pub const VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_STATE_CREATE_INFO: VkStructureType = 22;
pub const VK_STRUCTURE_TYPE_PRIVATE_DATA_SLOT_CREATE_INFO: VkStructureType = 1000295002;
pub const VK_STRUCTURE_TYPE_PROTECTED_SUBMIT_INFO: VkStructureType = 1000145000;
pub const VK_STRUCTURE_TYPE_PUSH_CONSTANTS_INFO: VkStructureType = 1000545004;
pub const VK_STRUCTURE_TYPE_PUSH_DESCRIPTOR_SET_INFO: VkStructureType = 1000545005;
pub const VK_STRUCTURE_TYPE_PUSH_DESCRIPTOR_SET_WITH_TEMPLATE_INFO: VkStructureType = 1000545006;
pub const VK_STRUCTURE_TYPE_QUERY_POOL_CREATE_INFO: VkStructureType = 11;
pub const VK_STRUCTURE_TYPE_QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES: VkStructureType = 1000388001;
pub const VK_STRUCTURE_TYPE_QUEUE_FAMILY_PROPERTIES_2: VkStructureType = 1000059005;
pub const VK_STRUCTURE_TYPE_RENDERING_AREA_INFO: VkStructureType = 1000470003;
pub const VK_STRUCTURE_TYPE_RENDERING_ATTACHMENT_INFO: VkStructureType = 1000044001;
pub const VK_STRUCTURE_TYPE_RENDERING_ATTACHMENT_LOCATION_INFO: VkStructureType = 1000232001;
pub const VK_STRUCTURE_TYPE_RENDERING_INFO: VkStructureType = 1000044000;
pub const VK_STRUCTURE_TYPE_RENDERING_INPUT_ATTACHMENT_INDEX_INFO: VkStructureType = 1000232002;
pub const VK_STRUCTURE_TYPE_RENDER_PASS_ATTACHMENT_BEGIN_INFO: VkStructureType = 1000108003;
pub const VK_STRUCTURE_TYPE_RENDER_PASS_BEGIN_INFO: VkStructureType = 43;
pub const VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO: VkStructureType = 38;
pub const VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO_2: VkStructureType = 1000109004;
pub const VK_STRUCTURE_TYPE_RENDER_PASS_INPUT_ATTACHMENT_ASPECT_CREATE_INFO: VkStructureType = 1000117001;
pub const VK_STRUCTURE_TYPE_RENDER_PASS_MULTIVIEW_CREATE_INFO: VkStructureType = 1000053000;
pub const VK_STRUCTURE_TYPE_RESOLVE_IMAGE_INFO_2: VkStructureType = 1000337005;
pub const VK_STRUCTURE_TYPE_SAMPLER_CREATE_INFO: VkStructureType = 31;
pub const VK_STRUCTURE_TYPE_SAMPLER_REDUCTION_MODE_CREATE_INFO: VkStructureType = 1000130001;
pub const VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_CREATE_INFO: VkStructureType = 1000156000;
pub const VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_IMAGE_FORMAT_PROPERTIES: VkStructureType = 1000156005;
pub const VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_INFO: VkStructureType = 1000156001;
pub const VK_STRUCTURE_TYPE_SEMAPHORE_CREATE_INFO: VkStructureType = 9;
pub const VK_STRUCTURE_TYPE_SEMAPHORE_SIGNAL_INFO: VkStructureType = 1000207005;
pub const VK_STRUCTURE_TYPE_SEMAPHORE_SUBMIT_INFO: VkStructureType = 1000314005;
pub const VK_STRUCTURE_TYPE_SEMAPHORE_TYPE_CREATE_INFO: VkStructureType = 1000207002;
pub const VK_STRUCTURE_TYPE_SEMAPHORE_WAIT_INFO: VkStructureType = 1000207004;
pub const VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO: VkStructureType = 16;
pub const VK_STRUCTURE_TYPE_SPARSE_IMAGE_FORMAT_PROPERTIES_2: VkStructureType = 1000059007;
pub const VK_STRUCTURE_TYPE_SPARSE_IMAGE_MEMORY_REQUIREMENTS_2: VkStructureType = 1000146004;
pub const VK_STRUCTURE_TYPE_SUBMIT_INFO: VkStructureType = 4;
pub const VK_STRUCTURE_TYPE_SUBMIT_INFO_2: VkStructureType = 1000314004;
pub const VK_STRUCTURE_TYPE_SUBPASS_BEGIN_INFO: VkStructureType = 1000109005;
pub const VK_STRUCTURE_TYPE_SUBPASS_DEPENDENCY_2: VkStructureType = 1000109003;
pub const VK_STRUCTURE_TYPE_SUBPASS_DESCRIPTION_2: VkStructureType = 1000109002;
pub const VK_STRUCTURE_TYPE_SUBPASS_DESCRIPTION_DEPTH_STENCIL_RESOLVE: VkStructureType = 1000199001;
pub const VK_STRUCTURE_TYPE_SUBPASS_END_INFO: VkStructureType = 1000109006;
pub const VK_STRUCTURE_TYPE_SUBRESOURCE_HOST_MEMCPY_SIZE: VkStructureType = 1000270008;
pub const VK_STRUCTURE_TYPE_SUBRESOURCE_LAYOUT_2: VkStructureType = 1000338002;
pub const VK_STRUCTURE_TYPE_TIMELINE_SEMAPHORE_SUBMIT_INFO: VkStructureType = 1000207003;
pub const VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET: VkStructureType = 35;
pub const VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET_INLINE_UNIFORM_BLOCK: VkStructureType = 1000138002;
pub const VK_SUBGROUP_FEATURE_ARITHMETIC_BIT: VkSubgroupFeatureFlagBits = 1 << 2;
pub const VK_SUBGROUP_FEATURE_BALLOT_BIT: VkSubgroupFeatureFlagBits = 1 << 3;
pub const VK_SUBGROUP_FEATURE_BASIC_BIT: VkSubgroupFeatureFlagBits = 1 << 0;
pub const VK_SUBGROUP_FEATURE_CLUSTERED_BIT: VkSubgroupFeatureFlagBits = 1 << 6;
pub const VK_SUBGROUP_FEATURE_QUAD_BIT: VkSubgroupFeatureFlagBits = 1 << 7;
pub const VK_SUBGROUP_FEATURE_ROTATE_BIT: VkSubgroupFeatureFlagBits = 1 << 9;
pub const VK_SUBGROUP_FEATURE_ROTATE_CLUSTERED_BIT: VkSubgroupFeatureFlagBits = 1 << 10;
pub const VK_SUBGROUP_FEATURE_SHUFFLE_BIT: VkSubgroupFeatureFlagBits = 1 << 4;
pub const VK_SUBGROUP_FEATURE_SHUFFLE_RELATIVE_BIT: VkSubgroupFeatureFlagBits = 1 << 5;
pub const VK_SUBGROUP_FEATURE_VOTE_BIT: VkSubgroupFeatureFlagBits = 1 << 1;
pub const VK_SUBMIT_PROTECTED_BIT: VkSubmitFlagBits = 1 << 0;
pub const VK_SUBPASS_CONTENTS_INLINE: VkSubpassContents = 0;
pub const VK_SUBPASS_CONTENTS_SECONDARY_COMMAND_BUFFERS: VkSubpassContents = 1;
pub const VK_SUBPASS_EXTERNAL: u32 = !0;
pub const VK_SUCCESS: VkResult = 0;
pub const VK_SYSTEM_ALLOCATION_SCOPE_CACHE: VkSystemAllocationScope = 2;
pub const VK_SYSTEM_ALLOCATION_SCOPE_COMMAND: VkSystemAllocationScope = 0;
pub const VK_SYSTEM_ALLOCATION_SCOPE_DEVICE: VkSystemAllocationScope = 3;
pub const VK_SYSTEM_ALLOCATION_SCOPE_INSTANCE: VkSystemAllocationScope = 4;
pub const VK_SYSTEM_ALLOCATION_SCOPE_OBJECT: VkSystemAllocationScope = 1;
pub const VK_TESSELLATION_DOMAIN_ORIGIN_LOWER_LEFT: VkTessellationDomainOrigin = 1;
pub const VK_TESSELLATION_DOMAIN_ORIGIN_UPPER_LEFT: VkTessellationDomainOrigin = 0;
pub const VK_TIMEOUT: VkResult = 2;
pub const VK_TOOL_PURPOSE_ADDITIONAL_FEATURES_BIT: VkToolPurposeFlagBits = 1 << 3;
pub const VK_TOOL_PURPOSE_MODIFYING_FEATURES_BIT: VkToolPurposeFlagBits = 1 << 4;
pub const VK_TOOL_PURPOSE_PROFILING_BIT: VkToolPurposeFlagBits = 1 << 1;
pub const VK_TOOL_PURPOSE_TRACING_BIT: VkToolPurposeFlagBits = 1 << 2;
pub const VK_TOOL_PURPOSE_VALIDATION_BIT: VkToolPurposeFlagBits = 1 << 0;
pub const VK_TRUE: u32 = 1;
pub const VK_UUID_SIZE: u32 = 16;
pub const VK_VENDOR_ID_CODEPLAY: VkVendorId = 0x10004;
pub const VK_VENDOR_ID_KAZAN: VkVendorId = 0x10003;
pub const VK_VENDOR_ID_KHRONOS: VkVendorId = 0x10000;
pub const VK_VENDOR_ID_MESA: VkVendorId = 0x10005;
pub const VK_VENDOR_ID_MOBILEYE: VkVendorId = 0x10007;
pub const VK_VENDOR_ID_POCL: VkVendorId = 0x10006;
pub const VK_VENDOR_ID_VIV: VkVendorId = 0x10001;
pub const VK_VENDOR_ID_VSI: VkVendorId = 0x10002;
pub const VK_VERTEX_INPUT_RATE_INSTANCE: VkVertexInputRate = 1;
pub const VK_VERTEX_INPUT_RATE_VERTEX: VkVertexInputRate = 0;
pub const VK_WHOLE_SIZE: u64 = !0;

unsafe extern "system" {
    #[cfg(feature = "exported_prototypes")]
    pub fn vkAllocateCommandBuffers(device: VkDevice, pAllocateInfo: *const VkCommandBufferAllocateInfo, pCommandBuffers: *mut VkCommandBuffer) -> VkResult;
    #[cfg(feature = "exported_prototypes")]
    pub fn vkAllocateDescriptorSets(device: VkDevice, pAllocateInfo: *const VkDescriptorSetAllocateInfo, pDescriptorSets: *mut VkDescriptorSet) -> VkResult;
    #[cfg(feature = "exported_prototypes")]
    pub fn vkAllocateMemory(device: VkDevice, pAllocateInfo: *const VkMemoryAllocateInfo, pAllocator: *const VkAllocationCallbacks, pMemory: *mut VkDeviceMemory) -> VkResult;
    #[cfg(feature = "exported_prototypes")]
    pub fn vkBeginCommandBuffer(commandBuffer: VkCommandBuffer, pBeginInfo: *const VkCommandBufferBeginInfo) -> VkResult;
    #[cfg(feature = "exported_prototypes")]
    pub fn vkBindBufferMemory(device: VkDevice, buffer: VkBuffer, memory: VkDeviceMemory, memoryOffset: VkDeviceSize) -> VkResult;
    #[cfg(feature = "exported_prototypes")]
    pub fn vkBindBufferMemory2(device: VkDevice, bindInfoCount: u32, pBindInfos: *const VkBindBufferMemoryInfo) -> VkResult;
    #[cfg(feature = "exported_prototypes")]
    pub fn vkBindImageMemory(device: VkDevice, image: VkImage, memory: VkDeviceMemory, memoryOffset: VkDeviceSize) -> VkResult;
    #[cfg(feature = "exported_prototypes")]
    pub fn vkBindImageMemory2(device: VkDevice, bindInfoCount: u32, pBindInfos: *const VkBindImageMemoryInfo) -> VkResult;
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCmdBeginQuery(commandBuffer: VkCommandBuffer, queryPool: VkQueryPool, query: u32, flags: VkQueryControlFlags);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCmdBeginRenderPass(commandBuffer: VkCommandBuffer, pRenderPassBegin: *const VkRenderPassBeginInfo, contents: VkSubpassContents);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCmdBeginRenderPass2(commandBuffer: VkCommandBuffer, pRenderPassBegin: *const VkRenderPassBeginInfo, pSubpassBeginInfo: *const VkSubpassBeginInfo);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCmdBeginRendering(commandBuffer: VkCommandBuffer, pRenderingInfo: *const VkRenderingInfo);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCmdBindDescriptorSets(commandBuffer: VkCommandBuffer, pipelineBindPoint: VkPipelineBindPoint, layout: VkPipelineLayout, firstSet: u32, descriptorSetCount: u32, pDescriptorSets: *const VkDescriptorSet, dynamicOffsetCount: u32, pDynamicOffsets: *const u32);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCmdBindDescriptorSets2(commandBuffer: VkCommandBuffer, pBindDescriptorSetsInfo: *const VkBindDescriptorSetsInfo);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCmdBindIndexBuffer(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize, indexType: VkIndexType);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCmdBindIndexBuffer2(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize, size: VkDeviceSize, indexType: VkIndexType);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCmdBindPipeline(commandBuffer: VkCommandBuffer, pipelineBindPoint: VkPipelineBindPoint, pipeline: VkPipeline);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCmdBindVertexBuffers(commandBuffer: VkCommandBuffer, firstBinding: u32, bindingCount: u32, pBuffers: *const VkBuffer, pOffsets: *const VkDeviceSize);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCmdBindVertexBuffers2(commandBuffer: VkCommandBuffer, firstBinding: u32, bindingCount: u32, pBuffers: *const VkBuffer, pOffsets: *const VkDeviceSize, pSizes: *const VkDeviceSize, pStrides: *const VkDeviceSize);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCmdBlitImage(commandBuffer: VkCommandBuffer, srcImage: VkImage, srcImageLayout: VkImageLayout, dstImage: VkImage, dstImageLayout: VkImageLayout, regionCount: u32, pRegions: *const VkImageBlit, filter: VkFilter);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCmdBlitImage2(commandBuffer: VkCommandBuffer, pBlitImageInfo: *const VkBlitImageInfo2);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCmdClearAttachments(commandBuffer: VkCommandBuffer, attachmentCount: u32, pAttachments: *const VkClearAttachment, rectCount: u32, pRects: *const VkClearRect);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCmdClearColorImage(commandBuffer: VkCommandBuffer, image: VkImage, imageLayout: VkImageLayout, pColor: *const VkClearColorValue, rangeCount: u32, pRanges: *const VkImageSubresourceRange);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCmdClearDepthStencilImage(commandBuffer: VkCommandBuffer, image: VkImage, imageLayout: VkImageLayout, pDepthStencil: *const VkClearDepthStencilValue, rangeCount: u32, pRanges: *const VkImageSubresourceRange);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCmdCopyBuffer(commandBuffer: VkCommandBuffer, srcBuffer: VkBuffer, dstBuffer: VkBuffer, regionCount: u32, pRegions: *const VkBufferCopy);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCmdCopyBuffer2(commandBuffer: VkCommandBuffer, pCopyBufferInfo: *const VkCopyBufferInfo2);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCmdCopyBufferToImage(commandBuffer: VkCommandBuffer, srcBuffer: VkBuffer, dstImage: VkImage, dstImageLayout: VkImageLayout, regionCount: u32, pRegions: *const VkBufferImageCopy);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCmdCopyBufferToImage2(commandBuffer: VkCommandBuffer, pCopyBufferToImageInfo: *const VkCopyBufferToImageInfo2);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCmdCopyImage(commandBuffer: VkCommandBuffer, srcImage: VkImage, srcImageLayout: VkImageLayout, dstImage: VkImage, dstImageLayout: VkImageLayout, regionCount: u32, pRegions: *const VkImageCopy);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCmdCopyImage2(commandBuffer: VkCommandBuffer, pCopyImageInfo: *const VkCopyImageInfo2);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCmdCopyImageToBuffer(commandBuffer: VkCommandBuffer, srcImage: VkImage, srcImageLayout: VkImageLayout, dstBuffer: VkBuffer, regionCount: u32, pRegions: *const VkBufferImageCopy);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCmdCopyImageToBuffer2(commandBuffer: VkCommandBuffer, pCopyImageToBufferInfo: *const VkCopyImageToBufferInfo2);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCmdCopyQueryPoolResults(commandBuffer: VkCommandBuffer, queryPool: VkQueryPool, firstQuery: u32, queryCount: u32, dstBuffer: VkBuffer, dstOffset: VkDeviceSize, stride: VkDeviceSize, flags: VkQueryResultFlags);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCmdDispatch(commandBuffer: VkCommandBuffer, groupCountX: u32, groupCountY: u32, groupCountZ: u32);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCmdDispatchBase(commandBuffer: VkCommandBuffer, baseGroupX: u32, baseGroupY: u32, baseGroupZ: u32, groupCountX: u32, groupCountY: u32, groupCountZ: u32);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCmdDispatchIndirect(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCmdDraw(commandBuffer: VkCommandBuffer, vertexCount: u32, instanceCount: u32, firstVertex: u32, firstInstance: u32);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCmdDrawIndexed(commandBuffer: VkCommandBuffer, indexCount: u32, instanceCount: u32, firstIndex: u32, vertexOffset: i32, firstInstance: u32);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCmdDrawIndexedIndirect(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize, drawCount: u32, stride: u32);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCmdDrawIndexedIndirectCount(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize, countBuffer: VkBuffer, countBufferOffset: VkDeviceSize, maxDrawCount: u32, stride: u32);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCmdDrawIndirect(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize, drawCount: u32, stride: u32);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCmdDrawIndirectCount(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize, countBuffer: VkBuffer, countBufferOffset: VkDeviceSize, maxDrawCount: u32, stride: u32);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCmdEndQuery(commandBuffer: VkCommandBuffer, queryPool: VkQueryPool, query: u32);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCmdEndRenderPass(commandBuffer: VkCommandBuffer);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCmdEndRenderPass2(commandBuffer: VkCommandBuffer, pSubpassEndInfo: *const VkSubpassEndInfo);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCmdEndRendering(commandBuffer: VkCommandBuffer);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCmdExecuteCommands(commandBuffer: VkCommandBuffer, commandBufferCount: u32, pCommandBuffers: *const VkCommandBuffer);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCmdFillBuffer(commandBuffer: VkCommandBuffer, dstBuffer: VkBuffer, dstOffset: VkDeviceSize, size: VkDeviceSize, data: u32);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCmdNextSubpass(commandBuffer: VkCommandBuffer, contents: VkSubpassContents);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCmdNextSubpass2(commandBuffer: VkCommandBuffer, pSubpassBeginInfo: *const VkSubpassBeginInfo, pSubpassEndInfo: *const VkSubpassEndInfo);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCmdPipelineBarrier(commandBuffer: VkCommandBuffer, srcStageMask: VkPipelineStageFlags, dstStageMask: VkPipelineStageFlags, dependencyFlags: VkDependencyFlags, memoryBarrierCount: u32, pMemoryBarriers: *const VkMemoryBarrier, bufferMemoryBarrierCount: u32, pBufferMemoryBarriers: *const VkBufferMemoryBarrier, imageMemoryBarrierCount: u32, pImageMemoryBarriers: *const VkImageMemoryBarrier);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCmdPipelineBarrier2(commandBuffer: VkCommandBuffer, pDependencyInfo: *const VkDependencyInfo);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCmdPushConstants(commandBuffer: VkCommandBuffer, layout: VkPipelineLayout, stageFlags: VkShaderStageFlags, offset: u32, size: u32, pValues: *const c_void);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCmdPushConstants2(commandBuffer: VkCommandBuffer, pPushConstantsInfo: *const VkPushConstantsInfo);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCmdPushDescriptorSet(commandBuffer: VkCommandBuffer, pipelineBindPoint: VkPipelineBindPoint, layout: VkPipelineLayout, set: u32, descriptorWriteCount: u32, pDescriptorWrites: *const VkWriteDescriptorSet);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCmdPushDescriptorSet2(commandBuffer: VkCommandBuffer, pPushDescriptorSetInfo: *const VkPushDescriptorSetInfo);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCmdPushDescriptorSetWithTemplate(commandBuffer: VkCommandBuffer, descriptorUpdateTemplate: VkDescriptorUpdateTemplate, layout: VkPipelineLayout, set: u32, pData: *const c_void);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCmdPushDescriptorSetWithTemplate2(commandBuffer: VkCommandBuffer, pPushDescriptorSetWithTemplateInfo: *const VkPushDescriptorSetWithTemplateInfo);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCmdResetEvent(commandBuffer: VkCommandBuffer, event: VkEvent, stageMask: VkPipelineStageFlags);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCmdResetEvent2(commandBuffer: VkCommandBuffer, event: VkEvent, stageMask: VkPipelineStageFlags2);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCmdResetQueryPool(commandBuffer: VkCommandBuffer, queryPool: VkQueryPool, firstQuery: u32, queryCount: u32);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCmdResolveImage(commandBuffer: VkCommandBuffer, srcImage: VkImage, srcImageLayout: VkImageLayout, dstImage: VkImage, dstImageLayout: VkImageLayout, regionCount: u32, pRegions: *const VkImageResolve);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCmdResolveImage2(commandBuffer: VkCommandBuffer, pResolveImageInfo: *const VkResolveImageInfo2);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCmdSetBlendConstants(commandBuffer: VkCommandBuffer, blendConstants: *const c_float);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCmdSetCullMode(commandBuffer: VkCommandBuffer, cullMode: VkCullModeFlags);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCmdSetDepthBias(commandBuffer: VkCommandBuffer, depthBiasConstantFactor: c_float, depthBiasClamp: c_float, depthBiasSlopeFactor: c_float);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCmdSetDepthBiasEnable(commandBuffer: VkCommandBuffer, depthBiasEnable: VkBool32);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCmdSetDepthBounds(commandBuffer: VkCommandBuffer, minDepthBounds: c_float, maxDepthBounds: c_float);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCmdSetDepthBoundsTestEnable(commandBuffer: VkCommandBuffer, depthBoundsTestEnable: VkBool32);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCmdSetDepthCompareOp(commandBuffer: VkCommandBuffer, depthCompareOp: VkCompareOp);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCmdSetDepthTestEnable(commandBuffer: VkCommandBuffer, depthTestEnable: VkBool32);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCmdSetDepthWriteEnable(commandBuffer: VkCommandBuffer, depthWriteEnable: VkBool32);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCmdSetDeviceMask(commandBuffer: VkCommandBuffer, deviceMask: u32);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCmdSetEvent(commandBuffer: VkCommandBuffer, event: VkEvent, stageMask: VkPipelineStageFlags);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCmdSetEvent2(commandBuffer: VkCommandBuffer, event: VkEvent, pDependencyInfo: *const VkDependencyInfo);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCmdSetFrontFace(commandBuffer: VkCommandBuffer, frontFace: VkFrontFace);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCmdSetLineStipple(commandBuffer: VkCommandBuffer, lineStippleFactor: u32, lineStipplePattern: u16);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCmdSetLineWidth(commandBuffer: VkCommandBuffer, lineWidth: c_float);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCmdSetPrimitiveRestartEnable(commandBuffer: VkCommandBuffer, primitiveRestartEnable: VkBool32);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCmdSetPrimitiveTopology(commandBuffer: VkCommandBuffer, primitiveTopology: VkPrimitiveTopology);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCmdSetRasterizerDiscardEnable(commandBuffer: VkCommandBuffer, rasterizerDiscardEnable: VkBool32);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCmdSetRenderingAttachmentLocations(commandBuffer: VkCommandBuffer, pLocationInfo: *const VkRenderingAttachmentLocationInfo);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCmdSetRenderingInputAttachmentIndices(commandBuffer: VkCommandBuffer, pInputAttachmentIndexInfo: *const VkRenderingInputAttachmentIndexInfo);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCmdSetScissor(commandBuffer: VkCommandBuffer, firstScissor: u32, scissorCount: u32, pScissors: *const VkRect2D);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCmdSetScissorWithCount(commandBuffer: VkCommandBuffer, scissorCount: u32, pScissors: *const VkRect2D);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCmdSetStencilCompareMask(commandBuffer: VkCommandBuffer, faceMask: VkStencilFaceFlags, compareMask: u32);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCmdSetStencilOp(commandBuffer: VkCommandBuffer, faceMask: VkStencilFaceFlags, failOp: VkStencilOp, passOp: VkStencilOp, depthFailOp: VkStencilOp, compareOp: VkCompareOp);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCmdSetStencilReference(commandBuffer: VkCommandBuffer, faceMask: VkStencilFaceFlags, reference: u32);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCmdSetStencilTestEnable(commandBuffer: VkCommandBuffer, stencilTestEnable: VkBool32);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCmdSetStencilWriteMask(commandBuffer: VkCommandBuffer, faceMask: VkStencilFaceFlags, writeMask: u32);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCmdSetViewport(commandBuffer: VkCommandBuffer, firstViewport: u32, viewportCount: u32, pViewports: *const VkViewport);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCmdSetViewportWithCount(commandBuffer: VkCommandBuffer, viewportCount: u32, pViewports: *const VkViewport);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCmdUpdateBuffer(commandBuffer: VkCommandBuffer, dstBuffer: VkBuffer, dstOffset: VkDeviceSize, dataSize: VkDeviceSize, pData: *const c_void);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCmdWaitEvents(commandBuffer: VkCommandBuffer, eventCount: u32, pEvents: *const VkEvent, srcStageMask: VkPipelineStageFlags, dstStageMask: VkPipelineStageFlags, memoryBarrierCount: u32, pMemoryBarriers: *const VkMemoryBarrier, bufferMemoryBarrierCount: u32, pBufferMemoryBarriers: *const VkBufferMemoryBarrier, imageMemoryBarrierCount: u32, pImageMemoryBarriers: *const VkImageMemoryBarrier);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCmdWaitEvents2(commandBuffer: VkCommandBuffer, eventCount: u32, pEvents: *const VkEvent, pDependencyInfos: *const VkDependencyInfo);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCmdWriteTimestamp(commandBuffer: VkCommandBuffer, pipelineStage: VkPipelineStageFlagBits, queryPool: VkQueryPool, query: u32);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCmdWriteTimestamp2(commandBuffer: VkCommandBuffer, stage: VkPipelineStageFlags2, queryPool: VkQueryPool, query: u32);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCopyImageToImage(device: VkDevice, pCopyImageToImageInfo: *const VkCopyImageToImageInfo) -> VkResult;
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCopyImageToMemory(device: VkDevice, pCopyImageToMemoryInfo: *const VkCopyImageToMemoryInfo) -> VkResult;
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCopyMemoryToImage(device: VkDevice, pCopyMemoryToImageInfo: *const VkCopyMemoryToImageInfo) -> VkResult;
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCreateBuffer(device: VkDevice, pCreateInfo: *const VkBufferCreateInfo, pAllocator: *const VkAllocationCallbacks, pBuffer: *mut VkBuffer) -> VkResult;
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCreateBufferView(device: VkDevice, pCreateInfo: *const VkBufferViewCreateInfo, pAllocator: *const VkAllocationCallbacks, pView: *mut VkBufferView) -> VkResult;
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCreateCommandPool(device: VkDevice, pCreateInfo: *const VkCommandPoolCreateInfo, pAllocator: *const VkAllocationCallbacks, pCommandPool: *mut VkCommandPool) -> VkResult;
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCreateComputePipelines(device: VkDevice, pipelineCache: VkPipelineCache, createInfoCount: u32, pCreateInfos: *const VkComputePipelineCreateInfo, pAllocator: *const VkAllocationCallbacks, pPipelines: *mut VkPipeline) -> VkResult;
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCreateDescriptorPool(device: VkDevice, pCreateInfo: *const VkDescriptorPoolCreateInfo, pAllocator: *const VkAllocationCallbacks, pDescriptorPool: *mut VkDescriptorPool) -> VkResult;
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCreateDescriptorSetLayout(device: VkDevice, pCreateInfo: *const VkDescriptorSetLayoutCreateInfo, pAllocator: *const VkAllocationCallbacks, pSetLayout: *mut VkDescriptorSetLayout) -> VkResult;
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCreateDescriptorUpdateTemplate(device: VkDevice, pCreateInfo: *const VkDescriptorUpdateTemplateCreateInfo, pAllocator: *const VkAllocationCallbacks, pDescriptorUpdateTemplate: *mut VkDescriptorUpdateTemplate) -> VkResult;
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCreateDevice(physicalDevice: VkPhysicalDevice, pCreateInfo: *const VkDeviceCreateInfo, pAllocator: *const VkAllocationCallbacks, pDevice: *mut VkDevice) -> VkResult;
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCreateEvent(device: VkDevice, pCreateInfo: *const VkEventCreateInfo, pAllocator: *const VkAllocationCallbacks, pEvent: *mut VkEvent) -> VkResult;
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCreateFence(device: VkDevice, pCreateInfo: *const VkFenceCreateInfo, pAllocator: *const VkAllocationCallbacks, pFence: *mut VkFence) -> VkResult;
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCreateFramebuffer(device: VkDevice, pCreateInfo: *const VkFramebufferCreateInfo, pAllocator: *const VkAllocationCallbacks, pFramebuffer: *mut VkFramebuffer) -> VkResult;
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCreateGraphicsPipelines(device: VkDevice, pipelineCache: VkPipelineCache, createInfoCount: u32, pCreateInfos: *const VkGraphicsPipelineCreateInfo, pAllocator: *const VkAllocationCallbacks, pPipelines: *mut VkPipeline) -> VkResult;
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCreateImage(device: VkDevice, pCreateInfo: *const VkImageCreateInfo, pAllocator: *const VkAllocationCallbacks, pImage: *mut VkImage) -> VkResult;
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCreateImageView(device: VkDevice, pCreateInfo: *const VkImageViewCreateInfo, pAllocator: *const VkAllocationCallbacks, pView: *mut VkImageView) -> VkResult;
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCreateInstance(pCreateInfo: *const VkInstanceCreateInfo, pAllocator: *const VkAllocationCallbacks, pInstance: *mut VkInstance) -> VkResult;
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCreatePipelineCache(device: VkDevice, pCreateInfo: *const VkPipelineCacheCreateInfo, pAllocator: *const VkAllocationCallbacks, pPipelineCache: *mut VkPipelineCache) -> VkResult;
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCreatePipelineLayout(device: VkDevice, pCreateInfo: *const VkPipelineLayoutCreateInfo, pAllocator: *const VkAllocationCallbacks, pPipelineLayout: *mut VkPipelineLayout) -> VkResult;
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCreatePrivateDataSlot(device: VkDevice, pCreateInfo: *const VkPrivateDataSlotCreateInfo, pAllocator: *const VkAllocationCallbacks, pPrivateDataSlot: *mut VkPrivateDataSlot) -> VkResult;
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCreateQueryPool(device: VkDevice, pCreateInfo: *const VkQueryPoolCreateInfo, pAllocator: *const VkAllocationCallbacks, pQueryPool: *mut VkQueryPool) -> VkResult;
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCreateRenderPass(device: VkDevice, pCreateInfo: *const VkRenderPassCreateInfo, pAllocator: *const VkAllocationCallbacks, pRenderPass: *mut VkRenderPass) -> VkResult;
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCreateRenderPass2(device: VkDevice, pCreateInfo: *const VkRenderPassCreateInfo2, pAllocator: *const VkAllocationCallbacks, pRenderPass: *mut VkRenderPass) -> VkResult;
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCreateSampler(device: VkDevice, pCreateInfo: *const VkSamplerCreateInfo, pAllocator: *const VkAllocationCallbacks, pSampler: *mut VkSampler) -> VkResult;
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCreateSamplerYcbcrConversion(device: VkDevice, pCreateInfo: *const VkSamplerYcbcrConversionCreateInfo, pAllocator: *const VkAllocationCallbacks, pYcbcrConversion: *mut VkSamplerYcbcrConversion) -> VkResult;
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCreateSemaphore(device: VkDevice, pCreateInfo: *const VkSemaphoreCreateInfo, pAllocator: *const VkAllocationCallbacks, pSemaphore: *mut VkSemaphore) -> VkResult;
    #[cfg(feature = "exported_prototypes")]
    pub fn vkCreateShaderModule(device: VkDevice, pCreateInfo: *const VkShaderModuleCreateInfo, pAllocator: *const VkAllocationCallbacks, pShaderModule: *mut VkShaderModule) -> VkResult;
    #[cfg(feature = "exported_prototypes")]
    pub fn vkDestroyBuffer(device: VkDevice, buffer: VkBuffer, pAllocator: *const VkAllocationCallbacks);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkDestroyBufferView(device: VkDevice, bufferView: VkBufferView, pAllocator: *const VkAllocationCallbacks);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkDestroyCommandPool(device: VkDevice, commandPool: VkCommandPool, pAllocator: *const VkAllocationCallbacks);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkDestroyDescriptorPool(device: VkDevice, descriptorPool: VkDescriptorPool, pAllocator: *const VkAllocationCallbacks);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkDestroyDescriptorSetLayout(device: VkDevice, descriptorSetLayout: VkDescriptorSetLayout, pAllocator: *const VkAllocationCallbacks);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkDestroyDescriptorUpdateTemplate(device: VkDevice, descriptorUpdateTemplate: VkDescriptorUpdateTemplate, pAllocator: *const VkAllocationCallbacks);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkDestroyDevice(device: VkDevice, pAllocator: *const VkAllocationCallbacks);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkDestroyEvent(device: VkDevice, event: VkEvent, pAllocator: *const VkAllocationCallbacks);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkDestroyFence(device: VkDevice, fence: VkFence, pAllocator: *const VkAllocationCallbacks);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkDestroyFramebuffer(device: VkDevice, framebuffer: VkFramebuffer, pAllocator: *const VkAllocationCallbacks);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkDestroyImage(device: VkDevice, image: VkImage, pAllocator: *const VkAllocationCallbacks);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkDestroyImageView(device: VkDevice, imageView: VkImageView, pAllocator: *const VkAllocationCallbacks);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkDestroyInstance(instance: VkInstance, pAllocator: *const VkAllocationCallbacks);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkDestroyPipeline(device: VkDevice, pipeline: VkPipeline, pAllocator: *const VkAllocationCallbacks);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkDestroyPipelineCache(device: VkDevice, pipelineCache: VkPipelineCache, pAllocator: *const VkAllocationCallbacks);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkDestroyPipelineLayout(device: VkDevice, pipelineLayout: VkPipelineLayout, pAllocator: *const VkAllocationCallbacks);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkDestroyPrivateDataSlot(device: VkDevice, privateDataSlot: VkPrivateDataSlot, pAllocator: *const VkAllocationCallbacks);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkDestroyQueryPool(device: VkDevice, queryPool: VkQueryPool, pAllocator: *const VkAllocationCallbacks);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkDestroyRenderPass(device: VkDevice, renderPass: VkRenderPass, pAllocator: *const VkAllocationCallbacks);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkDestroySampler(device: VkDevice, sampler: VkSampler, pAllocator: *const VkAllocationCallbacks);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkDestroySamplerYcbcrConversion(device: VkDevice, ycbcrConversion: VkSamplerYcbcrConversion, pAllocator: *const VkAllocationCallbacks);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkDestroySemaphore(device: VkDevice, semaphore: VkSemaphore, pAllocator: *const VkAllocationCallbacks);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkDestroyShaderModule(device: VkDevice, shaderModule: VkShaderModule, pAllocator: *const VkAllocationCallbacks);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkDeviceWaitIdle(device: VkDevice) -> VkResult;
    #[cfg(feature = "exported_prototypes")]
    pub fn vkEndCommandBuffer(commandBuffer: VkCommandBuffer) -> VkResult;
    #[cfg(feature = "exported_prototypes")]
    pub fn vkEnumerateDeviceExtensionProperties(physicalDevice: VkPhysicalDevice, pLayerName: *const c_char, pPropertyCount: *mut u32, pProperties: *mut VkExtensionProperties) -> VkResult;
    #[cfg(feature = "exported_prototypes")]
    pub fn vkEnumerateDeviceLayerProperties(physicalDevice: VkPhysicalDevice, pPropertyCount: *mut u32, pProperties: *mut VkLayerProperties) -> VkResult;
    #[cfg(feature = "exported_prototypes")]
    pub fn vkEnumerateInstanceExtensionProperties(pLayerName: *const c_char, pPropertyCount: *mut u32, pProperties: *mut VkExtensionProperties) -> VkResult;
    #[cfg(feature = "exported_prototypes")]
    pub fn vkEnumerateInstanceLayerProperties(pPropertyCount: *mut u32, pProperties: *mut VkLayerProperties) -> VkResult;
    #[cfg(feature = "exported_prototypes")]
    pub fn vkEnumerateInstanceVersion(pApiVersion: *mut u32) -> VkResult;
    #[cfg(feature = "exported_prototypes")]
    pub fn vkEnumeratePhysicalDeviceGroups(instance: VkInstance, pPhysicalDeviceGroupCount: *mut u32, pPhysicalDeviceGroupProperties: *mut VkPhysicalDeviceGroupProperties) -> VkResult;
    #[cfg(feature = "exported_prototypes")]
    pub fn vkEnumeratePhysicalDevices(instance: VkInstance, pPhysicalDeviceCount: *mut u32, pPhysicalDevices: *mut VkPhysicalDevice) -> VkResult;
    #[cfg(feature = "exported_prototypes")]
    pub fn vkFlushMappedMemoryRanges(device: VkDevice, memoryRangeCount: u32, pMemoryRanges: *const VkMappedMemoryRange) -> VkResult;
    #[cfg(feature = "exported_prototypes")]
    pub fn vkFreeCommandBuffers(device: VkDevice, commandPool: VkCommandPool, commandBufferCount: u32, pCommandBuffers: *const VkCommandBuffer);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkFreeDescriptorSets(device: VkDevice, descriptorPool: VkDescriptorPool, descriptorSetCount: u32, pDescriptorSets: *const VkDescriptorSet) -> VkResult;
    #[cfg(feature = "exported_prototypes")]
    pub fn vkFreeMemory(device: VkDevice, memory: VkDeviceMemory, pAllocator: *const VkAllocationCallbacks);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkGetBufferDeviceAddress(device: VkDevice, pInfo: *const VkBufferDeviceAddressInfo) -> VkDeviceAddress;
    #[cfg(feature = "exported_prototypes")]
    pub fn vkGetBufferMemoryRequirements(device: VkDevice, buffer: VkBuffer, pMemoryRequirements: *mut VkMemoryRequirements);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkGetBufferMemoryRequirements2(device: VkDevice, pInfo: *const VkBufferMemoryRequirementsInfo2, pMemoryRequirements: *mut VkMemoryRequirements2);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkGetBufferOpaqueCaptureAddress(device: VkDevice, pInfo: *const VkBufferDeviceAddressInfo) -> u64;
    #[cfg(feature = "exported_prototypes")]
    pub fn vkGetDescriptorSetLayoutSupport(device: VkDevice, pCreateInfo: *const VkDescriptorSetLayoutCreateInfo, pSupport: *mut VkDescriptorSetLayoutSupport);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkGetDeviceBufferMemoryRequirements(device: VkDevice, pInfo: *const VkDeviceBufferMemoryRequirements, pMemoryRequirements: *mut VkMemoryRequirements2);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkGetDeviceGroupPeerMemoryFeatures(device: VkDevice, heapIndex: u32, localDeviceIndex: u32, remoteDeviceIndex: u32, pPeerMemoryFeatures: *mut VkPeerMemoryFeatureFlags);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkGetDeviceImageMemoryRequirements(device: VkDevice, pInfo: *const VkDeviceImageMemoryRequirements, pMemoryRequirements: *mut VkMemoryRequirements2);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkGetDeviceImageSparseMemoryRequirements(device: VkDevice, pInfo: *const VkDeviceImageMemoryRequirements, pSparseMemoryRequirementCount: *mut u32, pSparseMemoryRequirements: *mut VkSparseImageMemoryRequirements2);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkGetDeviceImageSubresourceLayout(device: VkDevice, pInfo: *const VkDeviceImageSubresourceInfo, pLayout: *mut VkSubresourceLayout2);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkGetDeviceMemoryCommitment(device: VkDevice, memory: VkDeviceMemory, pCommittedMemoryInBytes: *mut VkDeviceSize);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkGetDeviceMemoryOpaqueCaptureAddress(device: VkDevice, pInfo: *const VkDeviceMemoryOpaqueCaptureAddressInfo) -> u64;
    #[cfg(feature = "exported_prototypes")]
    pub fn vkGetDeviceProcAddr(device: VkDevice, pName: *const c_char) -> PFN_vkVoidFunction;
    #[cfg(feature = "exported_prototypes")]
    pub fn vkGetDeviceQueue(device: VkDevice, queueFamilyIndex: u32, queueIndex: u32, pQueue: *mut VkQueue);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkGetDeviceQueue2(device: VkDevice, pQueueInfo: *const VkDeviceQueueInfo2, pQueue: *mut VkQueue);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkGetEventStatus(device: VkDevice, event: VkEvent) -> VkResult;
    #[cfg(feature = "exported_prototypes")]
    pub fn vkGetFenceStatus(device: VkDevice, fence: VkFence) -> VkResult;
    #[cfg(feature = "exported_prototypes")]
    pub fn vkGetImageMemoryRequirements(device: VkDevice, image: VkImage, pMemoryRequirements: *mut VkMemoryRequirements);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkGetImageMemoryRequirements2(device: VkDevice, pInfo: *const VkImageMemoryRequirementsInfo2, pMemoryRequirements: *mut VkMemoryRequirements2);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkGetImageSparseMemoryRequirements(device: VkDevice, image: VkImage, pSparseMemoryRequirementCount: *mut u32, pSparseMemoryRequirements: *mut VkSparseImageMemoryRequirements);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkGetImageSparseMemoryRequirements2(device: VkDevice, pInfo: *const VkImageSparseMemoryRequirementsInfo2, pSparseMemoryRequirementCount: *mut u32, pSparseMemoryRequirements: *mut VkSparseImageMemoryRequirements2);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkGetImageSubresourceLayout(device: VkDevice, image: VkImage, pSubresource: *const VkImageSubresource, pLayout: *mut VkSubresourceLayout);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkGetImageSubresourceLayout2(device: VkDevice, image: VkImage, pSubresource: *const VkImageSubresource2, pLayout: *mut VkSubresourceLayout2);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkGetInstanceProcAddr(instance: VkInstance, pName: *const c_char) -> PFN_vkVoidFunction;
    #[cfg(feature = "exported_prototypes")]
    pub fn vkGetPhysicalDeviceExternalBufferProperties(physicalDevice: VkPhysicalDevice, pExternalBufferInfo: *const VkPhysicalDeviceExternalBufferInfo, pExternalBufferProperties: *mut VkExternalBufferProperties);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkGetPhysicalDeviceExternalFenceProperties(physicalDevice: VkPhysicalDevice, pExternalFenceInfo: *const VkPhysicalDeviceExternalFenceInfo, pExternalFenceProperties: *mut VkExternalFenceProperties);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkGetPhysicalDeviceExternalSemaphoreProperties(physicalDevice: VkPhysicalDevice, pExternalSemaphoreInfo: *const VkPhysicalDeviceExternalSemaphoreInfo, pExternalSemaphoreProperties: *mut VkExternalSemaphoreProperties);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkGetPhysicalDeviceFeatures(physicalDevice: VkPhysicalDevice, pFeatures: *mut VkPhysicalDeviceFeatures);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkGetPhysicalDeviceFeatures2(physicalDevice: VkPhysicalDevice, pFeatures: *mut VkPhysicalDeviceFeatures2);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkGetPhysicalDeviceFormatProperties(physicalDevice: VkPhysicalDevice, format: VkFormat, pFormatProperties: *mut VkFormatProperties);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkGetPhysicalDeviceFormatProperties2(physicalDevice: VkPhysicalDevice, format: VkFormat, pFormatProperties: *mut VkFormatProperties2);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkGetPhysicalDeviceImageFormatProperties(physicalDevice: VkPhysicalDevice, format: VkFormat, typ: VkImageType, tiling: VkImageTiling, usage: VkImageUsageFlags, flags: VkImageCreateFlags, pImageFormatProperties: *mut VkImageFormatProperties) -> VkResult;
    #[cfg(feature = "exported_prototypes")]
    pub fn vkGetPhysicalDeviceImageFormatProperties2(physicalDevice: VkPhysicalDevice, pImageFormatInfo: *const VkPhysicalDeviceImageFormatInfo2, pImageFormatProperties: *mut VkImageFormatProperties2) -> VkResult;
    #[cfg(feature = "exported_prototypes")]
    pub fn vkGetPhysicalDeviceMemoryProperties(physicalDevice: VkPhysicalDevice, pMemoryProperties: *mut VkPhysicalDeviceMemoryProperties);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkGetPhysicalDeviceMemoryProperties2(physicalDevice: VkPhysicalDevice, pMemoryProperties: *mut VkPhysicalDeviceMemoryProperties2);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkGetPhysicalDeviceProperties(physicalDevice: VkPhysicalDevice, pProperties: *mut VkPhysicalDeviceProperties);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkGetPhysicalDeviceProperties2(physicalDevice: VkPhysicalDevice, pProperties: *mut VkPhysicalDeviceProperties2);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkGetPhysicalDeviceQueueFamilyProperties(physicalDevice: VkPhysicalDevice, pQueueFamilyPropertyCount: *mut u32, pQueueFamilyProperties: *mut VkQueueFamilyProperties);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkGetPhysicalDeviceQueueFamilyProperties2(physicalDevice: VkPhysicalDevice, pQueueFamilyPropertyCount: *mut u32, pQueueFamilyProperties: *mut VkQueueFamilyProperties2);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkGetPhysicalDeviceSparseImageFormatProperties(physicalDevice: VkPhysicalDevice, format: VkFormat, typ: VkImageType, samples: VkSampleCountFlagBits, usage: VkImageUsageFlags, tiling: VkImageTiling, pPropertyCount: *mut u32, pProperties: *mut VkSparseImageFormatProperties);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkGetPhysicalDeviceSparseImageFormatProperties2(physicalDevice: VkPhysicalDevice, pFormatInfo: *const VkPhysicalDeviceSparseImageFormatInfo2, pPropertyCount: *mut u32, pProperties: *mut VkSparseImageFormatProperties2);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkGetPhysicalDeviceToolProperties(physicalDevice: VkPhysicalDevice, pToolCount: *mut u32, pToolProperties: *mut VkPhysicalDeviceToolProperties) -> VkResult;
    #[cfg(feature = "exported_prototypes")]
    pub fn vkGetPipelineCacheData(device: VkDevice, pipelineCache: VkPipelineCache, pDataSize: *mut usize, pData: *mut c_void) -> VkResult;
    #[cfg(feature = "exported_prototypes")]
    pub fn vkGetPrivateData(device: VkDevice, objectType: VkObjectType, objectHandle: u64, privateDataSlot: VkPrivateDataSlot, pData: *mut u64);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkGetQueryPoolResults(device: VkDevice, queryPool: VkQueryPool, firstQuery: u32, queryCount: u32, dataSize: usize, pData: *mut c_void, stride: VkDeviceSize, flags: VkQueryResultFlags) -> VkResult;
    #[cfg(feature = "exported_prototypes")]
    pub fn vkGetRenderAreaGranularity(device: VkDevice, renderPass: VkRenderPass, pGranularity: *mut VkExtent2D);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkGetRenderingAreaGranularity(device: VkDevice, pRenderingAreaInfo: *const VkRenderingAreaInfo, pGranularity: *mut VkExtent2D);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkGetSemaphoreCounterValue(device: VkDevice, semaphore: VkSemaphore, pValue: *mut u64) -> VkResult;
    #[cfg(feature = "exported_prototypes")]
    pub fn vkInvalidateMappedMemoryRanges(device: VkDevice, memoryRangeCount: u32, pMemoryRanges: *const VkMappedMemoryRange) -> VkResult;
    #[cfg(feature = "exported_prototypes")]
    pub fn vkMapMemory(device: VkDevice, memory: VkDeviceMemory, offset: VkDeviceSize, size: VkDeviceSize, flags: VkMemoryMapFlags, ppData: *mut *mut c_void) -> VkResult;
    #[cfg(feature = "exported_prototypes")]
    pub fn vkMapMemory2(device: VkDevice, pMemoryMapInfo: *const VkMemoryMapInfo, ppData: *mut *mut c_void) -> VkResult;
    #[cfg(feature = "exported_prototypes")]
    pub fn vkMergePipelineCaches(device: VkDevice, dstCache: VkPipelineCache, srcCacheCount: u32, pSrcCaches: *const VkPipelineCache) -> VkResult;
    #[cfg(feature = "exported_prototypes")]
    pub fn vkQueueBindSparse(queue: VkQueue, bindInfoCount: u32, pBindInfo: *const VkBindSparseInfo, fence: VkFence) -> VkResult;
    #[cfg(feature = "exported_prototypes")]
    pub fn vkQueueSubmit(queue: VkQueue, submitCount: u32, pSubmits: *const VkSubmitInfo, fence: VkFence) -> VkResult;
    #[cfg(feature = "exported_prototypes")]
    pub fn vkQueueSubmit2(queue: VkQueue, submitCount: u32, pSubmits: *const VkSubmitInfo2, fence: VkFence) -> VkResult;
    #[cfg(feature = "exported_prototypes")]
    pub fn vkQueueWaitIdle(queue: VkQueue) -> VkResult;
    #[cfg(feature = "exported_prototypes")]
    pub fn vkResetCommandBuffer(commandBuffer: VkCommandBuffer, flags: VkCommandBufferResetFlags) -> VkResult;
    #[cfg(feature = "exported_prototypes")]
    pub fn vkResetCommandPool(device: VkDevice, commandPool: VkCommandPool, flags: VkCommandPoolResetFlags) -> VkResult;
    #[cfg(feature = "exported_prototypes")]
    pub fn vkResetDescriptorPool(device: VkDevice, descriptorPool: VkDescriptorPool, flags: VkDescriptorPoolResetFlags) -> VkResult;
    #[cfg(feature = "exported_prototypes")]
    pub fn vkResetEvent(device: VkDevice, event: VkEvent) -> VkResult;
    #[cfg(feature = "exported_prototypes")]
    pub fn vkResetFences(device: VkDevice, fenceCount: u32, pFences: *const VkFence) -> VkResult;
    #[cfg(feature = "exported_prototypes")]
    pub fn vkResetQueryPool(device: VkDevice, queryPool: VkQueryPool, firstQuery: u32, queryCount: u32);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkSetEvent(device: VkDevice, event: VkEvent) -> VkResult;
    #[cfg(feature = "exported_prototypes")]
    pub fn vkSetPrivateData(device: VkDevice, objectType: VkObjectType, objectHandle: u64, privateDataSlot: VkPrivateDataSlot, data: u64) -> VkResult;
    #[cfg(feature = "exported_prototypes")]
    pub fn vkSignalSemaphore(device: VkDevice, pSignalInfo: *const VkSemaphoreSignalInfo) -> VkResult;
    #[cfg(feature = "exported_prototypes")]
    pub fn vkTransitionImageLayout(device: VkDevice, transitionCount: u32, pTransitions: *const VkHostImageLayoutTransitionInfo) -> VkResult;
    #[cfg(feature = "exported_prototypes")]
    pub fn vkTrimCommandPool(device: VkDevice, commandPool: VkCommandPool, flags: VkCommandPoolTrimFlags);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkUnmapMemory(device: VkDevice, memory: VkDeviceMemory);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkUnmapMemory2(device: VkDevice, pMemoryUnmapInfo: *const VkMemoryUnmapInfo) -> VkResult;
    #[cfg(feature = "exported_prototypes")]
    pub fn vkUpdateDescriptorSetWithTemplate(device: VkDevice, descriptorSet: VkDescriptorSet, descriptorUpdateTemplate: VkDescriptorUpdateTemplate, pData: *const c_void);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkUpdateDescriptorSets(device: VkDevice, descriptorWriteCount: u32, pDescriptorWrites: *const VkWriteDescriptorSet, descriptorCopyCount: u32, pDescriptorCopies: *const VkCopyDescriptorSet);
    #[cfg(feature = "exported_prototypes")]
    pub fn vkWaitForFences(device: VkDevice, fenceCount: u32, pFences: *const VkFence, waitAll: VkBool32, timeout: u64) -> VkResult;
    #[cfg(feature = "exported_prototypes")]
    pub fn vkWaitSemaphores(device: VkDevice, pWaitInfo: *const VkSemaphoreWaitInfo, timeout: u64) -> VkResult;
}

pub type NonNullPFN_vkAllocateCommandBuffers = unsafe extern "system" fn(device: VkDevice, pAllocateInfo: *const VkCommandBufferAllocateInfo, pCommandBuffers: *mut VkCommandBuffer) -> VkResult;
pub type NonNullPFN_vkAllocateDescriptorSets = unsafe extern "system" fn(device: VkDevice, pAllocateInfo: *const VkDescriptorSetAllocateInfo, pDescriptorSets: *mut VkDescriptorSet) -> VkResult;
pub type NonNullPFN_vkAllocateMemory = unsafe extern "system" fn(device: VkDevice, pAllocateInfo: *const VkMemoryAllocateInfo, pAllocator: *const VkAllocationCallbacks, pMemory: *mut VkDeviceMemory) -> VkResult;
pub type NonNullPFN_vkAllocationFunction = unsafe extern "system" fn(pUserData: *mut c_void, size: usize, alignment: usize, allocationScope: VkSystemAllocationScope) -> *mut c_void;
pub type NonNullPFN_vkBeginCommandBuffer = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pBeginInfo: *const VkCommandBufferBeginInfo) -> VkResult;
pub type NonNullPFN_vkBindBufferMemory = unsafe extern "system" fn(device: VkDevice, buffer: VkBuffer, memory: VkDeviceMemory, memoryOffset: VkDeviceSize) -> VkResult;
pub type NonNullPFN_vkBindBufferMemory2 = unsafe extern "system" fn(device: VkDevice, bindInfoCount: u32, pBindInfos: *const VkBindBufferMemoryInfo) -> VkResult;
pub type NonNullPFN_vkBindImageMemory = unsafe extern "system" fn(device: VkDevice, image: VkImage, memory: VkDeviceMemory, memoryOffset: VkDeviceSize) -> VkResult;
pub type NonNullPFN_vkBindImageMemory2 = unsafe extern "system" fn(device: VkDevice, bindInfoCount: u32, pBindInfos: *const VkBindImageMemoryInfo) -> VkResult;
pub type NonNullPFN_vkCmdBeginQuery = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, queryPool: VkQueryPool, query: u32, flags: VkQueryControlFlags);
pub type NonNullPFN_vkCmdBeginRenderPass = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pRenderPassBegin: *const VkRenderPassBeginInfo, contents: VkSubpassContents);
pub type NonNullPFN_vkCmdBeginRenderPass2 = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pRenderPassBegin: *const VkRenderPassBeginInfo, pSubpassBeginInfo: *const VkSubpassBeginInfo);
pub type NonNullPFN_vkCmdBeginRendering = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pRenderingInfo: *const VkRenderingInfo);
pub type NonNullPFN_vkCmdBindDescriptorSets = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pipelineBindPoint: VkPipelineBindPoint, layout: VkPipelineLayout, firstSet: u32, descriptorSetCount: u32, pDescriptorSets: *const VkDescriptorSet, dynamicOffsetCount: u32, pDynamicOffsets: *const u32);
pub type NonNullPFN_vkCmdBindDescriptorSets2 = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pBindDescriptorSetsInfo: *const VkBindDescriptorSetsInfo);
pub type NonNullPFN_vkCmdBindIndexBuffer = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize, indexType: VkIndexType);
pub type NonNullPFN_vkCmdBindIndexBuffer2 = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize, size: VkDeviceSize, indexType: VkIndexType);
pub type NonNullPFN_vkCmdBindPipeline = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pipelineBindPoint: VkPipelineBindPoint, pipeline: VkPipeline);
pub type NonNullPFN_vkCmdBindVertexBuffers = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, firstBinding: u32, bindingCount: u32, pBuffers: *const VkBuffer, pOffsets: *const VkDeviceSize);
pub type NonNullPFN_vkCmdBindVertexBuffers2 = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, firstBinding: u32, bindingCount: u32, pBuffers: *const VkBuffer, pOffsets: *const VkDeviceSize, pSizes: *const VkDeviceSize, pStrides: *const VkDeviceSize);
pub type NonNullPFN_vkCmdBlitImage = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, srcImage: VkImage, srcImageLayout: VkImageLayout, dstImage: VkImage, dstImageLayout: VkImageLayout, regionCount: u32, pRegions: *const VkImageBlit, filter: VkFilter);
pub type NonNullPFN_vkCmdBlitImage2 = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pBlitImageInfo: *const VkBlitImageInfo2);
pub type NonNullPFN_vkCmdClearAttachments = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, attachmentCount: u32, pAttachments: *const VkClearAttachment, rectCount: u32, pRects: *const VkClearRect);
pub type NonNullPFN_vkCmdClearColorImage = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, image: VkImage, imageLayout: VkImageLayout, pColor: *const VkClearColorValue, rangeCount: u32, pRanges: *const VkImageSubresourceRange);
pub type NonNullPFN_vkCmdClearDepthStencilImage = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, image: VkImage, imageLayout: VkImageLayout, pDepthStencil: *const VkClearDepthStencilValue, rangeCount: u32, pRanges: *const VkImageSubresourceRange);
pub type NonNullPFN_vkCmdCopyBuffer = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, srcBuffer: VkBuffer, dstBuffer: VkBuffer, regionCount: u32, pRegions: *const VkBufferCopy);
pub type NonNullPFN_vkCmdCopyBuffer2 = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pCopyBufferInfo: *const VkCopyBufferInfo2);
pub type NonNullPFN_vkCmdCopyBufferToImage = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, srcBuffer: VkBuffer, dstImage: VkImage, dstImageLayout: VkImageLayout, regionCount: u32, pRegions: *const VkBufferImageCopy);
pub type NonNullPFN_vkCmdCopyBufferToImage2 = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pCopyBufferToImageInfo: *const VkCopyBufferToImageInfo2);
pub type NonNullPFN_vkCmdCopyImage = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, srcImage: VkImage, srcImageLayout: VkImageLayout, dstImage: VkImage, dstImageLayout: VkImageLayout, regionCount: u32, pRegions: *const VkImageCopy);
pub type NonNullPFN_vkCmdCopyImage2 = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pCopyImageInfo: *const VkCopyImageInfo2);
pub type NonNullPFN_vkCmdCopyImageToBuffer = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, srcImage: VkImage, srcImageLayout: VkImageLayout, dstBuffer: VkBuffer, regionCount: u32, pRegions: *const VkBufferImageCopy);
pub type NonNullPFN_vkCmdCopyImageToBuffer2 = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pCopyImageToBufferInfo: *const VkCopyImageToBufferInfo2);
pub type NonNullPFN_vkCmdCopyQueryPoolResults = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, queryPool: VkQueryPool, firstQuery: u32, queryCount: u32, dstBuffer: VkBuffer, dstOffset: VkDeviceSize, stride: VkDeviceSize, flags: VkQueryResultFlags);
pub type NonNullPFN_vkCmdDispatch = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, groupCountX: u32, groupCountY: u32, groupCountZ: u32);
pub type NonNullPFN_vkCmdDispatchBase = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, baseGroupX: u32, baseGroupY: u32, baseGroupZ: u32, groupCountX: u32, groupCountY: u32, groupCountZ: u32);
pub type NonNullPFN_vkCmdDispatchIndirect = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize);
pub type NonNullPFN_vkCmdDraw = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, vertexCount: u32, instanceCount: u32, firstVertex: u32, firstInstance: u32);
pub type NonNullPFN_vkCmdDrawIndexed = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, indexCount: u32, instanceCount: u32, firstIndex: u32, vertexOffset: i32, firstInstance: u32);
pub type NonNullPFN_vkCmdDrawIndexedIndirect = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize, drawCount: u32, stride: u32);
pub type NonNullPFN_vkCmdDrawIndexedIndirectCount = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize, countBuffer: VkBuffer, countBufferOffset: VkDeviceSize, maxDrawCount: u32, stride: u32);
pub type NonNullPFN_vkCmdDrawIndirect = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize, drawCount: u32, stride: u32);
pub type NonNullPFN_vkCmdDrawIndirectCount = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize, countBuffer: VkBuffer, countBufferOffset: VkDeviceSize, maxDrawCount: u32, stride: u32);
pub type NonNullPFN_vkCmdEndQuery = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, queryPool: VkQueryPool, query: u32);
pub type NonNullPFN_vkCmdEndRenderPass = unsafe extern "system" fn(commandBuffer: VkCommandBuffer);
pub type NonNullPFN_vkCmdEndRenderPass2 = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pSubpassEndInfo: *const VkSubpassEndInfo);
pub type NonNullPFN_vkCmdEndRendering = unsafe extern "system" fn(commandBuffer: VkCommandBuffer);
pub type NonNullPFN_vkCmdExecuteCommands = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, commandBufferCount: u32, pCommandBuffers: *const VkCommandBuffer);
pub type NonNullPFN_vkCmdFillBuffer = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, dstBuffer: VkBuffer, dstOffset: VkDeviceSize, size: VkDeviceSize, data: u32);
pub type NonNullPFN_vkCmdNextSubpass = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, contents: VkSubpassContents);
pub type NonNullPFN_vkCmdNextSubpass2 = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pSubpassBeginInfo: *const VkSubpassBeginInfo, pSubpassEndInfo: *const VkSubpassEndInfo);
pub type NonNullPFN_vkCmdPipelineBarrier = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, srcStageMask: VkPipelineStageFlags, dstStageMask: VkPipelineStageFlags, dependencyFlags: VkDependencyFlags, memoryBarrierCount: u32, pMemoryBarriers: *const VkMemoryBarrier, bufferMemoryBarrierCount: u32, pBufferMemoryBarriers: *const VkBufferMemoryBarrier, imageMemoryBarrierCount: u32, pImageMemoryBarriers: *const VkImageMemoryBarrier);
pub type NonNullPFN_vkCmdPipelineBarrier2 = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pDependencyInfo: *const VkDependencyInfo);
pub type NonNullPFN_vkCmdPushConstants = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, layout: VkPipelineLayout, stageFlags: VkShaderStageFlags, offset: u32, size: u32, pValues: *const c_void);
pub type NonNullPFN_vkCmdPushConstants2 = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pPushConstantsInfo: *const VkPushConstantsInfo);
pub type NonNullPFN_vkCmdPushDescriptorSet = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pipelineBindPoint: VkPipelineBindPoint, layout: VkPipelineLayout, set: u32, descriptorWriteCount: u32, pDescriptorWrites: *const VkWriteDescriptorSet);
pub type NonNullPFN_vkCmdPushDescriptorSet2 = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pPushDescriptorSetInfo: *const VkPushDescriptorSetInfo);
pub type NonNullPFN_vkCmdPushDescriptorSetWithTemplate = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, descriptorUpdateTemplate: VkDescriptorUpdateTemplate, layout: VkPipelineLayout, set: u32, pData: *const c_void);
pub type NonNullPFN_vkCmdPushDescriptorSetWithTemplate2 = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pPushDescriptorSetWithTemplateInfo: *const VkPushDescriptorSetWithTemplateInfo);
pub type NonNullPFN_vkCmdResetEvent = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, event: VkEvent, stageMask: VkPipelineStageFlags);
pub type NonNullPFN_vkCmdResetEvent2 = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, event: VkEvent, stageMask: VkPipelineStageFlags2);
pub type NonNullPFN_vkCmdResetQueryPool = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, queryPool: VkQueryPool, firstQuery: u32, queryCount: u32);
pub type NonNullPFN_vkCmdResolveImage = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, srcImage: VkImage, srcImageLayout: VkImageLayout, dstImage: VkImage, dstImageLayout: VkImageLayout, regionCount: u32, pRegions: *const VkImageResolve);
pub type NonNullPFN_vkCmdResolveImage2 = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pResolveImageInfo: *const VkResolveImageInfo2);
pub type NonNullPFN_vkCmdSetBlendConstants = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, blendConstants: *const c_float);
pub type NonNullPFN_vkCmdSetCullMode = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, cullMode: VkCullModeFlags);
pub type NonNullPFN_vkCmdSetDepthBias = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, depthBiasConstantFactor: c_float, depthBiasClamp: c_float, depthBiasSlopeFactor: c_float);
pub type NonNullPFN_vkCmdSetDepthBiasEnable = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, depthBiasEnable: VkBool32);
pub type NonNullPFN_vkCmdSetDepthBounds = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, minDepthBounds: c_float, maxDepthBounds: c_float);
pub type NonNullPFN_vkCmdSetDepthBoundsTestEnable = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, depthBoundsTestEnable: VkBool32);
pub type NonNullPFN_vkCmdSetDepthCompareOp = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, depthCompareOp: VkCompareOp);
pub type NonNullPFN_vkCmdSetDepthTestEnable = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, depthTestEnable: VkBool32);
pub type NonNullPFN_vkCmdSetDepthWriteEnable = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, depthWriteEnable: VkBool32);
pub type NonNullPFN_vkCmdSetDeviceMask = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, deviceMask: u32);
pub type NonNullPFN_vkCmdSetEvent = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, event: VkEvent, stageMask: VkPipelineStageFlags);
pub type NonNullPFN_vkCmdSetEvent2 = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, event: VkEvent, pDependencyInfo: *const VkDependencyInfo);
pub type NonNullPFN_vkCmdSetFrontFace = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, frontFace: VkFrontFace);
pub type NonNullPFN_vkCmdSetLineStipple = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, lineStippleFactor: u32, lineStipplePattern: u16);
pub type NonNullPFN_vkCmdSetLineWidth = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, lineWidth: c_float);
pub type NonNullPFN_vkCmdSetPrimitiveRestartEnable = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, primitiveRestartEnable: VkBool32);
pub type NonNullPFN_vkCmdSetPrimitiveTopology = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, primitiveTopology: VkPrimitiveTopology);
pub type NonNullPFN_vkCmdSetRasterizerDiscardEnable = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, rasterizerDiscardEnable: VkBool32);
pub type NonNullPFN_vkCmdSetRenderingAttachmentLocations = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pLocationInfo: *const VkRenderingAttachmentLocationInfo);
pub type NonNullPFN_vkCmdSetRenderingInputAttachmentIndices = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pInputAttachmentIndexInfo: *const VkRenderingInputAttachmentIndexInfo);
pub type NonNullPFN_vkCmdSetScissor = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, firstScissor: u32, scissorCount: u32, pScissors: *const VkRect2D);
pub type NonNullPFN_vkCmdSetScissorWithCount = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, scissorCount: u32, pScissors: *const VkRect2D);
pub type NonNullPFN_vkCmdSetStencilCompareMask = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, faceMask: VkStencilFaceFlags, compareMask: u32);
pub type NonNullPFN_vkCmdSetStencilOp = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, faceMask: VkStencilFaceFlags, failOp: VkStencilOp, passOp: VkStencilOp, depthFailOp: VkStencilOp, compareOp: VkCompareOp);
pub type NonNullPFN_vkCmdSetStencilReference = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, faceMask: VkStencilFaceFlags, reference: u32);
pub type NonNullPFN_vkCmdSetStencilTestEnable = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, stencilTestEnable: VkBool32);
pub type NonNullPFN_vkCmdSetStencilWriteMask = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, faceMask: VkStencilFaceFlags, writeMask: u32);
pub type NonNullPFN_vkCmdSetViewport = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, firstViewport: u32, viewportCount: u32, pViewports: *const VkViewport);
pub type NonNullPFN_vkCmdSetViewportWithCount = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, viewportCount: u32, pViewports: *const VkViewport);
pub type NonNullPFN_vkCmdUpdateBuffer = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, dstBuffer: VkBuffer, dstOffset: VkDeviceSize, dataSize: VkDeviceSize, pData: *const c_void);
pub type NonNullPFN_vkCmdWaitEvents = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, eventCount: u32, pEvents: *const VkEvent, srcStageMask: VkPipelineStageFlags, dstStageMask: VkPipelineStageFlags, memoryBarrierCount: u32, pMemoryBarriers: *const VkMemoryBarrier, bufferMemoryBarrierCount: u32, pBufferMemoryBarriers: *const VkBufferMemoryBarrier, imageMemoryBarrierCount: u32, pImageMemoryBarriers: *const VkImageMemoryBarrier);
pub type NonNullPFN_vkCmdWaitEvents2 = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, eventCount: u32, pEvents: *const VkEvent, pDependencyInfos: *const VkDependencyInfo);
pub type NonNullPFN_vkCmdWriteTimestamp = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pipelineStage: VkPipelineStageFlagBits, queryPool: VkQueryPool, query: u32);
pub type NonNullPFN_vkCmdWriteTimestamp2 = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, stage: VkPipelineStageFlags2, queryPool: VkQueryPool, query: u32);
pub type NonNullPFN_vkCopyImageToImage = unsafe extern "system" fn(device: VkDevice, pCopyImageToImageInfo: *const VkCopyImageToImageInfo) -> VkResult;
pub type NonNullPFN_vkCopyImageToMemory = unsafe extern "system" fn(device: VkDevice, pCopyImageToMemoryInfo: *const VkCopyImageToMemoryInfo) -> VkResult;
pub type NonNullPFN_vkCopyMemoryToImage = unsafe extern "system" fn(device: VkDevice, pCopyMemoryToImageInfo: *const VkCopyMemoryToImageInfo) -> VkResult;
pub type NonNullPFN_vkCreateBuffer = unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkBufferCreateInfo, pAllocator: *const VkAllocationCallbacks, pBuffer: *mut VkBuffer) -> VkResult;
pub type NonNullPFN_vkCreateBufferView = unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkBufferViewCreateInfo, pAllocator: *const VkAllocationCallbacks, pView: *mut VkBufferView) -> VkResult;
pub type NonNullPFN_vkCreateCommandPool = unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkCommandPoolCreateInfo, pAllocator: *const VkAllocationCallbacks, pCommandPool: *mut VkCommandPool) -> VkResult;
pub type NonNullPFN_vkCreateComputePipelines = unsafe extern "system" fn(device: VkDevice, pipelineCache: VkPipelineCache, createInfoCount: u32, pCreateInfos: *const VkComputePipelineCreateInfo, pAllocator: *const VkAllocationCallbacks, pPipelines: *mut VkPipeline) -> VkResult;
pub type NonNullPFN_vkCreateDescriptorPool = unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkDescriptorPoolCreateInfo, pAllocator: *const VkAllocationCallbacks, pDescriptorPool: *mut VkDescriptorPool) -> VkResult;
pub type NonNullPFN_vkCreateDescriptorSetLayout = unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkDescriptorSetLayoutCreateInfo, pAllocator: *const VkAllocationCallbacks, pSetLayout: *mut VkDescriptorSetLayout) -> VkResult;
pub type NonNullPFN_vkCreateDescriptorUpdateTemplate = unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkDescriptorUpdateTemplateCreateInfo, pAllocator: *const VkAllocationCallbacks, pDescriptorUpdateTemplate: *mut VkDescriptorUpdateTemplate) -> VkResult;
pub type NonNullPFN_vkCreateDevice = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pCreateInfo: *const VkDeviceCreateInfo, pAllocator: *const VkAllocationCallbacks, pDevice: *mut VkDevice) -> VkResult;
pub type NonNullPFN_vkCreateEvent = unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkEventCreateInfo, pAllocator: *const VkAllocationCallbacks, pEvent: *mut VkEvent) -> VkResult;
pub type NonNullPFN_vkCreateFence = unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkFenceCreateInfo, pAllocator: *const VkAllocationCallbacks, pFence: *mut VkFence) -> VkResult;
pub type NonNullPFN_vkCreateFramebuffer = unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkFramebufferCreateInfo, pAllocator: *const VkAllocationCallbacks, pFramebuffer: *mut VkFramebuffer) -> VkResult;
pub type NonNullPFN_vkCreateGraphicsPipelines = unsafe extern "system" fn(device: VkDevice, pipelineCache: VkPipelineCache, createInfoCount: u32, pCreateInfos: *const VkGraphicsPipelineCreateInfo, pAllocator: *const VkAllocationCallbacks, pPipelines: *mut VkPipeline) -> VkResult;
pub type NonNullPFN_vkCreateImage = unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkImageCreateInfo, pAllocator: *const VkAllocationCallbacks, pImage: *mut VkImage) -> VkResult;
pub type NonNullPFN_vkCreateImageView = unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkImageViewCreateInfo, pAllocator: *const VkAllocationCallbacks, pView: *mut VkImageView) -> VkResult;
pub type NonNullPFN_vkCreateInstance = unsafe extern "system" fn(pCreateInfo: *const VkInstanceCreateInfo, pAllocator: *const VkAllocationCallbacks, pInstance: *mut VkInstance) -> VkResult;
pub type NonNullPFN_vkCreatePipelineCache = unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkPipelineCacheCreateInfo, pAllocator: *const VkAllocationCallbacks, pPipelineCache: *mut VkPipelineCache) -> VkResult;
pub type NonNullPFN_vkCreatePipelineLayout = unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkPipelineLayoutCreateInfo, pAllocator: *const VkAllocationCallbacks, pPipelineLayout: *mut VkPipelineLayout) -> VkResult;
pub type NonNullPFN_vkCreatePrivateDataSlot = unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkPrivateDataSlotCreateInfo, pAllocator: *const VkAllocationCallbacks, pPrivateDataSlot: *mut VkPrivateDataSlot) -> VkResult;
pub type NonNullPFN_vkCreateQueryPool = unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkQueryPoolCreateInfo, pAllocator: *const VkAllocationCallbacks, pQueryPool: *mut VkQueryPool) -> VkResult;
pub type NonNullPFN_vkCreateRenderPass = unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkRenderPassCreateInfo, pAllocator: *const VkAllocationCallbacks, pRenderPass: *mut VkRenderPass) -> VkResult;
pub type NonNullPFN_vkCreateRenderPass2 = unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkRenderPassCreateInfo2, pAllocator: *const VkAllocationCallbacks, pRenderPass: *mut VkRenderPass) -> VkResult;
pub type NonNullPFN_vkCreateSampler = unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkSamplerCreateInfo, pAllocator: *const VkAllocationCallbacks, pSampler: *mut VkSampler) -> VkResult;
pub type NonNullPFN_vkCreateSamplerYcbcrConversion = unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkSamplerYcbcrConversionCreateInfo, pAllocator: *const VkAllocationCallbacks, pYcbcrConversion: *mut VkSamplerYcbcrConversion) -> VkResult;
pub type NonNullPFN_vkCreateSemaphore = unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkSemaphoreCreateInfo, pAllocator: *const VkAllocationCallbacks, pSemaphore: *mut VkSemaphore) -> VkResult;
pub type NonNullPFN_vkCreateShaderModule = unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkShaderModuleCreateInfo, pAllocator: *const VkAllocationCallbacks, pShaderModule: *mut VkShaderModule) -> VkResult;
pub type NonNullPFN_vkDestroyBuffer = unsafe extern "system" fn(device: VkDevice, buffer: VkBuffer, pAllocator: *const VkAllocationCallbacks);
pub type NonNullPFN_vkDestroyBufferView = unsafe extern "system" fn(device: VkDevice, bufferView: VkBufferView, pAllocator: *const VkAllocationCallbacks);
pub type NonNullPFN_vkDestroyCommandPool = unsafe extern "system" fn(device: VkDevice, commandPool: VkCommandPool, pAllocator: *const VkAllocationCallbacks);
pub type NonNullPFN_vkDestroyDescriptorPool = unsafe extern "system" fn(device: VkDevice, descriptorPool: VkDescriptorPool, pAllocator: *const VkAllocationCallbacks);
pub type NonNullPFN_vkDestroyDescriptorSetLayout = unsafe extern "system" fn(device: VkDevice, descriptorSetLayout: VkDescriptorSetLayout, pAllocator: *const VkAllocationCallbacks);
pub type NonNullPFN_vkDestroyDescriptorUpdateTemplate = unsafe extern "system" fn(device: VkDevice, descriptorUpdateTemplate: VkDescriptorUpdateTemplate, pAllocator: *const VkAllocationCallbacks);
pub type NonNullPFN_vkDestroyDevice = unsafe extern "system" fn(device: VkDevice, pAllocator: *const VkAllocationCallbacks);
pub type NonNullPFN_vkDestroyEvent = unsafe extern "system" fn(device: VkDevice, event: VkEvent, pAllocator: *const VkAllocationCallbacks);
pub type NonNullPFN_vkDestroyFence = unsafe extern "system" fn(device: VkDevice, fence: VkFence, pAllocator: *const VkAllocationCallbacks);
pub type NonNullPFN_vkDestroyFramebuffer = unsafe extern "system" fn(device: VkDevice, framebuffer: VkFramebuffer, pAllocator: *const VkAllocationCallbacks);
pub type NonNullPFN_vkDestroyImage = unsafe extern "system" fn(device: VkDevice, image: VkImage, pAllocator: *const VkAllocationCallbacks);
pub type NonNullPFN_vkDestroyImageView = unsafe extern "system" fn(device: VkDevice, imageView: VkImageView, pAllocator: *const VkAllocationCallbacks);
pub type NonNullPFN_vkDestroyInstance = unsafe extern "system" fn(instance: VkInstance, pAllocator: *const VkAllocationCallbacks);
pub type NonNullPFN_vkDestroyPipeline = unsafe extern "system" fn(device: VkDevice, pipeline: VkPipeline, pAllocator: *const VkAllocationCallbacks);
pub type NonNullPFN_vkDestroyPipelineCache = unsafe extern "system" fn(device: VkDevice, pipelineCache: VkPipelineCache, pAllocator: *const VkAllocationCallbacks);
pub type NonNullPFN_vkDestroyPipelineLayout = unsafe extern "system" fn(device: VkDevice, pipelineLayout: VkPipelineLayout, pAllocator: *const VkAllocationCallbacks);
pub type NonNullPFN_vkDestroyPrivateDataSlot = unsafe extern "system" fn(device: VkDevice, privateDataSlot: VkPrivateDataSlot, pAllocator: *const VkAllocationCallbacks);
pub type NonNullPFN_vkDestroyQueryPool = unsafe extern "system" fn(device: VkDevice, queryPool: VkQueryPool, pAllocator: *const VkAllocationCallbacks);
pub type NonNullPFN_vkDestroyRenderPass = unsafe extern "system" fn(device: VkDevice, renderPass: VkRenderPass, pAllocator: *const VkAllocationCallbacks);
pub type NonNullPFN_vkDestroySampler = unsafe extern "system" fn(device: VkDevice, sampler: VkSampler, pAllocator: *const VkAllocationCallbacks);
pub type NonNullPFN_vkDestroySamplerYcbcrConversion = unsafe extern "system" fn(device: VkDevice, ycbcrConversion: VkSamplerYcbcrConversion, pAllocator: *const VkAllocationCallbacks);
pub type NonNullPFN_vkDestroySemaphore = unsafe extern "system" fn(device: VkDevice, semaphore: VkSemaphore, pAllocator: *const VkAllocationCallbacks);
pub type NonNullPFN_vkDestroyShaderModule = unsafe extern "system" fn(device: VkDevice, shaderModule: VkShaderModule, pAllocator: *const VkAllocationCallbacks);
pub type NonNullPFN_vkDeviceWaitIdle = unsafe extern "system" fn(device: VkDevice) -> VkResult;
pub type NonNullPFN_vkEndCommandBuffer = unsafe extern "system" fn(commandBuffer: VkCommandBuffer) -> VkResult;
pub type NonNullPFN_vkEnumerateDeviceExtensionProperties = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pLayerName: *const c_char, pPropertyCount: *mut u32, pProperties: *mut VkExtensionProperties) -> VkResult;
pub type NonNullPFN_vkEnumerateDeviceLayerProperties = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pPropertyCount: *mut u32, pProperties: *mut VkLayerProperties) -> VkResult;
pub type NonNullPFN_vkEnumerateInstanceExtensionProperties = unsafe extern "system" fn(pLayerName: *const c_char, pPropertyCount: *mut u32, pProperties: *mut VkExtensionProperties) -> VkResult;
pub type NonNullPFN_vkEnumerateInstanceLayerProperties = unsafe extern "system" fn(pPropertyCount: *mut u32, pProperties: *mut VkLayerProperties) -> VkResult;
pub type NonNullPFN_vkEnumerateInstanceVersion = unsafe extern "system" fn(pApiVersion: *mut u32) -> VkResult;
pub type NonNullPFN_vkEnumeratePhysicalDeviceGroups = unsafe extern "system" fn(instance: VkInstance, pPhysicalDeviceGroupCount: *mut u32, pPhysicalDeviceGroupProperties: *mut VkPhysicalDeviceGroupProperties) -> VkResult;
pub type NonNullPFN_vkEnumeratePhysicalDevices = unsafe extern "system" fn(instance: VkInstance, pPhysicalDeviceCount: *mut u32, pPhysicalDevices: *mut VkPhysicalDevice) -> VkResult;
pub type NonNullPFN_vkFlushMappedMemoryRanges = unsafe extern "system" fn(device: VkDevice, memoryRangeCount: u32, pMemoryRanges: *const VkMappedMemoryRange) -> VkResult;
pub type NonNullPFN_vkFreeCommandBuffers = unsafe extern "system" fn(device: VkDevice, commandPool: VkCommandPool, commandBufferCount: u32, pCommandBuffers: *const VkCommandBuffer);
pub type NonNullPFN_vkFreeDescriptorSets = unsafe extern "system" fn(device: VkDevice, descriptorPool: VkDescriptorPool, descriptorSetCount: u32, pDescriptorSets: *const VkDescriptorSet) -> VkResult;
pub type NonNullPFN_vkFreeFunction = unsafe extern "system" fn(pUserData: *mut c_void, pMemory: *mut c_void);
pub type NonNullPFN_vkFreeMemory = unsafe extern "system" fn(device: VkDevice, memory: VkDeviceMemory, pAllocator: *const VkAllocationCallbacks);
pub type NonNullPFN_vkGetBufferDeviceAddress = unsafe extern "system" fn(device: VkDevice, pInfo: *const VkBufferDeviceAddressInfo) -> VkDeviceAddress;
pub type NonNullPFN_vkGetBufferMemoryRequirements = unsafe extern "system" fn(device: VkDevice, buffer: VkBuffer, pMemoryRequirements: *mut VkMemoryRequirements);
pub type NonNullPFN_vkGetBufferMemoryRequirements2 = unsafe extern "system" fn(device: VkDevice, pInfo: *const VkBufferMemoryRequirementsInfo2, pMemoryRequirements: *mut VkMemoryRequirements2);
pub type NonNullPFN_vkGetBufferOpaqueCaptureAddress = unsafe extern "system" fn(device: VkDevice, pInfo: *const VkBufferDeviceAddressInfo) -> u64;
pub type NonNullPFN_vkGetDescriptorSetLayoutSupport = unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkDescriptorSetLayoutCreateInfo, pSupport: *mut VkDescriptorSetLayoutSupport);
pub type NonNullPFN_vkGetDeviceBufferMemoryRequirements = unsafe extern "system" fn(device: VkDevice, pInfo: *const VkDeviceBufferMemoryRequirements, pMemoryRequirements: *mut VkMemoryRequirements2);
pub type NonNullPFN_vkGetDeviceGroupPeerMemoryFeatures = unsafe extern "system" fn(device: VkDevice, heapIndex: u32, localDeviceIndex: u32, remoteDeviceIndex: u32, pPeerMemoryFeatures: *mut VkPeerMemoryFeatureFlags);
pub type NonNullPFN_vkGetDeviceImageMemoryRequirements = unsafe extern "system" fn(device: VkDevice, pInfo: *const VkDeviceImageMemoryRequirements, pMemoryRequirements: *mut VkMemoryRequirements2);
pub type NonNullPFN_vkGetDeviceImageSparseMemoryRequirements = unsafe extern "system" fn(device: VkDevice, pInfo: *const VkDeviceImageMemoryRequirements, pSparseMemoryRequirementCount: *mut u32, pSparseMemoryRequirements: *mut VkSparseImageMemoryRequirements2);
pub type NonNullPFN_vkGetDeviceImageSubresourceLayout = unsafe extern "system" fn(device: VkDevice, pInfo: *const VkDeviceImageSubresourceInfo, pLayout: *mut VkSubresourceLayout2);
pub type NonNullPFN_vkGetDeviceMemoryCommitment = unsafe extern "system" fn(device: VkDevice, memory: VkDeviceMemory, pCommittedMemoryInBytes: *mut VkDeviceSize);
pub type NonNullPFN_vkGetDeviceMemoryOpaqueCaptureAddress = unsafe extern "system" fn(device: VkDevice, pInfo: *const VkDeviceMemoryOpaqueCaptureAddressInfo) -> u64;
pub type NonNullPFN_vkGetDeviceProcAddr = unsafe extern "system" fn(device: VkDevice, pName: *const c_char) -> PFN_vkVoidFunction;
pub type NonNullPFN_vkGetDeviceQueue = unsafe extern "system" fn(device: VkDevice, queueFamilyIndex: u32, queueIndex: u32, pQueue: *mut VkQueue);
pub type NonNullPFN_vkGetDeviceQueue2 = unsafe extern "system" fn(device: VkDevice, pQueueInfo: *const VkDeviceQueueInfo2, pQueue: *mut VkQueue);
pub type NonNullPFN_vkGetEventStatus = unsafe extern "system" fn(device: VkDevice, event: VkEvent) -> VkResult;
pub type NonNullPFN_vkGetFenceStatus = unsafe extern "system" fn(device: VkDevice, fence: VkFence) -> VkResult;
pub type NonNullPFN_vkGetImageMemoryRequirements = unsafe extern "system" fn(device: VkDevice, image: VkImage, pMemoryRequirements: *mut VkMemoryRequirements);
pub type NonNullPFN_vkGetImageMemoryRequirements2 = unsafe extern "system" fn(device: VkDevice, pInfo: *const VkImageMemoryRequirementsInfo2, pMemoryRequirements: *mut VkMemoryRequirements2);
pub type NonNullPFN_vkGetImageSparseMemoryRequirements = unsafe extern "system" fn(device: VkDevice, image: VkImage, pSparseMemoryRequirementCount: *mut u32, pSparseMemoryRequirements: *mut VkSparseImageMemoryRequirements);
pub type NonNullPFN_vkGetImageSparseMemoryRequirements2 = unsafe extern "system" fn(device: VkDevice, pInfo: *const VkImageSparseMemoryRequirementsInfo2, pSparseMemoryRequirementCount: *mut u32, pSparseMemoryRequirements: *mut VkSparseImageMemoryRequirements2);
pub type NonNullPFN_vkGetImageSubresourceLayout = unsafe extern "system" fn(device: VkDevice, image: VkImage, pSubresource: *const VkImageSubresource, pLayout: *mut VkSubresourceLayout);
pub type NonNullPFN_vkGetImageSubresourceLayout2 = unsafe extern "system" fn(device: VkDevice, image: VkImage, pSubresource: *const VkImageSubresource2, pLayout: *mut VkSubresourceLayout2);
pub type NonNullPFN_vkGetInstanceProcAddr = unsafe extern "system" fn(instance: VkInstance, pName: *const c_char) -> PFN_vkVoidFunction;
pub type NonNullPFN_vkGetPhysicalDeviceExternalBufferProperties = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pExternalBufferInfo: *const VkPhysicalDeviceExternalBufferInfo, pExternalBufferProperties: *mut VkExternalBufferProperties);
pub type NonNullPFN_vkGetPhysicalDeviceExternalFenceProperties = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pExternalFenceInfo: *const VkPhysicalDeviceExternalFenceInfo, pExternalFenceProperties: *mut VkExternalFenceProperties);
pub type NonNullPFN_vkGetPhysicalDeviceExternalSemaphoreProperties = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pExternalSemaphoreInfo: *const VkPhysicalDeviceExternalSemaphoreInfo, pExternalSemaphoreProperties: *mut VkExternalSemaphoreProperties);
pub type NonNullPFN_vkGetPhysicalDeviceFeatures = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pFeatures: *mut VkPhysicalDeviceFeatures);
pub type NonNullPFN_vkGetPhysicalDeviceFeatures2 = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pFeatures: *mut VkPhysicalDeviceFeatures2);
pub type NonNullPFN_vkGetPhysicalDeviceFormatProperties = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, format: VkFormat, pFormatProperties: *mut VkFormatProperties);
pub type NonNullPFN_vkGetPhysicalDeviceFormatProperties2 = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, format: VkFormat, pFormatProperties: *mut VkFormatProperties2);
pub type NonNullPFN_vkGetPhysicalDeviceImageFormatProperties = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, format: VkFormat, typ: VkImageType, tiling: VkImageTiling, usage: VkImageUsageFlags, flags: VkImageCreateFlags, pImageFormatProperties: *mut VkImageFormatProperties) -> VkResult;
pub type NonNullPFN_vkGetPhysicalDeviceImageFormatProperties2 = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pImageFormatInfo: *const VkPhysicalDeviceImageFormatInfo2, pImageFormatProperties: *mut VkImageFormatProperties2) -> VkResult;
pub type NonNullPFN_vkGetPhysicalDeviceMemoryProperties = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pMemoryProperties: *mut VkPhysicalDeviceMemoryProperties);
pub type NonNullPFN_vkGetPhysicalDeviceMemoryProperties2 = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pMemoryProperties: *mut VkPhysicalDeviceMemoryProperties2);
pub type NonNullPFN_vkGetPhysicalDeviceProperties = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pProperties: *mut VkPhysicalDeviceProperties);
pub type NonNullPFN_vkGetPhysicalDeviceProperties2 = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pProperties: *mut VkPhysicalDeviceProperties2);
pub type NonNullPFN_vkGetPhysicalDeviceQueueFamilyProperties = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pQueueFamilyPropertyCount: *mut u32, pQueueFamilyProperties: *mut VkQueueFamilyProperties);
pub type NonNullPFN_vkGetPhysicalDeviceQueueFamilyProperties2 = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pQueueFamilyPropertyCount: *mut u32, pQueueFamilyProperties: *mut VkQueueFamilyProperties2);
pub type NonNullPFN_vkGetPhysicalDeviceSparseImageFormatProperties = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, format: VkFormat, typ: VkImageType, samples: VkSampleCountFlagBits, usage: VkImageUsageFlags, tiling: VkImageTiling, pPropertyCount: *mut u32, pProperties: *mut VkSparseImageFormatProperties);
pub type NonNullPFN_vkGetPhysicalDeviceSparseImageFormatProperties2 = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pFormatInfo: *const VkPhysicalDeviceSparseImageFormatInfo2, pPropertyCount: *mut u32, pProperties: *mut VkSparseImageFormatProperties2);
pub type NonNullPFN_vkGetPhysicalDeviceToolProperties = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pToolCount: *mut u32, pToolProperties: *mut VkPhysicalDeviceToolProperties) -> VkResult;
pub type NonNullPFN_vkGetPipelineCacheData = unsafe extern "system" fn(device: VkDevice, pipelineCache: VkPipelineCache, pDataSize: *mut usize, pData: *mut c_void) -> VkResult;
pub type NonNullPFN_vkGetPrivateData = unsafe extern "system" fn(device: VkDevice, objectType: VkObjectType, objectHandle: u64, privateDataSlot: VkPrivateDataSlot, pData: *mut u64);
pub type NonNullPFN_vkGetQueryPoolResults = unsafe extern "system" fn(device: VkDevice, queryPool: VkQueryPool, firstQuery: u32, queryCount: u32, dataSize: usize, pData: *mut c_void, stride: VkDeviceSize, flags: VkQueryResultFlags) -> VkResult;
pub type NonNullPFN_vkGetRenderAreaGranularity = unsafe extern "system" fn(device: VkDevice, renderPass: VkRenderPass, pGranularity: *mut VkExtent2D);
pub type NonNullPFN_vkGetRenderingAreaGranularity = unsafe extern "system" fn(device: VkDevice, pRenderingAreaInfo: *const VkRenderingAreaInfo, pGranularity: *mut VkExtent2D);
pub type NonNullPFN_vkGetSemaphoreCounterValue = unsafe extern "system" fn(device: VkDevice, semaphore: VkSemaphore, pValue: *mut u64) -> VkResult;
pub type NonNullPFN_vkInternalAllocationNotification = unsafe extern "system" fn(pUserData: *mut c_void, size: usize, allocationType: VkInternalAllocationType, allocationScope: VkSystemAllocationScope);
pub type NonNullPFN_vkInternalFreeNotification = unsafe extern "system" fn(pUserData: *mut c_void, size: usize, allocationType: VkInternalAllocationType, allocationScope: VkSystemAllocationScope);
pub type NonNullPFN_vkInvalidateMappedMemoryRanges = unsafe extern "system" fn(device: VkDevice, memoryRangeCount: u32, pMemoryRanges: *const VkMappedMemoryRange) -> VkResult;
pub type NonNullPFN_vkMapMemory = unsafe extern "system" fn(device: VkDevice, memory: VkDeviceMemory, offset: VkDeviceSize, size: VkDeviceSize, flags: VkMemoryMapFlags, ppData: *mut *mut c_void) -> VkResult;
pub type NonNullPFN_vkMapMemory2 = unsafe extern "system" fn(device: VkDevice, pMemoryMapInfo: *const VkMemoryMapInfo, ppData: *mut *mut c_void) -> VkResult;
pub type NonNullPFN_vkMergePipelineCaches = unsafe extern "system" fn(device: VkDevice, dstCache: VkPipelineCache, srcCacheCount: u32, pSrcCaches: *const VkPipelineCache) -> VkResult;
pub type NonNullPFN_vkQueueBindSparse = unsafe extern "system" fn(queue: VkQueue, bindInfoCount: u32, pBindInfo: *const VkBindSparseInfo, fence: VkFence) -> VkResult;
pub type NonNullPFN_vkQueueSubmit = unsafe extern "system" fn(queue: VkQueue, submitCount: u32, pSubmits: *const VkSubmitInfo, fence: VkFence) -> VkResult;
pub type NonNullPFN_vkQueueSubmit2 = unsafe extern "system" fn(queue: VkQueue, submitCount: u32, pSubmits: *const VkSubmitInfo2, fence: VkFence) -> VkResult;
pub type NonNullPFN_vkQueueWaitIdle = unsafe extern "system" fn(queue: VkQueue) -> VkResult;
pub type NonNullPFN_vkReallocationFunction = unsafe extern "system" fn(pUserData: *mut c_void, pOriginal: *mut c_void, size: usize, alignment: usize, allocationScope: VkSystemAllocationScope) -> *mut c_void;
pub type NonNullPFN_vkResetCommandBuffer = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, flags: VkCommandBufferResetFlags) -> VkResult;
pub type NonNullPFN_vkResetCommandPool = unsafe extern "system" fn(device: VkDevice, commandPool: VkCommandPool, flags: VkCommandPoolResetFlags) -> VkResult;
pub type NonNullPFN_vkResetDescriptorPool = unsafe extern "system" fn(device: VkDevice, descriptorPool: VkDescriptorPool, flags: VkDescriptorPoolResetFlags) -> VkResult;
pub type NonNullPFN_vkResetEvent = unsafe extern "system" fn(device: VkDevice, event: VkEvent) -> VkResult;
pub type NonNullPFN_vkResetFences = unsafe extern "system" fn(device: VkDevice, fenceCount: u32, pFences: *const VkFence) -> VkResult;
pub type NonNullPFN_vkResetQueryPool = unsafe extern "system" fn(device: VkDevice, queryPool: VkQueryPool, firstQuery: u32, queryCount: u32);
pub type NonNullPFN_vkSetEvent = unsafe extern "system" fn(device: VkDevice, event: VkEvent) -> VkResult;
pub type NonNullPFN_vkSetPrivateData = unsafe extern "system" fn(device: VkDevice, objectType: VkObjectType, objectHandle: u64, privateDataSlot: VkPrivateDataSlot, data: u64) -> VkResult;
pub type NonNullPFN_vkSignalSemaphore = unsafe extern "system" fn(device: VkDevice, pSignalInfo: *const VkSemaphoreSignalInfo) -> VkResult;
pub type NonNullPFN_vkTransitionImageLayout = unsafe extern "system" fn(device: VkDevice, transitionCount: u32, pTransitions: *const VkHostImageLayoutTransitionInfo) -> VkResult;
pub type NonNullPFN_vkTrimCommandPool = unsafe extern "system" fn(device: VkDevice, commandPool: VkCommandPool, flags: VkCommandPoolTrimFlags);
pub type NonNullPFN_vkUnmapMemory = unsafe extern "system" fn(device: VkDevice, memory: VkDeviceMemory);
pub type NonNullPFN_vkUnmapMemory2 = unsafe extern "system" fn(device: VkDevice, pMemoryUnmapInfo: *const VkMemoryUnmapInfo) -> VkResult;
pub type NonNullPFN_vkUpdateDescriptorSetWithTemplate = unsafe extern "system" fn(device: VkDevice, descriptorSet: VkDescriptorSet, descriptorUpdateTemplate: VkDescriptorUpdateTemplate, pData: *const c_void);
pub type NonNullPFN_vkUpdateDescriptorSets = unsafe extern "system" fn(device: VkDevice, descriptorWriteCount: u32, pDescriptorWrites: *const VkWriteDescriptorSet, descriptorCopyCount: u32, pDescriptorCopies: *const VkCopyDescriptorSet);
pub type NonNullPFN_vkVoidFunction = unsafe extern "system" fn();
pub type NonNullPFN_vkWaitForFences = unsafe extern "system" fn(device: VkDevice, fenceCount: u32, pFences: *const VkFence, waitAll: VkBool32, timeout: u64) -> VkResult;
pub type NonNullPFN_vkWaitSemaphores = unsafe extern "system" fn(device: VkDevice, pWaitInfo: *const VkSemaphoreWaitInfo, timeout: u64) -> VkResult;
pub type NonNullVkBuffer = NonNull<VkBuffer_T>;
pub type NonNullVkBufferView = NonNull<VkBufferView_T>;
pub type NonNullVkCommandBuffer = NonNull<VkCommandBuffer_T>;
pub type NonNullVkCommandPool = NonNull<VkCommandPool_T>;
pub type NonNullVkDescriptorPool = NonNull<VkDescriptorPool_T>;
pub type NonNullVkDescriptorSet = NonNull<VkDescriptorSet_T>;
pub type NonNullVkDescriptorSetLayout = NonNull<VkDescriptorSetLayout_T>;
pub type NonNullVkDescriptorUpdateTemplate = NonNull<VkDescriptorUpdateTemplate_T>;
pub type NonNullVkDevice = NonNull<VkDevice_T>;
pub type NonNullVkDeviceMemory = NonNull<VkDeviceMemory_T>;
pub type NonNullVkEvent = NonNull<VkEvent_T>;
pub type NonNullVkFence = NonNull<VkFence_T>;
pub type NonNullVkFramebuffer = NonNull<VkFramebuffer_T>;
pub type NonNullVkImage = NonNull<VkImage_T>;
pub type NonNullVkImageView = NonNull<VkImageView_T>;
pub type NonNullVkInstance = NonNull<VkInstance_T>;
pub type NonNullVkPhysicalDevice = NonNull<VkPhysicalDevice_T>;
pub type NonNullVkPipeline = NonNull<VkPipeline_T>;
pub type NonNullVkPipelineCache = NonNull<VkPipelineCache_T>;
pub type NonNullVkPipelineLayout = NonNull<VkPipelineLayout_T>;
pub type NonNullVkPrivateDataSlot = NonNull<VkPrivateDataSlot_T>;
pub type NonNullVkQueryPool = NonNull<VkQueryPool_T>;
pub type NonNullVkQueue = NonNull<VkQueue_T>;
pub type NonNullVkRenderPass = NonNull<VkRenderPass_T>;
pub type NonNullVkSampler = NonNull<VkSampler_T>;
pub type NonNullVkSamplerYcbcrConversion = NonNull<VkSamplerYcbcrConversion_T>;
pub type NonNullVkSemaphore = NonNull<VkSemaphore_T>;
pub type NonNullVkShaderModule = NonNull<VkShaderModule_T>;
pub type PFN_vkAllocateCommandBuffers = Option<NonNullPFN_vkAllocateCommandBuffers>;
pub type PFN_vkAllocateDescriptorSets = Option<NonNullPFN_vkAllocateDescriptorSets>;
pub type PFN_vkAllocateMemory = Option<NonNullPFN_vkAllocateMemory>;
pub type PFN_vkAllocationFunction = Option<NonNullPFN_vkAllocationFunction>;
pub type PFN_vkBeginCommandBuffer = Option<NonNullPFN_vkBeginCommandBuffer>;
pub type PFN_vkBindBufferMemory = Option<NonNullPFN_vkBindBufferMemory>;
pub type PFN_vkBindBufferMemory2 = Option<NonNullPFN_vkBindBufferMemory2>;
pub type PFN_vkBindImageMemory = Option<NonNullPFN_vkBindImageMemory>;
pub type PFN_vkBindImageMemory2 = Option<NonNullPFN_vkBindImageMemory2>;
pub type PFN_vkCmdBeginQuery = Option<NonNullPFN_vkCmdBeginQuery>;
pub type PFN_vkCmdBeginRenderPass = Option<NonNullPFN_vkCmdBeginRenderPass>;
pub type PFN_vkCmdBeginRenderPass2 = Option<NonNullPFN_vkCmdBeginRenderPass2>;
pub type PFN_vkCmdBeginRendering = Option<NonNullPFN_vkCmdBeginRendering>;
pub type PFN_vkCmdBindDescriptorSets = Option<NonNullPFN_vkCmdBindDescriptorSets>;
pub type PFN_vkCmdBindDescriptorSets2 = Option<NonNullPFN_vkCmdBindDescriptorSets2>;
pub type PFN_vkCmdBindIndexBuffer = Option<NonNullPFN_vkCmdBindIndexBuffer>;
pub type PFN_vkCmdBindIndexBuffer2 = Option<NonNullPFN_vkCmdBindIndexBuffer2>;
pub type PFN_vkCmdBindPipeline = Option<NonNullPFN_vkCmdBindPipeline>;
pub type PFN_vkCmdBindVertexBuffers = Option<NonNullPFN_vkCmdBindVertexBuffers>;
pub type PFN_vkCmdBindVertexBuffers2 = Option<NonNullPFN_vkCmdBindVertexBuffers2>;
pub type PFN_vkCmdBlitImage = Option<NonNullPFN_vkCmdBlitImage>;
pub type PFN_vkCmdBlitImage2 = Option<NonNullPFN_vkCmdBlitImage2>;
pub type PFN_vkCmdClearAttachments = Option<NonNullPFN_vkCmdClearAttachments>;
pub type PFN_vkCmdClearColorImage = Option<NonNullPFN_vkCmdClearColorImage>;
pub type PFN_vkCmdClearDepthStencilImage = Option<NonNullPFN_vkCmdClearDepthStencilImage>;
pub type PFN_vkCmdCopyBuffer = Option<NonNullPFN_vkCmdCopyBuffer>;
pub type PFN_vkCmdCopyBuffer2 = Option<NonNullPFN_vkCmdCopyBuffer2>;
pub type PFN_vkCmdCopyBufferToImage = Option<NonNullPFN_vkCmdCopyBufferToImage>;
pub type PFN_vkCmdCopyBufferToImage2 = Option<NonNullPFN_vkCmdCopyBufferToImage2>;
pub type PFN_vkCmdCopyImage = Option<NonNullPFN_vkCmdCopyImage>;
pub type PFN_vkCmdCopyImage2 = Option<NonNullPFN_vkCmdCopyImage2>;
pub type PFN_vkCmdCopyImageToBuffer = Option<NonNullPFN_vkCmdCopyImageToBuffer>;
pub type PFN_vkCmdCopyImageToBuffer2 = Option<NonNullPFN_vkCmdCopyImageToBuffer2>;
pub type PFN_vkCmdCopyQueryPoolResults = Option<NonNullPFN_vkCmdCopyQueryPoolResults>;
pub type PFN_vkCmdDispatch = Option<NonNullPFN_vkCmdDispatch>;
pub type PFN_vkCmdDispatchBase = Option<NonNullPFN_vkCmdDispatchBase>;
pub type PFN_vkCmdDispatchIndirect = Option<NonNullPFN_vkCmdDispatchIndirect>;
pub type PFN_vkCmdDraw = Option<NonNullPFN_vkCmdDraw>;
pub type PFN_vkCmdDrawIndexed = Option<NonNullPFN_vkCmdDrawIndexed>;
pub type PFN_vkCmdDrawIndexedIndirect = Option<NonNullPFN_vkCmdDrawIndexedIndirect>;
pub type PFN_vkCmdDrawIndexedIndirectCount = Option<NonNullPFN_vkCmdDrawIndexedIndirectCount>;
pub type PFN_vkCmdDrawIndirect = Option<NonNullPFN_vkCmdDrawIndirect>;
pub type PFN_vkCmdDrawIndirectCount = Option<NonNullPFN_vkCmdDrawIndirectCount>;
pub type PFN_vkCmdEndQuery = Option<NonNullPFN_vkCmdEndQuery>;
pub type PFN_vkCmdEndRenderPass = Option<NonNullPFN_vkCmdEndRenderPass>;
pub type PFN_vkCmdEndRenderPass2 = Option<NonNullPFN_vkCmdEndRenderPass2>;
pub type PFN_vkCmdEndRendering = Option<NonNullPFN_vkCmdEndRendering>;
pub type PFN_vkCmdExecuteCommands = Option<NonNullPFN_vkCmdExecuteCommands>;
pub type PFN_vkCmdFillBuffer = Option<NonNullPFN_vkCmdFillBuffer>;
pub type PFN_vkCmdNextSubpass = Option<NonNullPFN_vkCmdNextSubpass>;
pub type PFN_vkCmdNextSubpass2 = Option<NonNullPFN_vkCmdNextSubpass2>;
pub type PFN_vkCmdPipelineBarrier = Option<NonNullPFN_vkCmdPipelineBarrier>;
pub type PFN_vkCmdPipelineBarrier2 = Option<NonNullPFN_vkCmdPipelineBarrier2>;
pub type PFN_vkCmdPushConstants = Option<NonNullPFN_vkCmdPushConstants>;
pub type PFN_vkCmdPushConstants2 = Option<NonNullPFN_vkCmdPushConstants2>;
pub type PFN_vkCmdPushDescriptorSet = Option<NonNullPFN_vkCmdPushDescriptorSet>;
pub type PFN_vkCmdPushDescriptorSet2 = Option<NonNullPFN_vkCmdPushDescriptorSet2>;
pub type PFN_vkCmdPushDescriptorSetWithTemplate = Option<NonNullPFN_vkCmdPushDescriptorSetWithTemplate>;
pub type PFN_vkCmdPushDescriptorSetWithTemplate2 = Option<NonNullPFN_vkCmdPushDescriptorSetWithTemplate2>;
pub type PFN_vkCmdResetEvent = Option<NonNullPFN_vkCmdResetEvent>;
pub type PFN_vkCmdResetEvent2 = Option<NonNullPFN_vkCmdResetEvent2>;
pub type PFN_vkCmdResetQueryPool = Option<NonNullPFN_vkCmdResetQueryPool>;
pub type PFN_vkCmdResolveImage = Option<NonNullPFN_vkCmdResolveImage>;
pub type PFN_vkCmdResolveImage2 = Option<NonNullPFN_vkCmdResolveImage2>;
pub type PFN_vkCmdSetBlendConstants = Option<NonNullPFN_vkCmdSetBlendConstants>;
pub type PFN_vkCmdSetCullMode = Option<NonNullPFN_vkCmdSetCullMode>;
pub type PFN_vkCmdSetDepthBias = Option<NonNullPFN_vkCmdSetDepthBias>;
pub type PFN_vkCmdSetDepthBiasEnable = Option<NonNullPFN_vkCmdSetDepthBiasEnable>;
pub type PFN_vkCmdSetDepthBounds = Option<NonNullPFN_vkCmdSetDepthBounds>;
pub type PFN_vkCmdSetDepthBoundsTestEnable = Option<NonNullPFN_vkCmdSetDepthBoundsTestEnable>;
pub type PFN_vkCmdSetDepthCompareOp = Option<NonNullPFN_vkCmdSetDepthCompareOp>;
pub type PFN_vkCmdSetDepthTestEnable = Option<NonNullPFN_vkCmdSetDepthTestEnable>;
pub type PFN_vkCmdSetDepthWriteEnable = Option<NonNullPFN_vkCmdSetDepthWriteEnable>;
pub type PFN_vkCmdSetDeviceMask = Option<NonNullPFN_vkCmdSetDeviceMask>;
pub type PFN_vkCmdSetEvent = Option<NonNullPFN_vkCmdSetEvent>;
pub type PFN_vkCmdSetEvent2 = Option<NonNullPFN_vkCmdSetEvent2>;
pub type PFN_vkCmdSetFrontFace = Option<NonNullPFN_vkCmdSetFrontFace>;
pub type PFN_vkCmdSetLineStipple = Option<NonNullPFN_vkCmdSetLineStipple>;
pub type PFN_vkCmdSetLineWidth = Option<NonNullPFN_vkCmdSetLineWidth>;
pub type PFN_vkCmdSetPrimitiveRestartEnable = Option<NonNullPFN_vkCmdSetPrimitiveRestartEnable>;
pub type PFN_vkCmdSetPrimitiveTopology = Option<NonNullPFN_vkCmdSetPrimitiveTopology>;
pub type PFN_vkCmdSetRasterizerDiscardEnable = Option<NonNullPFN_vkCmdSetRasterizerDiscardEnable>;
pub type PFN_vkCmdSetRenderingAttachmentLocations = Option<NonNullPFN_vkCmdSetRenderingAttachmentLocations>;
pub type PFN_vkCmdSetRenderingInputAttachmentIndices = Option<NonNullPFN_vkCmdSetRenderingInputAttachmentIndices>;
pub type PFN_vkCmdSetScissor = Option<NonNullPFN_vkCmdSetScissor>;
pub type PFN_vkCmdSetScissorWithCount = Option<NonNullPFN_vkCmdSetScissorWithCount>;
pub type PFN_vkCmdSetStencilCompareMask = Option<NonNullPFN_vkCmdSetStencilCompareMask>;
pub type PFN_vkCmdSetStencilOp = Option<NonNullPFN_vkCmdSetStencilOp>;
pub type PFN_vkCmdSetStencilReference = Option<NonNullPFN_vkCmdSetStencilReference>;
pub type PFN_vkCmdSetStencilTestEnable = Option<NonNullPFN_vkCmdSetStencilTestEnable>;
pub type PFN_vkCmdSetStencilWriteMask = Option<NonNullPFN_vkCmdSetStencilWriteMask>;
pub type PFN_vkCmdSetViewport = Option<NonNullPFN_vkCmdSetViewport>;
pub type PFN_vkCmdSetViewportWithCount = Option<NonNullPFN_vkCmdSetViewportWithCount>;
pub type PFN_vkCmdUpdateBuffer = Option<NonNullPFN_vkCmdUpdateBuffer>;
pub type PFN_vkCmdWaitEvents = Option<NonNullPFN_vkCmdWaitEvents>;
pub type PFN_vkCmdWaitEvents2 = Option<NonNullPFN_vkCmdWaitEvents2>;
pub type PFN_vkCmdWriteTimestamp = Option<NonNullPFN_vkCmdWriteTimestamp>;
pub type PFN_vkCmdWriteTimestamp2 = Option<NonNullPFN_vkCmdWriteTimestamp2>;
pub type PFN_vkCopyImageToImage = Option<NonNullPFN_vkCopyImageToImage>;
pub type PFN_vkCopyImageToMemory = Option<NonNullPFN_vkCopyImageToMemory>;
pub type PFN_vkCopyMemoryToImage = Option<NonNullPFN_vkCopyMemoryToImage>;
pub type PFN_vkCreateBuffer = Option<NonNullPFN_vkCreateBuffer>;
pub type PFN_vkCreateBufferView = Option<NonNullPFN_vkCreateBufferView>;
pub type PFN_vkCreateCommandPool = Option<NonNullPFN_vkCreateCommandPool>;
pub type PFN_vkCreateComputePipelines = Option<NonNullPFN_vkCreateComputePipelines>;
pub type PFN_vkCreateDescriptorPool = Option<NonNullPFN_vkCreateDescriptorPool>;
pub type PFN_vkCreateDescriptorSetLayout = Option<NonNullPFN_vkCreateDescriptorSetLayout>;
pub type PFN_vkCreateDescriptorUpdateTemplate = Option<NonNullPFN_vkCreateDescriptorUpdateTemplate>;
pub type PFN_vkCreateDevice = Option<NonNullPFN_vkCreateDevice>;
pub type PFN_vkCreateEvent = Option<NonNullPFN_vkCreateEvent>;
pub type PFN_vkCreateFence = Option<NonNullPFN_vkCreateFence>;
pub type PFN_vkCreateFramebuffer = Option<NonNullPFN_vkCreateFramebuffer>;
pub type PFN_vkCreateGraphicsPipelines = Option<NonNullPFN_vkCreateGraphicsPipelines>;
pub type PFN_vkCreateImage = Option<NonNullPFN_vkCreateImage>;
pub type PFN_vkCreateImageView = Option<NonNullPFN_vkCreateImageView>;
pub type PFN_vkCreateInstance = Option<NonNullPFN_vkCreateInstance>;
pub type PFN_vkCreatePipelineCache = Option<NonNullPFN_vkCreatePipelineCache>;
pub type PFN_vkCreatePipelineLayout = Option<NonNullPFN_vkCreatePipelineLayout>;
pub type PFN_vkCreatePrivateDataSlot = Option<NonNullPFN_vkCreatePrivateDataSlot>;
pub type PFN_vkCreateQueryPool = Option<NonNullPFN_vkCreateQueryPool>;
pub type PFN_vkCreateRenderPass = Option<NonNullPFN_vkCreateRenderPass>;
pub type PFN_vkCreateRenderPass2 = Option<NonNullPFN_vkCreateRenderPass2>;
pub type PFN_vkCreateSampler = Option<NonNullPFN_vkCreateSampler>;
pub type PFN_vkCreateSamplerYcbcrConversion = Option<NonNullPFN_vkCreateSamplerYcbcrConversion>;
pub type PFN_vkCreateSemaphore = Option<NonNullPFN_vkCreateSemaphore>;
pub type PFN_vkCreateShaderModule = Option<NonNullPFN_vkCreateShaderModule>;
pub type PFN_vkDestroyBuffer = Option<NonNullPFN_vkDestroyBuffer>;
pub type PFN_vkDestroyBufferView = Option<NonNullPFN_vkDestroyBufferView>;
pub type PFN_vkDestroyCommandPool = Option<NonNullPFN_vkDestroyCommandPool>;
pub type PFN_vkDestroyDescriptorPool = Option<NonNullPFN_vkDestroyDescriptorPool>;
pub type PFN_vkDestroyDescriptorSetLayout = Option<NonNullPFN_vkDestroyDescriptorSetLayout>;
pub type PFN_vkDestroyDescriptorUpdateTemplate = Option<NonNullPFN_vkDestroyDescriptorUpdateTemplate>;
pub type PFN_vkDestroyDevice = Option<NonNullPFN_vkDestroyDevice>;
pub type PFN_vkDestroyEvent = Option<NonNullPFN_vkDestroyEvent>;
pub type PFN_vkDestroyFence = Option<NonNullPFN_vkDestroyFence>;
pub type PFN_vkDestroyFramebuffer = Option<NonNullPFN_vkDestroyFramebuffer>;
pub type PFN_vkDestroyImage = Option<NonNullPFN_vkDestroyImage>;
pub type PFN_vkDestroyImageView = Option<NonNullPFN_vkDestroyImageView>;
pub type PFN_vkDestroyInstance = Option<NonNullPFN_vkDestroyInstance>;
pub type PFN_vkDestroyPipeline = Option<NonNullPFN_vkDestroyPipeline>;
pub type PFN_vkDestroyPipelineCache = Option<NonNullPFN_vkDestroyPipelineCache>;
pub type PFN_vkDestroyPipelineLayout = Option<NonNullPFN_vkDestroyPipelineLayout>;
pub type PFN_vkDestroyPrivateDataSlot = Option<NonNullPFN_vkDestroyPrivateDataSlot>;
pub type PFN_vkDestroyQueryPool = Option<NonNullPFN_vkDestroyQueryPool>;
pub type PFN_vkDestroyRenderPass = Option<NonNullPFN_vkDestroyRenderPass>;
pub type PFN_vkDestroySampler = Option<NonNullPFN_vkDestroySampler>;
pub type PFN_vkDestroySamplerYcbcrConversion = Option<NonNullPFN_vkDestroySamplerYcbcrConversion>;
pub type PFN_vkDestroySemaphore = Option<NonNullPFN_vkDestroySemaphore>;
pub type PFN_vkDestroyShaderModule = Option<NonNullPFN_vkDestroyShaderModule>;
pub type PFN_vkDeviceWaitIdle = Option<NonNullPFN_vkDeviceWaitIdle>;
pub type PFN_vkEndCommandBuffer = Option<NonNullPFN_vkEndCommandBuffer>;
pub type PFN_vkEnumerateDeviceExtensionProperties = Option<NonNullPFN_vkEnumerateDeviceExtensionProperties>;
pub type PFN_vkEnumerateDeviceLayerProperties = Option<NonNullPFN_vkEnumerateDeviceLayerProperties>;
pub type PFN_vkEnumerateInstanceExtensionProperties = Option<NonNullPFN_vkEnumerateInstanceExtensionProperties>;
pub type PFN_vkEnumerateInstanceLayerProperties = Option<NonNullPFN_vkEnumerateInstanceLayerProperties>;
pub type PFN_vkEnumerateInstanceVersion = Option<NonNullPFN_vkEnumerateInstanceVersion>;
pub type PFN_vkEnumeratePhysicalDeviceGroups = Option<NonNullPFN_vkEnumeratePhysicalDeviceGroups>;
pub type PFN_vkEnumeratePhysicalDevices = Option<NonNullPFN_vkEnumeratePhysicalDevices>;
pub type PFN_vkFlushMappedMemoryRanges = Option<NonNullPFN_vkFlushMappedMemoryRanges>;
pub type PFN_vkFreeCommandBuffers = Option<NonNullPFN_vkFreeCommandBuffers>;
pub type PFN_vkFreeDescriptorSets = Option<NonNullPFN_vkFreeDescriptorSets>;
pub type PFN_vkFreeFunction = Option<NonNullPFN_vkFreeFunction>;
pub type PFN_vkFreeMemory = Option<NonNullPFN_vkFreeMemory>;
pub type PFN_vkGetBufferDeviceAddress = Option<NonNullPFN_vkGetBufferDeviceAddress>;
pub type PFN_vkGetBufferMemoryRequirements = Option<NonNullPFN_vkGetBufferMemoryRequirements>;
pub type PFN_vkGetBufferMemoryRequirements2 = Option<NonNullPFN_vkGetBufferMemoryRequirements2>;
pub type PFN_vkGetBufferOpaqueCaptureAddress = Option<NonNullPFN_vkGetBufferOpaqueCaptureAddress>;
pub type PFN_vkGetDescriptorSetLayoutSupport = Option<NonNullPFN_vkGetDescriptorSetLayoutSupport>;
pub type PFN_vkGetDeviceBufferMemoryRequirements = Option<NonNullPFN_vkGetDeviceBufferMemoryRequirements>;
pub type PFN_vkGetDeviceGroupPeerMemoryFeatures = Option<NonNullPFN_vkGetDeviceGroupPeerMemoryFeatures>;
pub type PFN_vkGetDeviceImageMemoryRequirements = Option<NonNullPFN_vkGetDeviceImageMemoryRequirements>;
pub type PFN_vkGetDeviceImageSparseMemoryRequirements = Option<NonNullPFN_vkGetDeviceImageSparseMemoryRequirements>;
pub type PFN_vkGetDeviceImageSubresourceLayout = Option<NonNullPFN_vkGetDeviceImageSubresourceLayout>;
pub type PFN_vkGetDeviceMemoryCommitment = Option<NonNullPFN_vkGetDeviceMemoryCommitment>;
pub type PFN_vkGetDeviceMemoryOpaqueCaptureAddress = Option<NonNullPFN_vkGetDeviceMemoryOpaqueCaptureAddress>;
pub type PFN_vkGetDeviceProcAddr = Option<NonNullPFN_vkGetDeviceProcAddr>;
pub type PFN_vkGetDeviceQueue = Option<NonNullPFN_vkGetDeviceQueue>;
pub type PFN_vkGetDeviceQueue2 = Option<NonNullPFN_vkGetDeviceQueue2>;
pub type PFN_vkGetEventStatus = Option<NonNullPFN_vkGetEventStatus>;
pub type PFN_vkGetFenceStatus = Option<NonNullPFN_vkGetFenceStatus>;
pub type PFN_vkGetImageMemoryRequirements = Option<NonNullPFN_vkGetImageMemoryRequirements>;
pub type PFN_vkGetImageMemoryRequirements2 = Option<NonNullPFN_vkGetImageMemoryRequirements2>;
pub type PFN_vkGetImageSparseMemoryRequirements = Option<NonNullPFN_vkGetImageSparseMemoryRequirements>;
pub type PFN_vkGetImageSparseMemoryRequirements2 = Option<NonNullPFN_vkGetImageSparseMemoryRequirements2>;
pub type PFN_vkGetImageSubresourceLayout = Option<NonNullPFN_vkGetImageSubresourceLayout>;
pub type PFN_vkGetImageSubresourceLayout2 = Option<NonNullPFN_vkGetImageSubresourceLayout2>;
pub type PFN_vkGetInstanceProcAddr = Option<NonNullPFN_vkGetInstanceProcAddr>;
pub type PFN_vkGetPhysicalDeviceExternalBufferProperties = Option<NonNullPFN_vkGetPhysicalDeviceExternalBufferProperties>;
pub type PFN_vkGetPhysicalDeviceExternalFenceProperties = Option<NonNullPFN_vkGetPhysicalDeviceExternalFenceProperties>;
pub type PFN_vkGetPhysicalDeviceExternalSemaphoreProperties = Option<NonNullPFN_vkGetPhysicalDeviceExternalSemaphoreProperties>;
pub type PFN_vkGetPhysicalDeviceFeatures = Option<NonNullPFN_vkGetPhysicalDeviceFeatures>;
pub type PFN_vkGetPhysicalDeviceFeatures2 = Option<NonNullPFN_vkGetPhysicalDeviceFeatures2>;
pub type PFN_vkGetPhysicalDeviceFormatProperties = Option<NonNullPFN_vkGetPhysicalDeviceFormatProperties>;
pub type PFN_vkGetPhysicalDeviceFormatProperties2 = Option<NonNullPFN_vkGetPhysicalDeviceFormatProperties2>;
pub type PFN_vkGetPhysicalDeviceImageFormatProperties = Option<NonNullPFN_vkGetPhysicalDeviceImageFormatProperties>;
pub type PFN_vkGetPhysicalDeviceImageFormatProperties2 = Option<NonNullPFN_vkGetPhysicalDeviceImageFormatProperties2>;
pub type PFN_vkGetPhysicalDeviceMemoryProperties = Option<NonNullPFN_vkGetPhysicalDeviceMemoryProperties>;
pub type PFN_vkGetPhysicalDeviceMemoryProperties2 = Option<NonNullPFN_vkGetPhysicalDeviceMemoryProperties2>;
pub type PFN_vkGetPhysicalDeviceProperties = Option<NonNullPFN_vkGetPhysicalDeviceProperties>;
pub type PFN_vkGetPhysicalDeviceProperties2 = Option<NonNullPFN_vkGetPhysicalDeviceProperties2>;
pub type PFN_vkGetPhysicalDeviceQueueFamilyProperties = Option<NonNullPFN_vkGetPhysicalDeviceQueueFamilyProperties>;
pub type PFN_vkGetPhysicalDeviceQueueFamilyProperties2 = Option<NonNullPFN_vkGetPhysicalDeviceQueueFamilyProperties2>;
pub type PFN_vkGetPhysicalDeviceSparseImageFormatProperties = Option<NonNullPFN_vkGetPhysicalDeviceSparseImageFormatProperties>;
pub type PFN_vkGetPhysicalDeviceSparseImageFormatProperties2 = Option<NonNullPFN_vkGetPhysicalDeviceSparseImageFormatProperties2>;
pub type PFN_vkGetPhysicalDeviceToolProperties = Option<NonNullPFN_vkGetPhysicalDeviceToolProperties>;
pub type PFN_vkGetPipelineCacheData = Option<NonNullPFN_vkGetPipelineCacheData>;
pub type PFN_vkGetPrivateData = Option<NonNullPFN_vkGetPrivateData>;
pub type PFN_vkGetQueryPoolResults = Option<NonNullPFN_vkGetQueryPoolResults>;
pub type PFN_vkGetRenderAreaGranularity = Option<NonNullPFN_vkGetRenderAreaGranularity>;
pub type PFN_vkGetRenderingAreaGranularity = Option<NonNullPFN_vkGetRenderingAreaGranularity>;
pub type PFN_vkGetSemaphoreCounterValue = Option<NonNullPFN_vkGetSemaphoreCounterValue>;
pub type PFN_vkInternalAllocationNotification = Option<NonNullPFN_vkInternalAllocationNotification>;
pub type PFN_vkInternalFreeNotification = Option<NonNullPFN_vkInternalFreeNotification>;
pub type PFN_vkInvalidateMappedMemoryRanges = Option<NonNullPFN_vkInvalidateMappedMemoryRanges>;
pub type PFN_vkMapMemory = Option<NonNullPFN_vkMapMemory>;
pub type PFN_vkMapMemory2 = Option<NonNullPFN_vkMapMemory2>;
pub type PFN_vkMergePipelineCaches = Option<NonNullPFN_vkMergePipelineCaches>;
pub type PFN_vkQueueBindSparse = Option<NonNullPFN_vkQueueBindSparse>;
pub type PFN_vkQueueSubmit = Option<NonNullPFN_vkQueueSubmit>;
pub type PFN_vkQueueSubmit2 = Option<NonNullPFN_vkQueueSubmit2>;
pub type PFN_vkQueueWaitIdle = Option<NonNullPFN_vkQueueWaitIdle>;
pub type PFN_vkReallocationFunction = Option<NonNullPFN_vkReallocationFunction>;
pub type PFN_vkResetCommandBuffer = Option<NonNullPFN_vkResetCommandBuffer>;
pub type PFN_vkResetCommandPool = Option<NonNullPFN_vkResetCommandPool>;
pub type PFN_vkResetDescriptorPool = Option<NonNullPFN_vkResetDescriptorPool>;
pub type PFN_vkResetEvent = Option<NonNullPFN_vkResetEvent>;
pub type PFN_vkResetFences = Option<NonNullPFN_vkResetFences>;
pub type PFN_vkResetQueryPool = Option<NonNullPFN_vkResetQueryPool>;
pub type PFN_vkSetEvent = Option<NonNullPFN_vkSetEvent>;
pub type PFN_vkSetPrivateData = Option<NonNullPFN_vkSetPrivateData>;
pub type PFN_vkSignalSemaphore = Option<NonNullPFN_vkSignalSemaphore>;
pub type PFN_vkTransitionImageLayout = Option<NonNullPFN_vkTransitionImageLayout>;
pub type PFN_vkTrimCommandPool = Option<NonNullPFN_vkTrimCommandPool>;
pub type PFN_vkUnmapMemory = Option<NonNullPFN_vkUnmapMemory>;
pub type PFN_vkUnmapMemory2 = Option<NonNullPFN_vkUnmapMemory2>;
pub type PFN_vkUpdateDescriptorSetWithTemplate = Option<NonNullPFN_vkUpdateDescriptorSetWithTemplate>;
pub type PFN_vkUpdateDescriptorSets = Option<NonNullPFN_vkUpdateDescriptorSets>;
pub type PFN_vkVoidFunction = Option<NonNullPFN_vkVoidFunction>;
pub type PFN_vkWaitForFences = Option<NonNullPFN_vkWaitForFences>;
pub type PFN_vkWaitSemaphores = Option<NonNullPFN_vkWaitSemaphores>;
pub type VkAccessFlagBits = VkFlags;
pub type VkAccessFlagBits2 = VkFlags64;
pub type VkAccessFlags = VkFlags;
pub type VkAccessFlags2 = VkFlags64;
pub type VkAttachmentDescriptionFlagBits = VkFlags;
pub type VkAttachmentDescriptionFlags = VkFlags;
pub type VkAttachmentLoadOp = i32;
pub type VkAttachmentStoreOp = i32;
pub type VkBlendFactor = i32;
pub type VkBlendOp = i32;
pub type VkBool32 = u32;
pub type VkBorderColor = i32;
pub type VkBuffer = *mut VkBuffer_T;
pub type VkBufferCreateFlagBits = VkFlags;
pub type VkBufferCreateFlags = VkFlags;
pub type VkBufferUsageFlagBits = VkFlags;
pub type VkBufferUsageFlagBits2 = VkFlags64;
pub type VkBufferUsageFlags = VkFlags;
pub type VkBufferUsageFlags2 = VkFlags64;
pub type VkBufferView = *mut VkBufferView_T;
pub type VkBufferViewCreateFlags = VkFlags;
pub type VkChromaLocation = i32;
pub type VkColorComponentFlagBits = VkFlags;
pub type VkColorComponentFlags = VkFlags;
pub type VkCommandBuffer = *mut VkCommandBuffer_T;
pub type VkCommandBufferLevel = i32;
pub type VkCommandBufferResetFlagBits = VkFlags;
pub type VkCommandBufferResetFlags = VkFlags;
pub type VkCommandBufferUsageFlagBits = VkFlags;
pub type VkCommandBufferUsageFlags = VkFlags;
pub type VkCommandPool = *mut VkCommandPool_T;
pub type VkCommandPoolCreateFlagBits = VkFlags;
pub type VkCommandPoolCreateFlags = VkFlags;
pub type VkCommandPoolResetFlagBits = VkFlags;
pub type VkCommandPoolResetFlags = VkFlags;
pub type VkCommandPoolTrimFlags = VkFlags;
pub type VkCompareOp = i32;
pub type VkComponentSwizzle = i32;
pub type VkCullModeFlagBits = VkFlags;
pub type VkCullModeFlags = VkFlags;
pub type VkDependencyFlagBits = VkFlags;
pub type VkDependencyFlags = VkFlags;
pub type VkDescriptorBindingFlagBits = VkFlags;
pub type VkDescriptorBindingFlags = VkFlags;
pub type VkDescriptorPool = *mut VkDescriptorPool_T;
pub type VkDescriptorPoolCreateFlagBits = VkFlags;
pub type VkDescriptorPoolCreateFlags = VkFlags;
pub type VkDescriptorPoolResetFlags = VkFlags;
pub type VkDescriptorSet = *mut VkDescriptorSet_T;
pub type VkDescriptorSetLayout = *mut VkDescriptorSetLayout_T;
pub type VkDescriptorSetLayoutCreateFlagBits = VkFlags;
pub type VkDescriptorSetLayoutCreateFlags = VkFlags;
pub type VkDescriptorType = i32;
pub type VkDescriptorUpdateTemplate = *mut VkDescriptorUpdateTemplate_T;
pub type VkDescriptorUpdateTemplateCreateFlags = VkFlags;
pub type VkDescriptorUpdateTemplateType = i32;
pub type VkDevice = *mut VkDevice_T;
pub type VkDeviceAddress = u64;
pub type VkDeviceCreateFlags = VkFlags;
pub type VkDeviceMemory = *mut VkDeviceMemory_T;
pub type VkDeviceQueueCreateFlagBits = VkFlags;
pub type VkDeviceQueueCreateFlags = VkFlags;
pub type VkDeviceSize = u64;
pub type VkDriverId = i32;
pub type VkDynamicState = i32;
pub type VkEvent = *mut VkEvent_T;
pub type VkEventCreateFlagBits = VkFlags;
pub type VkEventCreateFlags = VkFlags;
pub type VkExternalFenceFeatureFlagBits = VkFlags;
pub type VkExternalFenceFeatureFlags = VkFlags;
pub type VkExternalFenceHandleTypeFlagBits = VkFlags;
pub type VkExternalFenceHandleTypeFlags = VkFlags;
pub type VkExternalMemoryFeatureFlagBits = VkFlags;
pub type VkExternalMemoryFeatureFlags = VkFlags;
pub type VkExternalMemoryHandleTypeFlagBits = VkFlags;
pub type VkExternalMemoryHandleTypeFlags = VkFlags;
pub type VkExternalSemaphoreFeatureFlagBits = VkFlags;
pub type VkExternalSemaphoreFeatureFlags = VkFlags;
pub type VkExternalSemaphoreHandleTypeFlagBits = VkFlags;
pub type VkExternalSemaphoreHandleTypeFlags = VkFlags;
pub type VkFence = *mut VkFence_T;
pub type VkFenceCreateFlagBits = VkFlags;
pub type VkFenceCreateFlags = VkFlags;
pub type VkFenceImportFlagBits = VkFlags;
pub type VkFenceImportFlags = VkFlags;
pub type VkFilter = i32;
pub type VkFlags = u32;
pub type VkFlags64 = u64;
pub type VkFormat = i32;
pub type VkFormatFeatureFlagBits = VkFlags;
pub type VkFormatFeatureFlagBits2 = VkFlags64;
pub type VkFormatFeatureFlags = VkFlags;
pub type VkFormatFeatureFlags2 = VkFlags64;
pub type VkFramebuffer = *mut VkFramebuffer_T;
pub type VkFramebufferCreateFlagBits = VkFlags;
pub type VkFramebufferCreateFlags = VkFlags;
pub type VkFrontFace = i32;
pub type VkHostImageCopyFlagBits = VkFlags;
pub type VkHostImageCopyFlags = VkFlags;
pub type VkImage = *mut VkImage_T;
pub type VkImageAspectFlagBits = VkFlags;
pub type VkImageAspectFlags = VkFlags;
pub type VkImageCreateFlagBits = VkFlags;
pub type VkImageCreateFlags = VkFlags;
pub type VkImageLayout = i32;
pub type VkImageTiling = i32;
pub type VkImageType = i32;
pub type VkImageUsageFlagBits = VkFlags;
pub type VkImageUsageFlags = VkFlags;
pub type VkImageView = *mut VkImageView_T;
pub type VkImageViewCreateFlagBits = VkFlags;
pub type VkImageViewCreateFlags = VkFlags;
pub type VkImageViewType = i32;
pub type VkIndexType = i32;
pub type VkInstance = *mut VkInstance_T;
pub type VkInstanceCreateFlagBits = VkFlags;
pub type VkInstanceCreateFlags = VkFlags;
pub type VkInternalAllocationType = i32;
pub type VkLineRasterizationMode = i32;
pub type VkLogicOp = i32;
pub type VkMemoryAllocateFlagBits = VkFlags;
pub type VkMemoryAllocateFlags = VkFlags;
pub type VkMemoryHeapFlagBits = VkFlags;
pub type VkMemoryHeapFlags = VkFlags;
pub type VkMemoryMapFlagBits = VkFlags;
pub type VkMemoryMapFlags = VkFlags;
pub type VkMemoryPropertyFlagBits = VkFlags;
pub type VkMemoryPropertyFlags = VkFlags;
pub type VkMemoryUnmapFlagBits = VkFlags;
pub type VkMemoryUnmapFlags = VkFlags;
pub type VkObjectType = i32;
pub type VkPeerMemoryFeatureFlagBits = VkFlags;
pub type VkPeerMemoryFeatureFlags = VkFlags;
pub type VkPhysicalDevice = *mut VkPhysicalDevice_T;
pub type VkPhysicalDeviceShaderDrawParameterFeatures = VkPhysicalDeviceShaderDrawParametersFeatures;
pub type VkPhysicalDeviceType = i32;
pub type VkPhysicalDeviceVariablePointerFeatures = VkPhysicalDeviceVariablePointersFeatures;
pub type VkPipeline = *mut VkPipeline_T;
pub type VkPipelineBindPoint = i32;
pub type VkPipelineCache = *mut VkPipelineCache_T;
pub type VkPipelineCacheCreateFlagBits = VkFlags;
pub type VkPipelineCacheCreateFlags = VkFlags;
pub type VkPipelineCacheHeaderVersion = i32;
pub type VkPipelineColorBlendStateCreateFlags = VkFlags;
pub type VkPipelineCreateFlagBits = VkFlags;
pub type VkPipelineCreateFlagBits2 = VkFlags64;
pub type VkPipelineCreateFlags = VkFlags;
pub type VkPipelineCreateFlags2 = VkFlags64;
pub type VkPipelineCreationFeedbackFlagBits = VkFlags;
pub type VkPipelineCreationFeedbackFlags = VkFlags;
pub type VkPipelineDepthStencilStateCreateFlags = VkFlags;
pub type VkPipelineDynamicStateCreateFlags = VkFlags;
pub type VkPipelineInputAssemblyStateCreateFlags = VkFlags;
pub type VkPipelineLayout = *mut VkPipelineLayout_T;
pub type VkPipelineLayoutCreateFlags = VkFlags;
pub type VkPipelineMultisampleStateCreateFlags = VkFlags;
pub type VkPipelineRasterizationStateCreateFlags = VkFlags;
pub type VkPipelineRobustnessBufferBehavior = i32;
pub type VkPipelineRobustnessImageBehavior = i32;
pub type VkPipelineShaderStageCreateFlagBits = VkFlags;
pub type VkPipelineShaderStageCreateFlags = VkFlags;
pub type VkPipelineStageFlagBits = VkFlags;
pub type VkPipelineStageFlagBits2 = VkFlags64;
pub type VkPipelineStageFlags = VkFlags;
pub type VkPipelineStageFlags2 = VkFlags64;
pub type VkPipelineTessellationStateCreateFlags = VkFlags;
pub type VkPipelineVertexInputStateCreateFlags = VkFlags;
pub type VkPipelineViewportStateCreateFlags = VkFlags;
pub type VkPointClippingBehavior = i32;
pub type VkPolygonMode = i32;
pub type VkPrimitiveTopology = i32;
pub type VkPrivateDataSlot = *mut VkPrivateDataSlot_T;
pub type VkPrivateDataSlotCreateFlags = VkFlags;
pub type VkQueryControlFlagBits = VkFlags;
pub type VkQueryControlFlags = VkFlags;
pub type VkQueryPipelineStatisticFlagBits = VkFlags;
pub type VkQueryPipelineStatisticFlags = VkFlags;
pub type VkQueryPool = *mut VkQueryPool_T;
pub type VkQueryPoolCreateFlagBits = VkFlags;
pub type VkQueryPoolCreateFlags = VkFlags;
pub type VkQueryResultFlagBits = VkFlags;
pub type VkQueryResultFlags = VkFlags;
pub type VkQueryType = i32;
pub type VkQueue = *mut VkQueue_T;
pub type VkQueueFlagBits = VkFlags;
pub type VkQueueFlags = VkFlags;
pub type VkQueueGlobalPriority = i32;
pub type VkRenderPass = *mut VkRenderPass_T;
pub type VkRenderPassCreateFlagBits = VkFlags;
pub type VkRenderPassCreateFlags = VkFlags;
pub type VkRenderingFlagBits = VkFlags;
pub type VkRenderingFlags = VkFlags;
pub type VkResolveModeFlagBits = VkFlags;
pub type VkResolveModeFlags = VkFlags;
pub type VkResult = i32;
pub type VkSampleCountFlagBits = VkFlags;
pub type VkSampleCountFlags = VkFlags;
pub type VkSampleMask = u32;
pub type VkSampler = *mut VkSampler_T;
pub type VkSamplerAddressMode = i32;
pub type VkSamplerCreateFlagBits = VkFlags;
pub type VkSamplerCreateFlags = VkFlags;
pub type VkSamplerMipmapMode = i32;
pub type VkSamplerReductionMode = i32;
pub type VkSamplerYcbcrConversion = *mut VkSamplerYcbcrConversion_T;
pub type VkSamplerYcbcrModelConversion = i32;
pub type VkSamplerYcbcrRange = i32;
pub type VkSemaphore = *mut VkSemaphore_T;
pub type VkSemaphoreCreateFlags = VkFlags;
pub type VkSemaphoreImportFlagBits = VkFlags;
pub type VkSemaphoreImportFlags = VkFlags;
pub type VkSemaphoreType = i32;
pub type VkSemaphoreWaitFlagBits = VkFlags;
pub type VkSemaphoreWaitFlags = VkFlags;
pub type VkShaderFloatControlsIndependence = i32;
pub type VkShaderModule = *mut VkShaderModule_T;
pub type VkShaderModuleCreateFlags = VkFlags;
pub type VkShaderStageFlagBits = VkFlags;
pub type VkShaderStageFlags = VkFlags;
pub type VkSharingMode = i32;
pub type VkSparseImageFormatFlagBits = VkFlags;
pub type VkSparseImageFormatFlags = VkFlags;
pub type VkSparseMemoryBindFlagBits = VkFlags;
pub type VkSparseMemoryBindFlags = VkFlags;
pub type VkStencilFaceFlagBits = VkFlags;
pub type VkStencilFaceFlags = VkFlags;
pub type VkStencilOp = i32;
pub type VkStructureType = i32;
pub type VkSubgroupFeatureFlagBits = VkFlags;
pub type VkSubgroupFeatureFlags = VkFlags;
pub type VkSubmitFlagBits = VkFlags;
pub type VkSubmitFlags = VkFlags;
pub type VkSubpassContents = i32;
pub type VkSubpassDescriptionFlagBits = VkFlags;
pub type VkSubpassDescriptionFlags = VkFlags;
pub type VkSystemAllocationScope = i32;
pub type VkTessellationDomainOrigin = i32;
pub type VkToolPurposeFlagBits = VkFlags;
pub type VkToolPurposeFlags = VkFlags;
pub type VkVendorId = i32;
pub type VkVertexInputRate = i32;

#[derive(Clone, Copy)]
#[repr(C)]
pub union VkClearColorValue {
    pub float32: [c_float; 4 as usize],
    pub int32: [i32; 4 as usize],
    pub uint32: [u32; 4 as usize],
}

#[derive(Clone, Copy)]
#[repr(C)]
pub union VkClearValue {
    pub color: VkClearColorValue,
    pub depthStencil: VkClearDepthStencilValue,
}
