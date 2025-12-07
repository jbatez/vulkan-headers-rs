use crate::code::*;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkD3D12FenceSubmitInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub waitSemaphoreValuesCount: u32,
    pub pWaitSemaphoreValues: *const u64,
    pub signalSemaphoreValuesCount: u32,
    pub pSignalSemaphoreValues: *const u64,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkExportFenceWin32HandleInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub pAttributes: *const SECURITY_ATTRIBUTES,
    pub dwAccess: DWORD,
    pub name: LPCWSTR,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkExportMemoryWin32HandleInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub pAttributes: *const SECURITY_ATTRIBUTES,
    pub dwAccess: DWORD,
    pub name: LPCWSTR,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkExportMemoryWin32HandleInfoNV {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub pAttributes: *const SECURITY_ATTRIBUTES,
    pub dwAccess: DWORD,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkExportSemaphoreWin32HandleInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub pAttributes: *const SECURITY_ATTRIBUTES,
    pub dwAccess: DWORD,
    pub name: LPCWSTR,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkFenceGetWin32HandleInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub fence: VkFence,
    pub handleType: VkExternalFenceHandleTypeFlagBits,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImportFenceWin32HandleInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub fence: VkFence,
    pub flags: VkFenceImportFlags,
    pub handleType: VkExternalFenceHandleTypeFlagBits,
    pub handle: HANDLE,
    pub name: LPCWSTR,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImportMemoryWin32HandleInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub handleType: VkExternalMemoryHandleTypeFlagBits,
    pub handle: HANDLE,
    pub name: LPCWSTR,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImportMemoryWin32HandleInfoNV {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub handleType: VkExternalMemoryHandleTypeFlagsNV,
    pub handle: HANDLE,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImportSemaphoreWin32HandleInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub semaphore: VkSemaphore,
    pub flags: VkSemaphoreImportFlags,
    pub handleType: VkExternalSemaphoreHandleTypeFlagBits,
    pub handle: HANDLE,
    pub name: LPCWSTR,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkMemoryGetWin32HandleInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub memory: VkDeviceMemory,
    pub handleType: VkExternalMemoryHandleTypeFlagBits,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkMemoryWin32HandlePropertiesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub memoryTypeBits: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSemaphoreGetWin32HandleInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub semaphore: VkSemaphore,
    pub handleType: VkExternalSemaphoreHandleTypeFlagBits,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSurfaceCapabilitiesFullScreenExclusiveEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub fullScreenExclusiveSupported: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSurfaceFullScreenExclusiveInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub fullScreenExclusive: VkFullScreenExclusiveEXT,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSurfaceFullScreenExclusiveWin32InfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub hmonitor: HMONITOR,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkWin32KeyedMutexAcquireReleaseInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub acquireCount: u32,
    pub pAcquireSyncs: *const VkDeviceMemory,
    pub pAcquireKeys: *const u64,
    pub pAcquireTimeouts: *const u32,
    pub releaseCount: u32,
    pub pReleaseSyncs: *const VkDeviceMemory,
    pub pReleaseKeys: *const u64,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkWin32KeyedMutexAcquireReleaseInfoNV {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub acquireCount: u32,
    pub pAcquireSyncs: *const VkDeviceMemory,
    pub pAcquireKeys: *const u64,
    pub pAcquireTimeoutMilliseconds: *const u32,
    pub releaseCount: u32,
    pub pReleaseSyncs: *const VkDeviceMemory,
    pub pReleaseKeys: *const u64,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkWin32SurfaceCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkWin32SurfaceCreateFlagsKHR,
    pub hinstance: HINSTANCE,
    pub hwnd: HWND,
}

pub const VK_ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT: VkResult = -1000255000;
pub const VK_EXT_FULL_SCREEN_EXCLUSIVE_EXTENSION_NAME: &CStr = c"VK_EXT_full_screen_exclusive";
pub const VK_EXT_FULL_SCREEN_EXCLUSIVE_SPEC_VERSION: u32 = 4;
pub const VK_FULL_SCREEN_EXCLUSIVE_ALLOWED_EXT: VkFullScreenExclusiveEXT = 1;
pub const VK_FULL_SCREEN_EXCLUSIVE_APPLICATION_CONTROLLED_EXT: VkFullScreenExclusiveEXT = 3;
pub const VK_FULL_SCREEN_EXCLUSIVE_DEFAULT_EXT: VkFullScreenExclusiveEXT = 0;
pub const VK_FULL_SCREEN_EXCLUSIVE_DISALLOWED_EXT: VkFullScreenExclusiveEXT = 2;
pub const VK_KHR_EXTERNAL_FENCE_WIN32_EXTENSION_NAME: &CStr = c"VK_KHR_external_fence_win32";
pub const VK_KHR_EXTERNAL_FENCE_WIN32_SPEC_VERSION: u32 = 1;
pub const VK_KHR_EXTERNAL_MEMORY_WIN32_EXTENSION_NAME: &CStr = c"VK_KHR_external_memory_win32";
pub const VK_KHR_EXTERNAL_MEMORY_WIN32_SPEC_VERSION: u32 = 1;
pub const VK_KHR_EXTERNAL_SEMAPHORE_WIN32_EXTENSION_NAME: &CStr = c"VK_KHR_external_semaphore_win32";
pub const VK_KHR_EXTERNAL_SEMAPHORE_WIN32_SPEC_VERSION: u32 = 1;
pub const VK_KHR_WIN32_KEYED_MUTEX_EXTENSION_NAME: &CStr = c"VK_KHR_win32_keyed_mutex";
pub const VK_KHR_WIN32_KEYED_MUTEX_SPEC_VERSION: u32 = 1;
pub const VK_KHR_WIN32_SURFACE_EXTENSION_NAME: &CStr = c"VK_KHR_win32_surface";
pub const VK_KHR_WIN32_SURFACE_SPEC_VERSION: u32 = 6;
pub const VK_NV_ACQUIRE_WINRT_DISPLAY_EXTENSION_NAME: &CStr = c"VK_NV_acquire_winrt_display";
pub const VK_NV_ACQUIRE_WINRT_DISPLAY_SPEC_VERSION: u32 = 1;
pub const VK_NV_EXTERNAL_MEMORY_WIN32_EXTENSION_NAME: &CStr = c"VK_NV_external_memory_win32";
pub const VK_NV_EXTERNAL_MEMORY_WIN32_SPEC_VERSION: u32 = 1;
pub const VK_NV_WIN32_KEYED_MUTEX_EXTENSION_NAME: &CStr = c"VK_NV_win32_keyed_mutex";
pub const VK_NV_WIN32_KEYED_MUTEX_SPEC_VERSION: u32 = 2;
pub const VK_STRUCTURE_TYPE_D3D12_FENCE_SUBMIT_INFO_KHR: VkStructureType = 1000078002;
pub const VK_STRUCTURE_TYPE_EXPORT_FENCE_WIN32_HANDLE_INFO_KHR: VkStructureType = 1000114001;
pub const VK_STRUCTURE_TYPE_EXPORT_MEMORY_WIN32_HANDLE_INFO_KHR: VkStructureType = 1000073001;
pub const VK_STRUCTURE_TYPE_EXPORT_MEMORY_WIN32_HANDLE_INFO_NV: VkStructureType = 1000057001;
pub const VK_STRUCTURE_TYPE_EXPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR: VkStructureType = 1000078001;
pub const VK_STRUCTURE_TYPE_FENCE_GET_WIN32_HANDLE_INFO_KHR: VkStructureType = 1000114002;
pub const VK_STRUCTURE_TYPE_IMPORT_FENCE_WIN32_HANDLE_INFO_KHR: VkStructureType = 1000114000;
pub const VK_STRUCTURE_TYPE_IMPORT_MEMORY_WIN32_HANDLE_INFO_KHR: VkStructureType = 1000073000;
pub const VK_STRUCTURE_TYPE_IMPORT_MEMORY_WIN32_HANDLE_INFO_NV: VkStructureType = 1000057000;
pub const VK_STRUCTURE_TYPE_IMPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR: VkStructureType = 1000078000;
pub const VK_STRUCTURE_TYPE_MEMORY_GET_WIN32_HANDLE_INFO_KHR: VkStructureType = 1000073003;
pub const VK_STRUCTURE_TYPE_MEMORY_WIN32_HANDLE_PROPERTIES_KHR: VkStructureType = 1000073002;
pub const VK_STRUCTURE_TYPE_SEMAPHORE_GET_WIN32_HANDLE_INFO_KHR: VkStructureType = 1000078003;
pub const VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES_FULL_SCREEN_EXCLUSIVE_EXT: VkStructureType = 1000255002;
pub const VK_STRUCTURE_TYPE_SURFACE_FULL_SCREEN_EXCLUSIVE_INFO_EXT: VkStructureType = 1000255000;
pub const VK_STRUCTURE_TYPE_SURFACE_FULL_SCREEN_EXCLUSIVE_WIN32_INFO_EXT: VkStructureType = 1000255001;
pub const VK_STRUCTURE_TYPE_WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_KHR: VkStructureType = 1000075000;
pub const VK_STRUCTURE_TYPE_WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_NV: VkStructureType = 1000058000;
pub const VK_STRUCTURE_TYPE_WIN32_SURFACE_CREATE_INFO_KHR: VkStructureType = 1000009000;

unsafe extern "system" {
    /// Available if built with `prototypes`.
    #[cfg(any(doc, feature = "prototypes"))]
    pub fn vkAcquireFullScreenExclusiveModeEXT(device: VkDevice, swapchain: VkSwapchainKHR) -> VkResult;

    /// Available if built with `prototypes`.
    #[cfg(any(doc, feature = "prototypes"))]
    pub fn vkAcquireWinrtDisplayNV(physicalDevice: VkPhysicalDevice, display: VkDisplayKHR) -> VkResult;

    /// Available if built with `prototypes`.
    #[cfg(any(doc, feature = "prototypes"))]
    pub fn vkCreateWin32SurfaceKHR(instance: VkInstance, pCreateInfo: *const VkWin32SurfaceCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult;

    /// Available if built with `prototypes`.
    #[cfg(any(doc, feature = "prototypes"))]
    pub fn vkGetDeviceGroupSurfacePresentModes2EXT(device: VkDevice, pSurfaceInfo: *const VkPhysicalDeviceSurfaceInfo2KHR, pModes: *mut VkDeviceGroupPresentModeFlagsKHR) -> VkResult;

    /// Available if built with `prototypes`.
    #[cfg(any(doc, feature = "prototypes"))]
    pub fn vkGetFenceWin32HandleKHR(device: VkDevice, pGetWin32HandleInfo: *const VkFenceGetWin32HandleInfoKHR, pHandle: *mut HANDLE) -> VkResult;

    /// Available if built with `prototypes`.
    #[cfg(any(doc, feature = "prototypes"))]
    pub fn vkGetMemoryWin32HandleKHR(device: VkDevice, pGetWin32HandleInfo: *const VkMemoryGetWin32HandleInfoKHR, pHandle: *mut HANDLE) -> VkResult;

    /// Available if built with `prototypes`.
    #[cfg(any(doc, feature = "prototypes"))]
    pub fn vkGetMemoryWin32HandleNV(device: VkDevice, memory: VkDeviceMemory, handleType: VkExternalMemoryHandleTypeFlagsNV, pHandle: *mut HANDLE) -> VkResult;

    /// Available if built with `prototypes`.
    #[cfg(any(doc, feature = "prototypes"))]
    pub fn vkGetMemoryWin32HandlePropertiesKHR(device: VkDevice, handleType: VkExternalMemoryHandleTypeFlagBits, handle: HANDLE, pMemoryWin32HandleProperties: *mut VkMemoryWin32HandlePropertiesKHR) -> VkResult;

    /// Available if built with `prototypes`.
    #[cfg(any(doc, feature = "prototypes"))]
    pub fn vkGetPhysicalDeviceSurfacePresentModes2EXT(physicalDevice: VkPhysicalDevice, pSurfaceInfo: *const VkPhysicalDeviceSurfaceInfo2KHR, pPresentModeCount: *mut u32, pPresentModes: *mut VkPresentModeKHR) -> VkResult;

    /// Available if built with `prototypes`.
    #[cfg(any(doc, feature = "prototypes"))]
    pub fn vkGetPhysicalDeviceWin32PresentationSupportKHR(physicalDevice: VkPhysicalDevice, queueFamilyIndex: u32) -> VkBool32;

    /// Available if built with `prototypes`.
    #[cfg(any(doc, feature = "prototypes"))]
    pub fn vkGetSemaphoreWin32HandleKHR(device: VkDevice, pGetWin32HandleInfo: *const VkSemaphoreGetWin32HandleInfoKHR, pHandle: *mut HANDLE) -> VkResult;

    /// Available if built with `prototypes`.
    #[cfg(any(doc, feature = "prototypes"))]
    pub fn vkGetWinrtDisplayNV(physicalDevice: VkPhysicalDevice, deviceRelativeId: u32, pDisplay: *mut VkDisplayKHR) -> VkResult;

    /// Available if built with `prototypes`.
    #[cfg(any(doc, feature = "prototypes"))]
    pub fn vkImportFenceWin32HandleKHR(device: VkDevice, pImportFenceWin32HandleInfo: *const VkImportFenceWin32HandleInfoKHR) -> VkResult;

    /// Available if built with `prototypes`.
    #[cfg(any(doc, feature = "prototypes"))]
    pub fn vkImportSemaphoreWin32HandleKHR(device: VkDevice, pImportSemaphoreWin32HandleInfo: *const VkImportSemaphoreWin32HandleInfoKHR) -> VkResult;

    /// Available if built with `prototypes`.
    #[cfg(any(doc, feature = "prototypes"))]
    pub fn vkReleaseFullScreenExclusiveModeEXT(device: VkDevice, swapchain: VkSwapchainKHR) -> VkResult;
}

pub type NonNullPFN_vkAcquireFullScreenExclusiveModeEXT = unsafe extern "system" fn(device: VkDevice, swapchain: VkSwapchainKHR) -> VkResult;
pub type NonNullPFN_vkAcquireWinrtDisplayNV = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, display: VkDisplayKHR) -> VkResult;
pub type NonNullPFN_vkCreateWin32SurfaceKHR = unsafe extern "system" fn(instance: VkInstance, pCreateInfo: *const VkWin32SurfaceCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult;
pub type NonNullPFN_vkGetDeviceGroupSurfacePresentModes2EXT = unsafe extern "system" fn(device: VkDevice, pSurfaceInfo: *const VkPhysicalDeviceSurfaceInfo2KHR, pModes: *mut VkDeviceGroupPresentModeFlagsKHR) -> VkResult;
pub type NonNullPFN_vkGetFenceWin32HandleKHR = unsafe extern "system" fn(device: VkDevice, pGetWin32HandleInfo: *const VkFenceGetWin32HandleInfoKHR, pHandle: *mut HANDLE) -> VkResult;
pub type NonNullPFN_vkGetMemoryWin32HandleKHR = unsafe extern "system" fn(device: VkDevice, pGetWin32HandleInfo: *const VkMemoryGetWin32HandleInfoKHR, pHandle: *mut HANDLE) -> VkResult;
pub type NonNullPFN_vkGetMemoryWin32HandleNV = unsafe extern "system" fn(device: VkDevice, memory: VkDeviceMemory, handleType: VkExternalMemoryHandleTypeFlagsNV, pHandle: *mut HANDLE) -> VkResult;
pub type NonNullPFN_vkGetMemoryWin32HandlePropertiesKHR = unsafe extern "system" fn(device: VkDevice, handleType: VkExternalMemoryHandleTypeFlagBits, handle: HANDLE, pMemoryWin32HandleProperties: *mut VkMemoryWin32HandlePropertiesKHR) -> VkResult;
pub type NonNullPFN_vkGetPhysicalDeviceSurfacePresentModes2EXT = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pSurfaceInfo: *const VkPhysicalDeviceSurfaceInfo2KHR, pPresentModeCount: *mut u32, pPresentModes: *mut VkPresentModeKHR) -> VkResult;
pub type NonNullPFN_vkGetPhysicalDeviceWin32PresentationSupportKHR = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, queueFamilyIndex: u32) -> VkBool32;
pub type NonNullPFN_vkGetSemaphoreWin32HandleKHR = unsafe extern "system" fn(device: VkDevice, pGetWin32HandleInfo: *const VkSemaphoreGetWin32HandleInfoKHR, pHandle: *mut HANDLE) -> VkResult;
pub type NonNullPFN_vkGetWinrtDisplayNV = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, deviceRelativeId: u32, pDisplay: *mut VkDisplayKHR) -> VkResult;
pub type NonNullPFN_vkImportFenceWin32HandleKHR = unsafe extern "system" fn(device: VkDevice, pImportFenceWin32HandleInfo: *const VkImportFenceWin32HandleInfoKHR) -> VkResult;
pub type NonNullPFN_vkImportSemaphoreWin32HandleKHR = unsafe extern "system" fn(device: VkDevice, pImportSemaphoreWin32HandleInfo: *const VkImportSemaphoreWin32HandleInfoKHR) -> VkResult;
pub type NonNullPFN_vkReleaseFullScreenExclusiveModeEXT = unsafe extern "system" fn(device: VkDevice, swapchain: VkSwapchainKHR) -> VkResult;
pub type PFN_vkAcquireFullScreenExclusiveModeEXT = Option<NonNullPFN_vkAcquireFullScreenExclusiveModeEXT>;
pub type PFN_vkAcquireWinrtDisplayNV = Option<NonNullPFN_vkAcquireWinrtDisplayNV>;
pub type PFN_vkCreateWin32SurfaceKHR = Option<NonNullPFN_vkCreateWin32SurfaceKHR>;
pub type PFN_vkGetDeviceGroupSurfacePresentModes2EXT = Option<NonNullPFN_vkGetDeviceGroupSurfacePresentModes2EXT>;
pub type PFN_vkGetFenceWin32HandleKHR = Option<NonNullPFN_vkGetFenceWin32HandleKHR>;
pub type PFN_vkGetMemoryWin32HandleKHR = Option<NonNullPFN_vkGetMemoryWin32HandleKHR>;
pub type PFN_vkGetMemoryWin32HandleNV = Option<NonNullPFN_vkGetMemoryWin32HandleNV>;
pub type PFN_vkGetMemoryWin32HandlePropertiesKHR = Option<NonNullPFN_vkGetMemoryWin32HandlePropertiesKHR>;
pub type PFN_vkGetPhysicalDeviceSurfacePresentModes2EXT = Option<NonNullPFN_vkGetPhysicalDeviceSurfacePresentModes2EXT>;
pub type PFN_vkGetPhysicalDeviceWin32PresentationSupportKHR = Option<NonNullPFN_vkGetPhysicalDeviceWin32PresentationSupportKHR>;
pub type PFN_vkGetSemaphoreWin32HandleKHR = Option<NonNullPFN_vkGetSemaphoreWin32HandleKHR>;
pub type PFN_vkGetWinrtDisplayNV = Option<NonNullPFN_vkGetWinrtDisplayNV>;
pub type PFN_vkImportFenceWin32HandleKHR = Option<NonNullPFN_vkImportFenceWin32HandleKHR>;
pub type PFN_vkImportSemaphoreWin32HandleKHR = Option<NonNullPFN_vkImportSemaphoreWin32HandleKHR>;
pub type PFN_vkReleaseFullScreenExclusiveModeEXT = Option<NonNullPFN_vkReleaseFullScreenExclusiveModeEXT>;
pub type VkFullScreenExclusiveEXT = i32;
pub type VkWin32SurfaceCreateFlagsKHR = VkFlags;
