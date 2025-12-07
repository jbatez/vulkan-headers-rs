#[cfg(any(doc, feature = "android_extensions"))]
pub(crate) use android::*;
#[cfg(any(doc, feature = "android_extensions"))]
pub mod android {
    #[cfg_attr(not(doc), repr(u8))]
    pub enum AHardwareBuffer {
        #[doc(hidden)]
        __variant1,
        #[doc(hidden)]
        __variant2,
    }

    #[cfg_attr(not(doc), repr(u8))]
    pub enum ANativeWindow {
        #[doc(hidden)]
        __variant1,
        #[doc(hidden)]
        __variant2,
    }
}

#[cfg(any(doc, feature = "directfb_extensions"))]
pub(crate) use directfb::*;
#[cfg(any(doc, feature = "directfb_extensions"))]
pub mod directfb {
    #[cfg_attr(not(doc), repr(u8))]
    pub enum IDirectFB {
        #[doc(hidden)]
        __variant1,
        #[doc(hidden)]
        __variant2,
    }

    #[cfg_attr(not(doc), repr(u8))]
    pub enum IDirectFBSurface {
        #[doc(hidden)]
        __variant1,
        #[doc(hidden)]
        __variant2,
    }
}

#[cfg(any(doc, feature = "fuchsia_extensions"))]
pub(crate) use fuchsia::*;
#[cfg(any(doc, feature = "fuchsia_extensions"))]
pub mod fuchsia {
    pub type zx_handle_t = u32;
}

#[cfg(any(doc, feature = "ggp_extensions"))]
pub(crate) use ggp::*;
#[cfg(any(doc, feature = "ggp_extensions"))]
pub mod ggp {
    pub type GgpFrameToken = u64;
    pub type GgpStreamDescriptor = u32;
}

#[cfg(any(doc, feature = "metal_extensions"))]
pub(crate) use metal::*;
#[cfg(any(doc, feature = "metal_extensions"))]
pub mod metal {
    use core::ffi::c_void;

    #[cfg_attr(not(doc), repr(u8))]
    pub enum __IOSurface {
        #[doc(hidden)]
        __variant1,
        #[doc(hidden)]
        __variant2,
    }

    pub type CAMetalLayer = c_void;
    pub type IOSurfaceRef = *mut __IOSurface;
    pub type MTLBuffer_id = *mut c_void;
    pub type MTLCommandQueue_id = *mut c_void;
    pub type MTLDevice_id = *mut c_void;
    pub type MTLSharedEvent_id = *mut c_void;
    pub type MTLTexture_id = *mut c_void;
}

#[cfg(any(doc, feature = "ohos_extensions"))]
pub(crate) use ohos::*;
#[cfg(any(doc, feature = "ohos_extensions"))]
pub mod ohos {
    #[cfg_attr(not(doc), repr(u8))]
    pub enum NativeWindow {
        #[doc(hidden)]
        __variant1,
        #[doc(hidden)]
        __variant2,
    }

    #[cfg_attr(not(doc), repr(u8))]
    pub enum OHBufferHandle {
        #[doc(hidden)]
        __variant1,
        #[doc(hidden)]
        __variant2,
    }

    #[cfg_attr(not(doc), repr(u8))]
    pub enum OH_NativeBuffer {
        #[doc(hidden)]
        __variant1,
        #[doc(hidden)]
        __variant2,
    }

    pub type OHNativeWindow = NativeWindow;
}

#[cfg(any(doc, feature = "screen_extensions"))]
pub(crate) use screen::*;
#[cfg(any(doc, feature = "screen_extensions"))]
pub mod screen {
    #[cfg_attr(not(doc), repr(u8))]
    pub enum _screen_buffer {
        #[doc(hidden)]
        __variant1,
        #[doc(hidden)]
        __variant2,
    }

    #[cfg_attr(not(doc), repr(u8))]
    pub enum _screen_context {
        #[doc(hidden)]
        __variant1,
        #[doc(hidden)]
        __variant2,
    }

    #[cfg_attr(not(doc), repr(u8))]
    pub enum _screen_window {
        #[doc(hidden)]
        __variant1,
        #[doc(hidden)]
        __variant2,
    }
}

#[cfg(any(doc, feature = "wayland_extensions"))]
pub(crate) use wayland::*;
#[cfg(any(doc, feature = "wayland_extensions"))]
pub mod wayland {
    #[cfg_attr(not(doc), repr(u8))]
    pub enum wl_display {
        #[doc(hidden)]
        __variant1,
        #[doc(hidden)]
        __variant2,
    }

    #[cfg_attr(not(doc), repr(u8))]
    pub enum wl_surface {
        #[doc(hidden)]
        __variant1,
        #[doc(hidden)]
        __variant2,
    }
}

#[cfg(any(doc, feature = "win32_extensions"))]
pub(crate) use win32::*;
#[cfg(any(doc, feature = "win32_extensions"))]
pub mod win32 {
    use core::ffi::c_void;

    #[cfg_attr(not(doc), repr(u8))]
    pub enum HINSTANCE__ {
        #[doc(hidden)]
        __variant1,
        #[doc(hidden)]
        __variant2,
    }

    #[cfg_attr(not(doc), repr(u8))]
    pub enum HMONITOR__ {
        #[doc(hidden)]
        __variant1,
        #[doc(hidden)]
        __variant2,
    }

    #[cfg_attr(not(doc), repr(u8))]
    pub enum HWND__ {
        #[doc(hidden)]
        __variant1,
        #[doc(hidden)]
        __variant2,
    }

    #[cfg_attr(not(doc), repr(u8))]
    pub enum _SECURITY_ATTRIBUTES {
        #[doc(hidden)]
        __variant1,
        #[doc(hidden)]
        __variant2,
    }

    pub type DWORD = u32;
    pub type HANDLE = *mut c_void;
    pub type HINSTANCE = *mut HINSTANCE__;
    pub type HMONITOR = *mut HMONITOR__;
    pub type HWND = *mut HWND__;
    pub type LPCWSTR = *const u16;
    pub type SECURITY_ATTRIBUTES = *mut _SECURITY_ATTRIBUTES;
}

#[cfg(any(doc, feature = "xcb_extensions"))]
pub(crate) use xcb::*;
#[cfg(any(doc, feature = "xcb_extensions"))]
pub mod xcb {
    #[cfg_attr(not(doc), repr(u8))]
    pub enum xcb_connection_t {
        #[doc(hidden)]
        __variant1,
        #[doc(hidden)]
        __variant2,
    }

    pub type xcb_visualid_t = u32;
    pub type xcb_window_t = u32;
}

#[cfg(any(doc, feature = "xlib_extensions"))]
pub(crate) use xlib::*;
#[cfg(any(doc, feature = "xlib_extensions"))]
pub mod xlib {
    use core::ffi::{c_uint, c_ulong};

    #[cfg_attr(not(doc), repr(u8))]
    pub enum Display {
        #[doc(hidden)]
        __variant1,
        #[doc(hidden)]
        __variant2,
    }

    pub type VisualID = c_uint;
    pub type Window = c_ulong;
}

#[cfg(any(doc, feature = "xlib_xrandr_extensions"))]
pub(crate) use xlib_xrandr::*;
#[cfg(any(doc, feature = "xlib_xrandr_extensions"))]
pub mod xlib_xrandr {
    use core::ffi::c_ulong;

    pub type RROutput = c_ulong;
}
