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

pub type VkFullScreenExclusiveEXT = i32;
pub type VkWin32SurfaceCreateFlagsKHR = VkFlags;
