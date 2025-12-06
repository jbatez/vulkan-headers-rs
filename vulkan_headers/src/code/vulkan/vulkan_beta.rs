use crate::code::*;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkAccelerationStructureDenseGeometryFormatTrianglesDataAMDX {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub compressedData: VkDeviceOrHostAddressConstKHR,
    pub dataSize: VkDeviceSize,
    pub numTriangles: u32,
    pub numVertices: u32,
    pub maxPrimitiveIndex: u32,
    pub maxGeometryIndex: u32,
    pub format: VkCompressedTriangleFormatAMDX,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkAccelerationStructureTrianglesDisplacementMicromapNV {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub displacementBiasAndScaleFormat: VkFormat,
    pub displacementVectorFormat: VkFormat,
    pub displacementBiasAndScaleBuffer: VkDeviceOrHostAddressConstKHR,
    pub displacementBiasAndScaleStride: VkDeviceSize,
    pub displacementVectorBuffer: VkDeviceOrHostAddressConstKHR,
    pub displacementVectorStride: VkDeviceSize,
    pub displacedMicromapPrimitiveFlags: VkDeviceOrHostAddressConstKHR,
    pub displacedMicromapPrimitiveFlagsStride: VkDeviceSize,
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
pub struct VkCudaFunctionCreateInfoNV {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub module: VkCudaModuleNV,
    pub pName: *const c_char,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkCudaLaunchInfoNV {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub function: VkCudaFunctionNV,
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
pub struct VkCudaModuleCreateInfoNV {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub dataSize: usize,
    pub pData: *const c_void,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDispatchGraphCountInfoAMDX {
    pub count: u32,
    pub infos: VkDeviceOrHostAddressConstAMDX,
    pub stride: u64,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDispatchGraphInfoAMDX {
    pub nodeIndex: u32,
    pub payloadCount: u32,
    pub payloads: VkDeviceOrHostAddressConstAMDX,
    pub payloadStride: u64,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkExecutionGraphPipelineCreateInfoAMDX {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineCreateFlags,
    pub stageCount: u32,
    pub pStages: *const VkPipelineShaderStageCreateInfo,
    pub pLibraryInfo: *const VkPipelineLibraryCreateInfoKHR,
    pub layout: VkPipelineLayout,
    pub basePipelineHandle: VkPipeline,
    pub basePipelineIndex: i32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkExecutionGraphPipelineScratchSizeAMDX {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub minSize: VkDeviceSize,
    pub maxSize: VkDeviceSize,
    pub sizeGranularity: VkDeviceSize,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceCudaKernelLaunchFeaturesNV {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub cudaKernelLaunchFeatures: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceCudaKernelLaunchPropertiesNV {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub computeCapabilityMinor: u32,
    pub computeCapabilityMajor: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceDenseGeometryFormatFeaturesAMDX {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub denseGeometryFormat: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceDisplacementMicromapFeaturesNV {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub displacementMicromap: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceDisplacementMicromapPropertiesNV {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub maxDisplacementMicromapSubdivisionLevel: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDevicePortabilitySubsetFeaturesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub constantAlphaColorBlendFactors: VkBool32,
    pub events: VkBool32,
    pub imageViewFormatReinterpretation: VkBool32,
    pub imageViewFormatSwizzle: VkBool32,
    pub imageView2DOn3DImage: VkBool32,
    pub multisampleArrayImage: VkBool32,
    pub mutableComparisonSamplers: VkBool32,
    pub pointPolygons: VkBool32,
    pub samplerMipLodBias: VkBool32,
    pub separateStencilMaskRef: VkBool32,
    pub shaderSampleRateInterpolationFunctions: VkBool32,
    pub tessellationIsolines: VkBool32,
    pub tessellationPointMode: VkBool32,
    pub triangleFans: VkBool32,
    pub vertexAttributeAccessBeyondStride: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDevicePortabilitySubsetPropertiesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub minVertexInputBindingStrideAlignment: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDevicePresentMeteringFeaturesNV {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub presentMetering: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceShaderEnqueueFeaturesAMDX {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub shaderEnqueue: VkBool32,
    pub shaderMeshEnqueue: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceShaderEnqueuePropertiesAMDX {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub maxExecutionGraphDepth: u32,
    pub maxExecutionGraphShaderOutputNodes: u32,
    pub maxExecutionGraphShaderPayloadSize: u32,
    pub maxExecutionGraphShaderPayloadCount: u32,
    pub executionGraphDispatchAddressAlignment: u32,
    pub maxExecutionGraphWorkgroupCount: [u32; 3 as usize],
    pub maxExecutionGraphWorkgroups: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineShaderStageNodeCreateInfoAMDX {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub pName: *const c_char,
    pub index: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSetPresentConfigNV {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub numFramesPerBatch: u32,
    pub presentConfigFeedback: u32,
}

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

#[derive(Clone, Copy)]
#[repr(C)]
pub union VkDeviceOrHostAddressConstAMDX {
    pub deviceAddress: VkDeviceAddress,
    pub hostAddress: *const c_void,
}
