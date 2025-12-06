use crate::code::*;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkAabbPositionsKHR {
    pub minX: f32,
    pub minY: f32,
    pub minZ: f32,
    pub maxX: f32,
    pub maxY: f32,
    pub maxZ: f32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkAccelerationStructureBuildGeometryInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub typ: VkAccelerationStructureTypeKHR,
    pub flags: VkBuildAccelerationStructureFlagsKHR,
    pub mode: VkBuildAccelerationStructureModeKHR,
    pub srcAccelerationStructure: VkAccelerationStructureKHR,
    pub dstAccelerationStructure: VkAccelerationStructureKHR,
    pub geometryCount: u32,
    pub pGeometries: *const VkAccelerationStructureGeometryKHR,
    pub ppGeometries: *const *const VkAccelerationStructureGeometryKHR,
    pub scratchData: VkDeviceOrHostAddressKHR,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkAccelerationStructureBuildRangeInfoKHR {
    pub primitiveCount: u32,
    pub primitiveOffset: u32,
    pub firstVertex: u32,
    pub transformOffset: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkAccelerationStructureBuildSizesInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub accelerationStructureSize: VkDeviceSize,
    pub updateScratchSize: VkDeviceSize,
    pub buildScratchSize: VkDeviceSize,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkAccelerationStructureCaptureDescriptorDataInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub accelerationStructure: VkAccelerationStructureKHR,
    pub accelerationStructureNV: VkAccelerationStructureNV,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkAccelerationStructureCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub createFlags: VkAccelerationStructureCreateFlagsKHR,
    pub buffer: VkBuffer,
    pub offset: VkDeviceSize,
    pub size: VkDeviceSize,
    pub typ: VkAccelerationStructureTypeKHR,
    pub deviceAddress: VkDeviceAddress,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkAccelerationStructureCreateInfoNV {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub compactedSize: VkDeviceSize,
    pub info: VkAccelerationStructureInfoNV,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkAccelerationStructureDeviceAddressInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub accelerationStructure: VkAccelerationStructureKHR,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkAccelerationStructureGeometryAabbsDataKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub data: VkDeviceOrHostAddressConstKHR,
    pub stride: VkDeviceSize,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkAccelerationStructureGeometryInstancesDataKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub arrayOfPointers: VkBool32,
    pub data: VkDeviceOrHostAddressConstKHR,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkAccelerationStructureGeometryKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub geometryType: VkGeometryTypeKHR,
    pub geometry: VkAccelerationStructureGeometryDataKHR,
    pub flags: VkGeometryFlagsKHR,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkAccelerationStructureGeometryLinearSweptSpheresDataNV {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub vertexFormat: VkFormat,
    pub vertexData: VkDeviceOrHostAddressConstKHR,
    pub vertexStride: VkDeviceSize,
    pub radiusFormat: VkFormat,
    pub radiusData: VkDeviceOrHostAddressConstKHR,
    pub radiusStride: VkDeviceSize,
    pub indexType: VkIndexType,
    pub indexData: VkDeviceOrHostAddressConstKHR,
    pub indexStride: VkDeviceSize,
    pub indexingMode: VkRayTracingLssIndexingModeNV,
    pub endCapsMode: VkRayTracingLssPrimitiveEndCapsModeNV,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkAccelerationStructureGeometryMotionTrianglesDataNV {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub vertexData: VkDeviceOrHostAddressConstKHR,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkAccelerationStructureGeometrySpheresDataNV {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub vertexFormat: VkFormat,
    pub vertexData: VkDeviceOrHostAddressConstKHR,
    pub vertexStride: VkDeviceSize,
    pub radiusFormat: VkFormat,
    pub radiusData: VkDeviceOrHostAddressConstKHR,
    pub radiusStride: VkDeviceSize,
    pub indexType: VkIndexType,
    pub indexData: VkDeviceOrHostAddressConstKHR,
    pub indexStride: VkDeviceSize,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkAccelerationStructureGeometryTrianglesDataKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub vertexFormat: VkFormat,
    pub vertexData: VkDeviceOrHostAddressConstKHR,
    pub vertexStride: VkDeviceSize,
    pub maxVertex: u32,
    pub indexType: VkIndexType,
    pub indexData: VkDeviceOrHostAddressConstKHR,
    pub transformData: VkDeviceOrHostAddressConstKHR,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkAccelerationStructureInfoNV {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub typ: VkAccelerationStructureTypeNV,
    pub flags: VkBuildAccelerationStructureFlagsNV,
    pub instanceCount: u32,
    pub geometryCount: u32,
    pub pGeometries: *const VkGeometryNV,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkAccelerationStructureInstanceKHR {
    pub transform: VkTransformMatrixKHR,
    pub instanceCustomIndex: u32,
    pub mask: u32,
    pub instanceShaderBindingTableRecordOffset: u32,
    pub flags: VkGeometryInstanceFlagsKHR,
    pub accelerationStructureReference: u64,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkAccelerationStructureMatrixMotionInstanceNV {
    pub transformT0: VkTransformMatrixKHR,
    pub transformT1: VkTransformMatrixKHR,
    pub instanceCustomIndex: u32,
    pub mask: u32,
    pub instanceShaderBindingTableRecordOffset: u32,
    pub flags: VkGeometryInstanceFlagsKHR,
    pub accelerationStructureReference: u64,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkAccelerationStructureMemoryRequirementsInfoNV {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub typ: VkAccelerationStructureMemoryRequirementsTypeNV,
    pub accelerationStructure: VkAccelerationStructureNV,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkAccelerationStructureMotionInfoNV {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub maxInstances: u32,
    pub flags: VkAccelerationStructureMotionInfoFlagsNV,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkAccelerationStructureMotionInstanceNV {
    pub typ: VkAccelerationStructureMotionInstanceTypeNV,
    pub flags: VkAccelerationStructureMotionInstanceFlagsNV,
    pub data: VkAccelerationStructureMotionInstanceDataNV,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkAccelerationStructureSRTMotionInstanceNV {
    pub transformT0: VkSRTDataNV,
    pub transformT1: VkSRTDataNV,
    pub instanceCustomIndex: u32,
    pub mask: u32,
    pub instanceShaderBindingTableRecordOffset: u32,
    pub flags: VkGeometryInstanceFlagsKHR,
    pub accelerationStructureReference: u64,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkAccelerationStructureTrianglesOpacityMicromapEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub indexType: VkIndexType,
    pub indexBuffer: VkDeviceOrHostAddressConstKHR,
    pub indexStride: VkDeviceSize,
    pub baseTriangle: u32,
    pub usageCountsCount: u32,
    pub pUsageCounts: *const VkMicromapUsageEXT,
    pub ppUsageCounts: *const *const VkMicromapUsageEXT,
    pub micromap: VkMicromapEXT,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkAccelerationStructureVersionInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub pVersionData: *const u8,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkAcquireNextImageInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub swapchain: VkSwapchainKHR,
    pub timeout: u64,
    pub semaphore: VkSemaphore,
    pub fence: VkFence,
    pub deviceMask: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkAcquireProfilingLockInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkAcquireProfilingLockFlagsKHR,
    pub timeout: u64,
}

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
pub struct VkAmigoProfilingSubmitInfoSEC {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub firstDrawTimestamp: u64,
    pub swapBufferTimestamp: u64,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkAntiLagDataAMD {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub mode: VkAntiLagModeAMD,
    pub maxFPS: u32,
    pub pPresentationInfo: *const VkAntiLagPresentationInfoAMD,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkAntiLagPresentationInfoAMD {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub stage: VkAntiLagStageAMD,
    pub frameIndex: u64,
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
pub struct VkAttachmentFeedbackLoopInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub feedbackLoopEnable: VkBool32,
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
pub struct VkAttachmentSampleCountInfoAMD {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub colorAttachmentCount: u32,
    pub pColorAttachmentSamples: *const VkSampleCountFlagBits,
    pub depthStencilAttachmentSamples: VkSampleCountFlagBits,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkAttachmentSampleLocationsEXT {
    pub attachmentIndex: u32,
    pub sampleLocationsInfo: VkSampleLocationsInfoEXT,
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
pub struct VkBeginCustomResolveInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkBindAccelerationStructureMemoryInfoNV {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub accelerationStructure: VkAccelerationStructureNV,
    pub memory: VkDeviceMemory,
    pub memoryOffset: VkDeviceSize,
    pub deviceIndexCount: u32,
    pub pDeviceIndices: *const u32,
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
pub struct VkBindDataGraphPipelineSessionMemoryInfoARM {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub session: VkDataGraphPipelineSessionARM,
    pub bindPoint: VkDataGraphPipelineSessionBindPointARM,
    pub objectIndex: u32,
    pub memory: VkDeviceMemory,
    pub memoryOffset: VkDeviceSize,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkBindDescriptorBufferEmbeddedSamplersInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub stageFlags: VkShaderStageFlags,
    pub layout: VkPipelineLayout,
    pub set: u32,
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
pub struct VkBindImageMemorySwapchainInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub swapchain: VkSwapchainKHR,
    pub imageIndex: u32,
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
pub struct VkBindIndexBufferIndirectCommandEXT {
    pub bufferAddress: VkDeviceAddress,
    pub size: u32,
    pub indexType: VkIndexType,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkBindIndexBufferIndirectCommandNV {
    pub bufferAddress: VkDeviceAddress,
    pub size: u32,
    pub indexType: VkIndexType,
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
pub struct VkBindPipelineIndirectCommandNV {
    pub pipelineAddress: VkDeviceAddress,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkBindShaderGroupIndirectCommandNV {
    pub groupIndex: u32,
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
pub struct VkBindTensorMemoryInfoARM {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub tensor: VkTensorARM,
    pub memory: VkDeviceMemory,
    pub memoryOffset: VkDeviceSize,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkBindVertexBufferIndirectCommandEXT {
    pub bufferAddress: VkDeviceAddress,
    pub size: u32,
    pub stride: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkBindVertexBufferIndirectCommandNV {
    pub bufferAddress: VkDeviceAddress,
    pub size: u32,
    pub stride: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkBindVideoSessionMemoryInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub memoryBindIndex: u32,
    pub memory: VkDeviceMemory,
    pub memoryOffset: VkDeviceSize,
    pub memorySize: VkDeviceSize,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkBlitImageCubicWeightsInfoQCOM {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub cubicWeights: VkCubicFilterWeightsQCOM,
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
pub struct VkBufferCaptureDescriptorDataInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub buffer: VkBuffer,
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
pub struct VkBufferDeviceAddressCreateInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub deviceAddress: VkDeviceAddress,
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
pub struct VkBuildPartitionedAccelerationStructureIndirectCommandNV {
    pub opType: VkPartitionedAccelerationStructureOpTypeNV,
    pub argCount: u32,
    pub argData: VkStridedDeviceAddressNV,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkBuildPartitionedAccelerationStructureInfoNV {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub input: VkPartitionedAccelerationStructureInstancesInputNV,
    pub srcAccelerationStructureData: VkDeviceAddress,
    pub dstAccelerationStructureData: VkDeviceAddress,
    pub scratchData: VkDeviceAddress,
    pub srcInfos: VkDeviceAddress,
    pub srcInfosCount: VkDeviceAddress,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkCalibratedTimestampInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub timeDomain: VkTimeDomainKHR,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkCheckpointData2NV {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub stage: VkPipelineStageFlags2,
    pub pCheckpointMarker: *mut c_void,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkCheckpointDataNV {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub stage: VkPipelineStageFlagBits,
    pub pCheckpointMarker: *mut c_void,
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
    pub depth: f32,
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
pub struct VkClusterAccelerationStructureBuildClustersBottomLevelInfoNV {
    pub clusterReferencesCount: u32,
    pub clusterReferencesStride: u32,
    pub clusterReferences: VkDeviceAddress,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkClusterAccelerationStructureBuildTriangleClusterInfoNV {
    pub clusterID: u32,
    pub clusterFlags: VkClusterAccelerationStructureClusterFlagsNV,
    pub triangleCount: u32,
    pub vertexCount: u32,
    pub positionTruncateBitCount: u32,
    pub indexType: u32,
    pub opacityMicromapIndexType: u32,
    pub baseGeometryIndexAndGeometryFlags: VkClusterAccelerationStructureGeometryIndexAndGeometryFlagsNV,
    pub indexBufferStride: u16,
    pub vertexBufferStride: u16,
    pub geometryIndexAndFlagsBufferStride: u16,
    pub opacityMicromapIndexBufferStride: u16,
    pub indexBuffer: VkDeviceAddress,
    pub vertexBuffer: VkDeviceAddress,
    pub geometryIndexAndFlagsBuffer: VkDeviceAddress,
    pub opacityMicromapArray: VkDeviceAddress,
    pub opacityMicromapIndexBuffer: VkDeviceAddress,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkClusterAccelerationStructureBuildTriangleClusterTemplateInfoNV {
    pub clusterID: u32,
    pub clusterFlags: VkClusterAccelerationStructureClusterFlagsNV,
    pub triangleCount: u32,
    pub vertexCount: u32,
    pub positionTruncateBitCount: u32,
    pub indexType: u32,
    pub opacityMicromapIndexType: u32,
    pub baseGeometryIndexAndGeometryFlags: VkClusterAccelerationStructureGeometryIndexAndGeometryFlagsNV,
    pub indexBufferStride: u16,
    pub vertexBufferStride: u16,
    pub geometryIndexAndFlagsBufferStride: u16,
    pub opacityMicromapIndexBufferStride: u16,
    pub indexBuffer: VkDeviceAddress,
    pub vertexBuffer: VkDeviceAddress,
    pub geometryIndexAndFlagsBuffer: VkDeviceAddress,
    pub opacityMicromapArray: VkDeviceAddress,
    pub opacityMicromapIndexBuffer: VkDeviceAddress,
    pub instantiationBoundingBoxLimit: VkDeviceAddress,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkClusterAccelerationStructureClustersBottomLevelInputNV {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub maxTotalClusterCount: u32,
    pub maxClusterCountPerAccelerationStructure: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkClusterAccelerationStructureCommandsInfoNV {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub input: VkClusterAccelerationStructureInputInfoNV,
    pub dstImplicitData: VkDeviceAddress,
    pub scratchData: VkDeviceAddress,
    pub dstAddressesArray: VkStridedDeviceAddressRegionKHR,
    pub dstSizesArray: VkStridedDeviceAddressRegionKHR,
    pub srcInfosArray: VkStridedDeviceAddressRegionKHR,
    pub srcInfosCount: VkDeviceAddress,
    pub addressResolutionFlags: VkClusterAccelerationStructureAddressResolutionFlagsNV,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkClusterAccelerationStructureGeometryIndexAndGeometryFlagsNV {
    pub geometryIndex: u32,
    pub reserved: u32,
    pub geometryFlags: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkClusterAccelerationStructureGetTemplateIndicesInfoNV {
    pub clusterTemplateAddress: VkDeviceAddress,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkClusterAccelerationStructureInputInfoNV {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub maxAccelerationStructureCount: u32,
    pub flags: VkBuildAccelerationStructureFlagsKHR,
    pub opType: VkClusterAccelerationStructureOpTypeNV,
    pub opMode: VkClusterAccelerationStructureOpModeNV,
    pub opInput: VkClusterAccelerationStructureOpInputNV,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkClusterAccelerationStructureInstantiateClusterInfoNV {
    pub clusterIdOffset: u32,
    pub geometryIndexOffset: u32,
    pub reserved: u32,
    pub clusterTemplateAddress: VkDeviceAddress,
    pub vertexBuffer: VkStridedDeviceAddressNV,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkClusterAccelerationStructureMoveObjectsInfoNV {
    pub srcAccelerationStructure: VkDeviceAddress,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkClusterAccelerationStructureMoveObjectsInputNV {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub typ: VkClusterAccelerationStructureTypeNV,
    pub noMoveOverlap: VkBool32,
    pub maxMovedBytes: VkDeviceSize,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkClusterAccelerationStructureTriangleClusterInputNV {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub vertexFormat: VkFormat,
    pub maxGeometryIndexValue: u32,
    pub maxClusterUniqueGeometryCount: u32,
    pub maxClusterTriangleCount: u32,
    pub maxClusterVertexCount: u32,
    pub maxTotalTriangleCount: u32,
    pub maxTotalVertexCount: u32,
    pub minPositionTruncateBitCount: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkCoarseSampleLocationNV {
    pub pixelX: u32,
    pub pixelY: u32,
    pub sample: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkCoarseSampleOrderCustomNV {
    pub shadingRate: VkShadingRatePaletteEntryNV,
    pub sampleCount: u32,
    pub sampleLocationCount: u32,
    pub pSampleLocations: *const VkCoarseSampleLocationNV,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkColorBlendAdvancedEXT {
    pub advancedBlendOp: VkBlendOp,
    pub srcPremultiplied: VkBool32,
    pub dstPremultiplied: VkBool32,
    pub blendOverlap: VkBlendOverlapEXT,
    pub clampResults: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkColorBlendEquationEXT {
    pub srcColorBlendFactor: VkBlendFactor,
    pub dstColorBlendFactor: VkBlendFactor,
    pub colorBlendOp: VkBlendOp,
    pub srcAlphaBlendFactor: VkBlendFactor,
    pub dstAlphaBlendFactor: VkBlendFactor,
    pub alphaBlendOp: VkBlendOp,
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
pub struct VkCommandBufferInheritanceConditionalRenderingInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub conditionalRenderingEnable: VkBool32,
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
pub struct VkCommandBufferInheritanceRenderPassTransformInfoQCOM {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub transform: VkSurfaceTransformFlagBitsKHR,
    pub renderArea: VkRect2D,
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
pub struct VkCommandBufferInheritanceViewportScissorInfoNV {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub viewportScissor2D: VkBool32,
    pub viewportDepthCount: u32,
    pub pViewportDepths: *const VkViewport,
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
pub struct VkComputePipelineIndirectBufferInfoNV {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub deviceAddress: VkDeviceAddress,
    pub size: VkDeviceSize,
    pub pipelineDeviceAddressCaptureReplay: VkDeviceAddress,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkConditionalRenderingBeginInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub buffer: VkBuffer,
    pub offset: VkDeviceSize,
    pub flags: VkConditionalRenderingFlagsEXT,
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
pub struct VkConvertCooperativeVectorMatrixInfoNV {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub srcSize: usize,
    pub srcData: VkDeviceOrHostAddressConstKHR,
    pub pDstSize: *mut usize,
    pub dstData: VkDeviceOrHostAddressKHR,
    pub srcComponentType: VkComponentTypeKHR,
    pub dstComponentType: VkComponentTypeKHR,
    pub numRows: u32,
    pub numColumns: u32,
    pub srcLayout: VkCooperativeVectorMatrixLayoutNV,
    pub srcStride: usize,
    pub dstLayout: VkCooperativeVectorMatrixLayoutNV,
    pub dstStride: usize,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkCooperativeMatrixFlexibleDimensionsPropertiesNV {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub MGranularity: u32,
    pub NGranularity: u32,
    pub KGranularity: u32,
    pub AType: VkComponentTypeKHR,
    pub BType: VkComponentTypeKHR,
    pub CType: VkComponentTypeKHR,
    pub ResultType: VkComponentTypeKHR,
    pub saturatingAccumulation: VkBool32,
    pub scope: VkScopeKHR,
    pub workgroupInvocations: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkCooperativeMatrixPropertiesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub MSize: u32,
    pub NSize: u32,
    pub KSize: u32,
    pub AType: VkComponentTypeKHR,
    pub BType: VkComponentTypeKHR,
    pub CType: VkComponentTypeKHR,
    pub ResultType: VkComponentTypeKHR,
    pub saturatingAccumulation: VkBool32,
    pub scope: VkScopeKHR,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkCooperativeMatrixPropertiesNV {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub MSize: u32,
    pub NSize: u32,
    pub KSize: u32,
    pub AType: VkComponentTypeNV,
    pub BType: VkComponentTypeNV,
    pub CType: VkComponentTypeNV,
    pub DType: VkComponentTypeNV,
    pub scope: VkScopeNV,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkCooperativeVectorPropertiesNV {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub inputType: VkComponentTypeKHR,
    pub inputInterpretation: VkComponentTypeKHR,
    pub matrixInterpretation: VkComponentTypeKHR,
    pub biasInterpretation: VkComponentTypeKHR,
    pub resultType: VkComponentTypeKHR,
    pub transpose: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkCopyAccelerationStructureInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub src: VkAccelerationStructureKHR,
    pub dst: VkAccelerationStructureKHR,
    pub mode: VkCopyAccelerationStructureModeKHR,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkCopyAccelerationStructureToMemoryInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub src: VkAccelerationStructureKHR,
    pub dst: VkDeviceOrHostAddressKHR,
    pub mode: VkCopyAccelerationStructureModeKHR,
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
pub struct VkCopyCommandTransformInfoQCOM {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub transform: VkSurfaceTransformFlagBitsKHR,
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
pub struct VkCopyMemoryIndirectCommandKHR {
    pub srcAddress: VkDeviceAddress,
    pub dstAddress: VkDeviceAddress,
    pub size: VkDeviceSize,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkCopyMemoryIndirectInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub srcCopyFlags: VkAddressCopyFlagsKHR,
    pub dstCopyFlags: VkAddressCopyFlagsKHR,
    pub copyCount: u32,
    pub copyAddressRange: VkStridedDeviceAddressRangeKHR,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkCopyMemoryToAccelerationStructureInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub src: VkDeviceOrHostAddressConstKHR,
    pub dst: VkAccelerationStructureKHR,
    pub mode: VkCopyAccelerationStructureModeKHR,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkCopyMemoryToImageIndirectCommandKHR {
    pub srcAddress: VkDeviceAddress,
    pub bufferRowLength: u32,
    pub bufferImageHeight: u32,
    pub imageSubresource: VkImageSubresourceLayers,
    pub imageOffset: VkOffset3D,
    pub imageExtent: VkExtent3D,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkCopyMemoryToImageIndirectInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub srcCopyFlags: VkAddressCopyFlagsKHR,
    pub copyCount: u32,
    pub copyAddressRange: VkStridedDeviceAddressRangeKHR,
    pub dstImage: VkImage,
    pub dstImageLayout: VkImageLayout,
    pub pImageSubresources: *const VkImageSubresourceLayers,
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
pub struct VkCopyMemoryToMicromapInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub src: VkDeviceOrHostAddressConstKHR,
    pub dst: VkMicromapEXT,
    pub mode: VkCopyMicromapModeEXT,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkCopyMicromapInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub src: VkMicromapEXT,
    pub dst: VkMicromapEXT,
    pub mode: VkCopyMicromapModeEXT,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkCopyMicromapToMemoryInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub src: VkMicromapEXT,
    pub dst: VkDeviceOrHostAddressKHR,
    pub mode: VkCopyMicromapModeEXT,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkCopyTensorInfoARM {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub srcTensor: VkTensorARM,
    pub dstTensor: VkTensorARM,
    pub regionCount: u32,
    pub pRegions: *const VkTensorCopyARM,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkCuFunctionCreateInfoNVX {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub module: VkCuModuleNVX,
    pub pName: *const c_char,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkCuLaunchInfoNVX {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub function: VkCuFunctionNVX,
    pub gridDimX: u32,
    pub gridDimY: u32,
    pub gridDimZ: u32,
    pub blockDimX: u32,
    pub blockDimY: u32,
    pub blockDimZ: u32,
    pub sharedMemBytes: u32,
    pub paramCount: usize,
    pub pParams: *const *const c_void,
    pub extraCount: usize,
    pub pExtras: *const *const c_void,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkCuModuleCreateInfoNVX {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub dataSize: usize,
    pub pData: *const c_void,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkCuModuleTexturingModeCreateInfoNVX {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub use64bitTexturing: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkCustomResolveCreateInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub customResolve: VkBool32,
    pub colorAttachmentCount: u32,
    pub pColorAttachmentFormats: *const VkFormat,
    pub depthAttachmentFormat: VkFormat,
    pub stencilAttachmentFormat: VkFormat,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDataGraphPipelineBuiltinModelCreateInfoQCOM {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub pOperation: *const VkPhysicalDeviceDataGraphOperationSupportARM,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDataGraphPipelineCompilerControlCreateInfoARM {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub pVendorOptions: *const c_char,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDataGraphPipelineConstantARM {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub id: u32,
    pub pConstantData: *const c_void,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDataGraphPipelineConstantTensorSemiStructuredSparsityInfoARM {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub dimension: u32,
    pub zeroCount: u32,
    pub groupSize: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDataGraphPipelineCreateInfoARM {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineCreateFlags2KHR,
    pub layout: VkPipelineLayout,
    pub resourceInfoCount: u32,
    pub pResourceInfos: *const VkDataGraphPipelineResourceInfoARM,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDataGraphPipelineDispatchInfoARM {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub flags: VkDataGraphPipelineDispatchFlagsARM,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDataGraphPipelineIdentifierCreateInfoARM {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub identifierSize: u32,
    pub pIdentifier: *const u8,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDataGraphPipelineInfoARM {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub dataGraphPipeline: VkPipeline,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDataGraphPipelinePropertyQueryResultARM {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub property: VkDataGraphPipelinePropertyARM,
    pub isText: VkBool32,
    pub dataSize: usize,
    pub pData: *mut c_void,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDataGraphPipelineResourceInfoARM {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub descriptorSet: u32,
    pub binding: u32,
    pub arrayElement: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDataGraphPipelineSessionBindPointRequirementARM {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub bindPoint: VkDataGraphPipelineSessionBindPointARM,
    pub bindPointType: VkDataGraphPipelineSessionBindPointTypeARM,
    pub numObjects: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDataGraphPipelineSessionBindPointRequirementsInfoARM {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub session: VkDataGraphPipelineSessionARM,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDataGraphPipelineSessionCreateInfoARM {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkDataGraphPipelineSessionCreateFlagsARM,
    pub dataGraphPipeline: VkPipeline,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDataGraphPipelineSessionMemoryRequirementsInfoARM {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub session: VkDataGraphPipelineSessionARM,
    pub bindPoint: VkDataGraphPipelineSessionBindPointARM,
    pub objectIndex: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDataGraphPipelineShaderModuleCreateInfoARM {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub module: VkShaderModule,
    pub pName: *const c_char,
    pub pSpecializationInfo: *const VkSpecializationInfo,
    pub constantCount: u32,
    pub pConstants: *const VkDataGraphPipelineConstantARM,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDataGraphProcessingEngineCreateInfoARM {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub processingEngineCount: u32,
    pub pProcessingEngines: *mut VkPhysicalDeviceDataGraphProcessingEngineARM,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDebugMarkerMarkerInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub pMarkerName: *const c_char,
    pub color: [f32; 4 as usize],
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDebugMarkerObjectNameInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub objectType: VkDebugReportObjectTypeEXT,
    pub object: u64,
    pub pObjectName: *const c_char,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDebugMarkerObjectTagInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub objectType: VkDebugReportObjectTypeEXT,
    pub object: u64,
    pub tagName: u64,
    pub tagSize: usize,
    pub pTag: *const c_void,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDebugReportCallbackCreateInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkDebugReportFlagsEXT,
    pub pfnCallback: PFN_vkDebugReportCallbackEXT,
    pub pUserData: *mut c_void,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDebugUtilsLabelEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub pLabelName: *const c_char,
    pub color: [f32; 4 as usize],
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDebugUtilsMessengerCallbackDataEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkDebugUtilsMessengerCallbackDataFlagsEXT,
    pub pMessageIdName: *const c_char,
    pub messageIdNumber: i32,
    pub pMessage: *const c_char,
    pub queueLabelCount: u32,
    pub pQueueLabels: *const VkDebugUtilsLabelEXT,
    pub cmdBufLabelCount: u32,
    pub pCmdBufLabels: *const VkDebugUtilsLabelEXT,
    pub objectCount: u32,
    pub pObjects: *const VkDebugUtilsObjectNameInfoEXT,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDebugUtilsMessengerCreateInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkDebugUtilsMessengerCreateFlagsEXT,
    pub messageSeverity: VkDebugUtilsMessageSeverityFlagsEXT,
    pub messageType: VkDebugUtilsMessageTypeFlagsEXT,
    pub pfnUserCallback: PFN_vkDebugUtilsMessengerCallbackEXT,
    pub pUserData: *mut c_void,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDebugUtilsObjectNameInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub objectType: VkObjectType,
    pub objectHandle: u64,
    pub pObjectName: *const c_char,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDebugUtilsObjectTagInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub objectType: VkObjectType,
    pub objectHandle: u64,
    pub tagName: u64,
    pub tagSize: usize,
    pub pTag: *const c_void,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDecompressMemoryInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub decompressionMethod: VkMemoryDecompressionMethodFlagsEXT,
    pub regionCount: u32,
    pub pRegions: *const VkDecompressMemoryRegionEXT,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDecompressMemoryRegionEXT {
    pub srcAddress: VkDeviceAddress,
    pub dstAddress: VkDeviceAddress,
    pub compressedSize: VkDeviceSize,
    pub decompressedSize: VkDeviceSize,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDecompressMemoryRegionNV {
    pub srcAddress: VkDeviceAddress,
    pub dstAddress: VkDeviceAddress,
    pub compressedSize: VkDeviceSize,
    pub decompressedSize: VkDeviceSize,
    pub decompressionMethod: VkMemoryDecompressionMethodFlagsNV,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDedicatedAllocationBufferCreateInfoNV {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub dedicatedAllocation: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDedicatedAllocationImageCreateInfoNV {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub dedicatedAllocation: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDedicatedAllocationMemoryAllocateInfoNV {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub image: VkImage,
    pub buffer: VkBuffer,
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
pub struct VkDepthBiasInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub depthBiasConstantFactor: f32,
    pub depthBiasClamp: f32,
    pub depthBiasSlopeFactor: f32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDepthBiasRepresentationInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub depthBiasRepresentation: VkDepthBiasRepresentationEXT,
    pub depthBiasExact: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDepthClampRangeEXT {
    pub minDepthClamp: f32,
    pub maxDepthClamp: f32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDescriptorAddressInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub address: VkDeviceAddress,
    pub range: VkDeviceSize,
    pub format: VkFormat,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDescriptorBufferBindingInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub address: VkDeviceAddress,
    pub usage: VkBufferUsageFlags,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDescriptorBufferBindingPushDescriptorBufferHandleEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub buffer: VkBuffer,
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
pub struct VkDescriptorGetInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub typ: VkDescriptorType,
    pub data: VkDescriptorDataEXT,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDescriptorGetTensorInfoARM {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub tensorView: VkTensorViewARM,
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
pub struct VkDescriptorSetBindingReferenceVALVE {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub descriptorSetLayout: VkDescriptorSetLayout,
    pub binding: u32,
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
pub struct VkDescriptorSetLayoutHostMappingInfoVALVE {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub descriptorOffset: usize,
    pub descriptorSize: u32,
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
pub struct VkDeviceAddressBindingCallbackDataEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub flags: VkDeviceAddressBindingFlagsEXT,
    pub baseAddress: VkDeviceAddress,
    pub size: VkDeviceSize,
    pub bindingType: VkDeviceAddressBindingTypeEXT,
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
pub struct VkDeviceDeviceMemoryReportCreateInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkDeviceMemoryReportFlagsEXT,
    pub pfnUserCallback: PFN_vkDeviceMemoryReportCallbackEXT,
    pub pUserData: *mut c_void,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDeviceDiagnosticsConfigCreateInfoNV {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkDeviceDiagnosticsConfigFlagsNV,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDeviceEventInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub deviceEvent: VkDeviceEventTypeEXT,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDeviceFaultAddressInfoEXT {
    pub addressType: VkDeviceFaultAddressTypeEXT,
    pub reportedAddress: VkDeviceAddress,
    pub addressPrecision: VkDeviceSize,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDeviceFaultCountsEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub addressInfoCount: u32,
    pub vendorInfoCount: u32,
    pub vendorBinarySize: VkDeviceSize,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDeviceFaultInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub description: [c_char; VK_MAX_DESCRIPTION_SIZE as usize],
    pub pAddressInfos: *mut VkDeviceFaultAddressInfoEXT,
    pub pVendorInfos: *mut VkDeviceFaultVendorInfoEXT,
    pub pVendorBinaryData: *mut c_void,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDeviceFaultVendorBinaryHeaderVersionOneEXT {
    pub headerSize: u32,
    pub headerVersion: VkDeviceFaultVendorBinaryHeaderVersionEXT,
    pub vendorID: u32,
    pub deviceID: u32,
    pub driverVersion: u32,
    pub pipelineCacheUUID: [u8; VK_UUID_SIZE as usize],
    pub applicationNameOffset: u32,
    pub applicationVersion: u32,
    pub engineNameOffset: u32,
    pub engineVersion: u32,
    pub apiVersion: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDeviceFaultVendorInfoEXT {
    pub description: [c_char; VK_MAX_DESCRIPTION_SIZE as usize],
    pub vendorFaultCode: u64,
    pub vendorFaultData: u64,
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
pub struct VkDeviceGroupPresentCapabilitiesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub presentMask: [u32; VK_MAX_DEVICE_GROUP_SIZE as usize],
    pub modes: VkDeviceGroupPresentModeFlagsKHR,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDeviceGroupPresentInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub swapchainCount: u32,
    pub pDeviceMasks: *const u32,
    pub mode: VkDeviceGroupPresentModeFlagBitsKHR,
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
pub struct VkDeviceGroupSwapchainCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub modes: VkDeviceGroupPresentModeFlagsKHR,
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
pub struct VkDeviceMemoryOverallocationCreateInfoAMD {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub overallocationBehavior: VkMemoryOverallocationBehaviorAMD,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDeviceMemoryReportCallbackDataEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub flags: VkDeviceMemoryReportFlagsEXT,
    pub typ: VkDeviceMemoryReportEventTypeEXT,
    pub memoryObjectId: u64,
    pub size: VkDeviceSize,
    pub objectType: VkObjectType,
    pub objectHandle: u64,
    pub heapIndex: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDevicePipelineBinaryInternalCacheControlKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub disableInternalCache: VkBool32,
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
    pub pQueuePriorities: *const f32,
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
pub struct VkDeviceQueueShaderCoreControlCreateInfoARM {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub shaderCoreCount: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDeviceTensorMemoryRequirementsARM {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub pCreateInfo: *const VkTensorCreateInfoARM,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDirectDriverLoadingInfoLUNARG {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub flags: VkDirectDriverLoadingFlagsLUNARG,
    pub pfnGetInstanceProcAddr: PFN_vkGetInstanceProcAddrLUNARG,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDirectDriverLoadingListLUNARG {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub mode: VkDirectDriverLoadingModeLUNARG,
    pub driverCount: u32,
    pub pDrivers: *const VkDirectDriverLoadingInfoLUNARG,
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
pub struct VkDispatchTileInfoQCOM {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDisplayEventInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub displayEvent: VkDisplayEventTypeEXT,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDisplayModeCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkDisplayModeCreateFlagsKHR,
    pub parameters: VkDisplayModeParametersKHR,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDisplayModeParametersKHR {
    pub visibleRegion: VkExtent2D,
    pub refreshRate: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDisplayModeProperties2KHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub displayModeProperties: VkDisplayModePropertiesKHR,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDisplayModePropertiesKHR {
    pub displayMode: VkDisplayModeKHR,
    pub parameters: VkDisplayModeParametersKHR,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDisplayModeStereoPropertiesNV {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub hdmi3DSupported: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDisplayNativeHdrSurfaceCapabilitiesAMD {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub localDimmingSupport: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDisplayPlaneCapabilities2KHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub capabilities: VkDisplayPlaneCapabilitiesKHR,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDisplayPlaneCapabilitiesKHR {
    pub supportedAlpha: VkDisplayPlaneAlphaFlagsKHR,
    pub minSrcPosition: VkOffset2D,
    pub maxSrcPosition: VkOffset2D,
    pub minSrcExtent: VkExtent2D,
    pub maxSrcExtent: VkExtent2D,
    pub minDstPosition: VkOffset2D,
    pub maxDstPosition: VkOffset2D,
    pub minDstExtent: VkExtent2D,
    pub maxDstExtent: VkExtent2D,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDisplayPlaneInfo2KHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub mode: VkDisplayModeKHR,
    pub planeIndex: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDisplayPlaneProperties2KHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub displayPlaneProperties: VkDisplayPlanePropertiesKHR,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDisplayPlanePropertiesKHR {
    pub currentDisplay: VkDisplayKHR,
    pub currentStackIndex: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDisplayPowerInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub powerState: VkDisplayPowerStateEXT,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDisplayPresentInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub srcRect: VkRect2D,
    pub dstRect: VkRect2D,
    pub persistent: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDisplayProperties2KHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub displayProperties: VkDisplayPropertiesKHR,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDisplayPropertiesKHR {
    pub display: VkDisplayKHR,
    pub displayName: *const c_char,
    pub physicalDimensions: VkExtent2D,
    pub physicalResolution: VkExtent2D,
    pub supportedTransforms: VkSurfaceTransformFlagsKHR,
    pub planeReorderPossible: VkBool32,
    pub persistentContent: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDisplaySurfaceCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkDisplaySurfaceCreateFlagsKHR,
    pub displayMode: VkDisplayModeKHR,
    pub planeIndex: u32,
    pub planeStackIndex: u32,
    pub transform: VkSurfaceTransformFlagBitsKHR,
    pub globalAlpha: f32,
    pub alphaMode: VkDisplayPlaneAlphaFlagBitsKHR,
    pub imageExtent: VkExtent2D,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDisplaySurfaceStereoCreateInfoNV {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub stereoType: VkDisplaySurfaceStereoTypeNV,
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
pub struct VkDrawIndirectCountIndirectCommandEXT {
    pub bufferAddress: VkDeviceAddress,
    pub stride: u32,
    pub commandCount: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDrawMeshTasksIndirectCommandEXT {
    pub groupCountX: u32,
    pub groupCountY: u32,
    pub groupCountZ: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDrawMeshTasksIndirectCommandNV {
    pub taskCount: u32,
    pub firstTask: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDrmFormatModifierProperties2EXT {
    pub drmFormatModifier: u64,
    pub drmFormatModifierPlaneCount: u32,
    pub drmFormatModifierTilingFeatures: VkFormatFeatureFlags2,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDrmFormatModifierPropertiesEXT {
    pub drmFormatModifier: u64,
    pub drmFormatModifierPlaneCount: u32,
    pub drmFormatModifierTilingFeatures: VkFormatFeatureFlags,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDrmFormatModifierPropertiesList2EXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub drmFormatModifierCount: u32,
    pub pDrmFormatModifierProperties: *mut VkDrmFormatModifierProperties2EXT,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDrmFormatModifierPropertiesListEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub drmFormatModifierCount: u32,
    pub pDrmFormatModifierProperties: *mut VkDrmFormatModifierPropertiesEXT,
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
pub struct VkExportMemoryAllocateInfoNV {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub handleTypes: VkExternalMemoryHandleTypeFlagsNV,
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
pub struct VkExternalComputeQueueCreateInfoNV {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub preferredQueue: VkQueue,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkExternalComputeQueueDataParamsNV {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub deviceIndex: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkExternalComputeQueueDeviceCreateInfoNV {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub reservedExternalQueues: u32,
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
pub struct VkExternalImageFormatPropertiesNV {
    pub imageFormatProperties: VkImageFormatProperties,
    pub externalMemoryFeatures: VkExternalMemoryFeatureFlagsNV,
    pub exportFromImportedHandleTypes: VkExternalMemoryHandleTypeFlagsNV,
    pub compatibleHandleTypes: VkExternalMemoryHandleTypeFlagsNV,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkExternalMemoryAcquireUnmodifiedEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub acquireUnmodifiedMemory: VkBool32,
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
pub struct VkExternalMemoryImageCreateInfoNV {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub handleTypes: VkExternalMemoryHandleTypeFlagsNV,
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
pub struct VkExternalMemoryTensorCreateInfoARM {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub handleTypes: VkExternalMemoryHandleTypeFlags,
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
pub struct VkExternalTensorPropertiesARM {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub externalMemoryProperties: VkExternalMemoryProperties,
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
pub struct VkFenceGetFdInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub fence: VkFence,
    pub handleType: VkExternalFenceHandleTypeFlagBits,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkFilterCubicImageViewImageFormatPropertiesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub filterCubic: VkBool32,
    pub filterCubicMinmax: VkBool32,
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
pub struct VkFragmentShadingRateAttachmentInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub pFragmentShadingRateAttachment: *const VkAttachmentReference2,
    pub shadingRateAttachmentTexelSize: VkExtent2D,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkFrameBoundaryEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkFrameBoundaryFlagsEXT,
    pub frameID: u64,
    pub imageCount: u32,
    pub pImages: *const VkImage,
    pub bufferCount: u32,
    pub pBuffers: *const VkBuffer,
    pub tagName: u64,
    pub tagSize: usize,
    pub pTag: *const c_void,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkFrameBoundaryTensorsARM {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub tensorCount: u32,
    pub pTensors: *const VkTensorARM,
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
pub struct VkFramebufferMixedSamplesCombinationNV {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub coverageReductionMode: VkCoverageReductionModeNV,
    pub rasterizationSamples: VkSampleCountFlagBits,
    pub depthStencilSamples: VkSampleCountFlags,
    pub colorSamples: VkSampleCountFlags,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkGeneratedCommandsInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub shaderStages: VkShaderStageFlags,
    pub indirectExecutionSet: VkIndirectExecutionSetEXT,
    pub indirectCommandsLayout: VkIndirectCommandsLayoutEXT,
    pub indirectAddress: VkDeviceAddress,
    pub indirectAddressSize: VkDeviceSize,
    pub preprocessAddress: VkDeviceAddress,
    pub preprocessSize: VkDeviceSize,
    pub maxSequenceCount: u32,
    pub sequenceCountAddress: VkDeviceAddress,
    pub maxDrawCount: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkGeneratedCommandsInfoNV {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub pipelineBindPoint: VkPipelineBindPoint,
    pub pipeline: VkPipeline,
    pub indirectCommandsLayout: VkIndirectCommandsLayoutNV,
    pub streamCount: u32,
    pub pStreams: *const VkIndirectCommandsStreamNV,
    pub sequencesCount: u32,
    pub preprocessBuffer: VkBuffer,
    pub preprocessOffset: VkDeviceSize,
    pub preprocessSize: VkDeviceSize,
    pub sequencesCountBuffer: VkBuffer,
    pub sequencesCountOffset: VkDeviceSize,
    pub sequencesIndexBuffer: VkBuffer,
    pub sequencesIndexOffset: VkDeviceSize,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkGeneratedCommandsMemoryRequirementsInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub indirectExecutionSet: VkIndirectExecutionSetEXT,
    pub indirectCommandsLayout: VkIndirectCommandsLayoutEXT,
    pub maxSequenceCount: u32,
    pub maxDrawCount: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkGeneratedCommandsMemoryRequirementsInfoNV {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub pipelineBindPoint: VkPipelineBindPoint,
    pub pipeline: VkPipeline,
    pub indirectCommandsLayout: VkIndirectCommandsLayoutNV,
    pub maxSequencesCount: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkGeneratedCommandsPipelineInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub pipeline: VkPipeline,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkGeneratedCommandsShaderInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub shaderCount: u32,
    pub pShaders: *const VkShaderEXT,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkGeometryAABBNV {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub aabbData: VkBuffer,
    pub numAABBs: u32,
    pub stride: u32,
    pub offset: VkDeviceSize,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkGeometryDataNV {
    pub triangles: VkGeometryTrianglesNV,
    pub aabbs: VkGeometryAABBNV,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkGeometryNV {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub geometryType: VkGeometryTypeKHR,
    pub geometry: VkGeometryDataNV,
    pub flags: VkGeometryFlagsKHR,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkGeometryTrianglesNV {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub vertexData: VkBuffer,
    pub vertexOffset: VkDeviceSize,
    pub vertexCount: u32,
    pub vertexStride: VkDeviceSize,
    pub vertexFormat: VkFormat,
    pub indexData: VkBuffer,
    pub indexOffset: VkDeviceSize,
    pub indexCount: u32,
    pub indexType: VkIndexType,
    pub transformData: VkBuffer,
    pub transformOffset: VkDeviceSize,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkGetLatencyMarkerInfoNV {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub timingCount: u32,
    pub pTimings: *mut VkLatencyTimingsFrameReportNV,
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
pub struct VkGraphicsPipelineLibraryCreateInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkGraphicsPipelineLibraryFlagsEXT,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkGraphicsPipelineShaderGroupsCreateInfoNV {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub groupCount: u32,
    pub pGroups: *const VkGraphicsShaderGroupCreateInfoNV,
    pub pipelineCount: u32,
    pub pPipelines: *const VkPipeline,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkGraphicsShaderGroupCreateInfoNV {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub stageCount: u32,
    pub pStages: *const VkPipelineShaderStageCreateInfo,
    pub pVertexInputState: *const VkPipelineVertexInputStateCreateInfo,
    pub pTessellationState: *const VkPipelineTessellationStateCreateInfo,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkHdrMetadataEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub displayPrimaryRed: VkXYColorEXT,
    pub displayPrimaryGreen: VkXYColorEXT,
    pub displayPrimaryBlue: VkXYColorEXT,
    pub whitePoint: VkXYColorEXT,
    pub maxLuminance: f32,
    pub minLuminance: f32,
    pub maxContentLightLevel: f32,
    pub maxFrameAverageLightLevel: f32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkHdrVividDynamicMetadataHUAWEI {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub dynamicMetadataSize: usize,
    pub pDynamicMetadata: *const c_void,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkHeadlessSurfaceCreateInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkHeadlessSurfaceCreateFlagsEXT,
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
pub struct VkImageAlignmentControlCreateInfoMESA {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub maximumRequestedAlignment: u32,
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
pub struct VkImageCaptureDescriptorDataInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub image: VkImage,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImageCompressionControlEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkImageCompressionFlagsEXT,
    pub compressionControlPlaneCount: u32,
    pub pFixedRateFlags: *mut VkImageCompressionFixedRateFlagsEXT,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImageCompressionPropertiesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub imageCompressionFlags: VkImageCompressionFlagsEXT,
    pub imageCompressionFixedRateFlags: VkImageCompressionFixedRateFlagsEXT,
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
pub struct VkImageDrmFormatModifierExplicitCreateInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub drmFormatModifier: u64,
    pub drmFormatModifierPlaneCount: u32,
    pub pPlaneLayouts: *const VkSubresourceLayout,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImageDrmFormatModifierListCreateInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub drmFormatModifierCount: u32,
    pub pDrmFormatModifiers: *const u64,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImageDrmFormatModifierPropertiesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub drmFormatModifier: u64,
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
pub struct VkImageSwapchainCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub swapchain: VkSwapchainKHR,
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
pub struct VkImageViewASTCDecodeModeEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub decodeMode: VkFormat,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImageViewAddressPropertiesNVX {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub deviceAddress: VkDeviceAddress,
    pub size: VkDeviceSize,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImageViewCaptureDescriptorDataInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub imageView: VkImageView,
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
pub struct VkImageViewHandleInfoNVX {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub imageView: VkImageView,
    pub descriptorType: VkDescriptorType,
    pub sampler: VkSampler,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImageViewMinLodCreateInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub minLod: f32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImageViewSampleWeightCreateInfoQCOM {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub filterCenter: VkOffset2D,
    pub filterSize: VkExtent2D,
    pub numPhases: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImageViewSlicedCreateInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub sliceOffset: u32,
    pub sliceCount: u32,
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
pub struct VkImportFenceFdInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub fence: VkFence,
    pub flags: VkFenceImportFlags,
    pub handleType: VkExternalFenceHandleTypeFlagBits,
    pub fd: c_int,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImportMemoryFdInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub handleType: VkExternalMemoryHandleTypeFlagBits,
    pub fd: c_int,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImportMemoryHostPointerInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub handleType: VkExternalMemoryHandleTypeFlagBits,
    pub pHostPointer: *mut c_void,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImportSemaphoreFdInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub semaphore: VkSemaphore,
    pub flags: VkSemaphoreImportFlags,
    pub handleType: VkExternalSemaphoreHandleTypeFlagBits,
    pub fd: c_int,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkIndirectCommandsExecutionSetTokenEXT {
    pub typ: VkIndirectExecutionSetInfoTypeEXT,
    pub shaderStages: VkShaderStageFlags,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkIndirectCommandsIndexBufferTokenEXT {
    pub mode: VkIndirectCommandsInputModeFlagBitsEXT,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkIndirectCommandsLayoutCreateInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkIndirectCommandsLayoutUsageFlagsEXT,
    pub shaderStages: VkShaderStageFlags,
    pub indirectStride: u32,
    pub pipelineLayout: VkPipelineLayout,
    pub tokenCount: u32,
    pub pTokens: *const VkIndirectCommandsLayoutTokenEXT,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkIndirectCommandsLayoutCreateInfoNV {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkIndirectCommandsLayoutUsageFlagsNV,
    pub pipelineBindPoint: VkPipelineBindPoint,
    pub tokenCount: u32,
    pub pTokens: *const VkIndirectCommandsLayoutTokenNV,
    pub streamCount: u32,
    pub pStreamStrides: *const u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkIndirectCommandsLayoutTokenEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub typ: VkIndirectCommandsTokenTypeEXT,
    pub data: VkIndirectCommandsTokenDataEXT,
    pub offset: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkIndirectCommandsLayoutTokenNV {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub tokenType: VkIndirectCommandsTokenTypeNV,
    pub stream: u32,
    pub offset: u32,
    pub vertexBindingUnit: u32,
    pub vertexDynamicStride: VkBool32,
    pub pushconstantPipelineLayout: VkPipelineLayout,
    pub pushconstantShaderStageFlags: VkShaderStageFlags,
    pub pushconstantOffset: u32,
    pub pushconstantSize: u32,
    pub indirectStateFlags: VkIndirectStateFlagsNV,
    pub indexTypeCount: u32,
    pub pIndexTypes: *const VkIndexType,
    pub pIndexTypeValues: *const u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkIndirectCommandsPushConstantTokenEXT {
    pub updateRange: VkPushConstantRange,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkIndirectCommandsStreamNV {
    pub buffer: VkBuffer,
    pub offset: VkDeviceSize,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkIndirectCommandsVertexBufferTokenEXT {
    pub vertexBindingUnit: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkIndirectExecutionSetCreateInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub typ: VkIndirectExecutionSetInfoTypeEXT,
    pub info: VkIndirectExecutionSetInfoEXT,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkIndirectExecutionSetPipelineInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub initialPipeline: VkPipeline,
    pub maxPipelineCount: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkIndirectExecutionSetShaderInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub shaderCount: u32,
    pub pInitialShaders: *const VkShaderEXT,
    pub pSetLayoutInfos: *const VkIndirectExecutionSetShaderLayoutInfoEXT,
    pub maxShaderCount: u32,
    pub pushConstantRangeCount: u32,
    pub pPushConstantRanges: *const VkPushConstantRange,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkIndirectExecutionSetShaderLayoutInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub setLayoutCount: u32,
    pub pSetLayouts: *const VkDescriptorSetLayout,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkInitializePerformanceApiInfoINTEL {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub pUserData: *mut c_void,
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
pub struct VkLatencySleepInfoNV {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub signalSemaphore: VkSemaphore,
    pub value: u64,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkLatencySleepModeInfoNV {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub lowLatencyMode: VkBool32,
    pub lowLatencyBoost: VkBool32,
    pub minimumIntervalUs: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkLatencySubmissionPresentIdNV {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub presentID: u64,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkLatencySurfaceCapabilitiesNV {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub presentModeCount: u32,
    pub pPresentModes: *mut VkPresentModeKHR,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkLatencyTimingsFrameReportNV {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub presentID: u64,
    pub inputSampleTimeUs: u64,
    pub simStartTimeUs: u64,
    pub simEndTimeUs: u64,
    pub renderSubmitStartTimeUs: u64,
    pub renderSubmitEndTimeUs: u64,
    pub presentStartTimeUs: u64,
    pub presentEndTimeUs: u64,
    pub driverStartTimeUs: u64,
    pub driverEndTimeUs: u64,
    pub osRenderQueueStartTimeUs: u64,
    pub osRenderQueueEndTimeUs: u64,
    pub gpuRenderStartTimeUs: u64,
    pub gpuRenderEndTimeUs: u64,
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
pub struct VkLayerSettingEXT {
    pub pLayerName: *const c_char,
    pub pSettingName: *const c_char,
    pub typ: VkLayerSettingTypeEXT,
    pub valueCount: u32,
    pub pValues: *const c_void,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkLayerSettingsCreateInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub settingCount: u32,
    pub pSettings: *const VkLayerSettingEXT,
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
pub struct VkMemoryBarrierAccessFlags3KHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub srcAccessMask3: VkAccessFlags3KHR,
    pub dstAccessMask3: VkAccessFlags3KHR,
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
pub struct VkMemoryDedicatedAllocateInfoTensorARM {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub tensor: VkTensorARM,
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
pub struct VkMemoryFdPropertiesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub memoryTypeBits: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkMemoryGetFdInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub memory: VkDeviceMemory,
    pub handleType: VkExternalMemoryHandleTypeFlagBits,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkMemoryGetRemoteAddressInfoNV {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub memory: VkDeviceMemory,
    pub handleType: VkExternalMemoryHandleTypeFlagBits,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkMemoryHeap {
    pub size: VkDeviceSize,
    pub flags: VkMemoryHeapFlags,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkMemoryHostPointerPropertiesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub memoryTypeBits: u32,
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
pub struct VkMemoryMapPlacedInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub pPlacedAddress: *mut c_void,
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
pub struct VkMemoryPriorityAllocateInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub priority: f32,
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
pub struct VkMicromapBuildInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub typ: VkMicromapTypeEXT,
    pub flags: VkBuildMicromapFlagsEXT,
    pub mode: VkBuildMicromapModeEXT,
    pub dstMicromap: VkMicromapEXT,
    pub usageCountsCount: u32,
    pub pUsageCounts: *const VkMicromapUsageEXT,
    pub ppUsageCounts: *const *const VkMicromapUsageEXT,
    pub data: VkDeviceOrHostAddressConstKHR,
    pub scratchData: VkDeviceOrHostAddressKHR,
    pub triangleArray: VkDeviceOrHostAddressConstKHR,
    pub triangleArrayStride: VkDeviceSize,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkMicromapBuildSizesInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub micromapSize: VkDeviceSize,
    pub buildScratchSize: VkDeviceSize,
    pub discardable: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkMicromapCreateInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub createFlags: VkMicromapCreateFlagsEXT,
    pub buffer: VkBuffer,
    pub offset: VkDeviceSize,
    pub size: VkDeviceSize,
    pub typ: VkMicromapTypeEXT,
    pub deviceAddress: VkDeviceAddress,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkMicromapTriangleEXT {
    pub dataOffset: u32,
    pub subdivisionLevel: u16,
    pub format: u16,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkMicromapUsageEXT {
    pub count: u32,
    pub subdivisionLevel: u32,
    pub format: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkMicromapVersionInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub pVersionData: *const u8,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkMultiDrawIndexedInfoEXT {
    pub firstIndex: u32,
    pub indexCount: u32,
    pub vertexOffset: i32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkMultiDrawInfoEXT {
    pub firstVertex: u32,
    pub vertexCount: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkMultisamplePropertiesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub maxSampleLocationGridSize: VkExtent2D,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkMultisampledRenderToSingleSampledInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub multisampledRenderToSingleSampledEnable: VkBool32,
    pub rasterizationSamples: VkSampleCountFlagBits,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkMultiviewPerViewAttributesInfoNVX {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub perViewAttributes: VkBool32,
    pub perViewAttributesPositionXOnly: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkMultiviewPerViewRenderAreasRenderPassBeginInfoQCOM {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub perViewRenderAreaCount: u32,
    pub pPerViewRenderAreas: *const VkRect2D,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkMutableDescriptorTypeCreateInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub mutableDescriptorTypeListCount: u32,
    pub pMutableDescriptorTypeLists: *const VkMutableDescriptorTypeListEXT,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkMutableDescriptorTypeListEXT {
    pub descriptorTypeCount: u32,
    pub pDescriptorTypes: *const VkDescriptorType,
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
pub struct VkOpaqueCaptureDescriptorDataCreateInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub opaqueCaptureDescriptorData: *const c_void,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkOpticalFlowExecuteInfoNV {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub flags: VkOpticalFlowExecuteFlagsNV,
    pub regionCount: u32,
    pub pRegions: *const VkRect2D,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkOpticalFlowImageFormatInfoNV {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub usage: VkOpticalFlowUsageFlagsNV,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkOpticalFlowImageFormatPropertiesNV {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub format: VkFormat,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkOpticalFlowSessionCreateInfoNV {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub width: u32,
    pub height: u32,
    pub imageFormat: VkFormat,
    pub flowVectorFormat: VkFormat,
    pub costFormat: VkFormat,
    pub outputGridSize: VkOpticalFlowGridSizeFlagsNV,
    pub hintGridSize: VkOpticalFlowGridSizeFlagsNV,
    pub performanceLevel: VkOpticalFlowPerformanceLevelNV,
    pub flags: VkOpticalFlowSessionCreateFlagsNV,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkOpticalFlowSessionCreatePrivateDataInfoNV {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub id: u32,
    pub size: u32,
    pub pPrivateData: *const c_void,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkOutOfBandQueueTypeInfoNV {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub queueType: VkOutOfBandQueueTypeNV,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPartitionedAccelerationStructureFlagsNV {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub enablePartitionTranslation: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPartitionedAccelerationStructureInstancesInputNV {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub flags: VkBuildAccelerationStructureFlagsKHR,
    pub instanceCount: u32,
    pub maxInstancePerPartitionCount: u32,
    pub partitionCount: u32,
    pub maxInstanceInGlobalPartitionCount: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPartitionedAccelerationStructureUpdateInstanceDataNV {
    pub instanceIndex: u32,
    pub instanceContributionToHitGroupIndex: u32,
    pub accelerationStructure: VkDeviceAddress,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPartitionedAccelerationStructureWriteInstanceDataNV {
    pub transform: VkTransformMatrixKHR,
    pub explicitAABB: [f32; 6 as usize],
    pub instanceID: u32,
    pub instanceMask: u32,
    pub instanceContributionToHitGroupIndex: u32,
    pub instanceFlags: VkPartitionedAccelerationStructureInstanceFlagsNV,
    pub instanceIndex: u32,
    pub partitionIndex: u32,
    pub accelerationStructure: VkDeviceAddress,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPartitionedAccelerationStructureWritePartitionTranslationDataNV {
    pub partitionIndex: u32,
    pub partitionTranslation: [f32; 3 as usize],
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPastPresentationTimingEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub presentId: u64,
    pub targetTime: u64,
    pub presentStageCount: u32,
    pub pPresentStages: *mut VkPresentStageTimeEXT,
    pub timeDomain: VkTimeDomainKHR,
    pub timeDomainId: u64,
    pub reportComplete: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPastPresentationTimingGOOGLE {
    pub presentID: u32,
    pub desiredPresentTime: u64,
    pub actualPresentTime: u64,
    pub earliestPresentTime: u64,
    pub presentMargin: u64,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPastPresentationTimingInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPastPresentationTimingFlagsEXT,
    pub swapchain: VkSwapchainKHR,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPastPresentationTimingPropertiesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub timingPropertiesCounter: u64,
    pub timeDomainsCounter: u64,
    pub presentationTimingCount: u32,
    pub pPresentationTimings: *mut VkPastPresentationTimingEXT,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPerTileBeginInfoQCOM {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPerTileEndInfoQCOM {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPerformanceConfigurationAcquireInfoINTEL {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub typ: VkPerformanceConfigurationTypeINTEL,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPerformanceCounterARM {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub counterID: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPerformanceCounterDescriptionARM {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub flags: VkPerformanceCounterDescriptionFlagsARM,
    pub name: [c_char; VK_MAX_DESCRIPTION_SIZE as usize],
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPerformanceCounterDescriptionKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub flags: VkPerformanceCounterDescriptionFlagsKHR,
    pub name: [c_char; VK_MAX_DESCRIPTION_SIZE as usize],
    pub category: [c_char; VK_MAX_DESCRIPTION_SIZE as usize],
    pub description: [c_char; VK_MAX_DESCRIPTION_SIZE as usize],
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPerformanceCounterKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub unit: VkPerformanceCounterUnitKHR,
    pub scope: VkPerformanceCounterScopeKHR,
    pub storage: VkPerformanceCounterStorageKHR,
    pub uuid: [u8; VK_UUID_SIZE as usize],
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPerformanceMarkerInfoINTEL {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub marker: u64,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPerformanceOverrideInfoINTEL {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub typ: VkPerformanceOverrideTypeINTEL,
    pub enable: VkBool32,
    pub parameter: u64,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPerformanceQuerySubmitInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub counterPassIndex: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPerformanceStreamMarkerInfoINTEL {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub marker: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPerformanceValueINTEL {
    pub typ: VkPerformanceValueTypeINTEL,
    pub data: VkPerformanceValueDataINTEL,
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
pub struct VkPhysicalDevice4444FormatsFeaturesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub formatA4R4G4B4: VkBool32,
    pub formatA4B4G4R4: VkBool32,
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
pub struct VkPhysicalDeviceASTCDecodeFeaturesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub decodeModeSharedExponent: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceAccelerationStructureFeaturesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub accelerationStructure: VkBool32,
    pub accelerationStructureCaptureReplay: VkBool32,
    pub accelerationStructureIndirectBuild: VkBool32,
    pub accelerationStructureHostCommands: VkBool32,
    pub descriptorBindingAccelerationStructureUpdateAfterBind: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceAccelerationStructurePropertiesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub maxGeometryCount: u64,
    pub maxInstanceCount: u64,
    pub maxPrimitiveCount: u64,
    pub maxPerStageDescriptorAccelerationStructures: u32,
    pub maxPerStageDescriptorUpdateAfterBindAccelerationStructures: u32,
    pub maxDescriptorSetAccelerationStructures: u32,
    pub maxDescriptorSetUpdateAfterBindAccelerationStructures: u32,
    pub minAccelerationStructureScratchOffsetAlignment: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceAddressBindingReportFeaturesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub reportAddressBinding: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceAmigoProfilingFeaturesSEC {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub amigoProfiling: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceAntiLagFeaturesAMD {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub antiLag: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceAttachmentFeedbackLoopDynamicStateFeaturesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub attachmentFeedbackLoopDynamicState: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceAttachmentFeedbackLoopLayoutFeaturesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub attachmentFeedbackLoopLayout: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceBlendOperationAdvancedFeaturesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub advancedBlendCoherentOperations: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceBlendOperationAdvancedPropertiesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub advancedBlendMaxColorAttachments: u32,
    pub advancedBlendIndependentBlend: VkBool32,
    pub advancedBlendNonPremultipliedSrcColor: VkBool32,
    pub advancedBlendNonPremultipliedDstColor: VkBool32,
    pub advancedBlendCorrelatedOverlap: VkBool32,
    pub advancedBlendAllOperations: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceBorderColorSwizzleFeaturesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub borderColorSwizzle: VkBool32,
    pub borderColorSwizzleFromImage: VkBool32,
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
pub struct VkPhysicalDeviceBufferDeviceAddressFeaturesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub bufferDeviceAddress: VkBool32,
    pub bufferDeviceAddressCaptureReplay: VkBool32,
    pub bufferDeviceAddressMultiDevice: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceClusterAccelerationStructureFeaturesNV {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub clusterAccelerationStructure: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceClusterAccelerationStructurePropertiesNV {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub maxVerticesPerCluster: u32,
    pub maxTrianglesPerCluster: u32,
    pub clusterScratchByteAlignment: u32,
    pub clusterByteAlignment: u32,
    pub clusterTemplateByteAlignment: u32,
    pub clusterBottomLevelByteAlignment: u32,
    pub clusterTemplateBoundsByteAlignment: u32,
    pub maxClusterGeometryIndex: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceClusterCullingShaderFeaturesHUAWEI {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub clustercullingShader: VkBool32,
    pub multiviewClusterCullingShader: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceClusterCullingShaderPropertiesHUAWEI {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub maxWorkGroupCount: [u32; 3 as usize],
    pub maxWorkGroupSize: [u32; 3 as usize],
    pub maxOutputClusterCount: u32,
    pub indirectBufferOffsetAlignment: VkDeviceSize,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceClusterCullingShaderVrsFeaturesHUAWEI {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub clusterShadingRate: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceCoherentMemoryFeaturesAMD {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub deviceCoherentMemory: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceColorWriteEnableFeaturesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub colorWriteEnable: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceCommandBufferInheritanceFeaturesNV {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub commandBufferInheritance: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceComputeShaderDerivativesFeaturesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub computeDerivativeGroupQuads: VkBool32,
    pub computeDerivativeGroupLinear: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceComputeShaderDerivativesPropertiesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub meshAndTaskShaderDerivatives: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceConditionalRenderingFeaturesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub conditionalRendering: VkBool32,
    pub inheritedConditionalRendering: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceConservativeRasterizationPropertiesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub primitiveOverestimationSize: f32,
    pub maxExtraPrimitiveOverestimationSize: f32,
    pub extraPrimitiveOverestimationSizeGranularity: f32,
    pub primitiveUnderestimation: VkBool32,
    pub conservativePointAndLineRasterization: VkBool32,
    pub degenerateTrianglesRasterized: VkBool32,
    pub degenerateLinesRasterized: VkBool32,
    pub fullyCoveredFragmentShaderInputVariable: VkBool32,
    pub conservativeRasterizationPostDepthCoverage: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceCooperativeMatrix2FeaturesNV {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub cooperativeMatrixWorkgroupScope: VkBool32,
    pub cooperativeMatrixFlexibleDimensions: VkBool32,
    pub cooperativeMatrixReductions: VkBool32,
    pub cooperativeMatrixConversions: VkBool32,
    pub cooperativeMatrixPerElementOperations: VkBool32,
    pub cooperativeMatrixTensorAddressing: VkBool32,
    pub cooperativeMatrixBlockLoads: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceCooperativeMatrix2PropertiesNV {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub cooperativeMatrixWorkgroupScopeMaxWorkgroupSize: u32,
    pub cooperativeMatrixFlexibleDimensionsMaxDimension: u32,
    pub cooperativeMatrixWorkgroupScopeReservedSharedMemory: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceCooperativeMatrixFeaturesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub cooperativeMatrix: VkBool32,
    pub cooperativeMatrixRobustBufferAccess: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceCooperativeMatrixFeaturesNV {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub cooperativeMatrix: VkBool32,
    pub cooperativeMatrixRobustBufferAccess: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceCooperativeMatrixPropertiesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub cooperativeMatrixSupportedStages: VkShaderStageFlags,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceCooperativeMatrixPropertiesNV {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub cooperativeMatrixSupportedStages: VkShaderStageFlags,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceCooperativeVectorFeaturesNV {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub cooperativeVector: VkBool32,
    pub cooperativeVectorTraining: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceCooperativeVectorPropertiesNV {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub cooperativeVectorSupportedStages: VkShaderStageFlags,
    pub cooperativeVectorTrainingFloat16Accumulation: VkBool32,
    pub cooperativeVectorTrainingFloat32Accumulation: VkBool32,
    pub maxCooperativeVectorComponents: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceCopyMemoryIndirectFeaturesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub indirectMemoryCopy: VkBool32,
    pub indirectMemoryToImageCopy: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceCopyMemoryIndirectFeaturesNV {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub indirectCopy: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceCopyMemoryIndirectPropertiesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub supportedQueues: VkQueueFlags,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceCornerSampledImageFeaturesNV {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub cornerSampledImage: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceCoverageReductionModeFeaturesNV {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub coverageReductionMode: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceCubicClampFeaturesQCOM {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub cubicRangeClamp: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceCubicWeightsFeaturesQCOM {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub selectableCubicWeights: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceCustomBorderColorFeaturesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub customBorderColors: VkBool32,
    pub customBorderColorWithoutFormat: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceCustomBorderColorPropertiesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub maxCustomBorderColorSamplers: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceCustomResolveFeaturesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub customResolve: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceDataGraphFeaturesARM {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub dataGraph: VkBool32,
    pub dataGraphUpdateAfterBind: VkBool32,
    pub dataGraphSpecializationConstants: VkBool32,
    pub dataGraphDescriptorBuffer: VkBool32,
    pub dataGraphShaderModule: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceDataGraphModelFeaturesQCOM {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub dataGraphModel: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceDataGraphOperationSupportARM {
    pub operationType: VkPhysicalDeviceDataGraphOperationTypeARM,
    pub name: [c_char; VK_MAX_PHYSICAL_DEVICE_DATA_GRAPH_OPERATION_SET_NAME_SIZE_ARM as usize],
    pub version: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceDataGraphProcessingEngineARM {
    pub typ: VkPhysicalDeviceDataGraphProcessingEngineTypeARM,
    pub isForeign: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub dedicatedAllocationImageAliasing: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceDepthBiasControlFeaturesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub depthBiasControl: VkBool32,
    pub leastRepresentableValueForceUnormRepresentation: VkBool32,
    pub floatRepresentation: VkBool32,
    pub depthBiasExact: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceDepthClampControlFeaturesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub depthClampControl: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceDepthClampZeroOneFeaturesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub depthClampZeroOne: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceDepthClipControlFeaturesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub depthClipControl: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceDepthClipEnableFeaturesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub depthClipEnable: VkBool32,
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
pub struct VkPhysicalDeviceDescriptorBufferDensityMapPropertiesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub combinedImageSamplerDensityMapDescriptorSize: usize,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceDescriptorBufferFeaturesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub descriptorBuffer: VkBool32,
    pub descriptorBufferCaptureReplay: VkBool32,
    pub descriptorBufferImageLayoutIgnored: VkBool32,
    pub descriptorBufferPushDescriptors: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceDescriptorBufferPropertiesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub combinedImageSamplerDescriptorSingleArray: VkBool32,
    pub bufferlessPushDescriptors: VkBool32,
    pub allowSamplerImageViewPostSubmitCreation: VkBool32,
    pub descriptorBufferOffsetAlignment: VkDeviceSize,
    pub maxDescriptorBufferBindings: u32,
    pub maxResourceDescriptorBufferBindings: u32,
    pub maxSamplerDescriptorBufferBindings: u32,
    pub maxEmbeddedImmutableSamplerBindings: u32,
    pub maxEmbeddedImmutableSamplers: u32,
    pub bufferCaptureReplayDescriptorDataSize: usize,
    pub imageCaptureReplayDescriptorDataSize: usize,
    pub imageViewCaptureReplayDescriptorDataSize: usize,
    pub samplerCaptureReplayDescriptorDataSize: usize,
    pub accelerationStructureCaptureReplayDescriptorDataSize: usize,
    pub samplerDescriptorSize: usize,
    pub combinedImageSamplerDescriptorSize: usize,
    pub sampledImageDescriptorSize: usize,
    pub storageImageDescriptorSize: usize,
    pub uniformTexelBufferDescriptorSize: usize,
    pub robustUniformTexelBufferDescriptorSize: usize,
    pub storageTexelBufferDescriptorSize: usize,
    pub robustStorageTexelBufferDescriptorSize: usize,
    pub uniformBufferDescriptorSize: usize,
    pub robustUniformBufferDescriptorSize: usize,
    pub storageBufferDescriptorSize: usize,
    pub robustStorageBufferDescriptorSize: usize,
    pub inputAttachmentDescriptorSize: usize,
    pub accelerationStructureDescriptorSize: usize,
    pub maxSamplerDescriptorBufferRange: VkDeviceSize,
    pub maxResourceDescriptorBufferRange: VkDeviceSize,
    pub samplerDescriptorBufferAddressSpaceSize: VkDeviceSize,
    pub resourceDescriptorBufferAddressSpaceSize: VkDeviceSize,
    pub descriptorBufferAddressSpaceSize: VkDeviceSize,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceDescriptorBufferTensorFeaturesARM {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub descriptorBufferTensorDescriptors: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceDescriptorBufferTensorPropertiesARM {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub tensorCaptureReplayDescriptorDataSize: usize,
    pub tensorViewCaptureReplayDescriptorDataSize: usize,
    pub tensorDescriptorSize: usize,
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
pub struct VkPhysicalDeviceDescriptorPoolOverallocationFeaturesNV {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub descriptorPoolOverallocation: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceDescriptorSetHostMappingFeaturesVALVE {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub descriptorSetHostMapping: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceDeviceGeneratedCommandsComputeFeaturesNV {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub deviceGeneratedCompute: VkBool32,
    pub deviceGeneratedComputePipelines: VkBool32,
    pub deviceGeneratedComputeCaptureReplay: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceDeviceGeneratedCommandsFeaturesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub deviceGeneratedCommands: VkBool32,
    pub dynamicGeneratedPipelineLayout: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceDeviceGeneratedCommandsFeaturesNV {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub deviceGeneratedCommands: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceDeviceGeneratedCommandsPropertiesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub maxIndirectPipelineCount: u32,
    pub maxIndirectShaderObjectCount: u32,
    pub maxIndirectSequenceCount: u32,
    pub maxIndirectCommandsTokenCount: u32,
    pub maxIndirectCommandsTokenOffset: u32,
    pub maxIndirectCommandsIndirectStride: u32,
    pub supportedIndirectCommandsInputModes: VkIndirectCommandsInputModeFlagsEXT,
    pub supportedIndirectCommandsShaderStages: VkShaderStageFlags,
    pub supportedIndirectCommandsShaderStagesPipelineBinding: VkShaderStageFlags,
    pub supportedIndirectCommandsShaderStagesShaderBinding: VkShaderStageFlags,
    pub deviceGeneratedCommandsTransformFeedback: VkBool32,
    pub deviceGeneratedCommandsMultiDrawIndirectCount: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceDeviceGeneratedCommandsPropertiesNV {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub maxGraphicsShaderGroupCount: u32,
    pub maxIndirectSequenceCount: u32,
    pub maxIndirectCommandsTokenCount: u32,
    pub maxIndirectCommandsStreamCount: u32,
    pub maxIndirectCommandsTokenOffset: u32,
    pub maxIndirectCommandsStreamStride: u32,
    pub minSequencesCountBufferOffsetAlignment: u32,
    pub minSequencesIndexBufferOffsetAlignment: u32,
    pub minIndirectCommandsBufferOffsetAlignment: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceDeviceMemoryReportFeaturesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub deviceMemoryReport: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceDiagnosticsConfigFeaturesNV {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub diagnosticsConfig: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceDiscardRectanglePropertiesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub maxDiscardRectangles: u32,
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
pub struct VkPhysicalDeviceDrmPropertiesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub hasPrimary: VkBool32,
    pub hasRender: VkBool32,
    pub primaryMajor: i64,
    pub primaryMinor: i64,
    pub renderMajor: i64,
    pub renderMinor: i64,
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
pub struct VkPhysicalDeviceDynamicRenderingUnusedAttachmentsFeaturesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub dynamicRenderingUnusedAttachments: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceExclusiveScissorFeaturesNV {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub exclusiveScissor: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceExtendedDynamicState2FeaturesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub extendedDynamicState2: VkBool32,
    pub extendedDynamicState2LogicOp: VkBool32,
    pub extendedDynamicState2PatchControlPoints: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceExtendedDynamicState3FeaturesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub extendedDynamicState3TessellationDomainOrigin: VkBool32,
    pub extendedDynamicState3DepthClampEnable: VkBool32,
    pub extendedDynamicState3PolygonMode: VkBool32,
    pub extendedDynamicState3RasterizationSamples: VkBool32,
    pub extendedDynamicState3SampleMask: VkBool32,
    pub extendedDynamicState3AlphaToCoverageEnable: VkBool32,
    pub extendedDynamicState3AlphaToOneEnable: VkBool32,
    pub extendedDynamicState3LogicOpEnable: VkBool32,
    pub extendedDynamicState3ColorBlendEnable: VkBool32,
    pub extendedDynamicState3ColorBlendEquation: VkBool32,
    pub extendedDynamicState3ColorWriteMask: VkBool32,
    pub extendedDynamicState3RasterizationStream: VkBool32,
    pub extendedDynamicState3ConservativeRasterizationMode: VkBool32,
    pub extendedDynamicState3ExtraPrimitiveOverestimationSize: VkBool32,
    pub extendedDynamicState3DepthClipEnable: VkBool32,
    pub extendedDynamicState3SampleLocationsEnable: VkBool32,
    pub extendedDynamicState3ColorBlendAdvanced: VkBool32,
    pub extendedDynamicState3ProvokingVertexMode: VkBool32,
    pub extendedDynamicState3LineRasterizationMode: VkBool32,
    pub extendedDynamicState3LineStippleEnable: VkBool32,
    pub extendedDynamicState3DepthClipNegativeOneToOne: VkBool32,
    pub extendedDynamicState3ViewportWScalingEnable: VkBool32,
    pub extendedDynamicState3ViewportSwizzle: VkBool32,
    pub extendedDynamicState3CoverageToColorEnable: VkBool32,
    pub extendedDynamicState3CoverageToColorLocation: VkBool32,
    pub extendedDynamicState3CoverageModulationMode: VkBool32,
    pub extendedDynamicState3CoverageModulationTableEnable: VkBool32,
    pub extendedDynamicState3CoverageModulationTable: VkBool32,
    pub extendedDynamicState3CoverageReductionMode: VkBool32,
    pub extendedDynamicState3RepresentativeFragmentTestEnable: VkBool32,
    pub extendedDynamicState3ShadingRateImageEnable: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceExtendedDynamicState3PropertiesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub dynamicPrimitiveTopologyUnrestricted: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceExtendedDynamicStateFeaturesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub extendedDynamicState: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceExtendedSparseAddressSpaceFeaturesNV {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub extendedSparseAddressSpace: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceExtendedSparseAddressSpacePropertiesNV {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub extendedSparseAddressSpaceSize: VkDeviceSize,
    pub extendedSparseImageUsageFlags: VkImageUsageFlags,
    pub extendedSparseBufferUsageFlags: VkBufferUsageFlags,
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
pub struct VkPhysicalDeviceExternalComputeQueuePropertiesNV {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub externalDataSize: u32,
    pub maxExternalQueues: u32,
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
pub struct VkPhysicalDeviceExternalMemoryHostPropertiesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub minImportedHostPointerAlignment: VkDeviceSize,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceExternalMemoryRDMAFeaturesNV {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub externalMemoryRDMA: VkBool32,
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
pub struct VkPhysicalDeviceExternalTensorInfoARM {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkTensorCreateFlagsARM,
    pub pDescription: *const VkTensorDescriptionARM,
    pub handleType: VkExternalMemoryHandleTypeFlagBits,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceFaultFeaturesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub deviceFault: VkBool32,
    pub deviceFaultVendorBinary: VkBool32,
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
pub struct VkPhysicalDeviceFormatPackFeaturesARM {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub formatPack: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceFragmentDensityMap2FeaturesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub fragmentDensityMapDeferred: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceFragmentDensityMap2PropertiesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub subsampledLoads: VkBool32,
    pub subsampledCoarseReconstructionEarlyAccess: VkBool32,
    pub maxSubsampledArrayLayers: u32,
    pub maxDescriptorSetSubsampledSamplers: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceFragmentDensityMapFeaturesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub fragmentDensityMap: VkBool32,
    pub fragmentDensityMapDynamic: VkBool32,
    pub fragmentDensityMapNonSubsampledImages: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceFragmentDensityMapLayeredFeaturesVALVE {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub fragmentDensityMapLayered: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceFragmentDensityMapLayeredPropertiesVALVE {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub maxFragmentDensityMapLayers: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceFragmentDensityMapOffsetFeaturesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub fragmentDensityMapOffset: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceFragmentDensityMapOffsetPropertiesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub fragmentDensityOffsetGranularity: VkExtent2D,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceFragmentDensityMapPropertiesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub minFragmentDensityTexelSize: VkExtent2D,
    pub maxFragmentDensityTexelSize: VkExtent2D,
    pub fragmentDensityInvocations: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceFragmentShaderBarycentricFeaturesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub fragmentShaderBarycentric: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceFragmentShaderBarycentricPropertiesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub triStripVertexOrderIndependentOfProvokingVertex: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceFragmentShaderInterlockFeaturesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub fragmentShaderSampleInterlock: VkBool32,
    pub fragmentShaderPixelInterlock: VkBool32,
    pub fragmentShaderShadingRateInterlock: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceFragmentShadingRateEnumsFeaturesNV {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub fragmentShadingRateEnums: VkBool32,
    pub supersampleFragmentShadingRates: VkBool32,
    pub noInvocationFragmentShadingRates: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceFragmentShadingRateEnumsPropertiesNV {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub maxFragmentShadingRateInvocationCount: VkSampleCountFlagBits,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceFragmentShadingRateFeaturesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub pipelineFragmentShadingRate: VkBool32,
    pub primitiveFragmentShadingRate: VkBool32,
    pub attachmentFragmentShadingRate: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceFragmentShadingRateKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub sampleCounts: VkSampleCountFlags,
    pub fragmentSize: VkExtent2D,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceFragmentShadingRatePropertiesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub minFragmentShadingRateAttachmentTexelSize: VkExtent2D,
    pub maxFragmentShadingRateAttachmentTexelSize: VkExtent2D,
    pub maxFragmentShadingRateAttachmentTexelSizeAspectRatio: u32,
    pub primitiveFragmentShadingRateWithMultipleViewports: VkBool32,
    pub layeredShadingRateAttachments: VkBool32,
    pub fragmentShadingRateNonTrivialCombinerOps: VkBool32,
    pub maxFragmentSize: VkExtent2D,
    pub maxFragmentSizeAspectRatio: u32,
    pub maxFragmentShadingRateCoverageSamples: u32,
    pub maxFragmentShadingRateRasterizationSamples: VkSampleCountFlagBits,
    pub fragmentShadingRateWithShaderDepthStencilWrites: VkBool32,
    pub fragmentShadingRateWithSampleMask: VkBool32,
    pub fragmentShadingRateWithShaderSampleMask: VkBool32,
    pub fragmentShadingRateWithConservativeRasterization: VkBool32,
    pub fragmentShadingRateWithFragmentShaderInterlock: VkBool32,
    pub fragmentShadingRateWithCustomSampleLocations: VkBool32,
    pub fragmentShadingRateStrictMultiplyCombiner: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceFrameBoundaryFeaturesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub frameBoundary: VkBool32,
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
pub struct VkPhysicalDeviceGraphicsPipelineLibraryFeaturesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub graphicsPipelineLibrary: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceGraphicsPipelineLibraryPropertiesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub graphicsPipelineLibraryFastLinking: VkBool32,
    pub graphicsPipelineLibraryIndependentInterpolationDecoration: VkBool32,
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
pub struct VkPhysicalDeviceHdrVividFeaturesHUAWEI {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub hdrVivid: VkBool32,
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
pub struct VkPhysicalDeviceImage2DViewOf3DFeaturesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub image2DViewOf3D: VkBool32,
    pub sampler2DViewOf3D: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceImageAlignmentControlFeaturesMESA {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub imageAlignmentControl: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceImageAlignmentControlPropertiesMESA {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub supportedImageAlignmentMask: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceImageCompressionControlFeaturesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub imageCompressionControl: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceImageCompressionControlSwapchainFeaturesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub imageCompressionControlSwapchain: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceImageDrmFormatModifierInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub drmFormatModifier: u64,
    pub sharingMode: VkSharingMode,
    pub queueFamilyIndexCount: u32,
    pub pQueueFamilyIndices: *const u32,
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
pub struct VkPhysicalDeviceImageProcessing2FeaturesQCOM {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub textureBlockMatch2: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceImageProcessing2PropertiesQCOM {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub maxBlockMatchWindow: VkExtent2D,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceImageProcessingFeaturesQCOM {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub textureSampleWeighted: VkBool32,
    pub textureBoxFilter: VkBool32,
    pub textureBlockMatch: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceImageProcessingPropertiesQCOM {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub maxWeightFilterPhases: u32,
    pub maxWeightFilterDimension: VkExtent2D,
    pub maxBlockMatchRegion: VkExtent2D,
    pub maxBoxFilterBlockSize: VkExtent2D,
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
pub struct VkPhysicalDeviceImageSlicedViewOf3DFeaturesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub imageSlicedViewOf3D: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceImageViewImageFormatInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub imageViewType: VkImageViewType,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceImageViewMinLodFeaturesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub minLod: VkBool32,
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
pub struct VkPhysicalDeviceInheritedViewportScissorFeaturesNV {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub inheritedViewportScissor2D: VkBool32,
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
pub struct VkPhysicalDeviceInvocationMaskFeaturesHUAWEI {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub invocationMask: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceLayeredApiPropertiesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub vendorID: u32,
    pub deviceID: u32,
    pub layeredAPI: VkPhysicalDeviceLayeredApiKHR,
    pub deviceName: [c_char; VK_MAX_PHYSICAL_DEVICE_NAME_SIZE as usize],
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceLayeredApiPropertiesListKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub layeredApiCount: u32,
    pub pLayeredApis: *mut VkPhysicalDeviceLayeredApiPropertiesKHR,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceLayeredApiVulkanPropertiesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub properties: VkPhysicalDeviceProperties2,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceLayeredDriverPropertiesMSFT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub underlyingAPI: VkLayeredDriverUnderlyingApiMSFT,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceLegacyDitheringFeaturesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub legacyDithering: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceLegacyVertexAttributesFeaturesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub legacyVertexAttributes: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceLegacyVertexAttributesPropertiesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub nativeUnalignedPerformance: VkBool32,
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
    pub maxSamplerLodBias: f32,
    pub maxSamplerAnisotropy: f32,
    pub maxViewports: u32,
    pub maxViewportDimensions: [u32; 2 as usize],
    pub viewportBoundsRange: [f32; 2 as usize],
    pub viewportSubPixelBits: u32,
    pub minMemoryMapAlignment: usize,
    pub minTexelBufferOffsetAlignment: VkDeviceSize,
    pub minUniformBufferOffsetAlignment: VkDeviceSize,
    pub minStorageBufferOffsetAlignment: VkDeviceSize,
    pub minTexelOffset: i32,
    pub maxTexelOffset: u32,
    pub minTexelGatherOffset: i32,
    pub maxTexelGatherOffset: u32,
    pub minInterpolationOffset: f32,
    pub maxInterpolationOffset: f32,
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
    pub timestampPeriod: f32,
    pub maxClipDistances: u32,
    pub maxCullDistances: u32,
    pub maxCombinedClipAndCullDistances: u32,
    pub discreteQueuePriorities: u32,
    pub pointSizeRange: [f32; 2 as usize],
    pub lineWidthRange: [f32; 2 as usize],
    pub pointSizeGranularity: f32,
    pub lineWidthGranularity: f32,
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
pub struct VkPhysicalDeviceLinearColorAttachmentFeaturesNV {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub linearColorAttachment: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceMaintenance10FeaturesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub maintenance10: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceMaintenance10PropertiesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub rgba4OpaqueBlackSwizzled: VkBool32,
    pub resolveSrgbFormatAppliesTransferFunction: VkBool32,
    pub resolveSrgbFormatSupportsTransferFunctionControl: VkBool32,
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
pub struct VkPhysicalDeviceMaintenance7FeaturesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub maintenance7: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceMaintenance7PropertiesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub robustFragmentShadingRateAttachmentAccess: VkBool32,
    pub separateDepthStencilAttachmentAccess: VkBool32,
    pub maxDescriptorSetTotalUniformBuffersDynamic: u32,
    pub maxDescriptorSetTotalStorageBuffersDynamic: u32,
    pub maxDescriptorSetTotalBuffersDynamic: u32,
    pub maxDescriptorSetUpdateAfterBindTotalUniformBuffersDynamic: u32,
    pub maxDescriptorSetUpdateAfterBindTotalStorageBuffersDynamic: u32,
    pub maxDescriptorSetUpdateAfterBindTotalBuffersDynamic: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceMaintenance8FeaturesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub maintenance8: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceMaintenance9FeaturesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub maintenance9: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceMaintenance9PropertiesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub image2DViewOf3DSparse: VkBool32,
    pub defaultVertexAttributeValue: VkDefaultVertexAttributeValueKHR,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceMapMemoryPlacedFeaturesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub memoryMapPlaced: VkBool32,
    pub memoryMapRangePlaced: VkBool32,
    pub memoryUnmapReserve: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceMapMemoryPlacedPropertiesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub minPlacedMemoryMapAlignment: VkDeviceSize,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceMemoryBudgetPropertiesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub heapBudget: [VkDeviceSize; VK_MAX_MEMORY_HEAPS as usize],
    pub heapUsage: [VkDeviceSize; VK_MAX_MEMORY_HEAPS as usize],
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceMemoryDecompressionFeaturesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub memoryDecompression: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceMemoryDecompressionPropertiesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub decompressionMethods: VkMemoryDecompressionMethodFlagsEXT,
    pub maxDecompressionIndirectCount: u64,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceMemoryPriorityFeaturesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub memoryPriority: VkBool32,
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
pub struct VkPhysicalDeviceMeshShaderFeaturesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub taskShader: VkBool32,
    pub meshShader: VkBool32,
    pub multiviewMeshShader: VkBool32,
    pub primitiveFragmentShadingRateMeshShader: VkBool32,
    pub meshShaderQueries: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceMeshShaderFeaturesNV {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub taskShader: VkBool32,
    pub meshShader: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceMeshShaderPropertiesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub maxTaskWorkGroupTotalCount: u32,
    pub maxTaskWorkGroupCount: [u32; 3 as usize],
    pub maxTaskWorkGroupInvocations: u32,
    pub maxTaskWorkGroupSize: [u32; 3 as usize],
    pub maxTaskPayloadSize: u32,
    pub maxTaskSharedMemorySize: u32,
    pub maxTaskPayloadAndSharedMemorySize: u32,
    pub maxMeshWorkGroupTotalCount: u32,
    pub maxMeshWorkGroupCount: [u32; 3 as usize],
    pub maxMeshWorkGroupInvocations: u32,
    pub maxMeshWorkGroupSize: [u32; 3 as usize],
    pub maxMeshSharedMemorySize: u32,
    pub maxMeshPayloadAndSharedMemorySize: u32,
    pub maxMeshOutputMemorySize: u32,
    pub maxMeshPayloadAndOutputMemorySize: u32,
    pub maxMeshOutputComponents: u32,
    pub maxMeshOutputVertices: u32,
    pub maxMeshOutputPrimitives: u32,
    pub maxMeshOutputLayers: u32,
    pub maxMeshMultiviewViewCount: u32,
    pub meshOutputPerVertexGranularity: u32,
    pub meshOutputPerPrimitiveGranularity: u32,
    pub maxPreferredTaskWorkGroupInvocations: u32,
    pub maxPreferredMeshWorkGroupInvocations: u32,
    pub prefersLocalInvocationVertexOutput: VkBool32,
    pub prefersLocalInvocationPrimitiveOutput: VkBool32,
    pub prefersCompactVertexOutput: VkBool32,
    pub prefersCompactPrimitiveOutput: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceMeshShaderPropertiesNV {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub maxDrawMeshTasksCount: u32,
    pub maxTaskWorkGroupInvocations: u32,
    pub maxTaskWorkGroupSize: [u32; 3 as usize],
    pub maxTaskTotalMemorySize: u32,
    pub maxTaskOutputCount: u32,
    pub maxMeshWorkGroupInvocations: u32,
    pub maxMeshWorkGroupSize: [u32; 3 as usize],
    pub maxMeshTotalMemorySize: u32,
    pub maxMeshOutputVertices: u32,
    pub maxMeshOutputPrimitives: u32,
    pub maxMeshMultiviewViewCount: u32,
    pub meshOutputPerVertexGranularity: u32,
    pub meshOutputPerPrimitiveGranularity: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceMultiDrawFeaturesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub multiDraw: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceMultiDrawPropertiesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub maxMultiDrawCount: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceMultisampledRenderToSingleSampledFeaturesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub multisampledRenderToSingleSampled: VkBool32,
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
pub struct VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub perViewPositionAllComponents: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceMultiviewPerViewRenderAreasFeaturesQCOM {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub multiviewPerViewRenderAreas: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceMultiviewPerViewViewportsFeaturesQCOM {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub multiviewPerViewViewports: VkBool32,
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
pub struct VkPhysicalDeviceMutableDescriptorTypeFeaturesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub mutableDescriptorType: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceNestedCommandBufferFeaturesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub nestedCommandBuffer: VkBool32,
    pub nestedCommandBufferRendering: VkBool32,
    pub nestedCommandBufferSimultaneousUse: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceNestedCommandBufferPropertiesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub maxCommandBufferNestingLevel: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceNonSeamlessCubeMapFeaturesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub nonSeamlessCubeMap: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceOpacityMicromapFeaturesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub micromap: VkBool32,
    pub micromapCaptureReplay: VkBool32,
    pub micromapHostCommands: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceOpacityMicromapPropertiesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub maxOpacity2StateSubdivisionLevel: u32,
    pub maxOpacity4StateSubdivisionLevel: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceOpticalFlowFeaturesNV {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub opticalFlow: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceOpticalFlowPropertiesNV {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub supportedOutputGridSizes: VkOpticalFlowGridSizeFlagsNV,
    pub supportedHintGridSizes: VkOpticalFlowGridSizeFlagsNV,
    pub hintSupported: VkBool32,
    pub costSupported: VkBool32,
    pub bidirectionalFlowSupported: VkBool32,
    pub globalFlowSupported: VkBool32,
    pub minWidth: u32,
    pub minHeight: u32,
    pub maxWidth: u32,
    pub maxHeight: u32,
    pub maxNumRegionsOfInterest: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDevicePCIBusInfoPropertiesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub pciDomain: u32,
    pub pciBus: u32,
    pub pciDevice: u32,
    pub pciFunction: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDevicePageableDeviceLocalMemoryFeaturesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub pageableDeviceLocalMemory: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDevicePartitionedAccelerationStructureFeaturesNV {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub partitionedAccelerationStructure: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDevicePartitionedAccelerationStructurePropertiesNV {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub maxPartitionCount: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDevicePerStageDescriptorSetFeaturesNV {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub perStageDescriptorSet: VkBool32,
    pub dynamicPipelineLayout: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDevicePerformanceCountersByRegionFeaturesARM {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub performanceCountersByRegion: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDevicePerformanceCountersByRegionPropertiesARM {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub maxPerRegionPerformanceCounters: u32,
    pub performanceCounterRegionSize: VkExtent2D,
    pub rowStrideAlignment: u32,
    pub regionAlignment: u32,
    pub identityTransformOrder: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDevicePerformanceQueryFeaturesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub performanceCounterQueryPools: VkBool32,
    pub performanceCounterMultipleQueryPools: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDevicePerformanceQueryPropertiesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub allowCommandBufferQueryCopies: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDevicePipelineBinaryFeaturesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub pipelineBinaries: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDevicePipelineBinaryPropertiesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub pipelineBinaryInternalCache: VkBool32,
    pub pipelineBinaryInternalCacheControl: VkBool32,
    pub pipelineBinaryPrefersInternalCache: VkBool32,
    pub pipelineBinaryPrecompiledInternalCache: VkBool32,
    pub pipelineBinaryCompressedData: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDevicePipelineCacheIncrementalModeFeaturesSEC {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub pipelineCacheIncrementalMode: VkBool32,
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
pub struct VkPhysicalDevicePipelineExecutablePropertiesFeaturesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub pipelineExecutableInfo: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDevicePipelineLibraryGroupHandlesFeaturesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub pipelineLibraryGroupHandles: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDevicePipelineOpacityMicromapFeaturesARM {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub pipelineOpacityMicromap: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDevicePipelinePropertiesFeaturesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub pipelinePropertiesIdentifier: VkBool32,
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
pub struct VkPhysicalDevicePresentBarrierFeaturesNV {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub presentBarrier: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDevicePresentId2FeaturesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub presentId2: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDevicePresentIdFeaturesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub presentId: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDevicePresentModeFifoLatestReadyFeaturesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub presentModeFifoLatestReady: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDevicePresentTimingFeaturesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub presentTiming: VkBool32,
    pub presentAtAbsoluteTime: VkBool32,
    pub presentAtRelativeTime: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDevicePresentWait2FeaturesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub presentWait2: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDevicePresentWaitFeaturesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub presentWait: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDevicePrimitiveTopologyListRestartFeaturesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub primitiveTopologyListRestart: VkBool32,
    pub primitiveTopologyPatchListRestart: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDevicePrimitivesGeneratedQueryFeaturesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub primitivesGeneratedQuery: VkBool32,
    pub primitivesGeneratedQueryWithRasterizerDiscard: VkBool32,
    pub primitivesGeneratedQueryWithNonZeroStreams: VkBool32,
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
pub struct VkPhysicalDeviceProvokingVertexFeaturesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub provokingVertexLast: VkBool32,
    pub transformFeedbackPreservesProvokingVertex: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceProvokingVertexPropertiesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub provokingVertexModePerPipeline: VkBool32,
    pub transformFeedbackPreservesTriangleFanProvokingVertex: VkBool32,
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
pub struct VkPhysicalDeviceQueueFamilyDataGraphProcessingEngineInfoARM {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub queueFamilyIndex: u32,
    pub engineType: VkPhysicalDeviceDataGraphProcessingEngineTypeARM,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceRGBA10X6FormatsFeaturesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub formatRgba10x6WithoutYCbCrSampler: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceRasterizationOrderAttachmentAccessFeaturesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub rasterizationOrderColorAttachmentAccess: VkBool32,
    pub rasterizationOrderDepthAttachmentAccess: VkBool32,
    pub rasterizationOrderStencilAttachmentAccess: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceRawAccessChainsFeaturesNV {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub shaderRawAccessChains: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceRayQueryFeaturesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub rayQuery: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceRayTracingInvocationReorderFeaturesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub rayTracingInvocationReorder: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceRayTracingInvocationReorderFeaturesNV {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub rayTracingInvocationReorder: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceRayTracingInvocationReorderPropertiesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub rayTracingInvocationReorderReorderingHint: VkRayTracingInvocationReorderModeEXT,
    pub maxShaderBindingTableRecordIndex: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceRayTracingInvocationReorderPropertiesNV {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub rayTracingInvocationReorderReorderingHint: VkRayTracingInvocationReorderModeEXT,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceRayTracingLinearSweptSpheresFeaturesNV {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub spheres: VkBool32,
    pub linearSweptSpheres: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceRayTracingMaintenance1FeaturesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub rayTracingMaintenance1: VkBool32,
    pub rayTracingPipelineTraceRaysIndirect2: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceRayTracingMotionBlurFeaturesNV {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub rayTracingMotionBlur: VkBool32,
    pub rayTracingMotionBlurPipelineTraceRaysIndirect: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceRayTracingPipelineFeaturesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub rayTracingPipeline: VkBool32,
    pub rayTracingPipelineShaderGroupHandleCaptureReplay: VkBool32,
    pub rayTracingPipelineShaderGroupHandleCaptureReplayMixed: VkBool32,
    pub rayTracingPipelineTraceRaysIndirect: VkBool32,
    pub rayTraversalPrimitiveCulling: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceRayTracingPipelinePropertiesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub shaderGroupHandleSize: u32,
    pub maxRayRecursionDepth: u32,
    pub maxShaderGroupStride: u32,
    pub shaderGroupBaseAlignment: u32,
    pub shaderGroupHandleCaptureReplaySize: u32,
    pub maxRayDispatchInvocationCount: u32,
    pub shaderGroupHandleAlignment: u32,
    pub maxRayHitAttributeSize: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceRayTracingPositionFetchFeaturesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub rayTracingPositionFetch: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceRayTracingPropertiesNV {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub shaderGroupHandleSize: u32,
    pub maxRecursionDepth: u32,
    pub maxShaderGroupStride: u32,
    pub shaderGroupBaseAlignment: u32,
    pub maxGeometryCount: u64,
    pub maxInstanceCount: u64,
    pub maxTriangleCount: u64,
    pub maxDescriptorSetAccelerationStructures: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceRayTracingValidationFeaturesNV {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub rayTracingValidation: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceRelaxedLineRasterizationFeaturesIMG {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub relaxedLineRasterization: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceRenderPassStripedFeaturesARM {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub renderPassStriped: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceRenderPassStripedPropertiesARM {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub renderPassStripeGranularity: VkExtent2D,
    pub maxRenderPassStripes: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceRepresentativeFragmentTestFeaturesNV {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub representativeFragmentTest: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceRobustness2FeaturesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub robustBufferAccess2: VkBool32,
    pub robustImageAccess2: VkBool32,
    pub nullDescriptor: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceRobustness2PropertiesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub robustStorageBufferAccessSizeAlignment: VkDeviceSize,
    pub robustUniformBufferAccessSizeAlignment: VkDeviceSize,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceSampleLocationsPropertiesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub sampleLocationSampleCounts: VkSampleCountFlags,
    pub maxSampleLocationGridSize: VkExtent2D,
    pub sampleLocationCoordinateRange: [f32; 2 as usize],
    pub sampleLocationSubPixelBits: u32,
    pub variableSampleLocations: VkBool32,
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
pub struct VkPhysicalDeviceSchedulingControlsFeaturesARM {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub schedulingControls: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceSchedulingControlsPropertiesARM {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub schedulingControlsFlags: VkPhysicalDeviceSchedulingControlsFlagsARM,
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
pub struct VkPhysicalDeviceShader64BitIndexingFeaturesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub shader64BitIndexing: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceShaderAtomicFloat16VectorFeaturesNV {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub shaderFloat16VectorAtomics: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceShaderAtomicFloat2FeaturesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub shaderBufferFloat16Atomics: VkBool32,
    pub shaderBufferFloat16AtomicAdd: VkBool32,
    pub shaderBufferFloat16AtomicMinMax: VkBool32,
    pub shaderBufferFloat32AtomicMinMax: VkBool32,
    pub shaderBufferFloat64AtomicMinMax: VkBool32,
    pub shaderSharedFloat16Atomics: VkBool32,
    pub shaderSharedFloat16AtomicAdd: VkBool32,
    pub shaderSharedFloat16AtomicMinMax: VkBool32,
    pub shaderSharedFloat32AtomicMinMax: VkBool32,
    pub shaderSharedFloat64AtomicMinMax: VkBool32,
    pub shaderImageFloat32AtomicMinMax: VkBool32,
    pub sparseImageFloat32AtomicMinMax: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceShaderAtomicFloatFeaturesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub shaderBufferFloat32Atomics: VkBool32,
    pub shaderBufferFloat32AtomicAdd: VkBool32,
    pub shaderBufferFloat64Atomics: VkBool32,
    pub shaderBufferFloat64AtomicAdd: VkBool32,
    pub shaderSharedFloat32Atomics: VkBool32,
    pub shaderSharedFloat32AtomicAdd: VkBool32,
    pub shaderSharedFloat64Atomics: VkBool32,
    pub shaderSharedFloat64AtomicAdd: VkBool32,
    pub shaderImageFloat32Atomics: VkBool32,
    pub shaderImageFloat32AtomicAdd: VkBool32,
    pub sparseImageFloat32Atomics: VkBool32,
    pub sparseImageFloat32AtomicAdd: VkBool32,
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
pub struct VkPhysicalDeviceShaderBfloat16FeaturesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub shaderBFloat16Type: VkBool32,
    pub shaderBFloat16DotProduct: VkBool32,
    pub shaderBFloat16CooperativeMatrix: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceShaderClockFeaturesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub shaderSubgroupClock: VkBool32,
    pub shaderDeviceClock: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceShaderCoreBuiltinsFeaturesARM {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub shaderCoreBuiltins: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceShaderCoreBuiltinsPropertiesARM {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub shaderCoreMask: u64,
    pub shaderCoreCount: u32,
    pub shaderWarpsPerCore: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceShaderCoreProperties2AMD {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub shaderCoreFeatures: VkShaderCorePropertiesFlagsAMD,
    pub activeComputeUnitCount: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceShaderCorePropertiesAMD {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub shaderEngineCount: u32,
    pub shaderArraysPerEngineCount: u32,
    pub computeUnitsPerShaderArray: u32,
    pub simdPerComputeUnit: u32,
    pub wavefrontsPerSimd: u32,
    pub wavefrontSize: u32,
    pub sgprsPerSimd: u32,
    pub minSgprAllocation: u32,
    pub maxSgprAllocation: u32,
    pub sgprAllocationGranularity: u32,
    pub vgprsPerSimd: u32,
    pub minVgprAllocation: u32,
    pub maxVgprAllocation: u32,
    pub vgprAllocationGranularity: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceShaderCorePropertiesARM {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub pixelRate: u32,
    pub texelRate: u32,
    pub fmaRate: u32,
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
pub struct VkPhysicalDeviceShaderEarlyAndLateFragmentTestsFeaturesAMD {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub shaderEarlyAndLateFragmentTests: VkBool32,
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
pub struct VkPhysicalDeviceShaderFloat8FeaturesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub shaderFloat8: VkBool32,
    pub shaderFloat8CooperativeMatrix: VkBool32,
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
pub struct VkPhysicalDeviceShaderFmaFeaturesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub shaderFmaFloat16: VkBool32,
    pub shaderFmaFloat32: VkBool32,
    pub shaderFmaFloat64: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceShaderImageAtomicInt64FeaturesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub shaderImageInt64Atomics: VkBool32,
    pub sparseImageInt64Atomics: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceShaderImageFootprintFeaturesNV {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub imageFootprint: VkBool32,
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
pub struct VkPhysicalDeviceShaderIntegerFunctions2FeaturesINTEL {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub shaderIntegerFunctions2: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceShaderMaximalReconvergenceFeaturesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub shaderMaximalReconvergence: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceShaderModuleIdentifierFeaturesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub shaderModuleIdentifier: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceShaderModuleIdentifierPropertiesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub shaderModuleIdentifierAlgorithmUUID: [u8; VK_UUID_SIZE as usize],
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceShaderObjectFeaturesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub shaderObject: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceShaderObjectPropertiesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub shaderBinaryUUID: [u8; VK_UUID_SIZE as usize],
    pub shaderBinaryVersion: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceShaderQuadControlFeaturesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub shaderQuadControl: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceShaderRelaxedExtendedInstructionFeaturesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub shaderRelaxedExtendedInstruction: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceShaderReplicatedCompositesFeaturesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub shaderReplicatedComposites: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceShaderSMBuiltinsFeaturesNV {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub shaderSMBuiltins: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceShaderSMBuiltinsPropertiesNV {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub shaderSMCount: u32,
    pub shaderWarpsPerSM: u32,
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
pub struct VkPhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub shaderSubgroupUniformControlFlow: VkBool32,
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
pub struct VkPhysicalDeviceShaderTileImageFeaturesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub shaderTileImageColorReadAccess: VkBool32,
    pub shaderTileImageDepthReadAccess: VkBool32,
    pub shaderTileImageStencilReadAccess: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceShaderTileImagePropertiesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub shaderTileImageCoherentReadAccelerated: VkBool32,
    pub shaderTileImageReadSampleFromPixelRateInvocation: VkBool32,
    pub shaderTileImageReadFromHelperInvocation: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceShaderUniformBufferUnsizedArrayFeaturesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub shaderUniformBufferUnsizedArray: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceShaderUntypedPointersFeaturesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub shaderUntypedPointers: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceShadingRateImageFeaturesNV {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub shadingRateImage: VkBool32,
    pub shadingRateCoarseSampleOrder: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceShadingRateImagePropertiesNV {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub shadingRateTexelSize: VkExtent2D,
    pub shadingRatePaletteSize: u32,
    pub shadingRateMaxCoarseSamples: u32,
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
pub struct VkPhysicalDeviceSubpassMergeFeedbackFeaturesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub subpassMergeFeedback: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceSubpassShadingFeaturesHUAWEI {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub subpassShading: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceSubpassShadingPropertiesHUAWEI {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub maxSubpassShadingWorkgroupSizeAspectRatio: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceSurfaceInfo2KHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub surface: VkSurfaceKHR,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceSwapchainMaintenance1FeaturesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub swapchainMaintenance1: VkBool32,
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
pub struct VkPhysicalDeviceTensorFeaturesARM {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub tensorNonPacked: VkBool32,
    pub shaderTensorAccess: VkBool32,
    pub shaderStorageTensorArrayDynamicIndexing: VkBool32,
    pub shaderStorageTensorArrayNonUniformIndexing: VkBool32,
    pub descriptorBindingStorageTensorUpdateAfterBind: VkBool32,
    pub tensors: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceTensorPropertiesARM {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub maxTensorDimensionCount: u32,
    pub maxTensorElements: u64,
    pub maxPerDimensionTensorElements: u64,
    pub maxTensorStride: i64,
    pub maxTensorSize: u64,
    pub maxTensorShaderAccessArrayLength: u32,
    pub maxTensorShaderAccessSize: u32,
    pub maxDescriptorSetStorageTensors: u32,
    pub maxPerStageDescriptorSetStorageTensors: u32,
    pub maxDescriptorSetUpdateAfterBindStorageTensors: u32,
    pub maxPerStageDescriptorUpdateAfterBindStorageTensors: u32,
    pub shaderStorageTensorArrayNonUniformIndexingNative: VkBool32,
    pub shaderTensorSupportedStages: VkShaderStageFlags,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceTexelBufferAlignmentFeaturesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub texelBufferAlignment: VkBool32,
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
pub struct VkPhysicalDeviceTileMemoryHeapFeaturesQCOM {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub tileMemoryHeap: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceTileMemoryHeapPropertiesQCOM {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub queueSubmitBoundary: VkBool32,
    pub tileBufferTransfers: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceTilePropertiesFeaturesQCOM {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub tileProperties: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceTileShadingFeaturesQCOM {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub tileShading: VkBool32,
    pub tileShadingFragmentStage: VkBool32,
    pub tileShadingColorAttachments: VkBool32,
    pub tileShadingDepthAttachments: VkBool32,
    pub tileShadingStencilAttachments: VkBool32,
    pub tileShadingInputAttachments: VkBool32,
    pub tileShadingSampledAttachments: VkBool32,
    pub tileShadingPerTileDraw: VkBool32,
    pub tileShadingPerTileDispatch: VkBool32,
    pub tileShadingDispatchTile: VkBool32,
    pub tileShadingApron: VkBool32,
    pub tileShadingAnisotropicApron: VkBool32,
    pub tileShadingAtomicOps: VkBool32,
    pub tileShadingImageProcessing: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceTileShadingPropertiesQCOM {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub maxApronSize: u32,
    pub preferNonCoherent: VkBool32,
    pub tileGranularity: VkExtent2D,
    pub maxTileShadingRate: VkExtent2D,
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
pub struct VkPhysicalDeviceTransformFeedbackFeaturesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub transformFeedback: VkBool32,
    pub geometryStreams: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceTransformFeedbackPropertiesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub maxTransformFeedbackStreams: u32,
    pub maxTransformFeedbackBuffers: u32,
    pub maxTransformFeedbackBufferSize: VkDeviceSize,
    pub maxTransformFeedbackStreamDataSize: u32,
    pub maxTransformFeedbackBufferDataSize: u32,
    pub maxTransformFeedbackBufferDataStride: u32,
    pub transformFeedbackQueries: VkBool32,
    pub transformFeedbackStreamsLinesTriangles: VkBool32,
    pub transformFeedbackRasterizationStreamSelect: VkBool32,
    pub transformFeedbackDraw: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceUnifiedImageLayoutsFeaturesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub unifiedImageLayouts: VkBool32,
    pub unifiedImageLayoutsVideo: VkBool32,
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
pub struct VkPhysicalDeviceVertexAttributeDivisorPropertiesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub maxVertexAttribDivisor: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceVertexAttributeRobustnessFeaturesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub vertexAttributeRobustness: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceVertexInputDynamicStateFeaturesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub vertexInputDynamicState: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceVideoDecodeVP9FeaturesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub videoDecodeVP9: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceVideoEncodeAV1FeaturesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub videoEncodeAV1: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceVideoEncodeIntraRefreshFeaturesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub videoEncodeIntraRefresh: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceVideoEncodeQualityLevelInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub pVideoProfile: *const VkVideoProfileInfoKHR,
    pub qualityLevel: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceVideoEncodeQuantizationMapFeaturesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub videoEncodeQuantizationMap: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceVideoEncodeRgbConversionFeaturesVALVE {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub videoEncodeRgbConversion: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceVideoFormatInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub imageUsage: VkImageUsageFlags,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceVideoMaintenance1FeaturesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub videoMaintenance1: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceVideoMaintenance2FeaturesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub videoMaintenance2: VkBool32,
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
pub struct VkPhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub workgroupMemoryExplicitLayout: VkBool32,
    pub workgroupMemoryExplicitLayoutScalarBlockLayout: VkBool32,
    pub workgroupMemoryExplicitLayout8BitAccess: VkBool32,
    pub workgroupMemoryExplicitLayout16BitAccess: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub ycbcr2plane444Formats: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceYcbcrDegammaFeaturesQCOM {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub ycbcrDegamma: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceYcbcrImageArraysFeaturesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub ycbcrImageArrays: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceZeroInitializeDeviceMemoryFeaturesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub zeroInitializeDeviceMemory: VkBool32,
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
pub struct VkPipelineBinaryCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub pKeysAndDataInfo: *const VkPipelineBinaryKeysAndDataKHR,
    pub pipeline: VkPipeline,
    pub pPipelineCreateInfo: *const VkPipelineCreateInfoKHR,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineBinaryDataInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub pipelineBinary: VkPipelineBinaryKHR,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineBinaryDataKHR {
    pub dataSize: usize,
    pub pData: *mut c_void,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineBinaryHandlesInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub pipelineBinaryCount: u32,
    pub pPipelineBinaries: *mut VkPipelineBinaryKHR,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineBinaryInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub binaryCount: u32,
    pub pPipelineBinaries: *const VkPipelineBinaryKHR,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineBinaryKeyKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub keySize: u32,
    pub key: [u8; VK_MAX_PIPELINE_BINARY_KEY_SIZE_KHR as usize],
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineBinaryKeysAndDataKHR {
    pub binaryCount: u32,
    pub pPipelineBinaryKeys: *const VkPipelineBinaryKeyKHR,
    pub pPipelineBinaryData: *const VkPipelineBinaryDataKHR,
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
pub struct VkPipelineCacheHeaderVersionDataGraphQCOM {
    pub headerSize: u32,
    pub headerVersion: VkPipelineCacheHeaderVersion,
    pub cacheType: VkDataGraphModelCacheTypeQCOM,
    pub cacheVersion: u32,
    pub toolchainVersion: [u32; VK_DATA_GRAPH_MODEL_TOOLCHAIN_VERSION_LENGTH_QCOM as usize],
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
pub struct VkPipelineColorBlendAdvancedStateCreateInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub srcPremultiplied: VkBool32,
    pub dstPremultiplied: VkBool32,
    pub blendOverlap: VkBlendOverlapEXT,
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
    pub blendConstants: [f32; 4 as usize],
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineColorWriteCreateInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub attachmentCount: u32,
    pub pColorWriteEnables: *const VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineCompilerControlCreateInfoAMD {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub compilerControlFlags: VkPipelineCompilerControlFlagsAMD,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineCoverageModulationStateCreateInfoNV {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineCoverageModulationStateCreateFlagsNV,
    pub coverageModulationMode: VkCoverageModulationModeNV,
    pub coverageModulationTableEnable: VkBool32,
    pub coverageModulationTableCount: u32,
    pub pCoverageModulationTable: *const f32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineCoverageReductionStateCreateInfoNV {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineCoverageReductionStateCreateFlagsNV,
    pub coverageReductionMode: VkCoverageReductionModeNV,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineCoverageToColorStateCreateInfoNV {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineCoverageToColorStateCreateFlagsNV,
    pub coverageToColorEnable: VkBool32,
    pub coverageToColorLocation: u32,
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
pub struct VkPipelineCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
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
    pub minDepthBounds: f32,
    pub maxDepthBounds: f32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineDiscardRectangleStateCreateInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineDiscardRectangleStateCreateFlagsEXT,
    pub discardRectangleMode: VkDiscardRectangleModeEXT,
    pub discardRectangleCount: u32,
    pub pDiscardRectangles: *const VkRect2D,
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
pub struct VkPipelineExecutableInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub pipeline: VkPipeline,
    pub executableIndex: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineExecutableInternalRepresentationKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub name: [c_char; VK_MAX_DESCRIPTION_SIZE as usize],
    pub description: [c_char; VK_MAX_DESCRIPTION_SIZE as usize],
    pub isText: VkBool32,
    pub dataSize: usize,
    pub pData: *mut c_void,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineExecutablePropertiesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub stages: VkShaderStageFlags,
    pub name: [c_char; VK_MAX_DESCRIPTION_SIZE as usize],
    pub description: [c_char; VK_MAX_DESCRIPTION_SIZE as usize],
    pub subgroupSize: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineExecutableStatisticKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub name: [c_char; VK_MAX_DESCRIPTION_SIZE as usize],
    pub description: [c_char; VK_MAX_DESCRIPTION_SIZE as usize],
    pub format: VkPipelineExecutableStatisticFormatKHR,
    pub value: VkPipelineExecutableStatisticValueKHR,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineFragmentDensityMapLayeredCreateInfoVALVE {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub maxFragmentDensityMapLayers: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineFragmentShadingRateEnumStateCreateInfoNV {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub shadingRateType: VkFragmentShadingRateTypeNV,
    pub shadingRate: VkFragmentShadingRateNV,
    pub combinerOps: [VkFragmentShadingRateCombinerOpKHR; 2 as usize],
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineFragmentShadingRateStateCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub fragmentSize: VkExtent2D,
    pub combinerOps: [VkFragmentShadingRateCombinerOpKHR; 2 as usize],
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineIndirectDeviceAddressInfoNV {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub pipelineBindPoint: VkPipelineBindPoint,
    pub pipeline: VkPipeline,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub pipeline: VkPipeline,
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
pub struct VkPipelineLibraryCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub libraryCount: u32,
    pub pLibraries: *const VkPipeline,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineMultisampleStateCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineMultisampleStateCreateFlags,
    pub rasterizationSamples: VkSampleCountFlagBits,
    pub sampleShadingEnable: VkBool32,
    pub minSampleShading: f32,
    pub pSampleMask: *const VkSampleMask,
    pub alphaToCoverageEnable: VkBool32,
    pub alphaToOneEnable: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelinePropertiesIdentifierEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub pipelineIdentifier: [u8; VK_UUID_SIZE as usize],
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineRasterizationConservativeStateCreateInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineRasterizationConservativeStateCreateFlagsEXT,
    pub conservativeRasterizationMode: VkConservativeRasterizationModeEXT,
    pub extraPrimitiveOverestimationSize: f32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineRasterizationDepthClipStateCreateInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineRasterizationDepthClipStateCreateFlagsEXT,
    pub depthClipEnable: VkBool32,
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
pub struct VkPipelineRasterizationProvokingVertexStateCreateInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub provokingVertexMode: VkProvokingVertexModeEXT,
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
    pub depthBiasConstantFactor: f32,
    pub depthBiasClamp: f32,
    pub depthBiasSlopeFactor: f32,
    pub lineWidth: f32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineRasterizationStateRasterizationOrderAMD {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub rasterizationOrder: VkRasterizationOrderAMD,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineRasterizationStateStreamCreateInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineRasterizationStateStreamCreateFlagsEXT,
    pub rasterizationStream: u32,
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
pub struct VkPipelineRepresentativeFragmentTestStateCreateInfoNV {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub representativeFragmentTestEnable: VkBool32,
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
pub struct VkPipelineSampleLocationsStateCreateInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub sampleLocationsEnable: VkBool32,
    pub sampleLocationsInfo: VkSampleLocationsInfoEXT,
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
pub struct VkPipelineShaderStageModuleIdentifierCreateInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub identifierSize: u32,
    pub pIdentifier: *const u8,
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
pub struct VkPipelineViewportCoarseSampleOrderStateCreateInfoNV {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub sampleOrderType: VkCoarseSampleOrderTypeNV,
    pub customSampleOrderCount: u32,
    pub pCustomSampleOrders: *const VkCoarseSampleOrderCustomNV,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineViewportDepthClampControlCreateInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub depthClampMode: VkDepthClampModeEXT,
    pub pDepthClampRange: *const VkDepthClampRangeEXT,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineViewportDepthClipControlCreateInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub negativeOneToOne: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineViewportExclusiveScissorStateCreateInfoNV {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub exclusiveScissorCount: u32,
    pub pExclusiveScissors: *const VkRect2D,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineViewportShadingRateImageStateCreateInfoNV {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub shadingRateImageEnable: VkBool32,
    pub viewportCount: u32,
    pub pShadingRatePalettes: *const VkShadingRatePaletteNV,
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
pub struct VkPipelineViewportSwizzleStateCreateInfoNV {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineViewportSwizzleStateCreateFlagsNV,
    pub viewportCount: u32,
    pub pViewportSwizzles: *const VkViewportSwizzleNV,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineViewportWScalingStateCreateInfoNV {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub viewportWScalingEnable: VkBool32,
    pub viewportCount: u32,
    pub pViewportWScalings: *const VkViewportWScalingNV,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPresentId2KHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub swapchainCount: u32,
    pub pPresentIds: *const u64,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPresentIdKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub swapchainCount: u32,
    pub pPresentIds: *const u64,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPresentInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub waitSemaphoreCount: u32,
    pub pWaitSemaphores: *const VkSemaphore,
    pub swapchainCount: u32,
    pub pSwapchains: *const VkSwapchainKHR,
    pub pImageIndices: *const u32,
    pub pResults: *mut VkResult,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPresentRegionKHR {
    pub rectangleCount: u32,
    pub pRectangles: *const VkRectLayerKHR,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPresentRegionsKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub swapchainCount: u32,
    pub pRegions: *const VkPresentRegionKHR,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPresentStageTimeEXT {
    pub stage: VkPresentStageFlagsEXT,
    pub time: u64,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPresentTimeGOOGLE {
    pub presentID: u32,
    pub desiredPresentTime: u64,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPresentTimesInfoGOOGLE {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub swapchainCount: u32,
    pub pTimes: *const VkPresentTimeGOOGLE,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPresentTimingInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPresentTimingInfoFlagsEXT,
    pub targetTime: u64,
    pub timeDomainId: u64,
    pub presentStageQueries: VkPresentStageFlagsEXT,
    pub targetTimeDomainPresentStage: VkPresentStageFlagsEXT,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPresentTimingSurfaceCapabilitiesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub presentTimingSupported: VkBool32,
    pub presentAtAbsoluteTimeSupported: VkBool32,
    pub presentAtRelativeTimeSupported: VkBool32,
    pub presentStageQueries: VkPresentStageFlagsEXT,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPresentTimingsInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub swapchainCount: u32,
    pub pTimingInfos: *const VkPresentTimingInfoEXT,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPresentWait2InfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub presentId: u64,
    pub timeout: u64,
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
pub struct VkQueryLowLatencySupportNV {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub pQueriedLowLatencyData: *mut c_void,
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
pub struct VkQueryPoolPerformanceCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub queueFamilyIndex: u32,
    pub counterIndexCount: u32,
    pub pCounterIndices: *const u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkQueryPoolPerformanceQueryCreateInfoINTEL {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub performanceCountersSampling: VkQueryPoolSamplingModeINTEL,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkQueryPoolVideoEncodeFeedbackCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub encodeFeedbackFlags: VkVideoEncodeFeedbackFlagsKHR,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkQueueFamilyCheckpointProperties2NV {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub checkpointExecutionStageMask: VkPipelineStageFlags2,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkQueueFamilyCheckpointPropertiesNV {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub checkpointExecutionStageMask: VkPipelineStageFlags,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkQueueFamilyDataGraphProcessingEnginePropertiesARM {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub foreignSemaphoreHandleTypes: VkExternalSemaphoreHandleTypeFlags,
    pub foreignMemoryHandleTypes: VkExternalMemoryHandleTypeFlags,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkQueueFamilyDataGraphPropertiesARM {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub engine: VkPhysicalDeviceDataGraphProcessingEngineARM,
    pub operation: VkPhysicalDeviceDataGraphOperationSupportARM,
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
pub struct VkQueueFamilyOwnershipTransferPropertiesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub optimalImageTransferToQueueFamilies: u32,
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
pub struct VkQueueFamilyQueryResultStatusPropertiesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub queryResultStatusSupport: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkQueueFamilyVideoPropertiesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub videoCodecOperations: VkVideoCodecOperationFlagsKHR,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkRayTracingPipelineClusterAccelerationStructureCreateInfoNV {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub allowClusterAccelerationStructure: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkRayTracingPipelineCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineCreateFlags,
    pub stageCount: u32,
    pub pStages: *const VkPipelineShaderStageCreateInfo,
    pub groupCount: u32,
    pub pGroups: *const VkRayTracingShaderGroupCreateInfoKHR,
    pub maxPipelineRayRecursionDepth: u32,
    pub pLibraryInfo: *const VkPipelineLibraryCreateInfoKHR,
    pub pLibraryInterface: *const VkRayTracingPipelineInterfaceCreateInfoKHR,
    pub pDynamicState: *const VkPipelineDynamicStateCreateInfo,
    pub layout: VkPipelineLayout,
    pub basePipelineHandle: VkPipeline,
    pub basePipelineIndex: i32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkRayTracingPipelineCreateInfoNV {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineCreateFlags,
    pub stageCount: u32,
    pub pStages: *const VkPipelineShaderStageCreateInfo,
    pub groupCount: u32,
    pub pGroups: *const VkRayTracingShaderGroupCreateInfoNV,
    pub maxRecursionDepth: u32,
    pub layout: VkPipelineLayout,
    pub basePipelineHandle: VkPipeline,
    pub basePipelineIndex: i32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkRayTracingPipelineInterfaceCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub maxPipelineRayPayloadSize: u32,
    pub maxPipelineRayHitAttributeSize: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkRayTracingShaderGroupCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub typ: VkRayTracingShaderGroupTypeKHR,
    pub generalShader: u32,
    pub closestHitShader: u32,
    pub anyHitShader: u32,
    pub intersectionShader: u32,
    pub pShaderGroupCaptureReplayHandle: *const c_void,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkRayTracingShaderGroupCreateInfoNV {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub typ: VkRayTracingShaderGroupTypeKHR,
    pub generalShader: u32,
    pub closestHitShader: u32,
    pub anyHitShader: u32,
    pub intersectionShader: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkRect2D {
    pub offset: VkOffset2D,
    pub extent: VkExtent2D,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkRectLayerKHR {
    pub offset: VkOffset2D,
    pub extent: VkExtent2D,
    pub layer: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkRefreshCycleDurationGOOGLE {
    pub refreshDuration: u64,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkReleaseCapturedPipelineDataInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub pipeline: VkPipeline,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkReleaseSwapchainImagesInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub swapchain: VkSwapchainKHR,
    pub imageIndexCount: u32,
    pub pImageIndices: *const u32,
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
pub struct VkRenderPassCreationControlEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub disallowMerging: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkRenderPassCreationFeedbackCreateInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub pRenderPassFeedback: *mut VkRenderPassCreationFeedbackInfoEXT,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkRenderPassCreationFeedbackInfoEXT {
    pub postMergeSubpassCount: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkRenderPassFragmentDensityMapCreateInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub fragmentDensityMapAttachment: VkAttachmentReference,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkRenderPassFragmentDensityMapOffsetEndInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub fragmentDensityOffsetCount: u32,
    pub pFragmentDensityOffsets: *const VkOffset2D,
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
pub struct VkRenderPassPerformanceCountersByRegionBeginInfoARM {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub counterAddressCount: u32,
    pub pCounterAddresses: *const VkDeviceAddress,
    pub serializeRegions: VkBool32,
    pub counterIndexCount: u32,
    pub pCounterIndices: *mut u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkRenderPassSampleLocationsBeginInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub attachmentInitialSampleLocationsCount: u32,
    pub pAttachmentInitialSampleLocations: *const VkAttachmentSampleLocationsEXT,
    pub postSubpassSampleLocationsCount: u32,
    pub pPostSubpassSampleLocations: *const VkSubpassSampleLocationsEXT,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkRenderPassStripeBeginInfoARM {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub stripeInfoCount: u32,
    pub pStripeInfos: *const VkRenderPassStripeInfoARM,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkRenderPassStripeInfoARM {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub stripeArea: VkRect2D,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkRenderPassStripeSubmitInfoARM {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub stripeSemaphoreInfoCount: u32,
    pub pStripeSemaphoreInfos: *const VkSemaphoreSubmitInfo,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkRenderPassSubpassFeedbackCreateInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub pSubpassFeedback: *mut VkRenderPassSubpassFeedbackInfoEXT,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkRenderPassSubpassFeedbackInfoEXT {
    pub subpassMergeStatus: VkSubpassMergeStatusEXT,
    pub description: [c_char; VK_MAX_DESCRIPTION_SIZE as usize],
    pub postMergeIndex: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkRenderPassTileShadingCreateInfoQCOM {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkTileShadingRenderPassFlagsQCOM,
    pub tileApronSize: VkExtent2D,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkRenderPassTransformBeginInfoQCOM {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub transform: VkSurfaceTransformFlagBitsKHR,
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
pub struct VkRenderingAttachmentFlagsInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkRenderingAttachmentFlagsKHR,
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
pub struct VkRenderingEndInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkRenderingFragmentDensityMapAttachmentInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub imageView: VkImageView,
    pub imageLayout: VkImageLayout,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkRenderingFragmentShadingRateAttachmentInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub imageView: VkImageView,
    pub imageLayout: VkImageLayout,
    pub shadingRateAttachmentTexelSize: VkExtent2D,
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
pub struct VkResolveImageModeInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkResolveImageFlagsKHR,
    pub resolveMode: VkResolveModeFlagBits,
    pub stencilResolveMode: VkResolveModeFlagBits,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSRTDataNV {
    pub sx: f32,
    pub a: f32,
    pub b: f32,
    pub pvx: f32,
    pub sy: f32,
    pub c: f32,
    pub pvy: f32,
    pub sz: f32,
    pub pvz: f32,
    pub qx: f32,
    pub qy: f32,
    pub qz: f32,
    pub qw: f32,
    pub tx: f32,
    pub ty: f32,
    pub tz: f32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSampleLocationEXT {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSampleLocationsInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub sampleLocationsPerPixel: VkSampleCountFlagBits,
    pub sampleLocationGridSize: VkExtent2D,
    pub sampleLocationsCount: u32,
    pub pSampleLocations: *const VkSampleLocationEXT,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSamplerBlockMatchWindowCreateInfoQCOM {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub windowExtent: VkExtent2D,
    pub windowCompareMode: VkBlockMatchWindowCompareModeQCOM,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSamplerBorderColorComponentMappingCreateInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub components: VkComponentMapping,
    pub srgb: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSamplerCaptureDescriptorDataInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub sampler: VkSampler,
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
    pub mipLodBias: f32,
    pub anisotropyEnable: VkBool32,
    pub maxAnisotropy: f32,
    pub compareEnable: VkBool32,
    pub compareOp: VkCompareOp,
    pub minLod: f32,
    pub maxLod: f32,
    pub borderColor: VkBorderColor,
    pub unnormalizedCoordinates: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSamplerCubicWeightsCreateInfoQCOM {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub cubicWeights: VkCubicFilterWeightsQCOM,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSamplerCustomBorderColorCreateInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub customBorderColor: VkClearColorValue,
    pub format: VkFormat,
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
pub struct VkSamplerYcbcrConversionYcbcrDegammaCreateInfoQCOM {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub enableYDegamma: VkBool32,
    pub enableCbCrDegamma: VkBool32,
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
pub struct VkSemaphoreGetFdInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub semaphore: VkSemaphore,
    pub handleType: VkExternalSemaphoreHandleTypeFlagBits,
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
pub struct VkSetDescriptorBufferOffsetsInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub stageFlags: VkShaderStageFlags,
    pub layout: VkPipelineLayout,
    pub firstSet: u32,
    pub setCount: u32,
    pub pBufferIndices: *const u32,
    pub pOffsets: *const VkDeviceSize,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSetLatencyMarkerInfoNV {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub presentID: u64,
    pub marker: VkLatencyMarkerNV,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSetStateFlagsIndirectCommandNV {
    pub data: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkShaderCreateInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkShaderCreateFlagsEXT,
    pub stage: VkShaderStageFlagBits,
    pub nextStage: VkShaderStageFlags,
    pub codeType: VkShaderCodeTypeEXT,
    pub codeSize: usize,
    pub pCode: *const c_void,
    pub pName: *const c_char,
    pub setLayoutCount: u32,
    pub pSetLayouts: *const VkDescriptorSetLayout,
    pub pushConstantRangeCount: u32,
    pub pPushConstantRanges: *const VkPushConstantRange,
    pub pSpecializationInfo: *const VkSpecializationInfo,
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
pub struct VkShaderModuleIdentifierEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub identifierSize: u32,
    pub identifier: [u8; VK_MAX_SHADER_MODULE_IDENTIFIER_SIZE_EXT as usize],
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkShaderModuleValidationCacheCreateInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub validationCache: VkValidationCacheEXT,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkShaderResourceUsageAMD {
    pub numUsedVgprs: u32,
    pub numUsedSgprs: u32,
    pub ldsSizePerLocalWorkGroup: u32,
    pub ldsUsageSizeInBytes: usize,
    pub scratchMemUsageInBytes: usize,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkShaderStatisticsInfoAMD {
    pub shaderStageMask: VkShaderStageFlags,
    pub resourceUsage: VkShaderResourceUsageAMD,
    pub numPhysicalVgprs: u32,
    pub numPhysicalSgprs: u32,
    pub numAvailableVgprs: u32,
    pub numAvailableSgprs: u32,
    pub computeWorkGroupSize: [u32; 3 as usize],
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkShadingRatePaletteNV {
    pub shadingRatePaletteEntryCount: u32,
    pub pShadingRatePaletteEntries: *const VkShadingRatePaletteEntryNV,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSharedPresentSurfaceCapabilitiesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub sharedPresentSupportedUsageFlags: VkImageUsageFlags,
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
pub struct VkStridedDeviceAddressNV {
    pub startAddress: VkDeviceAddress,
    pub strideInBytes: VkDeviceSize,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkStridedDeviceAddressRangeKHR {
    pub address: VkDeviceAddress,
    pub size: VkDeviceSize,
    pub stride: VkDeviceSize,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkStridedDeviceAddressRegionKHR {
    pub deviceAddress: VkDeviceAddress,
    pub stride: VkDeviceSize,
    pub size: VkDeviceSize,
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
pub struct VkSubpassResolvePerformanceQueryEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub optimal: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSubpassSampleLocationsEXT {
    pub subpassIndex: u32,
    pub sampleLocationsInfo: VkSampleLocationsInfoEXT,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSubpassShadingPipelineCreateInfoHUAWEI {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub renderPass: VkRenderPass,
    pub subpass: u32,
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
pub struct VkSurfaceCapabilities2EXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub minImageCount: u32,
    pub maxImageCount: u32,
    pub currentExtent: VkExtent2D,
    pub minImageExtent: VkExtent2D,
    pub maxImageExtent: VkExtent2D,
    pub maxImageArrayLayers: u32,
    pub supportedTransforms: VkSurfaceTransformFlagsKHR,
    pub currentTransform: VkSurfaceTransformFlagBitsKHR,
    pub supportedCompositeAlpha: VkCompositeAlphaFlagsKHR,
    pub supportedUsageFlags: VkImageUsageFlags,
    pub supportedSurfaceCounters: VkSurfaceCounterFlagsEXT,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSurfaceCapabilities2KHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub surfaceCapabilities: VkSurfaceCapabilitiesKHR,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSurfaceCapabilitiesKHR {
    pub minImageCount: u32,
    pub maxImageCount: u32,
    pub currentExtent: VkExtent2D,
    pub minImageExtent: VkExtent2D,
    pub maxImageExtent: VkExtent2D,
    pub maxImageArrayLayers: u32,
    pub supportedTransforms: VkSurfaceTransformFlagsKHR,
    pub currentTransform: VkSurfaceTransformFlagBitsKHR,
    pub supportedCompositeAlpha: VkCompositeAlphaFlagsKHR,
    pub supportedUsageFlags: VkImageUsageFlags,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSurfaceCapabilitiesPresentBarrierNV {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub presentBarrierSupported: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSurfaceCapabilitiesPresentId2KHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub presentId2Supported: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSurfaceCapabilitiesPresentWait2KHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub presentWait2Supported: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSurfaceFormat2KHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub surfaceFormat: VkSurfaceFormatKHR,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSurfaceFormatKHR {
    pub format: VkFormat,
    pub colorSpace: VkColorSpaceKHR,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSurfacePresentModeCompatibilityKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub presentModeCount: u32,
    pub pPresentModes: *mut VkPresentModeKHR,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSurfacePresentModeKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub presentMode: VkPresentModeKHR,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSurfacePresentScalingCapabilitiesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub supportedPresentScaling: VkPresentScalingFlagsKHR,
    pub supportedPresentGravityX: VkPresentGravityFlagsKHR,
    pub supportedPresentGravityY: VkPresentGravityFlagsKHR,
    pub minScaledImageExtent: VkExtent2D,
    pub maxScaledImageExtent: VkExtent2D,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSurfaceProtectedCapabilitiesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub supportsProtected: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSwapchainCalibratedTimestampInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub swapchain: VkSwapchainKHR,
    pub presentStage: VkPresentStageFlagsEXT,
    pub timeDomainId: u64,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSwapchainCounterCreateInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub surfaceCounters: VkSurfaceCounterFlagsEXT,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSwapchainCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkSwapchainCreateFlagsKHR,
    pub surface: VkSurfaceKHR,
    pub minImageCount: u32,
    pub imageFormat: VkFormat,
    pub imageColorSpace: VkColorSpaceKHR,
    pub imageExtent: VkExtent2D,
    pub imageArrayLayers: u32,
    pub imageUsage: VkImageUsageFlags,
    pub imageSharingMode: VkSharingMode,
    pub queueFamilyIndexCount: u32,
    pub pQueueFamilyIndices: *const u32,
    pub preTransform: VkSurfaceTransformFlagBitsKHR,
    pub compositeAlpha: VkCompositeAlphaFlagBitsKHR,
    pub presentMode: VkPresentModeKHR,
    pub clipped: VkBool32,
    pub oldSwapchain: VkSwapchainKHR,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSwapchainDisplayNativeHdrCreateInfoAMD {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub localDimmingEnable: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSwapchainLatencyCreateInfoNV {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub latencyModeEnable: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSwapchainPresentBarrierCreateInfoNV {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub presentBarrierEnable: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSwapchainPresentFenceInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub swapchainCount: u32,
    pub pFences: *const VkFence,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSwapchainPresentModeInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub swapchainCount: u32,
    pub pPresentModes: *const VkPresentModeKHR,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSwapchainPresentModesCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub presentModeCount: u32,
    pub pPresentModes: *const VkPresentModeKHR,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSwapchainPresentScalingCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub scalingBehavior: VkPresentScalingFlagsKHR,
    pub presentGravityX: VkPresentGravityFlagsKHR,
    pub presentGravityY: VkPresentGravityFlagsKHR,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSwapchainTimeDomainPropertiesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub timeDomainCount: u32,
    pub pTimeDomains: *mut VkTimeDomainKHR,
    pub pTimeDomainIds: *mut u64,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSwapchainTimingPropertiesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub refreshDuration: u64,
    pub refreshInterval: u64,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkTensorCaptureDescriptorDataInfoARM {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub tensor: VkTensorARM,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkTensorCopyARM {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub dimensionCount: u32,
    pub pSrcOffset: *const u64,
    pub pDstOffset: *const u64,
    pub pExtent: *const u64,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkTensorCreateInfoARM {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkTensorCreateFlagsARM,
    pub pDescription: *const VkTensorDescriptionARM,
    pub sharingMode: VkSharingMode,
    pub queueFamilyIndexCount: u32,
    pub pQueueFamilyIndices: *const u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkTensorDependencyInfoARM {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub tensorMemoryBarrierCount: u32,
    pub pTensorMemoryBarriers: *const VkTensorMemoryBarrierARM,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkTensorDescriptionARM {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub tiling: VkTensorTilingARM,
    pub format: VkFormat,
    pub dimensionCount: u32,
    pub pDimensions: *const i64,
    pub pStrides: *const i64,
    pub usage: VkTensorUsageFlagsARM,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkTensorFormatPropertiesARM {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub optimalTilingTensorFeatures: VkFormatFeatureFlags2,
    pub linearTilingTensorFeatures: VkFormatFeatureFlags2,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkTensorMemoryBarrierARM {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub srcStageMask: VkPipelineStageFlags2,
    pub srcAccessMask: VkAccessFlags2,
    pub dstStageMask: VkPipelineStageFlags2,
    pub dstAccessMask: VkAccessFlags2,
    pub srcQueueFamilyIndex: u32,
    pub dstQueueFamilyIndex: u32,
    pub tensor: VkTensorARM,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkTensorMemoryRequirementsInfoARM {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub tensor: VkTensorARM,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkTensorViewCaptureDescriptorDataInfoARM {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub tensorView: VkTensorViewARM,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkTensorViewCreateInfoARM {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkTensorViewCreateFlagsARM,
    pub tensor: VkTensorARM,
    pub format: VkFormat,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkTextureLODGatherFormatPropertiesAMD {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub supportsTextureGatherLODBiasAMD: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkTileMemoryBindInfoQCOM {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub memory: VkDeviceMemory,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkTileMemoryRequirementsQCOM {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub size: VkDeviceSize,
    pub alignment: VkDeviceSize,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkTileMemorySizeInfoQCOM {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub size: VkDeviceSize,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkTilePropertiesQCOM {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub tileSize: VkExtent3D,
    pub apronSize: VkExtent2D,
    pub origin: VkOffset2D,
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
pub struct VkTraceRaysIndirectCommand2KHR {
    pub raygenShaderRecordAddress: VkDeviceAddress,
    pub raygenShaderRecordSize: VkDeviceSize,
    pub missShaderBindingTableAddress: VkDeviceAddress,
    pub missShaderBindingTableSize: VkDeviceSize,
    pub missShaderBindingTableStride: VkDeviceSize,
    pub hitShaderBindingTableAddress: VkDeviceAddress,
    pub hitShaderBindingTableSize: VkDeviceSize,
    pub hitShaderBindingTableStride: VkDeviceSize,
    pub callableShaderBindingTableAddress: VkDeviceAddress,
    pub callableShaderBindingTableSize: VkDeviceSize,
    pub callableShaderBindingTableStride: VkDeviceSize,
    pub width: u32,
    pub height: u32,
    pub depth: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkTraceRaysIndirectCommandKHR {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkTransformMatrixKHR {
    pub matrix: [[f32; 4 as usize]; 3 as usize],
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkValidationCacheCreateInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkValidationCacheCreateFlagsEXT,
    pub initialDataSize: usize,
    pub pInitialData: *const c_void,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkValidationFeaturesEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub enabledValidationFeatureCount: u32,
    pub pEnabledValidationFeatures: *const VkValidationFeatureEnableEXT,
    pub disabledValidationFeatureCount: u32,
    pub pDisabledValidationFeatures: *const VkValidationFeatureDisableEXT,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkValidationFlagsEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub disabledValidationCheckCount: u32,
    pub pDisabledValidationChecks: *const VkValidationCheckEXT,
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
pub struct VkVertexInputAttributeDescription2EXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
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
pub struct VkVertexInputBindingDescription2EXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub binding: u32,
    pub stride: u32,
    pub inputRate: VkVertexInputRate,
    pub divisor: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVertexInputBindingDivisorDescription {
    pub binding: u32,
    pub divisor: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoBeginCodingInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkVideoBeginCodingFlagsKHR,
    pub videoSession: VkVideoSessionKHR,
    pub videoSessionParameters: VkVideoSessionParametersKHR,
    pub referenceSlotCount: u32,
    pub pReferenceSlots: *const VkVideoReferenceSlotInfoKHR,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoCapabilitiesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub flags: VkVideoCapabilityFlagsKHR,
    pub minBitstreamBufferOffsetAlignment: VkDeviceSize,
    pub minBitstreamBufferSizeAlignment: VkDeviceSize,
    pub pictureAccessGranularity: VkExtent2D,
    pub minCodedExtent: VkExtent2D,
    pub maxCodedExtent: VkExtent2D,
    pub maxDpbSlots: u32,
    pub maxActiveReferencePictures: u32,
    pub stdHeaderVersion: VkExtensionProperties,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoCodingControlInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkVideoCodingControlFlagsKHR,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoDecodeAV1CapabilitiesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub maxLevel: StdVideoAV1Level,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoDecodeAV1DpbSlotInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub pStdReferenceInfo: *const StdVideoDecodeAV1ReferenceInfo,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoDecodeAV1InlineSessionParametersInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub pStdSequenceHeader: *const StdVideoAV1SequenceHeader,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoDecodeAV1PictureInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub pStdPictureInfo: *const StdVideoDecodeAV1PictureInfo,
    pub referenceNameSlotIndices: [i32; VK_MAX_VIDEO_AV1_REFERENCES_PER_FRAME_KHR as usize],
    pub frameHeaderOffset: u32,
    pub tileCount: u32,
    pub pTileOffsets: *const u32,
    pub pTileSizes: *const u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoDecodeAV1ProfileInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub stdProfile: StdVideoAV1Profile,
    pub filmGrainSupport: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoDecodeAV1SessionParametersCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub pStdSequenceHeader: *const StdVideoAV1SequenceHeader,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoDecodeCapabilitiesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub flags: VkVideoDecodeCapabilityFlagsKHR,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoDecodeH264CapabilitiesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub maxLevelIdc: StdVideoH264LevelIdc,
    pub fieldOffsetGranularity: VkOffset2D,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoDecodeH264DpbSlotInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub pStdReferenceInfo: *const StdVideoDecodeH264ReferenceInfo,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoDecodeH264InlineSessionParametersInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub pStdSPS: *const StdVideoH264SequenceParameterSet,
    pub pStdPPS: *const StdVideoH264PictureParameterSet,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoDecodeH264PictureInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub pStdPictureInfo: *const StdVideoDecodeH264PictureInfo,
    pub sliceCount: u32,
    pub pSliceOffsets: *const u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoDecodeH264ProfileInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub stdProfileIdc: StdVideoH264ProfileIdc,
    pub pictureLayout: VkVideoDecodeH264PictureLayoutFlagBitsKHR,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoDecodeH264SessionParametersAddInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub stdSPSCount: u32,
    pub pStdSPSs: *const StdVideoH264SequenceParameterSet,
    pub stdPPSCount: u32,
    pub pStdPPSs: *const StdVideoH264PictureParameterSet,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoDecodeH264SessionParametersCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub maxStdSPSCount: u32,
    pub maxStdPPSCount: u32,
    pub pParametersAddInfo: *const VkVideoDecodeH264SessionParametersAddInfoKHR,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoDecodeH265CapabilitiesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub maxLevelIdc: StdVideoH265LevelIdc,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoDecodeH265DpbSlotInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub pStdReferenceInfo: *const StdVideoDecodeH265ReferenceInfo,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoDecodeH265InlineSessionParametersInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub pStdVPS: *const StdVideoH265VideoParameterSet,
    pub pStdSPS: *const StdVideoH265SequenceParameterSet,
    pub pStdPPS: *const StdVideoH265PictureParameterSet,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoDecodeH265PictureInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub pStdPictureInfo: *const StdVideoDecodeH265PictureInfo,
    pub sliceSegmentCount: u32,
    pub pSliceSegmentOffsets: *const u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoDecodeH265ProfileInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub stdProfileIdc: StdVideoH265ProfileIdc,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoDecodeH265SessionParametersAddInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub stdVPSCount: u32,
    pub pStdVPSs: *const StdVideoH265VideoParameterSet,
    pub stdSPSCount: u32,
    pub pStdSPSs: *const StdVideoH265SequenceParameterSet,
    pub stdPPSCount: u32,
    pub pStdPPSs: *const StdVideoH265PictureParameterSet,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoDecodeH265SessionParametersCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub maxStdVPSCount: u32,
    pub maxStdSPSCount: u32,
    pub maxStdPPSCount: u32,
    pub pParametersAddInfo: *const VkVideoDecodeH265SessionParametersAddInfoKHR,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoDecodeInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkVideoDecodeFlagsKHR,
    pub srcBuffer: VkBuffer,
    pub srcBufferOffset: VkDeviceSize,
    pub srcBufferRange: VkDeviceSize,
    pub dstPictureResource: VkVideoPictureResourceInfoKHR,
    pub pSetupReferenceSlot: *const VkVideoReferenceSlotInfoKHR,
    pub referenceSlotCount: u32,
    pub pReferenceSlots: *const VkVideoReferenceSlotInfoKHR,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoDecodeUsageInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub videoUsageHints: VkVideoDecodeUsageFlagsKHR,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoDecodeVP9CapabilitiesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub maxLevel: StdVideoVP9Level,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoDecodeVP9PictureInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub pStdPictureInfo: *const StdVideoDecodeVP9PictureInfo,
    pub referenceNameSlotIndices: [i32; VK_MAX_VIDEO_VP9_REFERENCES_PER_FRAME_KHR as usize],
    pub uncompressedHeaderOffset: u32,
    pub compressedHeaderOffset: u32,
    pub tilesOffset: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoDecodeVP9ProfileInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub stdProfile: StdVideoVP9Profile,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoEncodeAV1CapabilitiesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub flags: VkVideoEncodeAV1CapabilityFlagsKHR,
    pub maxLevel: StdVideoAV1Level,
    pub codedPictureAlignment: VkExtent2D,
    pub maxTiles: VkExtent2D,
    pub minTileSize: VkExtent2D,
    pub maxTileSize: VkExtent2D,
    pub superblockSizes: VkVideoEncodeAV1SuperblockSizeFlagsKHR,
    pub maxSingleReferenceCount: u32,
    pub singleReferenceNameMask: u32,
    pub maxUnidirectionalCompoundReferenceCount: u32,
    pub maxUnidirectionalCompoundGroup1ReferenceCount: u32,
    pub unidirectionalCompoundReferenceNameMask: u32,
    pub maxBidirectionalCompoundReferenceCount: u32,
    pub maxBidirectionalCompoundGroup1ReferenceCount: u32,
    pub maxBidirectionalCompoundGroup2ReferenceCount: u32,
    pub bidirectionalCompoundReferenceNameMask: u32,
    pub maxTemporalLayerCount: u32,
    pub maxSpatialLayerCount: u32,
    pub maxOperatingPoints: u32,
    pub minQIndex: u32,
    pub maxQIndex: u32,
    pub prefersGopRemainingFrames: VkBool32,
    pub requiresGopRemainingFrames: VkBool32,
    pub stdSyntaxFlags: VkVideoEncodeAV1StdFlagsKHR,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoEncodeAV1DpbSlotInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub pStdReferenceInfo: *const StdVideoEncodeAV1ReferenceInfo,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoEncodeAV1FrameSizeKHR {
    pub intraFrameSize: u32,
    pub predictiveFrameSize: u32,
    pub bipredictiveFrameSize: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoEncodeAV1GopRemainingFrameInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub useGopRemainingFrames: VkBool32,
    pub gopRemainingIntra: u32,
    pub gopRemainingPredictive: u32,
    pub gopRemainingBipredictive: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoEncodeAV1PictureInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub predictionMode: VkVideoEncodeAV1PredictionModeKHR,
    pub rateControlGroup: VkVideoEncodeAV1RateControlGroupKHR,
    pub constantQIndex: u32,
    pub pStdPictureInfo: *const StdVideoEncodeAV1PictureInfo,
    pub referenceNameSlotIndices: [i32; VK_MAX_VIDEO_AV1_REFERENCES_PER_FRAME_KHR as usize],
    pub primaryReferenceCdfOnly: VkBool32,
    pub generateObuExtensionHeader: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoEncodeAV1ProfileInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub stdProfile: StdVideoAV1Profile,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoEncodeAV1QIndexKHR {
    pub intraQIndex: u32,
    pub predictiveQIndex: u32,
    pub bipredictiveQIndex: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoEncodeAV1QualityLevelPropertiesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub preferredRateControlFlags: VkVideoEncodeAV1RateControlFlagsKHR,
    pub preferredGopFrameCount: u32,
    pub preferredKeyFramePeriod: u32,
    pub preferredConsecutiveBipredictiveFrameCount: u32,
    pub preferredTemporalLayerCount: u32,
    pub preferredConstantQIndex: VkVideoEncodeAV1QIndexKHR,
    pub preferredMaxSingleReferenceCount: u32,
    pub preferredSingleReferenceNameMask: u32,
    pub preferredMaxUnidirectionalCompoundReferenceCount: u32,
    pub preferredMaxUnidirectionalCompoundGroup1ReferenceCount: u32,
    pub preferredUnidirectionalCompoundReferenceNameMask: u32,
    pub preferredMaxBidirectionalCompoundReferenceCount: u32,
    pub preferredMaxBidirectionalCompoundGroup1ReferenceCount: u32,
    pub preferredMaxBidirectionalCompoundGroup2ReferenceCount: u32,
    pub preferredBidirectionalCompoundReferenceNameMask: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoEncodeAV1QuantizationMapCapabilitiesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub minQIndexDelta: i32,
    pub maxQIndexDelta: i32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoEncodeAV1RateControlInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkVideoEncodeAV1RateControlFlagsKHR,
    pub gopFrameCount: u32,
    pub keyFramePeriod: u32,
    pub consecutiveBipredictiveFrameCount: u32,
    pub temporalLayerCount: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoEncodeAV1RateControlLayerInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub useMinQIndex: VkBool32,
    pub minQIndex: VkVideoEncodeAV1QIndexKHR,
    pub useMaxQIndex: VkBool32,
    pub maxQIndex: VkVideoEncodeAV1QIndexKHR,
    pub useMaxFrameSize: VkBool32,
    pub maxFrameSize: VkVideoEncodeAV1FrameSizeKHR,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoEncodeAV1SessionCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub useMaxLevel: VkBool32,
    pub maxLevel: StdVideoAV1Level,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoEncodeAV1SessionParametersCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub pStdSequenceHeader: *const StdVideoAV1SequenceHeader,
    pub pStdDecoderModelInfo: *const StdVideoEncodeAV1DecoderModelInfo,
    pub stdOperatingPointCount: u32,
    pub pStdOperatingPoints: *const StdVideoEncodeAV1OperatingPointInfo,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoEncodeCapabilitiesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub flags: VkVideoEncodeCapabilityFlagsKHR,
    pub rateControlModes: VkVideoEncodeRateControlModeFlagsKHR,
    pub maxRateControlLayers: u32,
    pub maxBitrate: u64,
    pub maxQualityLevels: u32,
    pub encodeInputPictureGranularity: VkExtent2D,
    pub supportedEncodeFeedbackFlags: VkVideoEncodeFeedbackFlagsKHR,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoEncodeH264CapabilitiesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub flags: VkVideoEncodeH264CapabilityFlagsKHR,
    pub maxLevelIdc: StdVideoH264LevelIdc,
    pub maxSliceCount: u32,
    pub maxPPictureL0ReferenceCount: u32,
    pub maxBPictureL0ReferenceCount: u32,
    pub maxL1ReferenceCount: u32,
    pub maxTemporalLayerCount: u32,
    pub expectDyadicTemporalLayerPattern: VkBool32,
    pub minQp: i32,
    pub maxQp: i32,
    pub prefersGopRemainingFrames: VkBool32,
    pub requiresGopRemainingFrames: VkBool32,
    pub stdSyntaxFlags: VkVideoEncodeH264StdFlagsKHR,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoEncodeH264DpbSlotInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub pStdReferenceInfo: *const StdVideoEncodeH264ReferenceInfo,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoEncodeH264FrameSizeKHR {
    pub frameISize: u32,
    pub framePSize: u32,
    pub frameBSize: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoEncodeH264GopRemainingFrameInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub useGopRemainingFrames: VkBool32,
    pub gopRemainingI: u32,
    pub gopRemainingP: u32,
    pub gopRemainingB: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoEncodeH264NaluSliceInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub constantQp: i32,
    pub pStdSliceHeader: *const StdVideoEncodeH264SliceHeader,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoEncodeH264PictureInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub naluSliceEntryCount: u32,
    pub pNaluSliceEntries: *const VkVideoEncodeH264NaluSliceInfoKHR,
    pub pStdPictureInfo: *const StdVideoEncodeH264PictureInfo,
    pub generatePrefixNalu: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoEncodeH264ProfileInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub stdProfileIdc: StdVideoH264ProfileIdc,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoEncodeH264QpKHR {
    pub qpI: i32,
    pub qpP: i32,
    pub qpB: i32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoEncodeH264QualityLevelPropertiesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub preferredRateControlFlags: VkVideoEncodeH264RateControlFlagsKHR,
    pub preferredGopFrameCount: u32,
    pub preferredIdrPeriod: u32,
    pub preferredConsecutiveBFrameCount: u32,
    pub preferredTemporalLayerCount: u32,
    pub preferredConstantQp: VkVideoEncodeH264QpKHR,
    pub preferredMaxL0ReferenceCount: u32,
    pub preferredMaxL1ReferenceCount: u32,
    pub preferredStdEntropyCodingModeFlag: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoEncodeH264QuantizationMapCapabilitiesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub minQpDelta: i32,
    pub maxQpDelta: i32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoEncodeH264RateControlInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkVideoEncodeH264RateControlFlagsKHR,
    pub gopFrameCount: u32,
    pub idrPeriod: u32,
    pub consecutiveBFrameCount: u32,
    pub temporalLayerCount: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoEncodeH264RateControlLayerInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub useMinQp: VkBool32,
    pub minQp: VkVideoEncodeH264QpKHR,
    pub useMaxQp: VkBool32,
    pub maxQp: VkVideoEncodeH264QpKHR,
    pub useMaxFrameSize: VkBool32,
    pub maxFrameSize: VkVideoEncodeH264FrameSizeKHR,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoEncodeH264SessionCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub useMaxLevelIdc: VkBool32,
    pub maxLevelIdc: StdVideoH264LevelIdc,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoEncodeH264SessionParametersAddInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub stdSPSCount: u32,
    pub pStdSPSs: *const StdVideoH264SequenceParameterSet,
    pub stdPPSCount: u32,
    pub pStdPPSs: *const StdVideoH264PictureParameterSet,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoEncodeH264SessionParametersCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub maxStdSPSCount: u32,
    pub maxStdPPSCount: u32,
    pub pParametersAddInfo: *const VkVideoEncodeH264SessionParametersAddInfoKHR,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoEncodeH264SessionParametersFeedbackInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub hasStdSPSOverrides: VkBool32,
    pub hasStdPPSOverrides: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoEncodeH264SessionParametersGetInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub writeStdSPS: VkBool32,
    pub writeStdPPS: VkBool32,
    pub stdSPSId: u32,
    pub stdPPSId: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoEncodeH265CapabilitiesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub flags: VkVideoEncodeH265CapabilityFlagsKHR,
    pub maxLevelIdc: StdVideoH265LevelIdc,
    pub maxSliceSegmentCount: u32,
    pub maxTiles: VkExtent2D,
    pub ctbSizes: VkVideoEncodeH265CtbSizeFlagsKHR,
    pub transformBlockSizes: VkVideoEncodeH265TransformBlockSizeFlagsKHR,
    pub maxPPictureL0ReferenceCount: u32,
    pub maxBPictureL0ReferenceCount: u32,
    pub maxL1ReferenceCount: u32,
    pub maxSubLayerCount: u32,
    pub expectDyadicTemporalSubLayerPattern: VkBool32,
    pub minQp: i32,
    pub maxQp: i32,
    pub prefersGopRemainingFrames: VkBool32,
    pub requiresGopRemainingFrames: VkBool32,
    pub stdSyntaxFlags: VkVideoEncodeH265StdFlagsKHR,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoEncodeH265DpbSlotInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub pStdReferenceInfo: *const StdVideoEncodeH265ReferenceInfo,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoEncodeH265FrameSizeKHR {
    pub frameISize: u32,
    pub framePSize: u32,
    pub frameBSize: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoEncodeH265GopRemainingFrameInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub useGopRemainingFrames: VkBool32,
    pub gopRemainingI: u32,
    pub gopRemainingP: u32,
    pub gopRemainingB: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoEncodeH265NaluSliceSegmentInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub constantQp: i32,
    pub pStdSliceSegmentHeader: *const StdVideoEncodeH265SliceSegmentHeader,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoEncodeH265PictureInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub naluSliceSegmentEntryCount: u32,
    pub pNaluSliceSegmentEntries: *const VkVideoEncodeH265NaluSliceSegmentInfoKHR,
    pub pStdPictureInfo: *const StdVideoEncodeH265PictureInfo,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoEncodeH265ProfileInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub stdProfileIdc: StdVideoH265ProfileIdc,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoEncodeH265QpKHR {
    pub qpI: i32,
    pub qpP: i32,
    pub qpB: i32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoEncodeH265QualityLevelPropertiesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub preferredRateControlFlags: VkVideoEncodeH265RateControlFlagsKHR,
    pub preferredGopFrameCount: u32,
    pub preferredIdrPeriod: u32,
    pub preferredConsecutiveBFrameCount: u32,
    pub preferredSubLayerCount: u32,
    pub preferredConstantQp: VkVideoEncodeH265QpKHR,
    pub preferredMaxL0ReferenceCount: u32,
    pub preferredMaxL1ReferenceCount: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoEncodeH265QuantizationMapCapabilitiesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub minQpDelta: i32,
    pub maxQpDelta: i32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoEncodeH265RateControlInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkVideoEncodeH265RateControlFlagsKHR,
    pub gopFrameCount: u32,
    pub idrPeriod: u32,
    pub consecutiveBFrameCount: u32,
    pub subLayerCount: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoEncodeH265RateControlLayerInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub useMinQp: VkBool32,
    pub minQp: VkVideoEncodeH265QpKHR,
    pub useMaxQp: VkBool32,
    pub maxQp: VkVideoEncodeH265QpKHR,
    pub useMaxFrameSize: VkBool32,
    pub maxFrameSize: VkVideoEncodeH265FrameSizeKHR,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoEncodeH265SessionCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub useMaxLevelIdc: VkBool32,
    pub maxLevelIdc: StdVideoH265LevelIdc,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoEncodeH265SessionParametersAddInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub stdVPSCount: u32,
    pub pStdVPSs: *const StdVideoH265VideoParameterSet,
    pub stdSPSCount: u32,
    pub pStdSPSs: *const StdVideoH265SequenceParameterSet,
    pub stdPPSCount: u32,
    pub pStdPPSs: *const StdVideoH265PictureParameterSet,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoEncodeH265SessionParametersCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub maxStdVPSCount: u32,
    pub maxStdSPSCount: u32,
    pub maxStdPPSCount: u32,
    pub pParametersAddInfo: *const VkVideoEncodeH265SessionParametersAddInfoKHR,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoEncodeH265SessionParametersFeedbackInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub hasStdVPSOverrides: VkBool32,
    pub hasStdSPSOverrides: VkBool32,
    pub hasStdPPSOverrides: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoEncodeH265SessionParametersGetInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub writeStdVPS: VkBool32,
    pub writeStdSPS: VkBool32,
    pub writeStdPPS: VkBool32,
    pub stdVPSId: u32,
    pub stdSPSId: u32,
    pub stdPPSId: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoEncodeInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkVideoEncodeFlagsKHR,
    pub dstBuffer: VkBuffer,
    pub dstBufferOffset: VkDeviceSize,
    pub dstBufferRange: VkDeviceSize,
    pub srcPictureResource: VkVideoPictureResourceInfoKHR,
    pub pSetupReferenceSlot: *const VkVideoReferenceSlotInfoKHR,
    pub referenceSlotCount: u32,
    pub pReferenceSlots: *const VkVideoReferenceSlotInfoKHR,
    pub precedingExternallyEncodedBytes: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoEncodeIntraRefreshCapabilitiesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub intraRefreshModes: VkVideoEncodeIntraRefreshModeFlagsKHR,
    pub maxIntraRefreshCycleDuration: u32,
    pub maxIntraRefreshActiveReferencePictures: u32,
    pub partitionIndependentIntraRefreshRegions: VkBool32,
    pub nonRectangularIntraRefreshRegions: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoEncodeIntraRefreshInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub intraRefreshCycleDuration: u32,
    pub intraRefreshIndex: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoEncodeProfileRgbConversionInfoVALVE {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub performEncodeRgbConversion: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoEncodeQualityLevelInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub qualityLevel: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoEncodeQualityLevelPropertiesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub preferredRateControlMode: VkVideoEncodeRateControlModeFlagBitsKHR,
    pub preferredRateControlLayerCount: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoEncodeQuantizationMapCapabilitiesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub maxQuantizationMapExtent: VkExtent2D,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoEncodeQuantizationMapInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub quantizationMap: VkImageView,
    pub quantizationMapExtent: VkExtent2D,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoEncodeQuantizationMapSessionParametersCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub quantizationMapTexelSize: VkExtent2D,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoEncodeRateControlInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkVideoEncodeRateControlFlagsKHR,
    pub rateControlMode: VkVideoEncodeRateControlModeFlagBitsKHR,
    pub layerCount: u32,
    pub pLayers: *const VkVideoEncodeRateControlLayerInfoKHR,
    pub virtualBufferSizeInMs: u32,
    pub initialVirtualBufferSizeInMs: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoEncodeRateControlLayerInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub averageBitrate: u64,
    pub maxBitrate: u64,
    pub frameRateNumerator: u32,
    pub frameRateDenominator: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoEncodeRgbConversionCapabilitiesVALVE {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub rgbModels: VkVideoEncodeRgbModelConversionFlagsVALVE,
    pub rgbRanges: VkVideoEncodeRgbRangeCompressionFlagsVALVE,
    pub xChromaOffsets: VkVideoEncodeRgbChromaOffsetFlagsVALVE,
    pub yChromaOffsets: VkVideoEncodeRgbChromaOffsetFlagsVALVE,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoEncodeSessionIntraRefreshCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub intraRefreshMode: VkVideoEncodeIntraRefreshModeFlagBitsKHR,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoEncodeSessionParametersFeedbackInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub hasOverrides: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoEncodeSessionParametersGetInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub videoSessionParameters: VkVideoSessionParametersKHR,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoEncodeSessionRgbConversionCreateInfoVALVE {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub rgbModel: VkVideoEncodeRgbModelConversionFlagBitsVALVE,
    pub rgbRange: VkVideoEncodeRgbRangeCompressionFlagBitsVALVE,
    pub xChromaOffset: VkVideoEncodeRgbChromaOffsetFlagBitsVALVE,
    pub yChromaOffset: VkVideoEncodeRgbChromaOffsetFlagBitsVALVE,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoEncodeUsageInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub videoUsageHints: VkVideoEncodeUsageFlagsKHR,
    pub videoContentHints: VkVideoEncodeContentFlagsKHR,
    pub tuningMode: VkVideoEncodeTuningModeKHR,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoEndCodingInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkVideoEndCodingFlagsKHR,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoFormatAV1QuantizationMapPropertiesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub compatibleSuperblockSizes: VkVideoEncodeAV1SuperblockSizeFlagsKHR,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoFormatH265QuantizationMapPropertiesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub compatibleCtbSizes: VkVideoEncodeH265CtbSizeFlagsKHR,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoFormatPropertiesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub format: VkFormat,
    pub componentMapping: VkComponentMapping,
    pub imageCreateFlags: VkImageCreateFlags,
    pub imageType: VkImageType,
    pub imageTiling: VkImageTiling,
    pub imageUsageFlags: VkImageUsageFlags,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoFormatQuantizationMapPropertiesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub quantizationMapTexelSize: VkExtent2D,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoInlineQueryInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub queryPool: VkQueryPool,
    pub firstQuery: u32,
    pub queryCount: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoPictureResourceInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub codedOffset: VkOffset2D,
    pub codedExtent: VkExtent2D,
    pub baseArrayLayer: u32,
    pub imageViewBinding: VkImageView,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoProfileInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub videoCodecOperation: VkVideoCodecOperationFlagBitsKHR,
    pub chromaSubsampling: VkVideoChromaSubsamplingFlagsKHR,
    pub lumaBitDepth: VkVideoComponentBitDepthFlagsKHR,
    pub chromaBitDepth: VkVideoComponentBitDepthFlagsKHR,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoProfileListInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub profileCount: u32,
    pub pProfiles: *const VkVideoProfileInfoKHR,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoReferenceIntraRefreshInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub dirtyIntraRefreshRegions: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoReferenceSlotInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub slotIndex: i32,
    pub pPictureResource: *const VkVideoPictureResourceInfoKHR,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoSessionCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub queueFamilyIndex: u32,
    pub flags: VkVideoSessionCreateFlagsKHR,
    pub pVideoProfile: *const VkVideoProfileInfoKHR,
    pub pictureFormat: VkFormat,
    pub maxCodedExtent: VkExtent2D,
    pub referencePictureFormat: VkFormat,
    pub maxDpbSlots: u32,
    pub maxActiveReferencePictures: u32,
    pub pStdHeaderVersion: *const VkExtensionProperties,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoSessionMemoryRequirementsKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub memoryBindIndex: u32,
    pub memoryRequirements: VkMemoryRequirements,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoSessionParametersCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkVideoSessionParametersCreateFlagsKHR,
    pub videoSessionParametersTemplate: VkVideoSessionParametersKHR,
    pub videoSession: VkVideoSessionKHR,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoSessionParametersUpdateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub updateSequenceCount: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkViewport {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub minDepth: f32,
    pub maxDepth: f32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkViewportSwizzleNV {
    pub x: VkViewportCoordinateSwizzleNV,
    pub y: VkViewportCoordinateSwizzleNV,
    pub z: VkViewportCoordinateSwizzleNV,
    pub w: VkViewportCoordinateSwizzleNV,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkViewportWScalingNV {
    pub xcoeff: f32,
    pub ycoeff: f32,
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
pub struct VkWriteDescriptorSetAccelerationStructureKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub accelerationStructureCount: u32,
    pub pAccelerationStructures: *const VkAccelerationStructureKHR,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkWriteDescriptorSetAccelerationStructureNV {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub accelerationStructureCount: u32,
    pub pAccelerationStructures: *const VkAccelerationStructureNV,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkWriteDescriptorSetInlineUniformBlock {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub dataSize: u32,
    pub pData: *const c_void,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkWriteDescriptorSetPartitionedAccelerationStructureNV {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub accelerationStructureCount: u32,
    pub pAccelerationStructures: *const VkDeviceAddress,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkWriteDescriptorSetTensorARM {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub tensorViewCount: u32,
    pub pTensorViews: *const VkTensorViewARM,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkWriteIndirectExecutionSetPipelineEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub index: u32,
    pub pipeline: VkPipeline,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkWriteIndirectExecutionSetShaderEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub index: u32,
    pub shader: VkShaderEXT,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkXYColorEXT {
    pub x: f32,
    pub y: f32,
}

#[cfg_attr(not(doc), repr(u8))]
pub enum VkAccelerationStructureKHR_T {
    #[doc(hidden)]
    __variant1,
    #[doc(hidden)]
    __variant2,
}

#[cfg_attr(not(doc), repr(u8))]
pub enum VkAccelerationStructureNV_T {
    #[doc(hidden)]
    __variant1,
    #[doc(hidden)]
    __variant2,
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
pub enum VkCuFunctionNVX_T {
    #[doc(hidden)]
    __variant1,
    #[doc(hidden)]
    __variant2,
}

#[cfg_attr(not(doc), repr(u8))]
pub enum VkCuModuleNVX_T {
    #[doc(hidden)]
    __variant1,
    #[doc(hidden)]
    __variant2,
}

#[cfg_attr(not(doc), repr(u8))]
pub enum VkDataGraphPipelineSessionARM_T {
    #[doc(hidden)]
    __variant1,
    #[doc(hidden)]
    __variant2,
}

#[cfg_attr(not(doc), repr(u8))]
pub enum VkDebugReportCallbackEXT_T {
    #[doc(hidden)]
    __variant1,
    #[doc(hidden)]
    __variant2,
}

#[cfg_attr(not(doc), repr(u8))]
pub enum VkDebugUtilsMessengerEXT_T {
    #[doc(hidden)]
    __variant1,
    #[doc(hidden)]
    __variant2,
}

#[cfg_attr(not(doc), repr(u8))]
pub enum VkDeferredOperationKHR_T {
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
pub enum VkDisplayKHR_T {
    #[doc(hidden)]
    __variant1,
    #[doc(hidden)]
    __variant2,
}

#[cfg_attr(not(doc), repr(u8))]
pub enum VkDisplayModeKHR_T {
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
pub enum VkExternalComputeQueueNV_T {
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
pub enum VkIndirectCommandsLayoutEXT_T {
    #[doc(hidden)]
    __variant1,
    #[doc(hidden)]
    __variant2,
}

#[cfg_attr(not(doc), repr(u8))]
pub enum VkIndirectCommandsLayoutNV_T {
    #[doc(hidden)]
    __variant1,
    #[doc(hidden)]
    __variant2,
}

#[cfg_attr(not(doc), repr(u8))]
pub enum VkIndirectExecutionSetEXT_T {
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
pub enum VkMicromapEXT_T {
    #[doc(hidden)]
    __variant1,
    #[doc(hidden)]
    __variant2,
}

#[cfg_attr(not(doc), repr(u8))]
pub enum VkOpticalFlowSessionNV_T {
    #[doc(hidden)]
    __variant1,
    #[doc(hidden)]
    __variant2,
}

#[cfg_attr(not(doc), repr(u8))]
pub enum VkPerformanceConfigurationINTEL_T {
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
pub enum VkPipelineBinaryKHR_T {
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
pub enum VkShaderEXT_T {
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

#[cfg_attr(not(doc), repr(u8))]
pub enum VkSurfaceKHR_T {
    #[doc(hidden)]
    __variant1,
    #[doc(hidden)]
    __variant2,
}

#[cfg_attr(not(doc), repr(u8))]
pub enum VkSwapchainKHR_T {
    #[doc(hidden)]
    __variant1,
    #[doc(hidden)]
    __variant2,
}

#[cfg_attr(not(doc), repr(u8))]
pub enum VkTensorARM_T {
    #[doc(hidden)]
    __variant1,
    #[doc(hidden)]
    __variant2,
}

#[cfg_attr(not(doc), repr(u8))]
pub enum VkTensorViewARM_T {
    #[doc(hidden)]
    __variant1,
    #[doc(hidden)]
    __variant2,
}

#[cfg_attr(not(doc), repr(u8))]
pub enum VkValidationCacheEXT_T {
    #[doc(hidden)]
    __variant1,
    #[doc(hidden)]
    __variant2,
}

#[cfg_attr(not(doc), repr(u8))]
pub enum VkVideoSessionKHR_T {
    #[doc(hidden)]
    __variant1,
    #[doc(hidden)]
    __variant2,
}

#[cfg_attr(not(doc), repr(u8))]
pub enum VkVideoSessionParametersKHR_T {
    #[doc(hidden)]
    __variant1,
    #[doc(hidden)]
    __variant2,
}

pub type NonNullPFN_vkAllocationFunction = unsafe extern "system" fn(pUserData: *mut c_void, size: usize, alignment: usize, allocationScope: VkSystemAllocationScope) -> *mut c_void;
pub type NonNullPFN_vkDebugReportCallbackEXT = unsafe extern "system" fn(flags: VkDebugReportFlagsEXT, objectType: VkDebugReportObjectTypeEXT, object: u64, location: usize, messageCode: i32, pLayerPrefix: *const c_char, pMessage: *const c_char, pUserData: *mut c_void) -> VkBool32;
pub type NonNullPFN_vkDebugUtilsMessengerCallbackEXT = unsafe extern "system" fn(messageSeverity: VkDebugUtilsMessageSeverityFlagBitsEXT, messageTypes: VkDebugUtilsMessageTypeFlagsEXT, pCallbackData: *const VkDebugUtilsMessengerCallbackDataEXT, pUserData: *mut c_void) -> VkBool32;
pub type NonNullPFN_vkDeviceMemoryReportCallbackEXT = unsafe extern "system" fn(pCallbackData: *const VkDeviceMemoryReportCallbackDataEXT, pUserData: *mut c_void);
pub type NonNullPFN_vkFreeFunction = unsafe extern "system" fn(pUserData: *mut c_void, pMemory: *mut c_void);
pub type NonNullPFN_vkGetInstanceProcAddrLUNARG = unsafe extern "system" fn(instance: VkInstance, pName: *const c_char) -> PFN_vkVoidFunction;
pub type NonNullPFN_vkInternalAllocationNotification = unsafe extern "system" fn(pUserData: *mut c_void, size: usize, allocationType: VkInternalAllocationType, allocationScope: VkSystemAllocationScope);
pub type NonNullPFN_vkInternalFreeNotification = unsafe extern "system" fn(pUserData: *mut c_void, size: usize, allocationType: VkInternalAllocationType, allocationScope: VkSystemAllocationScope);
pub type NonNullPFN_vkReallocationFunction = unsafe extern "system" fn(pUserData: *mut c_void, pOriginal: *mut c_void, size: usize, alignment: usize, allocationScope: VkSystemAllocationScope) -> *mut c_void;
pub type NonNullPFN_vkVoidFunction = unsafe extern "system" fn();
pub type NonNullVkAccelerationStructureKHR = NonNull<VkAccelerationStructureKHR_T>;
pub type NonNullVkAccelerationStructureNV = NonNull<VkAccelerationStructureNV_T>;
pub type NonNullVkBuffer = NonNull<VkBuffer_T>;
pub type NonNullVkBufferView = NonNull<VkBufferView_T>;
pub type NonNullVkCommandBuffer = NonNull<VkCommandBuffer_T>;
pub type NonNullVkCommandPool = NonNull<VkCommandPool_T>;
pub type NonNullVkCuFunctionNVX = NonNull<VkCuFunctionNVX_T>;
pub type NonNullVkCuModuleNVX = NonNull<VkCuModuleNVX_T>;
pub type NonNullVkDataGraphPipelineSessionARM = NonNull<VkDataGraphPipelineSessionARM_T>;
pub type NonNullVkDebugReportCallbackEXT = NonNull<VkDebugReportCallbackEXT_T>;
pub type NonNullVkDebugUtilsMessengerEXT = NonNull<VkDebugUtilsMessengerEXT_T>;
pub type NonNullVkDeferredOperationKHR = NonNull<VkDeferredOperationKHR_T>;
pub type NonNullVkDescriptorPool = NonNull<VkDescriptorPool_T>;
pub type NonNullVkDescriptorSet = NonNull<VkDescriptorSet_T>;
pub type NonNullVkDescriptorSetLayout = NonNull<VkDescriptorSetLayout_T>;
pub type NonNullVkDescriptorUpdateTemplate = NonNull<VkDescriptorUpdateTemplate_T>;
pub type NonNullVkDevice = NonNull<VkDevice_T>;
pub type NonNullVkDeviceMemory = NonNull<VkDeviceMemory_T>;
pub type NonNullVkDisplayKHR = NonNull<VkDisplayKHR_T>;
pub type NonNullVkDisplayModeKHR = NonNull<VkDisplayModeKHR_T>;
pub type NonNullVkEvent = NonNull<VkEvent_T>;
pub type NonNullVkExternalComputeQueueNV = NonNull<VkExternalComputeQueueNV_T>;
pub type NonNullVkFence = NonNull<VkFence_T>;
pub type NonNullVkFramebuffer = NonNull<VkFramebuffer_T>;
pub type NonNullVkImage = NonNull<VkImage_T>;
pub type NonNullVkImageView = NonNull<VkImageView_T>;
pub type NonNullVkIndirectCommandsLayoutEXT = NonNull<VkIndirectCommandsLayoutEXT_T>;
pub type NonNullVkIndirectCommandsLayoutNV = NonNull<VkIndirectCommandsLayoutNV_T>;
pub type NonNullVkIndirectExecutionSetEXT = NonNull<VkIndirectExecutionSetEXT_T>;
pub type NonNullVkInstance = NonNull<VkInstance_T>;
pub type NonNullVkMicromapEXT = NonNull<VkMicromapEXT_T>;
pub type NonNullVkOpticalFlowSessionNV = NonNull<VkOpticalFlowSessionNV_T>;
pub type NonNullVkPerformanceConfigurationINTEL = NonNull<VkPerformanceConfigurationINTEL_T>;
pub type NonNullVkPhysicalDevice = NonNull<VkPhysicalDevice_T>;
pub type NonNullVkPipeline = NonNull<VkPipeline_T>;
pub type NonNullVkPipelineBinaryKHR = NonNull<VkPipelineBinaryKHR_T>;
pub type NonNullVkPipelineCache = NonNull<VkPipelineCache_T>;
pub type NonNullVkPipelineLayout = NonNull<VkPipelineLayout_T>;
pub type NonNullVkPrivateDataSlot = NonNull<VkPrivateDataSlot_T>;
pub type NonNullVkQueryPool = NonNull<VkQueryPool_T>;
pub type NonNullVkQueue = NonNull<VkQueue_T>;
pub type NonNullVkRenderPass = NonNull<VkRenderPass_T>;
pub type NonNullVkSampler = NonNull<VkSampler_T>;
pub type NonNullVkSamplerYcbcrConversion = NonNull<VkSamplerYcbcrConversion_T>;
pub type NonNullVkSemaphore = NonNull<VkSemaphore_T>;
pub type NonNullVkShaderEXT = NonNull<VkShaderEXT_T>;
pub type NonNullVkShaderModule = NonNull<VkShaderModule_T>;
pub type NonNullVkSurfaceKHR = NonNull<VkSurfaceKHR_T>;
pub type NonNullVkSwapchainKHR = NonNull<VkSwapchainKHR_T>;
pub type NonNullVkTensorARM = NonNull<VkTensorARM_T>;
pub type NonNullVkTensorViewARM = NonNull<VkTensorViewARM_T>;
pub type NonNullVkValidationCacheEXT = NonNull<VkValidationCacheEXT_T>;
pub type NonNullVkVideoSessionKHR = NonNull<VkVideoSessionKHR_T>;
pub type NonNullVkVideoSessionParametersKHR = NonNull<VkVideoSessionParametersKHR_T>;
pub type PFN_vkAllocationFunction = Option<NonNullPFN_vkAllocationFunction>;
pub type PFN_vkDebugReportCallbackEXT = Option<NonNullPFN_vkDebugReportCallbackEXT>;
pub type PFN_vkDebugUtilsMessengerCallbackEXT = Option<NonNullPFN_vkDebugUtilsMessengerCallbackEXT>;
pub type PFN_vkDeviceMemoryReportCallbackEXT = Option<NonNullPFN_vkDeviceMemoryReportCallbackEXT>;
pub type PFN_vkFreeFunction = Option<NonNullPFN_vkFreeFunction>;
pub type PFN_vkGetInstanceProcAddrLUNARG = Option<NonNullPFN_vkGetInstanceProcAddrLUNARG>;
pub type PFN_vkInternalAllocationNotification = Option<NonNullPFN_vkInternalAllocationNotification>;
pub type PFN_vkInternalFreeNotification = Option<NonNullPFN_vkInternalFreeNotification>;
pub type PFN_vkReallocationFunction = Option<NonNullPFN_vkReallocationFunction>;
pub type PFN_vkVoidFunction = Option<NonNullPFN_vkVoidFunction>;
pub type VkAabbPositionsNV = VkAabbPositionsKHR;
pub type VkAccelerationStructureBuildTypeKHR = i32;
pub type VkAccelerationStructureCompatibilityKHR = i32;
pub type VkAccelerationStructureCreateFlagBitsKHR = VkFlags;
pub type VkAccelerationStructureCreateFlagsKHR = VkFlags;
pub type VkAccelerationStructureInstanceNV = VkAccelerationStructureInstanceKHR;
pub type VkAccelerationStructureKHR = *mut VkAccelerationStructureKHR_T;
pub type VkAccelerationStructureMemoryRequirementsTypeNV = i32;
pub type VkAccelerationStructureMotionInfoFlagsNV = VkFlags;
pub type VkAccelerationStructureMotionInstanceFlagsNV = VkFlags;
pub type VkAccelerationStructureMotionInstanceTypeNV = i32;
pub type VkAccelerationStructureNV = *mut VkAccelerationStructureNV_T;
pub type VkAccelerationStructureTypeKHR = i32;
pub type VkAccelerationStructureTypeNV = VkAccelerationStructureTypeKHR;
pub type VkAccessFlagBits = VkFlags;
pub type VkAccessFlagBits2 = VkFlags64;
pub type VkAccessFlagBits2KHR = VkAccessFlagBits2;
pub type VkAccessFlagBits3KHR = VkFlags64;
pub type VkAccessFlags = VkFlags;
pub type VkAccessFlags2 = VkFlags64;
pub type VkAccessFlags2KHR = VkAccessFlags2;
pub type VkAccessFlags3KHR = VkFlags64;
pub type VkAcquireProfilingLockFlagBitsKHR = VkFlags;
pub type VkAcquireProfilingLockFlagsKHR = VkFlags;
pub type VkAddressCopyFlagBitsKHR = VkFlags;
pub type VkAddressCopyFlagsKHR = VkFlags;
pub type VkAntiLagModeAMD = i32;
pub type VkAntiLagStageAMD = i32;
pub type VkAttachmentDescription2KHR = VkAttachmentDescription2;
pub type VkAttachmentDescriptionFlagBits = VkFlags;
pub type VkAttachmentDescriptionFlags = VkFlags;
pub type VkAttachmentDescriptionStencilLayoutKHR = VkAttachmentDescriptionStencilLayout;
pub type VkAttachmentLoadOp = i32;
pub type VkAttachmentReference2KHR = VkAttachmentReference2;
pub type VkAttachmentReferenceStencilLayoutKHR = VkAttachmentReferenceStencilLayout;
pub type VkAttachmentSampleCountInfoNV = VkAttachmentSampleCountInfoAMD;
pub type VkAttachmentStoreOp = i32;
pub type VkBindBufferMemoryDeviceGroupInfoKHR = VkBindBufferMemoryDeviceGroupInfo;
pub type VkBindBufferMemoryInfoKHR = VkBindBufferMemoryInfo;
pub type VkBindDescriptorSetsInfoKHR = VkBindDescriptorSetsInfo;
pub type VkBindImageMemoryDeviceGroupInfoKHR = VkBindImageMemoryDeviceGroupInfo;
pub type VkBindImageMemoryInfoKHR = VkBindImageMemoryInfo;
pub type VkBindImagePlaneMemoryInfoKHR = VkBindImagePlaneMemoryInfo;
pub type VkBindMemoryStatusKHR = VkBindMemoryStatus;
pub type VkBlendFactor = i32;
pub type VkBlendOp = i32;
pub type VkBlendOverlapEXT = i32;
pub type VkBlitImageInfo2KHR = VkBlitImageInfo2;
pub type VkBlockMatchWindowCompareModeQCOM = i32;
pub type VkBool32 = u32;
pub type VkBorderColor = i32;
pub type VkBuffer = *mut VkBuffer_T;
pub type VkBufferCopy2KHR = VkBufferCopy2;
pub type VkBufferCreateFlagBits = VkFlags;
pub type VkBufferCreateFlags = VkFlags;
pub type VkBufferDeviceAddressInfoEXT = VkBufferDeviceAddressInfo;
pub type VkBufferDeviceAddressInfoKHR = VkBufferDeviceAddressInfo;
pub type VkBufferImageCopy2KHR = VkBufferImageCopy2;
pub type VkBufferMemoryBarrier2KHR = VkBufferMemoryBarrier2;
pub type VkBufferMemoryRequirementsInfo2KHR = VkBufferMemoryRequirementsInfo2;
pub type VkBufferOpaqueCaptureAddressCreateInfoKHR = VkBufferOpaqueCaptureAddressCreateInfo;
pub type VkBufferUsageFlagBits = VkFlags;
pub type VkBufferUsageFlagBits2 = VkFlags64;
pub type VkBufferUsageFlagBits2KHR = VkBufferUsageFlagBits2;
pub type VkBufferUsageFlags = VkFlags;
pub type VkBufferUsageFlags2 = VkFlags64;
pub type VkBufferUsageFlags2CreateInfoKHR = VkBufferUsageFlags2CreateInfo;
pub type VkBufferUsageFlags2KHR = VkBufferUsageFlags2;
pub type VkBufferView = *mut VkBufferView_T;
pub type VkBufferViewCreateFlags = VkFlags;
pub type VkBuildAccelerationStructureFlagBitsKHR = VkFlags;
pub type VkBuildAccelerationStructureFlagBitsNV = VkBuildAccelerationStructureFlagBitsKHR;
pub type VkBuildAccelerationStructureFlagsKHR = VkFlags;
pub type VkBuildAccelerationStructureFlagsNV = VkBuildAccelerationStructureFlagsKHR;
pub type VkBuildAccelerationStructureModeKHR = i32;
pub type VkBuildMicromapFlagBitsEXT = VkFlags;
pub type VkBuildMicromapFlagsEXT = VkFlags;
pub type VkBuildMicromapModeEXT = i32;
pub type VkCalibratedTimestampInfoEXT = VkCalibratedTimestampInfoKHR;
pub type VkChromaLocation = i32;
pub type VkChromaLocationKHR = VkChromaLocation;
pub type VkClusterAccelerationStructureAddressResolutionFlagBitsNV = VkFlags;
pub type VkClusterAccelerationStructureAddressResolutionFlagsNV = VkFlags;
pub type VkClusterAccelerationStructureClusterFlagBitsNV = VkFlags;
pub type VkClusterAccelerationStructureClusterFlagsNV = VkFlags;
pub type VkClusterAccelerationStructureGeometryFlagBitsNV = VkFlags;
pub type VkClusterAccelerationStructureGeometryFlagsNV = VkFlags;
pub type VkClusterAccelerationStructureIndexFormatFlagBitsNV = VkFlags;
pub type VkClusterAccelerationStructureIndexFormatFlagsNV = VkFlags;
pub type VkClusterAccelerationStructureOpModeNV = i32;
pub type VkClusterAccelerationStructureOpTypeNV = i32;
pub type VkClusterAccelerationStructureTypeNV = i32;
pub type VkCoarseSampleOrderTypeNV = i32;
pub type VkColorComponentFlagBits = VkFlags;
pub type VkColorComponentFlags = VkFlags;
pub type VkColorSpaceKHR = i32;
pub type VkCommandBuffer = *mut VkCommandBuffer_T;
pub type VkCommandBufferInheritanceRenderingInfoKHR = VkCommandBufferInheritanceRenderingInfo;
pub type VkCommandBufferLevel = i32;
pub type VkCommandBufferResetFlagBits = VkFlags;
pub type VkCommandBufferResetFlags = VkFlags;
pub type VkCommandBufferSubmitInfoKHR = VkCommandBufferSubmitInfo;
pub type VkCommandBufferUsageFlagBits = VkFlags;
pub type VkCommandBufferUsageFlags = VkFlags;
pub type VkCommandPool = *mut VkCommandPool_T;
pub type VkCommandPoolCreateFlagBits = VkFlags;
pub type VkCommandPoolCreateFlags = VkFlags;
pub type VkCommandPoolResetFlagBits = VkFlags;
pub type VkCommandPoolResetFlags = VkFlags;
pub type VkCommandPoolTrimFlags = VkFlags;
pub type VkCommandPoolTrimFlagsKHR = VkCommandPoolTrimFlags;
pub type VkCompareOp = i32;
pub type VkComponentSwizzle = i32;
pub type VkComponentTypeKHR = i32;
pub type VkComponentTypeNV = VkComponentTypeKHR;
pub type VkCompositeAlphaFlagBitsKHR = VkFlags;
pub type VkCompositeAlphaFlagsKHR = VkFlags;
pub type VkConditionalRenderingFlagBitsEXT = VkFlags;
pub type VkConditionalRenderingFlagsEXT = VkFlags;
pub type VkConformanceVersionKHR = VkConformanceVersion;
pub type VkConservativeRasterizationModeEXT = i32;
pub type VkCooperativeVectorMatrixLayoutNV = i32;
pub type VkCopyAccelerationStructureModeKHR = i32;
pub type VkCopyAccelerationStructureModeNV = VkCopyAccelerationStructureModeKHR;
pub type VkCopyBufferInfo2KHR = VkCopyBufferInfo2;
pub type VkCopyBufferToImageInfo2KHR = VkCopyBufferToImageInfo2;
pub type VkCopyImageInfo2KHR = VkCopyImageInfo2;
pub type VkCopyImageToBufferInfo2KHR = VkCopyImageToBufferInfo2;
pub type VkCopyImageToImageInfoEXT = VkCopyImageToImageInfo;
pub type VkCopyImageToMemoryInfoEXT = VkCopyImageToMemoryInfo;
pub type VkCopyMemoryIndirectCommandNV = VkCopyMemoryIndirectCommandKHR;
pub type VkCopyMemoryToImageIndirectCommandNV = VkCopyMemoryToImageIndirectCommandKHR;
pub type VkCopyMemoryToImageInfoEXT = VkCopyMemoryToImageInfo;
pub type VkCopyMicromapModeEXT = i32;
pub type VkCoverageModulationModeNV = i32;
pub type VkCoverageReductionModeNV = i32;
pub type VkCuFunctionNVX = *mut VkCuFunctionNVX_T;
pub type VkCuModuleNVX = *mut VkCuModuleNVX_T;
pub type VkCubicFilterWeightsQCOM = i32;
pub type VkCullModeFlagBits = VkFlags;
pub type VkCullModeFlags = VkFlags;
pub type VkDataGraphModelCacheTypeQCOM = i32;
pub type VkDataGraphPipelineDispatchFlagBitsARM = VkFlags64;
pub type VkDataGraphPipelineDispatchFlagsARM = VkFlags64;
pub type VkDataGraphPipelinePropertyARM = i32;
pub type VkDataGraphPipelineSessionARM = *mut VkDataGraphPipelineSessionARM_T;
pub type VkDataGraphPipelineSessionBindPointARM = i32;
pub type VkDataGraphPipelineSessionBindPointTypeARM = i32;
pub type VkDataGraphPipelineSessionCreateFlagBitsARM = VkFlags64;
pub type VkDataGraphPipelineSessionCreateFlagsARM = VkFlags64;
pub type VkDebugReportCallbackEXT = *mut VkDebugReportCallbackEXT_T;
pub type VkDebugReportFlagBitsEXT = VkFlags;
pub type VkDebugReportFlagsEXT = VkFlags;
pub type VkDebugReportObjectTypeEXT = i32;
pub type VkDebugUtilsMessageSeverityFlagBitsEXT = VkFlags;
pub type VkDebugUtilsMessageSeverityFlagsEXT = VkFlags;
pub type VkDebugUtilsMessageTypeFlagBitsEXT = VkFlags;
pub type VkDebugUtilsMessageTypeFlagsEXT = VkFlags;
pub type VkDebugUtilsMessengerCallbackDataFlagsEXT = VkFlags;
pub type VkDebugUtilsMessengerCreateFlagsEXT = VkFlags;
pub type VkDebugUtilsMessengerEXT = *mut VkDebugUtilsMessengerEXT_T;
pub type VkDefaultVertexAttributeValueKHR = i32;
pub type VkDeferredOperationKHR = *mut VkDeferredOperationKHR_T;
pub type VkDependencyFlagBits = VkFlags;
pub type VkDependencyFlags = VkFlags;
pub type VkDependencyInfoKHR = VkDependencyInfo;
pub type VkDepthBiasRepresentationEXT = i32;
pub type VkDepthClampModeEXT = i32;
pub type VkDescriptorBindingFlagBits = VkFlags;
pub type VkDescriptorBindingFlagBitsEXT = VkDescriptorBindingFlagBits;
pub type VkDescriptorBindingFlags = VkFlags;
pub type VkDescriptorBindingFlagsEXT = VkDescriptorBindingFlags;
pub type VkDescriptorPool = *mut VkDescriptorPool_T;
pub type VkDescriptorPoolCreateFlagBits = VkFlags;
pub type VkDescriptorPoolCreateFlags = VkFlags;
pub type VkDescriptorPoolInlineUniformBlockCreateInfoEXT = VkDescriptorPoolInlineUniformBlockCreateInfo;
pub type VkDescriptorPoolResetFlags = VkFlags;
pub type VkDescriptorSet = *mut VkDescriptorSet_T;
pub type VkDescriptorSetLayout = *mut VkDescriptorSetLayout_T;
pub type VkDescriptorSetLayoutBindingFlagsCreateInfoEXT = VkDescriptorSetLayoutBindingFlagsCreateInfo;
pub type VkDescriptorSetLayoutCreateFlagBits = VkFlags;
pub type VkDescriptorSetLayoutCreateFlags = VkFlags;
pub type VkDescriptorSetLayoutSupportKHR = VkDescriptorSetLayoutSupport;
pub type VkDescriptorSetVariableDescriptorCountAllocateInfoEXT = VkDescriptorSetVariableDescriptorCountAllocateInfo;
pub type VkDescriptorSetVariableDescriptorCountLayoutSupportEXT = VkDescriptorSetVariableDescriptorCountLayoutSupport;
pub type VkDescriptorType = i32;
pub type VkDescriptorUpdateTemplate = *mut VkDescriptorUpdateTemplate_T;
pub type VkDescriptorUpdateTemplateCreateFlags = VkFlags;
pub type VkDescriptorUpdateTemplateCreateFlagsKHR = VkDescriptorUpdateTemplateCreateFlags;
pub type VkDescriptorUpdateTemplateCreateInfoKHR = VkDescriptorUpdateTemplateCreateInfo;
pub type VkDescriptorUpdateTemplateEntryKHR = VkDescriptorUpdateTemplateEntry;
pub type VkDescriptorUpdateTemplateKHR = VkDescriptorUpdateTemplate;
pub type VkDescriptorUpdateTemplateType = i32;
pub type VkDescriptorUpdateTemplateTypeKHR = VkDescriptorUpdateTemplateType;
pub type VkDevice = *mut VkDevice_T;
pub type VkDeviceAddress = u64;
pub type VkDeviceAddressBindingFlagBitsEXT = VkFlags;
pub type VkDeviceAddressBindingFlagsEXT = VkFlags;
pub type VkDeviceAddressBindingTypeEXT = i32;
pub type VkDeviceBufferMemoryRequirementsKHR = VkDeviceBufferMemoryRequirements;
pub type VkDeviceCreateFlags = VkFlags;
pub type VkDeviceDiagnosticsConfigFlagBitsNV = VkFlags;
pub type VkDeviceDiagnosticsConfigFlagsNV = VkFlags;
pub type VkDeviceEventTypeEXT = i32;
pub type VkDeviceFaultAddressTypeEXT = i32;
pub type VkDeviceFaultVendorBinaryHeaderVersionEXT = i32;
pub type VkDeviceGroupBindSparseInfoKHR = VkDeviceGroupBindSparseInfo;
pub type VkDeviceGroupCommandBufferBeginInfoKHR = VkDeviceGroupCommandBufferBeginInfo;
pub type VkDeviceGroupDeviceCreateInfoKHR = VkDeviceGroupDeviceCreateInfo;
pub type VkDeviceGroupPresentModeFlagBitsKHR = VkFlags;
pub type VkDeviceGroupPresentModeFlagsKHR = VkFlags;
pub type VkDeviceGroupRenderPassBeginInfoKHR = VkDeviceGroupRenderPassBeginInfo;
pub type VkDeviceGroupSubmitInfoKHR = VkDeviceGroupSubmitInfo;
pub type VkDeviceImageMemoryRequirementsKHR = VkDeviceImageMemoryRequirements;
pub type VkDeviceImageSubresourceInfoKHR = VkDeviceImageSubresourceInfo;
pub type VkDeviceMemory = *mut VkDeviceMemory_T;
pub type VkDeviceMemoryOpaqueCaptureAddressInfoKHR = VkDeviceMemoryOpaqueCaptureAddressInfo;
pub type VkDeviceMemoryReportEventTypeEXT = i32;
pub type VkDeviceMemoryReportFlagsEXT = VkFlags;
pub type VkDevicePrivateDataCreateInfoEXT = VkDevicePrivateDataCreateInfo;
pub type VkDeviceQueueCreateFlagBits = VkFlags;
pub type VkDeviceQueueCreateFlags = VkFlags;
pub type VkDeviceQueueGlobalPriorityCreateInfoEXT = VkDeviceQueueGlobalPriorityCreateInfo;
pub type VkDeviceQueueGlobalPriorityCreateInfoKHR = VkDeviceQueueGlobalPriorityCreateInfo;
pub type VkDeviceSize = u64;
pub type VkDirectDriverLoadingFlagsLUNARG = VkFlags;
pub type VkDirectDriverLoadingModeLUNARG = i32;
pub type VkDiscardRectangleModeEXT = i32;
pub type VkDisplayEventTypeEXT = i32;
pub type VkDisplayKHR = *mut VkDisplayKHR_T;
pub type VkDisplayModeCreateFlagsKHR = VkFlags;
pub type VkDisplayModeKHR = *mut VkDisplayModeKHR_T;
pub type VkDisplayPlaneAlphaFlagBitsKHR = VkFlags;
pub type VkDisplayPlaneAlphaFlagsKHR = VkFlags;
pub type VkDisplayPowerStateEXT = i32;
pub type VkDisplaySurfaceCreateFlagsKHR = VkFlags;
pub type VkDisplaySurfaceStereoTypeNV = i32;
pub type VkDriverId = i32;
pub type VkDriverIdKHR = VkDriverId;
pub type VkDynamicState = i32;
pub type VkEvent = *mut VkEvent_T;
pub type VkEventCreateFlagBits = VkFlags;
pub type VkEventCreateFlags = VkFlags;
pub type VkExportFenceCreateInfoKHR = VkExportFenceCreateInfo;
pub type VkExportMemoryAllocateInfoKHR = VkExportMemoryAllocateInfo;
pub type VkExportSemaphoreCreateInfoKHR = VkExportSemaphoreCreateInfo;
pub type VkExternalBufferPropertiesKHR = VkExternalBufferProperties;
pub type VkExternalComputeQueueNV = *mut VkExternalComputeQueueNV_T;
pub type VkExternalFenceFeatureFlagBits = VkFlags;
pub type VkExternalFenceFeatureFlagBitsKHR = VkExternalFenceFeatureFlagBits;
pub type VkExternalFenceFeatureFlags = VkFlags;
pub type VkExternalFenceFeatureFlagsKHR = VkExternalFenceFeatureFlags;
pub type VkExternalFenceHandleTypeFlagBits = VkFlags;
pub type VkExternalFenceHandleTypeFlagBitsKHR = VkExternalFenceHandleTypeFlagBits;
pub type VkExternalFenceHandleTypeFlags = VkFlags;
pub type VkExternalFenceHandleTypeFlagsKHR = VkExternalFenceHandleTypeFlags;
pub type VkExternalFencePropertiesKHR = VkExternalFenceProperties;
pub type VkExternalImageFormatPropertiesKHR = VkExternalImageFormatProperties;
pub type VkExternalMemoryBufferCreateInfoKHR = VkExternalMemoryBufferCreateInfo;
pub type VkExternalMemoryFeatureFlagBits = VkFlags;
pub type VkExternalMemoryFeatureFlagBitsKHR = VkExternalMemoryFeatureFlagBits;
pub type VkExternalMemoryFeatureFlagBitsNV = VkFlags;
pub type VkExternalMemoryFeatureFlags = VkFlags;
pub type VkExternalMemoryFeatureFlagsKHR = VkExternalMemoryFeatureFlags;
pub type VkExternalMemoryFeatureFlagsNV = VkFlags;
pub type VkExternalMemoryHandleTypeFlagBits = VkFlags;
pub type VkExternalMemoryHandleTypeFlagBitsKHR = VkExternalMemoryHandleTypeFlagBits;
pub type VkExternalMemoryHandleTypeFlagBitsNV = VkFlags;
pub type VkExternalMemoryHandleTypeFlags = VkFlags;
pub type VkExternalMemoryHandleTypeFlagsKHR = VkExternalMemoryHandleTypeFlags;
pub type VkExternalMemoryHandleTypeFlagsNV = VkFlags;
pub type VkExternalMemoryImageCreateInfoKHR = VkExternalMemoryImageCreateInfo;
pub type VkExternalMemoryPropertiesKHR = VkExternalMemoryProperties;
pub type VkExternalSemaphoreFeatureFlagBits = VkFlags;
pub type VkExternalSemaphoreFeatureFlagBitsKHR = VkExternalSemaphoreFeatureFlagBits;
pub type VkExternalSemaphoreFeatureFlags = VkFlags;
pub type VkExternalSemaphoreFeatureFlagsKHR = VkExternalSemaphoreFeatureFlags;
pub type VkExternalSemaphoreHandleTypeFlagBits = VkFlags;
pub type VkExternalSemaphoreHandleTypeFlagBitsKHR = VkExternalSemaphoreHandleTypeFlagBits;
pub type VkExternalSemaphoreHandleTypeFlags = VkFlags;
pub type VkExternalSemaphoreHandleTypeFlagsKHR = VkExternalSemaphoreHandleTypeFlags;
pub type VkExternalSemaphorePropertiesKHR = VkExternalSemaphoreProperties;
pub type VkFence = *mut VkFence_T;
pub type VkFenceCreateFlagBits = VkFlags;
pub type VkFenceCreateFlags = VkFlags;
pub type VkFenceImportFlagBits = VkFlags;
pub type VkFenceImportFlagBitsKHR = VkFenceImportFlagBits;
pub type VkFenceImportFlags = VkFlags;
pub type VkFenceImportFlagsKHR = VkFenceImportFlags;
pub type VkFilter = i32;
pub type VkFlags = u32;
pub type VkFlags64 = u64;
pub type VkFormat = i32;
pub type VkFormatFeatureFlagBits = VkFlags;
pub type VkFormatFeatureFlagBits2 = VkFlags64;
pub type VkFormatFeatureFlagBits2KHR = VkFormatFeatureFlagBits2;
pub type VkFormatFeatureFlags = VkFlags;
pub type VkFormatFeatureFlags2 = VkFlags64;
pub type VkFormatFeatureFlags2KHR = VkFormatFeatureFlags2;
pub type VkFormatProperties2KHR = VkFormatProperties2;
pub type VkFormatProperties3KHR = VkFormatProperties3;
pub type VkFragmentShadingRateCombinerOpKHR = i32;
pub type VkFragmentShadingRateNV = i32;
pub type VkFragmentShadingRateTypeNV = i32;
pub type VkFrameBoundaryFlagBitsEXT = VkFlags;
pub type VkFrameBoundaryFlagsEXT = VkFlags;
pub type VkFramebuffer = *mut VkFramebuffer_T;
pub type VkFramebufferAttachmentImageInfoKHR = VkFramebufferAttachmentImageInfo;
pub type VkFramebufferAttachmentsCreateInfoKHR = VkFramebufferAttachmentsCreateInfo;
pub type VkFramebufferCreateFlagBits = VkFlags;
pub type VkFramebufferCreateFlags = VkFlags;
pub type VkFrontFace = i32;
pub type VkGeometryFlagBitsKHR = VkFlags;
pub type VkGeometryFlagBitsNV = VkGeometryFlagBitsKHR;
pub type VkGeometryFlagsKHR = VkFlags;
pub type VkGeometryFlagsNV = VkGeometryFlagsKHR;
pub type VkGeometryInstanceFlagBitsKHR = VkFlags;
pub type VkGeometryInstanceFlagBitsNV = VkGeometryInstanceFlagBitsKHR;
pub type VkGeometryInstanceFlagsKHR = VkFlags;
pub type VkGeometryInstanceFlagsNV = VkGeometryInstanceFlagsKHR;
pub type VkGeometryTypeKHR = i32;
pub type VkGeometryTypeNV = VkGeometryTypeKHR;
pub type VkGraphicsPipelineLibraryFlagBitsEXT = VkFlags;
pub type VkGraphicsPipelineLibraryFlagsEXT = VkFlags;
pub type VkHeadlessSurfaceCreateFlagsEXT = VkFlags;
pub type VkHostImageCopyDevicePerformanceQueryEXT = VkHostImageCopyDevicePerformanceQuery;
pub type VkHostImageCopyFlagBits = VkFlags;
pub type VkHostImageCopyFlagBitsEXT = VkHostImageCopyFlagBits;
pub type VkHostImageCopyFlags = VkFlags;
pub type VkHostImageCopyFlagsEXT = VkHostImageCopyFlags;
pub type VkHostImageLayoutTransitionInfoEXT = VkHostImageLayoutTransitionInfo;
pub type VkImage = *mut VkImage_T;
pub type VkImageAspectFlagBits = VkFlags;
pub type VkImageAspectFlags = VkFlags;
pub type VkImageBlit2KHR = VkImageBlit2;
pub type VkImageCompressionFixedRateFlagBitsEXT = VkFlags;
pub type VkImageCompressionFixedRateFlagsEXT = VkFlags;
pub type VkImageCompressionFlagBitsEXT = VkFlags;
pub type VkImageCompressionFlagsEXT = VkFlags;
pub type VkImageCopy2KHR = VkImageCopy2;
pub type VkImageCreateFlagBits = VkFlags;
pub type VkImageCreateFlags = VkFlags;
pub type VkImageFormatListCreateInfoKHR = VkImageFormatListCreateInfo;
pub type VkImageFormatProperties2KHR = VkImageFormatProperties2;
pub type VkImageLayout = i32;
pub type VkImageMemoryBarrier2KHR = VkImageMemoryBarrier2;
pub type VkImageMemoryRequirementsInfo2KHR = VkImageMemoryRequirementsInfo2;
pub type VkImagePlaneMemoryRequirementsInfoKHR = VkImagePlaneMemoryRequirementsInfo;
pub type VkImageResolve2KHR = VkImageResolve2;
pub type VkImageSparseMemoryRequirementsInfo2KHR = VkImageSparseMemoryRequirementsInfo2;
pub type VkImageStencilUsageCreateInfoEXT = VkImageStencilUsageCreateInfo;
pub type VkImageSubresource2EXT = VkImageSubresource2;
pub type VkImageSubresource2KHR = VkImageSubresource2;
pub type VkImageTiling = i32;
pub type VkImageToMemoryCopyEXT = VkImageToMemoryCopy;
pub type VkImageType = i32;
pub type VkImageUsageFlagBits = VkFlags;
pub type VkImageUsageFlags = VkFlags;
pub type VkImageView = *mut VkImageView_T;
pub type VkImageViewCreateFlagBits = VkFlags;
pub type VkImageViewCreateFlags = VkFlags;
pub type VkImageViewType = i32;
pub type VkImageViewUsageCreateInfoKHR = VkImageViewUsageCreateInfo;
pub type VkIndexType = i32;
pub type VkIndirectCommandsInputModeFlagBitsEXT = VkFlags;
pub type VkIndirectCommandsInputModeFlagsEXT = VkFlags;
pub type VkIndirectCommandsLayoutEXT = *mut VkIndirectCommandsLayoutEXT_T;
pub type VkIndirectCommandsLayoutNV = *mut VkIndirectCommandsLayoutNV_T;
pub type VkIndirectCommandsLayoutUsageFlagBitsEXT = VkFlags;
pub type VkIndirectCommandsLayoutUsageFlagBitsNV = VkFlags;
pub type VkIndirectCommandsLayoutUsageFlagsEXT = VkFlags;
pub type VkIndirectCommandsLayoutUsageFlagsNV = VkFlags;
pub type VkIndirectCommandsTokenTypeEXT = i32;
pub type VkIndirectCommandsTokenTypeNV = i32;
pub type VkIndirectExecutionSetEXT = *mut VkIndirectExecutionSetEXT_T;
pub type VkIndirectExecutionSetInfoTypeEXT = i32;
pub type VkIndirectStateFlagBitsNV = VkFlags;
pub type VkIndirectStateFlagsNV = VkFlags;
pub type VkInputAttachmentAspectReferenceKHR = VkInputAttachmentAspectReference;
pub type VkInstance = *mut VkInstance_T;
pub type VkInstanceCreateFlagBits = VkFlags;
pub type VkInstanceCreateFlags = VkFlags;
pub type VkInternalAllocationType = i32;
pub type VkLatencyMarkerNV = i32;
pub type VkLayerSettingTypeEXT = i32;
pub type VkLayeredDriverUnderlyingApiMSFT = i32;
pub type VkLineRasterizationMode = i32;
pub type VkLineRasterizationModeEXT = VkLineRasterizationMode;
pub type VkLineRasterizationModeKHR = VkLineRasterizationMode;
pub type VkLogicOp = i32;
pub type VkMemoryAllocateFlagBits = VkFlags;
pub type VkMemoryAllocateFlagBitsKHR = VkMemoryAllocateFlagBits;
pub type VkMemoryAllocateFlags = VkFlags;
pub type VkMemoryAllocateFlagsInfoKHR = VkMemoryAllocateFlagsInfo;
pub type VkMemoryAllocateFlagsKHR = VkMemoryAllocateFlags;
pub type VkMemoryBarrier2KHR = VkMemoryBarrier2;
pub type VkMemoryDecompressionMethodFlagBitsEXT = VkFlags64;
pub type VkMemoryDecompressionMethodFlagBitsNV = VkMemoryDecompressionMethodFlagBitsEXT;
pub type VkMemoryDecompressionMethodFlagsEXT = VkFlags64;
pub type VkMemoryDecompressionMethodFlagsNV = VkMemoryDecompressionMethodFlagsEXT;
pub type VkMemoryDedicatedAllocateInfoKHR = VkMemoryDedicatedAllocateInfo;
pub type VkMemoryDedicatedRequirementsKHR = VkMemoryDedicatedRequirements;
pub type VkMemoryHeapFlagBits = VkFlags;
pub type VkMemoryHeapFlags = VkFlags;
pub type VkMemoryMapFlagBits = VkFlags;
pub type VkMemoryMapFlags = VkFlags;
pub type VkMemoryMapInfoKHR = VkMemoryMapInfo;
pub type VkMemoryOpaqueCaptureAddressAllocateInfoKHR = VkMemoryOpaqueCaptureAddressAllocateInfo;
pub type VkMemoryOverallocationBehaviorAMD = i32;
pub type VkMemoryPropertyFlagBits = VkFlags;
pub type VkMemoryPropertyFlags = VkFlags;
pub type VkMemoryRequirements2KHR = VkMemoryRequirements2;
pub type VkMemoryToImageCopyEXT = VkMemoryToImageCopy;
pub type VkMemoryUnmapFlagBits = VkFlags;
pub type VkMemoryUnmapFlagBitsKHR = VkMemoryUnmapFlagBits;
pub type VkMemoryUnmapFlags = VkFlags;
pub type VkMemoryUnmapFlagsKHR = VkMemoryUnmapFlags;
pub type VkMemoryUnmapInfoKHR = VkMemoryUnmapInfo;
pub type VkMicromapCreateFlagBitsEXT = VkFlags;
pub type VkMicromapCreateFlagsEXT = VkFlags;
pub type VkMicromapEXT = *mut VkMicromapEXT_T;
pub type VkMicromapTypeEXT = i32;
pub type VkMutableDescriptorTypeCreateInfoVALVE = VkMutableDescriptorTypeCreateInfoEXT;
pub type VkMutableDescriptorTypeListVALVE = VkMutableDescriptorTypeListEXT;
pub type VkObjectType = i32;
pub type VkOpacityMicromapFormatEXT = i32;
pub type VkOpacityMicromapSpecialIndexEXT = i32;
pub type VkOpticalFlowExecuteFlagBitsNV = VkFlags;
pub type VkOpticalFlowExecuteFlagsNV = VkFlags;
pub type VkOpticalFlowGridSizeFlagBitsNV = VkFlags;
pub type VkOpticalFlowGridSizeFlagsNV = VkFlags;
pub type VkOpticalFlowPerformanceLevelNV = i32;
pub type VkOpticalFlowSessionBindingPointNV = i32;
pub type VkOpticalFlowSessionCreateFlagBitsNV = VkFlags;
pub type VkOpticalFlowSessionCreateFlagsNV = VkFlags;
pub type VkOpticalFlowSessionNV = *mut VkOpticalFlowSessionNV_T;
pub type VkOpticalFlowUsageFlagBitsNV = VkFlags;
pub type VkOpticalFlowUsageFlagsNV = VkFlags;
pub type VkOutOfBandQueueTypeNV = i32;
pub type VkPartitionedAccelerationStructureInstanceFlagBitsNV = VkFlags;
pub type VkPartitionedAccelerationStructureInstanceFlagsNV = VkFlags;
pub type VkPartitionedAccelerationStructureOpTypeNV = i32;
pub type VkPastPresentationTimingFlagBitsEXT = VkFlags;
pub type VkPastPresentationTimingFlagsEXT = VkFlags;
pub type VkPeerMemoryFeatureFlagBits = VkFlags;
pub type VkPeerMemoryFeatureFlagBitsKHR = VkPeerMemoryFeatureFlagBits;
pub type VkPeerMemoryFeatureFlags = VkFlags;
pub type VkPeerMemoryFeatureFlagsKHR = VkPeerMemoryFeatureFlags;
pub type VkPerformanceConfigurationINTEL = *mut VkPerformanceConfigurationINTEL_T;
pub type VkPerformanceConfigurationTypeINTEL = i32;
pub type VkPerformanceCounterDescriptionFlagBitsKHR = VkFlags;
pub type VkPerformanceCounterDescriptionFlagsARM = VkFlags;
pub type VkPerformanceCounterDescriptionFlagsKHR = VkFlags;
pub type VkPerformanceCounterScopeKHR = i32;
pub type VkPerformanceCounterStorageKHR = i32;
pub type VkPerformanceCounterUnitKHR = i32;
pub type VkPerformanceOverrideTypeINTEL = i32;
pub type VkPerformanceParameterTypeINTEL = i32;
pub type VkPerformanceValueTypeINTEL = i32;
pub type VkPhysicalDevice = *mut VkPhysicalDevice_T;
pub type VkPhysicalDevice16BitStorageFeaturesKHR = VkPhysicalDevice16BitStorageFeatures;
pub type VkPhysicalDevice8BitStorageFeaturesKHR = VkPhysicalDevice8BitStorageFeatures;
pub type VkPhysicalDeviceBufferAddressFeaturesEXT = VkPhysicalDeviceBufferDeviceAddressFeaturesEXT;
pub type VkPhysicalDeviceBufferDeviceAddressFeaturesKHR = VkPhysicalDeviceBufferDeviceAddressFeatures;
pub type VkPhysicalDeviceComputeShaderDerivativesFeaturesNV = VkPhysicalDeviceComputeShaderDerivativesFeaturesKHR;
pub type VkPhysicalDeviceCopyMemoryIndirectPropertiesNV = VkPhysicalDeviceCopyMemoryIndirectPropertiesKHR;
pub type VkPhysicalDeviceDataGraphOperationTypeARM = i32;
pub type VkPhysicalDeviceDataGraphProcessingEngineTypeARM = i32;
pub type VkPhysicalDeviceDepthClampZeroOneFeaturesEXT = VkPhysicalDeviceDepthClampZeroOneFeaturesKHR;
pub type VkPhysicalDeviceDepthStencilResolvePropertiesKHR = VkPhysicalDeviceDepthStencilResolveProperties;
pub type VkPhysicalDeviceDescriptorIndexingFeaturesEXT = VkPhysicalDeviceDescriptorIndexingFeatures;
pub type VkPhysicalDeviceDescriptorIndexingPropertiesEXT = VkPhysicalDeviceDescriptorIndexingProperties;
pub type VkPhysicalDeviceDriverPropertiesKHR = VkPhysicalDeviceDriverProperties;
pub type VkPhysicalDeviceDynamicRenderingFeaturesKHR = VkPhysicalDeviceDynamicRenderingFeatures;
pub type VkPhysicalDeviceDynamicRenderingLocalReadFeaturesKHR = VkPhysicalDeviceDynamicRenderingLocalReadFeatures;
pub type VkPhysicalDeviceExternalBufferInfoKHR = VkPhysicalDeviceExternalBufferInfo;
pub type VkPhysicalDeviceExternalFenceInfoKHR = VkPhysicalDeviceExternalFenceInfo;
pub type VkPhysicalDeviceExternalImageFormatInfoKHR = VkPhysicalDeviceExternalImageFormatInfo;
pub type VkPhysicalDeviceExternalSemaphoreInfoKHR = VkPhysicalDeviceExternalSemaphoreInfo;
pub type VkPhysicalDeviceFeatures2KHR = VkPhysicalDeviceFeatures2;
pub type VkPhysicalDeviceFloat16Int8FeaturesKHR = VkPhysicalDeviceShaderFloat16Int8Features;
pub type VkPhysicalDeviceFloatControlsPropertiesKHR = VkPhysicalDeviceFloatControlsProperties;
pub type VkPhysicalDeviceFragmentDensityMapOffsetFeaturesQCOM = VkPhysicalDeviceFragmentDensityMapOffsetFeaturesEXT;
pub type VkPhysicalDeviceFragmentDensityMapOffsetPropertiesQCOM = VkPhysicalDeviceFragmentDensityMapOffsetPropertiesEXT;
pub type VkPhysicalDeviceFragmentShaderBarycentricFeaturesNV = VkPhysicalDeviceFragmentShaderBarycentricFeaturesKHR;
pub type VkPhysicalDeviceGlobalPriorityQueryFeaturesEXT = VkPhysicalDeviceGlobalPriorityQueryFeatures;
pub type VkPhysicalDeviceGlobalPriorityQueryFeaturesKHR = VkPhysicalDeviceGlobalPriorityQueryFeatures;
pub type VkPhysicalDeviceGroupPropertiesKHR = VkPhysicalDeviceGroupProperties;
pub type VkPhysicalDeviceHostImageCopyFeaturesEXT = VkPhysicalDeviceHostImageCopyFeatures;
pub type VkPhysicalDeviceHostImageCopyPropertiesEXT = VkPhysicalDeviceHostImageCopyProperties;
pub type VkPhysicalDeviceHostQueryResetFeaturesEXT = VkPhysicalDeviceHostQueryResetFeatures;
pub type VkPhysicalDeviceIDPropertiesKHR = VkPhysicalDeviceIDProperties;
pub type VkPhysicalDeviceImageFormatInfo2KHR = VkPhysicalDeviceImageFormatInfo2;
pub type VkPhysicalDeviceImageRobustnessFeaturesEXT = VkPhysicalDeviceImageRobustnessFeatures;
pub type VkPhysicalDeviceImagelessFramebufferFeaturesKHR = VkPhysicalDeviceImagelessFramebufferFeatures;
pub type VkPhysicalDeviceIndexTypeUint8FeaturesEXT = VkPhysicalDeviceIndexTypeUint8Features;
pub type VkPhysicalDeviceIndexTypeUint8FeaturesKHR = VkPhysicalDeviceIndexTypeUint8Features;
pub type VkPhysicalDeviceInlineUniformBlockFeaturesEXT = VkPhysicalDeviceInlineUniformBlockFeatures;
pub type VkPhysicalDeviceInlineUniformBlockPropertiesEXT = VkPhysicalDeviceInlineUniformBlockProperties;
pub type VkPhysicalDeviceLayeredApiKHR = i32;
pub type VkPhysicalDeviceLineRasterizationFeaturesEXT = VkPhysicalDeviceLineRasterizationFeatures;
pub type VkPhysicalDeviceLineRasterizationFeaturesKHR = VkPhysicalDeviceLineRasterizationFeatures;
pub type VkPhysicalDeviceLineRasterizationPropertiesEXT = VkPhysicalDeviceLineRasterizationProperties;
pub type VkPhysicalDeviceLineRasterizationPropertiesKHR = VkPhysicalDeviceLineRasterizationProperties;
pub type VkPhysicalDeviceMaintenance3PropertiesKHR = VkPhysicalDeviceMaintenance3Properties;
pub type VkPhysicalDeviceMaintenance4FeaturesKHR = VkPhysicalDeviceMaintenance4Features;
pub type VkPhysicalDeviceMaintenance4PropertiesKHR = VkPhysicalDeviceMaintenance4Properties;
pub type VkPhysicalDeviceMaintenance5FeaturesKHR = VkPhysicalDeviceMaintenance5Features;
pub type VkPhysicalDeviceMaintenance5PropertiesKHR = VkPhysicalDeviceMaintenance5Properties;
pub type VkPhysicalDeviceMaintenance6FeaturesKHR = VkPhysicalDeviceMaintenance6Features;
pub type VkPhysicalDeviceMaintenance6PropertiesKHR = VkPhysicalDeviceMaintenance6Properties;
pub type VkPhysicalDeviceMemoryDecompressionFeaturesNV = VkPhysicalDeviceMemoryDecompressionFeaturesEXT;
pub type VkPhysicalDeviceMemoryDecompressionPropertiesNV = VkPhysicalDeviceMemoryDecompressionPropertiesEXT;
pub type VkPhysicalDeviceMemoryProperties2KHR = VkPhysicalDeviceMemoryProperties2;
pub type VkPhysicalDeviceMultiviewFeaturesKHR = VkPhysicalDeviceMultiviewFeatures;
pub type VkPhysicalDeviceMultiviewPropertiesKHR = VkPhysicalDeviceMultiviewProperties;
pub type VkPhysicalDeviceMutableDescriptorTypeFeaturesVALVE = VkPhysicalDeviceMutableDescriptorTypeFeaturesEXT;
pub type VkPhysicalDevicePipelineCreationCacheControlFeaturesEXT = VkPhysicalDevicePipelineCreationCacheControlFeatures;
pub type VkPhysicalDevicePipelineProtectedAccessFeaturesEXT = VkPhysicalDevicePipelineProtectedAccessFeatures;
pub type VkPhysicalDevicePipelineRobustnessFeaturesEXT = VkPhysicalDevicePipelineRobustnessFeatures;
pub type VkPhysicalDevicePipelineRobustnessPropertiesEXT = VkPhysicalDevicePipelineRobustnessProperties;
pub type VkPhysicalDevicePointClippingPropertiesKHR = VkPhysicalDevicePointClippingProperties;
pub type VkPhysicalDevicePresentModeFifoLatestReadyFeaturesEXT = VkPhysicalDevicePresentModeFifoLatestReadyFeaturesKHR;
pub type VkPhysicalDevicePrivateDataFeaturesEXT = VkPhysicalDevicePrivateDataFeatures;
pub type VkPhysicalDeviceProperties2KHR = VkPhysicalDeviceProperties2;
pub type VkPhysicalDevicePushDescriptorPropertiesKHR = VkPhysicalDevicePushDescriptorProperties;
pub type VkPhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM = VkPhysicalDeviceRasterizationOrderAttachmentAccessFeaturesEXT;
pub type VkPhysicalDeviceRobustness2FeaturesEXT = VkPhysicalDeviceRobustness2FeaturesKHR;
pub type VkPhysicalDeviceRobustness2PropertiesEXT = VkPhysicalDeviceRobustness2PropertiesKHR;
pub type VkPhysicalDeviceSamplerFilterMinmaxPropertiesEXT = VkPhysicalDeviceSamplerFilterMinmaxProperties;
pub type VkPhysicalDeviceSamplerYcbcrConversionFeaturesKHR = VkPhysicalDeviceSamplerYcbcrConversionFeatures;
pub type VkPhysicalDeviceScalarBlockLayoutFeaturesEXT = VkPhysicalDeviceScalarBlockLayoutFeatures;
pub type VkPhysicalDeviceSchedulingControlsFlagBitsARM = VkFlags64;
pub type VkPhysicalDeviceSchedulingControlsFlagsARM = VkFlags64;
pub type VkPhysicalDeviceSeparateDepthStencilLayoutsFeaturesKHR = VkPhysicalDeviceSeparateDepthStencilLayoutsFeatures;
pub type VkPhysicalDeviceShaderAtomicInt64FeaturesKHR = VkPhysicalDeviceShaderAtomicInt64Features;
pub type VkPhysicalDeviceShaderDemoteToHelperInvocationFeaturesEXT = VkPhysicalDeviceShaderDemoteToHelperInvocationFeatures;
pub type VkPhysicalDeviceShaderDrawParameterFeatures = VkPhysicalDeviceShaderDrawParametersFeatures;
pub type VkPhysicalDeviceShaderExpectAssumeFeaturesKHR = VkPhysicalDeviceShaderExpectAssumeFeatures;
pub type VkPhysicalDeviceShaderFloat16Int8FeaturesKHR = VkPhysicalDeviceShaderFloat16Int8Features;
pub type VkPhysicalDeviceShaderFloatControls2FeaturesKHR = VkPhysicalDeviceShaderFloatControls2Features;
pub type VkPhysicalDeviceShaderIntegerDotProductFeaturesKHR = VkPhysicalDeviceShaderIntegerDotProductFeatures;
pub type VkPhysicalDeviceShaderIntegerDotProductPropertiesKHR = VkPhysicalDeviceShaderIntegerDotProductProperties;
pub type VkPhysicalDeviceShaderSubgroupExtendedTypesFeaturesKHR = VkPhysicalDeviceShaderSubgroupExtendedTypesFeatures;
pub type VkPhysicalDeviceShaderSubgroupRotateFeaturesKHR = VkPhysicalDeviceShaderSubgroupRotateFeatures;
pub type VkPhysicalDeviceShaderTerminateInvocationFeaturesKHR = VkPhysicalDeviceShaderTerminateInvocationFeatures;
pub type VkPhysicalDeviceSparseImageFormatInfo2KHR = VkPhysicalDeviceSparseImageFormatInfo2;
pub type VkPhysicalDeviceSubgroupSizeControlFeaturesEXT = VkPhysicalDeviceSubgroupSizeControlFeatures;
pub type VkPhysicalDeviceSubgroupSizeControlPropertiesEXT = VkPhysicalDeviceSubgroupSizeControlProperties;
pub type VkPhysicalDeviceSwapchainMaintenance1FeaturesEXT = VkPhysicalDeviceSwapchainMaintenance1FeaturesKHR;
pub type VkPhysicalDeviceSynchronization2FeaturesKHR = VkPhysicalDeviceSynchronization2Features;
pub type VkPhysicalDeviceTexelBufferAlignmentPropertiesEXT = VkPhysicalDeviceTexelBufferAlignmentProperties;
pub type VkPhysicalDeviceTextureCompressionASTCHDRFeaturesEXT = VkPhysicalDeviceTextureCompressionASTCHDRFeatures;
pub type VkPhysicalDeviceTimelineSemaphoreFeaturesKHR = VkPhysicalDeviceTimelineSemaphoreFeatures;
pub type VkPhysicalDeviceTimelineSemaphorePropertiesKHR = VkPhysicalDeviceTimelineSemaphoreProperties;
pub type VkPhysicalDeviceToolPropertiesEXT = VkPhysicalDeviceToolProperties;
pub type VkPhysicalDeviceType = i32;
pub type VkPhysicalDeviceUniformBufferStandardLayoutFeaturesKHR = VkPhysicalDeviceUniformBufferStandardLayoutFeatures;
pub type VkPhysicalDeviceVariablePointerFeatures = VkPhysicalDeviceVariablePointersFeatures;
pub type VkPhysicalDeviceVariablePointerFeaturesKHR = VkPhysicalDeviceVariablePointersFeatures;
pub type VkPhysicalDeviceVariablePointersFeaturesKHR = VkPhysicalDeviceVariablePointersFeatures;
pub type VkPhysicalDeviceVertexAttributeDivisorFeaturesEXT = VkPhysicalDeviceVertexAttributeDivisorFeatures;
pub type VkPhysicalDeviceVertexAttributeDivisorFeaturesKHR = VkPhysicalDeviceVertexAttributeDivisorFeatures;
pub type VkPhysicalDeviceVertexAttributeDivisorPropertiesKHR = VkPhysicalDeviceVertexAttributeDivisorProperties;
pub type VkPhysicalDeviceVulkanMemoryModelFeaturesKHR = VkPhysicalDeviceVulkanMemoryModelFeatures;
pub type VkPhysicalDeviceZeroInitializeWorkgroupMemoryFeaturesKHR = VkPhysicalDeviceZeroInitializeWorkgroupMemoryFeatures;
pub type VkPipeline = *mut VkPipeline_T;
pub type VkPipelineBinaryKHR = *mut VkPipelineBinaryKHR_T;
pub type VkPipelineBindPoint = i32;
pub type VkPipelineCache = *mut VkPipelineCache_T;
pub type VkPipelineCacheCreateFlagBits = VkFlags;
pub type VkPipelineCacheCreateFlags = VkFlags;
pub type VkPipelineCacheHeaderVersion = i32;
pub type VkPipelineColorBlendStateCreateFlagBits = VkFlags;
pub type VkPipelineColorBlendStateCreateFlags = VkFlags;
pub type VkPipelineCompilerControlFlagBitsAMD = VkFlags;
pub type VkPipelineCompilerControlFlagsAMD = VkFlags;
pub type VkPipelineCoverageModulationStateCreateFlagsNV = VkFlags;
pub type VkPipelineCoverageReductionStateCreateFlagsNV = VkFlags;
pub type VkPipelineCoverageToColorStateCreateFlagsNV = VkFlags;
pub type VkPipelineCreateFlagBits = VkFlags;
pub type VkPipelineCreateFlagBits2 = VkFlags64;
pub type VkPipelineCreateFlagBits2KHR = VkPipelineCreateFlagBits2;
pub type VkPipelineCreateFlags = VkFlags;
pub type VkPipelineCreateFlags2 = VkFlags64;
pub type VkPipelineCreateFlags2CreateInfoKHR = VkPipelineCreateFlags2CreateInfo;
pub type VkPipelineCreateFlags2KHR = VkPipelineCreateFlags2;
pub type VkPipelineCreationFeedbackCreateInfoEXT = VkPipelineCreationFeedbackCreateInfo;
pub type VkPipelineCreationFeedbackEXT = VkPipelineCreationFeedback;
pub type VkPipelineCreationFeedbackFlagBits = VkFlags;
pub type VkPipelineCreationFeedbackFlagBitsEXT = VkPipelineCreationFeedbackFlagBits;
pub type VkPipelineCreationFeedbackFlags = VkFlags;
pub type VkPipelineCreationFeedbackFlagsEXT = VkPipelineCreationFeedbackFlags;
pub type VkPipelineDepthStencilStateCreateFlagBits = VkFlags;
pub type VkPipelineDepthStencilStateCreateFlags = VkFlags;
pub type VkPipelineDiscardRectangleStateCreateFlagsEXT = VkFlags;
pub type VkPipelineDynamicStateCreateFlags = VkFlags;
pub type VkPipelineExecutableStatisticFormatKHR = i32;
pub type VkPipelineInfoEXT = VkPipelineInfoKHR;
pub type VkPipelineInputAssemblyStateCreateFlags = VkFlags;
pub type VkPipelineLayout = *mut VkPipelineLayout_T;
pub type VkPipelineLayoutCreateFlagBits = VkFlags;
pub type VkPipelineLayoutCreateFlags = VkFlags;
pub type VkPipelineMultisampleStateCreateFlags = VkFlags;
pub type VkPipelineRasterizationConservativeStateCreateFlagsEXT = VkFlags;
pub type VkPipelineRasterizationDepthClipStateCreateFlagsEXT = VkFlags;
pub type VkPipelineRasterizationLineStateCreateInfoEXT = VkPipelineRasterizationLineStateCreateInfo;
pub type VkPipelineRasterizationLineStateCreateInfoKHR = VkPipelineRasterizationLineStateCreateInfo;
pub type VkPipelineRasterizationStateCreateFlags = VkFlags;
pub type VkPipelineRasterizationStateStreamCreateFlagsEXT = VkFlags;
pub type VkPipelineRenderingCreateInfoKHR = VkPipelineRenderingCreateInfo;
pub type VkPipelineRobustnessBufferBehavior = i32;
pub type VkPipelineRobustnessBufferBehaviorEXT = VkPipelineRobustnessBufferBehavior;
pub type VkPipelineRobustnessCreateInfoEXT = VkPipelineRobustnessCreateInfo;
pub type VkPipelineRobustnessImageBehavior = i32;
pub type VkPipelineRobustnessImageBehaviorEXT = VkPipelineRobustnessImageBehavior;
pub type VkPipelineShaderStageCreateFlagBits = VkFlags;
pub type VkPipelineShaderStageCreateFlags = VkFlags;
pub type VkPipelineShaderStageRequiredSubgroupSizeCreateInfoEXT = VkPipelineShaderStageRequiredSubgroupSizeCreateInfo;
pub type VkPipelineStageFlagBits = VkFlags;
pub type VkPipelineStageFlagBits2 = VkFlags64;
pub type VkPipelineStageFlagBits2KHR = VkPipelineStageFlagBits2;
pub type VkPipelineStageFlags = VkFlags;
pub type VkPipelineStageFlags2 = VkFlags64;
pub type VkPipelineStageFlags2KHR = VkPipelineStageFlags2;
pub type VkPipelineTessellationDomainOriginStateCreateInfoKHR = VkPipelineTessellationDomainOriginStateCreateInfo;
pub type VkPipelineTessellationStateCreateFlags = VkFlags;
pub type VkPipelineVertexInputDivisorStateCreateInfoEXT = VkPipelineVertexInputDivisorStateCreateInfo;
pub type VkPipelineVertexInputDivisorStateCreateInfoKHR = VkPipelineVertexInputDivisorStateCreateInfo;
pub type VkPipelineVertexInputStateCreateFlags = VkFlags;
pub type VkPipelineViewportStateCreateFlags = VkFlags;
pub type VkPipelineViewportSwizzleStateCreateFlagsNV = VkFlags;
pub type VkPointClippingBehavior = i32;
pub type VkPointClippingBehaviorKHR = VkPointClippingBehavior;
pub type VkPolygonMode = i32;
pub type VkPresentGravityFlagBitsEXT = VkPresentGravityFlagBitsKHR;
pub type VkPresentGravityFlagBitsKHR = VkFlags;
pub type VkPresentGravityFlagsEXT = VkPresentGravityFlagsKHR;
pub type VkPresentGravityFlagsKHR = VkFlags;
pub type VkPresentModeKHR = i32;
pub type VkPresentScalingFlagBitsEXT = VkPresentScalingFlagBitsKHR;
pub type VkPresentScalingFlagBitsKHR = VkFlags;
pub type VkPresentScalingFlagsEXT = VkPresentScalingFlagsKHR;
pub type VkPresentScalingFlagsKHR = VkFlags;
pub type VkPresentStageFlagBitsEXT = VkFlags;
pub type VkPresentStageFlagsEXT = VkFlags;
pub type VkPresentTimingInfoFlagBitsEXT = VkFlags;
pub type VkPresentTimingInfoFlagsEXT = VkFlags;
pub type VkPrimitiveTopology = i32;
pub type VkPrivateDataSlot = *mut VkPrivateDataSlot_T;
pub type VkPrivateDataSlotCreateFlags = VkFlags;
pub type VkPrivateDataSlotCreateFlagsEXT = VkPrivateDataSlotCreateFlags;
pub type VkPrivateDataSlotCreateInfoEXT = VkPrivateDataSlotCreateInfo;
pub type VkPrivateDataSlotEXT = VkPrivateDataSlot;
pub type VkProvokingVertexModeEXT = i32;
pub type VkPushConstantsInfoKHR = VkPushConstantsInfo;
pub type VkPushDescriptorSetInfoKHR = VkPushDescriptorSetInfo;
pub type VkPushDescriptorSetWithTemplateInfoKHR = VkPushDescriptorSetWithTemplateInfo;
pub type VkQueryControlFlagBits = VkFlags;
pub type VkQueryControlFlags = VkFlags;
pub type VkQueryPipelineStatisticFlagBits = VkFlags;
pub type VkQueryPipelineStatisticFlags = VkFlags;
pub type VkQueryPool = *mut VkQueryPool_T;
pub type VkQueryPoolCreateFlagBits = VkFlags;
pub type VkQueryPoolCreateFlags = VkFlags;
pub type VkQueryPoolCreateInfoINTEL = VkQueryPoolPerformanceQueryCreateInfoINTEL;
pub type VkQueryPoolSamplingModeINTEL = i32;
pub type VkQueryResultFlagBits = VkFlags;
pub type VkQueryResultFlags = VkFlags;
pub type VkQueryResultStatusKHR = i32;
pub type VkQueryType = i32;
pub type VkQueue = *mut VkQueue_T;
pub type VkQueueFamilyGlobalPriorityPropertiesEXT = VkQueueFamilyGlobalPriorityProperties;
pub type VkQueueFamilyGlobalPriorityPropertiesKHR = VkQueueFamilyGlobalPriorityProperties;
pub type VkQueueFamilyProperties2KHR = VkQueueFamilyProperties2;
pub type VkQueueFlagBits = VkFlags;
pub type VkQueueFlags = VkFlags;
pub type VkQueueGlobalPriority = i32;
pub type VkQueueGlobalPriorityEXT = VkQueueGlobalPriority;
pub type VkQueueGlobalPriorityKHR = VkQueueGlobalPriority;
pub type VkRasterizationOrderAMD = i32;
pub type VkRayTracingInvocationReorderModeEXT = i32;
pub type VkRayTracingInvocationReorderModeNV = VkRayTracingInvocationReorderModeEXT;
pub type VkRayTracingLssIndexingModeNV = i32;
pub type VkRayTracingLssPrimitiveEndCapsModeNV = i32;
pub type VkRayTracingShaderGroupTypeKHR = i32;
pub type VkRayTracingShaderGroupTypeNV = VkRayTracingShaderGroupTypeKHR;
pub type VkReleaseSwapchainImagesInfoEXT = VkReleaseSwapchainImagesInfoKHR;
pub type VkRemoteAddressNV = *mut c_void;
pub type VkRenderPass = *mut VkRenderPass_T;
pub type VkRenderPassAttachmentBeginInfoKHR = VkRenderPassAttachmentBeginInfo;
pub type VkRenderPassCreateFlagBits = VkFlags;
pub type VkRenderPassCreateFlags = VkFlags;
pub type VkRenderPassCreateInfo2KHR = VkRenderPassCreateInfo2;
pub type VkRenderPassInputAttachmentAspectCreateInfoKHR = VkRenderPassInputAttachmentAspectCreateInfo;
pub type VkRenderPassMultiviewCreateInfoKHR = VkRenderPassMultiviewCreateInfo;
pub type VkRenderingAreaInfoKHR = VkRenderingAreaInfo;
pub type VkRenderingAttachmentFlagBitsKHR = VkFlags;
pub type VkRenderingAttachmentFlagsKHR = VkFlags;
pub type VkRenderingAttachmentInfoKHR = VkRenderingAttachmentInfo;
pub type VkRenderingAttachmentLocationInfoKHR = VkRenderingAttachmentLocationInfo;
pub type VkRenderingEndInfoEXT = VkRenderingEndInfoKHR;
pub type VkRenderingFlagBits = VkFlags;
pub type VkRenderingFlagBitsKHR = VkRenderingFlagBits;
pub type VkRenderingFlags = VkFlags;
pub type VkRenderingFlagsKHR = VkRenderingFlags;
pub type VkRenderingInfoKHR = VkRenderingInfo;
pub type VkRenderingInputAttachmentIndexInfoKHR = VkRenderingInputAttachmentIndexInfo;
pub type VkResolveImageFlagBitsKHR = VkFlags;
pub type VkResolveImageFlagsKHR = VkFlags;
pub type VkResolveImageInfo2KHR = VkResolveImageInfo2;
pub type VkResolveModeFlagBits = VkFlags;
pub type VkResolveModeFlagBitsKHR = VkResolveModeFlagBits;
pub type VkResolveModeFlags = VkFlags;
pub type VkResolveModeFlagsKHR = VkResolveModeFlags;
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
pub type VkSamplerReductionModeCreateInfoEXT = VkSamplerReductionModeCreateInfo;
pub type VkSamplerReductionModeEXT = VkSamplerReductionMode;
pub type VkSamplerYcbcrConversion = *mut VkSamplerYcbcrConversion_T;
pub type VkSamplerYcbcrConversionCreateInfoKHR = VkSamplerYcbcrConversionCreateInfo;
pub type VkSamplerYcbcrConversionImageFormatPropertiesKHR = VkSamplerYcbcrConversionImageFormatProperties;
pub type VkSamplerYcbcrConversionInfoKHR = VkSamplerYcbcrConversionInfo;
pub type VkSamplerYcbcrConversionKHR = VkSamplerYcbcrConversion;
pub type VkSamplerYcbcrModelConversion = i32;
pub type VkSamplerYcbcrModelConversionKHR = VkSamplerYcbcrModelConversion;
pub type VkSamplerYcbcrRange = i32;
pub type VkSamplerYcbcrRangeKHR = VkSamplerYcbcrRange;
pub type VkScopeKHR = i32;
pub type VkScopeNV = VkScopeKHR;
pub type VkSemaphore = *mut VkSemaphore_T;
pub type VkSemaphoreCreateFlags = VkFlags;
pub type VkSemaphoreImportFlagBits = VkFlags;
pub type VkSemaphoreImportFlagBitsKHR = VkSemaphoreImportFlagBits;
pub type VkSemaphoreImportFlags = VkFlags;
pub type VkSemaphoreImportFlagsKHR = VkSemaphoreImportFlags;
pub type VkSemaphoreSignalInfoKHR = VkSemaphoreSignalInfo;
pub type VkSemaphoreSubmitInfoKHR = VkSemaphoreSubmitInfo;
pub type VkSemaphoreType = i32;
pub type VkSemaphoreTypeCreateInfoKHR = VkSemaphoreTypeCreateInfo;
pub type VkSemaphoreTypeKHR = VkSemaphoreType;
pub type VkSemaphoreWaitFlagBits = VkFlags;
pub type VkSemaphoreWaitFlagBitsKHR = VkSemaphoreWaitFlagBits;
pub type VkSemaphoreWaitFlags = VkFlags;
pub type VkSemaphoreWaitFlagsKHR = VkSemaphoreWaitFlags;
pub type VkSemaphoreWaitInfoKHR = VkSemaphoreWaitInfo;
pub type VkShaderCodeTypeEXT = i32;
pub type VkShaderCorePropertiesFlagBitsAMD = VkFlags;
pub type VkShaderCorePropertiesFlagsAMD = VkFlags;
pub type VkShaderCreateFlagBitsEXT = VkFlags;
pub type VkShaderCreateFlagsEXT = VkFlags;
pub type VkShaderEXT = *mut VkShaderEXT_T;
pub type VkShaderFloatControlsIndependence = i32;
pub type VkShaderFloatControlsIndependenceKHR = VkShaderFloatControlsIndependence;
pub type VkShaderGroupShaderKHR = i32;
pub type VkShaderInfoTypeAMD = i32;
pub type VkShaderModule = *mut VkShaderModule_T;
pub type VkShaderModuleCreateFlags = VkFlags;
pub type VkShaderRequiredSubgroupSizeCreateInfoEXT = VkPipelineShaderStageRequiredSubgroupSizeCreateInfo;
pub type VkShaderStageFlagBits = VkFlags;
pub type VkShaderStageFlags = VkFlags;
pub type VkShadingRatePaletteEntryNV = i32;
pub type VkSharingMode = i32;
pub type VkSparseImageFormatFlagBits = VkFlags;
pub type VkSparseImageFormatFlags = VkFlags;
pub type VkSparseImageFormatProperties2KHR = VkSparseImageFormatProperties2;
pub type VkSparseImageMemoryRequirements2KHR = VkSparseImageMemoryRequirements2;
pub type VkSparseMemoryBindFlagBits = VkFlags;
pub type VkSparseMemoryBindFlags = VkFlags;
pub type VkStencilFaceFlagBits = VkFlags;
pub type VkStencilFaceFlags = VkFlags;
pub type VkStencilOp = i32;
pub type VkStructureType = i32;
pub type VkSubgroupFeatureFlagBits = VkFlags;
pub type VkSubgroupFeatureFlags = VkFlags;
pub type VkSubmitFlagBits = VkFlags;
pub type VkSubmitFlagBitsKHR = VkSubmitFlagBits;
pub type VkSubmitFlags = VkFlags;
pub type VkSubmitFlagsKHR = VkSubmitFlags;
pub type VkSubmitInfo2KHR = VkSubmitInfo2;
pub type VkSubpassBeginInfoKHR = VkSubpassBeginInfo;
pub type VkSubpassContents = i32;
pub type VkSubpassDependency2KHR = VkSubpassDependency2;
pub type VkSubpassDescription2KHR = VkSubpassDescription2;
pub type VkSubpassDescriptionDepthStencilResolveKHR = VkSubpassDescriptionDepthStencilResolve;
pub type VkSubpassDescriptionFlagBits = VkFlags;
pub type VkSubpassDescriptionFlags = VkFlags;
pub type VkSubpassEndInfoKHR = VkSubpassEndInfo;
pub type VkSubpassFragmentDensityMapOffsetEndInfoQCOM = VkRenderPassFragmentDensityMapOffsetEndInfoEXT;
pub type VkSubpassMergeStatusEXT = i32;
pub type VkSubresourceHostMemcpySizeEXT = VkSubresourceHostMemcpySize;
pub type VkSubresourceLayout2EXT = VkSubresourceLayout2;
pub type VkSubresourceLayout2KHR = VkSubresourceLayout2;
pub type VkSurfaceCounterFlagBitsEXT = VkFlags;
pub type VkSurfaceCounterFlagsEXT = VkFlags;
pub type VkSurfaceKHR = *mut VkSurfaceKHR_T;
pub type VkSurfacePresentModeCompatibilityEXT = VkSurfacePresentModeCompatibilityKHR;
pub type VkSurfacePresentModeEXT = VkSurfacePresentModeKHR;
pub type VkSurfacePresentScalingCapabilitiesEXT = VkSurfacePresentScalingCapabilitiesKHR;
pub type VkSurfaceTransformFlagBitsKHR = VkFlags;
pub type VkSurfaceTransformFlagsKHR = VkFlags;
pub type VkSwapchainCreateFlagBitsKHR = VkFlags;
pub type VkSwapchainCreateFlagsKHR = VkFlags;
pub type VkSwapchainKHR = *mut VkSwapchainKHR_T;
pub type VkSwapchainPresentFenceInfoEXT = VkSwapchainPresentFenceInfoKHR;
pub type VkSwapchainPresentModeInfoEXT = VkSwapchainPresentModeInfoKHR;
pub type VkSwapchainPresentModesCreateInfoEXT = VkSwapchainPresentModesCreateInfoKHR;
pub type VkSwapchainPresentScalingCreateInfoEXT = VkSwapchainPresentScalingCreateInfoKHR;
pub type VkSystemAllocationScope = i32;
pub type VkTensorARM = *mut VkTensorARM_T;
pub type VkTensorCreateFlagBitsARM = VkFlags64;
pub type VkTensorCreateFlagsARM = VkFlags64;
pub type VkTensorTilingARM = i32;
pub type VkTensorUsageFlagBitsARM = VkFlags64;
pub type VkTensorUsageFlagsARM = VkFlags64;
pub type VkTensorViewARM = *mut VkTensorViewARM_T;
pub type VkTensorViewCreateFlagBitsARM = VkFlags64;
pub type VkTensorViewCreateFlagsARM = VkFlags64;
pub type VkTessellationDomainOrigin = i32;
pub type VkTessellationDomainOriginKHR = VkTessellationDomainOrigin;
pub type VkTileShadingRenderPassFlagBitsQCOM = VkFlags;
pub type VkTileShadingRenderPassFlagsQCOM = VkFlags;
pub type VkTimeDomainEXT = VkTimeDomainKHR;
pub type VkTimeDomainKHR = i32;
pub type VkTimelineSemaphoreSubmitInfoKHR = VkTimelineSemaphoreSubmitInfo;
pub type VkToolPurposeFlagBits = VkFlags;
pub type VkToolPurposeFlagBitsEXT = VkToolPurposeFlagBits;
pub type VkToolPurposeFlags = VkFlags;
pub type VkToolPurposeFlagsEXT = VkToolPurposeFlags;
pub type VkTransformMatrixNV = VkTransformMatrixKHR;
pub type VkValidationCacheCreateFlagsEXT = VkFlags;
pub type VkValidationCacheEXT = *mut VkValidationCacheEXT_T;
pub type VkValidationCacheHeaderVersionEXT = i32;
pub type VkValidationCheckEXT = i32;
pub type VkValidationFeatureDisableEXT = i32;
pub type VkValidationFeatureEnableEXT = i32;
pub type VkVendorId = i32;
pub type VkVertexInputBindingDivisorDescriptionEXT = VkVertexInputBindingDivisorDescription;
pub type VkVertexInputBindingDivisorDescriptionKHR = VkVertexInputBindingDivisorDescription;
pub type VkVertexInputRate = i32;
pub type VkVideoBeginCodingFlagsKHR = VkFlags;
pub type VkVideoCapabilityFlagBitsKHR = VkFlags;
pub type VkVideoCapabilityFlagsKHR = VkFlags;
pub type VkVideoChromaSubsamplingFlagBitsKHR = VkFlags;
pub type VkVideoChromaSubsamplingFlagsKHR = VkFlags;
pub type VkVideoCodecOperationFlagBitsKHR = VkFlags;
pub type VkVideoCodecOperationFlagsKHR = VkFlags;
pub type VkVideoCodingControlFlagBitsKHR = VkFlags;
pub type VkVideoCodingControlFlagsKHR = VkFlags;
pub type VkVideoComponentBitDepthFlagBitsKHR = VkFlags;
pub type VkVideoComponentBitDepthFlagsKHR = VkFlags;
pub type VkVideoDecodeCapabilityFlagBitsKHR = VkFlags;
pub type VkVideoDecodeCapabilityFlagsKHR = VkFlags;
pub type VkVideoDecodeFlagsKHR = VkFlags;
pub type VkVideoDecodeH264PictureLayoutFlagBitsKHR = VkFlags;
pub type VkVideoDecodeH264PictureLayoutFlagsKHR = VkFlags;
pub type VkVideoDecodeUsageFlagBitsKHR = VkFlags;
pub type VkVideoDecodeUsageFlagsKHR = VkFlags;
pub type VkVideoEncodeAV1CapabilityFlagBitsKHR = VkFlags;
pub type VkVideoEncodeAV1CapabilityFlagsKHR = VkFlags;
pub type VkVideoEncodeAV1PredictionModeKHR = i32;
pub type VkVideoEncodeAV1RateControlFlagBitsKHR = VkFlags;
pub type VkVideoEncodeAV1RateControlFlagsKHR = VkFlags;
pub type VkVideoEncodeAV1RateControlGroupKHR = i32;
pub type VkVideoEncodeAV1StdFlagBitsKHR = VkFlags;
pub type VkVideoEncodeAV1StdFlagsKHR = VkFlags;
pub type VkVideoEncodeAV1SuperblockSizeFlagBitsKHR = VkFlags;
pub type VkVideoEncodeAV1SuperblockSizeFlagsKHR = VkFlags;
pub type VkVideoEncodeCapabilityFlagBitsKHR = VkFlags;
pub type VkVideoEncodeCapabilityFlagsKHR = VkFlags;
pub type VkVideoEncodeContentFlagBitsKHR = VkFlags;
pub type VkVideoEncodeContentFlagsKHR = VkFlags;
pub type VkVideoEncodeFeedbackFlagBitsKHR = VkFlags;
pub type VkVideoEncodeFeedbackFlagsKHR = VkFlags;
pub type VkVideoEncodeFlagBitsKHR = VkFlags;
pub type VkVideoEncodeFlagsKHR = VkFlags;
pub type VkVideoEncodeH264CapabilityFlagBitsKHR = VkFlags;
pub type VkVideoEncodeH264CapabilityFlagsKHR = VkFlags;
pub type VkVideoEncodeH264RateControlFlagBitsKHR = VkFlags;
pub type VkVideoEncodeH264RateControlFlagsKHR = VkFlags;
pub type VkVideoEncodeH264StdFlagBitsKHR = VkFlags;
pub type VkVideoEncodeH264StdFlagsKHR = VkFlags;
pub type VkVideoEncodeH265CapabilityFlagBitsKHR = VkFlags;
pub type VkVideoEncodeH265CapabilityFlagsKHR = VkFlags;
pub type VkVideoEncodeH265CtbSizeFlagBitsKHR = VkFlags;
pub type VkVideoEncodeH265CtbSizeFlagsKHR = VkFlags;
pub type VkVideoEncodeH265RateControlFlagBitsKHR = VkFlags;
pub type VkVideoEncodeH265RateControlFlagsKHR = VkFlags;
pub type VkVideoEncodeH265StdFlagBitsKHR = VkFlags;
pub type VkVideoEncodeH265StdFlagsKHR = VkFlags;
pub type VkVideoEncodeH265TransformBlockSizeFlagBitsKHR = VkFlags;
pub type VkVideoEncodeH265TransformBlockSizeFlagsKHR = VkFlags;
pub type VkVideoEncodeIntraRefreshModeFlagBitsKHR = VkFlags;
pub type VkVideoEncodeIntraRefreshModeFlagsKHR = VkFlags;
pub type VkVideoEncodeRateControlFlagsKHR = VkFlags;
pub type VkVideoEncodeRateControlModeFlagBitsKHR = VkFlags;
pub type VkVideoEncodeRateControlModeFlagsKHR = VkFlags;
pub type VkVideoEncodeRgbChromaOffsetFlagBitsVALVE = VkFlags;
pub type VkVideoEncodeRgbChromaOffsetFlagsVALVE = VkFlags;
pub type VkVideoEncodeRgbModelConversionFlagBitsVALVE = VkFlags;
pub type VkVideoEncodeRgbModelConversionFlagsVALVE = VkFlags;
pub type VkVideoEncodeRgbRangeCompressionFlagBitsVALVE = VkFlags;
pub type VkVideoEncodeRgbRangeCompressionFlagsVALVE = VkFlags;
pub type VkVideoEncodeTuningModeKHR = i32;
pub type VkVideoEncodeUsageFlagBitsKHR = VkFlags;
pub type VkVideoEncodeUsageFlagsKHR = VkFlags;
pub type VkVideoEndCodingFlagsKHR = VkFlags;
pub type VkVideoSessionCreateFlagBitsKHR = VkFlags;
pub type VkVideoSessionCreateFlagsKHR = VkFlags;
pub type VkVideoSessionKHR = *mut VkVideoSessionKHR_T;
pub type VkVideoSessionParametersCreateFlagBitsKHR = VkFlags;
pub type VkVideoSessionParametersCreateFlagsKHR = VkFlags;
pub type VkVideoSessionParametersKHR = *mut VkVideoSessionParametersKHR_T;
pub type VkViewportCoordinateSwizzleNV = i32;
pub type VkWriteDescriptorSetInlineUniformBlockEXT = VkWriteDescriptorSetInlineUniformBlock;

#[derive(Clone, Copy)]
#[repr(C)]
pub union VkAccelerationStructureGeometryDataKHR {
    pub triangles: VkAccelerationStructureGeometryTrianglesDataKHR,
    pub aabbs: VkAccelerationStructureGeometryAabbsDataKHR,
    pub instances: VkAccelerationStructureGeometryInstancesDataKHR,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub union VkAccelerationStructureMotionInstanceDataNV {
    pub staticInstance: VkAccelerationStructureInstanceKHR,
    pub matrixMotionInstance: VkAccelerationStructureMatrixMotionInstanceNV,
    pub srtMotionInstance: VkAccelerationStructureSRTMotionInstanceNV,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub union VkClearColorValue {
    pub float32: [f32; 4 as usize],
    pub int32: [i32; 4 as usize],
    pub uint32: [u32; 4 as usize],
}

#[derive(Clone, Copy)]
#[repr(C)]
pub union VkClearValue {
    pub color: VkClearColorValue,
    pub depthStencil: VkClearDepthStencilValue,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub union VkClusterAccelerationStructureOpInputNV {
    pub pClustersBottomLevel: *mut VkClusterAccelerationStructureClustersBottomLevelInputNV,
    pub pTriangleClusters: *mut VkClusterAccelerationStructureTriangleClusterInputNV,
    pub pMoveObjects: *mut VkClusterAccelerationStructureMoveObjectsInputNV,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub union VkDescriptorDataEXT {
    pub pSampler: *const VkSampler,
    pub pCombinedImageSampler: *const VkDescriptorImageInfo,
    pub pInputAttachmentImage: *const VkDescriptorImageInfo,
    pub pSampledImage: *const VkDescriptorImageInfo,
    pub pStorageImage: *const VkDescriptorImageInfo,
    pub pUniformTexelBuffer: *const VkDescriptorAddressInfoEXT,
    pub pStorageTexelBuffer: *const VkDescriptorAddressInfoEXT,
    pub pUniformBuffer: *const VkDescriptorAddressInfoEXT,
    pub pStorageBuffer: *const VkDescriptorAddressInfoEXT,
    pub accelerationStructure: VkDeviceAddress,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub union VkDeviceOrHostAddressConstKHR {
    pub deviceAddress: VkDeviceAddress,
    pub hostAddress: *const c_void,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub union VkDeviceOrHostAddressKHR {
    pub deviceAddress: VkDeviceAddress,
    pub hostAddress: *mut c_void,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub union VkIndirectCommandsTokenDataEXT {
    pub pPushConstant: *const VkIndirectCommandsPushConstantTokenEXT,
    pub pVertexBuffer: *const VkIndirectCommandsVertexBufferTokenEXT,
    pub pIndexBuffer: *const VkIndirectCommandsIndexBufferTokenEXT,
    pub pExecutionSet: *const VkIndirectCommandsExecutionSetTokenEXT,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub union VkIndirectExecutionSetInfoEXT {
    pub pPipelineInfo: *const VkIndirectExecutionSetPipelineInfoEXT,
    pub pShaderInfo: *const VkIndirectExecutionSetShaderInfoEXT,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub union VkPerformanceCounterResultKHR {
    pub int32: i32,
    pub int64: i64,
    pub uint32: u32,
    pub uint64: u64,
    pub float32: f32,
    pub float64: f64,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub union VkPerformanceValueDataINTEL {
    pub value32: u32,
    pub value64: u64,
    pub valueFloat: f32,
    pub valueBool: VkBool32,
    pub valueString: *const c_char,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub union VkPipelineExecutableStatisticValueKHR {
    pub b32: VkBool32,
    pub i64: i64,
    pub u64: u64,
    pub f64: f64,
}
