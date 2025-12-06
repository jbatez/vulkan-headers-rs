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

pub const VK_AMDX_DENSE_GEOMETRY_FORMAT_EXTENSION_NAME: &CStr = c"VK_AMDX_dense_geometry_format";
pub const VK_AMDX_DENSE_GEOMETRY_FORMAT_SPEC_VERSION: u32 = 1;
pub const VK_AMDX_SHADER_ENQUEUE_EXTENSION_NAME: &CStr = c"VK_AMDX_shader_enqueue";
pub const VK_AMDX_SHADER_ENQUEUE_SPEC_VERSION: u32 = 2;
pub const VK_BUFFER_USAGE_2_COMPRESSED_DATA_DGF1_BIT_AMDX: VkBufferUsageFlagBits2 = 1 << 33;
pub const VK_BUFFER_USAGE_2_EXECUTION_GRAPH_SCRATCH_BIT_AMDX: VkBufferUsageFlagBits2 = 1 << 25;
pub const VK_BUFFER_USAGE_EXECUTION_GRAPH_SCRATCH_BIT_AMDX: VkBufferUsageFlagBits = 1 << 25;
pub const VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_DISPLACEMENT_MICROMAP_UPDATE_BIT_NV: VkBuildAccelerationStructureFlagBitsKHR = 1 << 9;
pub const VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_DISPLACEMENT_MICROMAP_UPDATE_NV: VkBuildAccelerationStructureFlagBitsKHR = VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_DISPLACEMENT_MICROMAP_UPDATE_BIT_NV;
pub const VK_COMPRESSED_TRIANGLE_FORMAT_DGF1_AMDX: VkCompressedTriangleFormatAMDX = 0;
pub const VK_COMPRESSED_TRIANGLE_FORMAT_DGF1_BYTE_ALIGNMENT_AMDX: u32 = 128;
pub const VK_COMPRESSED_TRIANGLE_FORMAT_DGF1_BYTE_STRIDE_AMDX: u32 = 128;
pub const VK_DEBUG_REPORT_OBJECT_TYPE_CUDA_FUNCTION_NV_EXT: VkDebugReportObjectTypeEXT = 1000307001;
pub const VK_DEBUG_REPORT_OBJECT_TYPE_CUDA_MODULE_NV_EXT: VkDebugReportObjectTypeEXT = 1000307000;
pub const VK_DISPLACEMENT_MICROMAP_FORMAT_1024_TRIANGLES_128_BYTES_NV: VkDisplacementMicromapFormatNV = 3;
pub const VK_DISPLACEMENT_MICROMAP_FORMAT_256_TRIANGLES_128_BYTES_NV: VkDisplacementMicromapFormatNV = 2;
pub const VK_DISPLACEMENT_MICROMAP_FORMAT_64_TRIANGLES_64_BYTES_NV: VkDisplacementMicromapFormatNV = 1;
pub const VK_GEOMETRY_TYPE_DENSE_GEOMETRY_FORMAT_TRIANGLES_AMDX: VkGeometryTypeKHR = 1000478000;
pub const VK_KHR_PORTABILITY_SUBSET_EXTENSION_NAME: &CStr = c"VK_KHR_portability_subset";
pub const VK_KHR_PORTABILITY_SUBSET_SPEC_VERSION: u32 = 1;
pub const VK_MICROMAP_TYPE_DISPLACEMENT_MICROMAP_NV: VkMicromapTypeEXT = 1000397000;
pub const VK_NV_CUDA_KERNEL_LAUNCH_EXTENSION_NAME: &CStr = c"VK_NV_cuda_kernel_launch";
pub const VK_NV_CUDA_KERNEL_LAUNCH_SPEC_VERSION: u32 = 2;
pub const VK_NV_DISPLACEMENT_MICROMAP_EXTENSION_NAME: &CStr = c"VK_NV_displacement_micromap";
pub const VK_NV_DISPLACEMENT_MICROMAP_SPEC_VERSION: u32 = 2;
pub const VK_NV_PRESENT_METERING_EXTENSION_NAME: &CStr = c"VK_NV_present_metering";
pub const VK_NV_PRESENT_METERING_SPEC_VERSION: u32 = 1;
pub const VK_OBJECT_TYPE_CUDA_FUNCTION_NV: VkObjectType = 1000307001;
pub const VK_OBJECT_TYPE_CUDA_MODULE_NV: VkObjectType = 1000307000;
pub const VK_PIPELINE_BIND_POINT_EXECUTION_GRAPH_AMDX: VkPipelineBindPoint = 1000134000;
pub const VK_PIPELINE_CREATE_2_EXECUTION_GRAPH_BIT_AMDX: VkPipelineCreateFlagBits2 = 1 << 32;
pub const VK_PIPELINE_CREATE_RAY_TRACING_DISPLACEMENT_MICROMAP_BIT_NV: VkPipelineCreateFlagBits = 1 << 28;
pub const VK_SHADER_INDEX_UNUSED_AMDX: u32 = !0;
pub const VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_DENSE_GEOMETRY_FORMAT_TRIANGLES_DATA_AMDX: VkStructureType = 1000478001;
pub const VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_TRIANGLES_DISPLACEMENT_MICROMAP_NV: VkStructureType = 1000397002;
pub const VK_STRUCTURE_TYPE_CUDA_FUNCTION_CREATE_INFO_NV: VkStructureType = 1000307001;
pub const VK_STRUCTURE_TYPE_CUDA_LAUNCH_INFO_NV: VkStructureType = 1000307002;
pub const VK_STRUCTURE_TYPE_CUDA_MODULE_CREATE_INFO_NV: VkStructureType = 1000307000;
pub const VK_STRUCTURE_TYPE_EXECUTION_GRAPH_PIPELINE_CREATE_INFO_AMDX: VkStructureType = 1000134003;
pub const VK_STRUCTURE_TYPE_EXECUTION_GRAPH_PIPELINE_SCRATCH_SIZE_AMDX: VkStructureType = 1000134002;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CUDA_KERNEL_LAUNCH_FEATURES_NV: VkStructureType = 1000307003;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CUDA_KERNEL_LAUNCH_PROPERTIES_NV: VkStructureType = 1000307004;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DENSE_GEOMETRY_FORMAT_FEATURES_AMDX: VkStructureType = 1000478000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DISPLACEMENT_MICROMAP_FEATURES_NV: VkStructureType = 1000397000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DISPLACEMENT_MICROMAP_PROPERTIES_NV: VkStructureType = 1000397001;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PORTABILITY_SUBSET_FEATURES_KHR: VkStructureType = 1000163000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PORTABILITY_SUBSET_PROPERTIES_KHR: VkStructureType = 1000163001;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PRESENT_METERING_FEATURES_NV: VkStructureType = 1000613001;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_ENQUEUE_FEATURES_AMDX: VkStructureType = 1000134000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_ENQUEUE_PROPERTIES_AMDX: VkStructureType = 1000134001;
pub const VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_NODE_CREATE_INFO_AMDX: VkStructureType = 1000134004;
pub const VK_STRUCTURE_TYPE_SET_PRESENT_CONFIG_NV: VkStructureType = 1000613000;

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
