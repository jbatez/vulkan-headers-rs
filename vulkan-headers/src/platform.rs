/// Available if built with `android-extensions`.
#[cfg(any(doc, feature = "android-extensions"))]
pub mod android {
    use crate::prelude::*;

    #[repr(C)]
    pub struct AHardwareBuffer {
        _data: (),
        _marker: PhantomData<(*mut u8, PhantomPinned)>,
    }

    #[repr(C)]
    pub struct ANativeWindow {
        _data: (),
        _marker: PhantomData<(*mut u8, PhantomPinned)>,
    }
}

/// Available if built with `directfb-extensions`.
#[cfg(any(doc, feature = "directfb-extensions"))]
pub mod directfb {
    use crate::prelude::*;

    #[repr(C)]
    pub struct IDirectFB {
        _data: (),
        _marker: PhantomData<(*mut u8, PhantomPinned)>,
    }

    #[repr(C)]
    pub struct IDirectFBSurface {
        _data: (),
        _marker: PhantomData<(*mut u8, PhantomPinned)>,
    }
}

/// Available if built with `fuchsia-extensions`.
#[cfg(any(doc, feature = "fuchsia-extensions"))]
pub mod fuchsia {
    use crate::prelude::*;

    pub type zx_handle_t = u32;
}

/// Available if built with `ggp-extensions`.
#[cfg(any(doc, feature = "ggp-extensions"))]
pub mod ggp {
    use crate::prelude::*;

    pub type GgpFrameToken = u64;
    pub type GgpStreamDescriptor = u32;
}

/// Available if built with `metal-extensions`.
#[cfg(any(doc, feature = "metal-extensions"))]
pub mod metal {
    use crate::prelude::*;

    #[repr(C)]
    pub struct __IOSurface {
        _data: (),
        _marker: PhantomData<(*mut u8, PhantomPinned)>,
    }

    pub type CAMetalLayer = c_void;
    pub type IOSurfaceRef = *mut __IOSurface;
    pub type MTLBuffer_id = *mut c_void;
    pub type MTLCommandQueue_id = *mut c_void;
    pub type MTLDevice_id = *mut c_void;
    pub type MTLSharedEvent_id = *mut c_void;
    pub type MTLTexture_id = *mut c_void;
}

/// Available if built with `ohos-extensions`.
#[cfg(any(doc, feature = "ohos-extensions"))]
pub mod ohos {
    use crate::prelude::*;

    #[repr(C)]
    pub struct OHBufferHandle {
        _data: (),
        _marker: PhantomData<(*mut u8, PhantomPinned)>,
    }

    #[repr(C)]
    pub struct OHNativeWindow {
        _data: (),
        _marker: PhantomData<(*mut u8, PhantomPinned)>,
    }

    #[repr(C)]
    pub struct OH_NativeBuffer {
        _data: (),
        _marker: PhantomData<(*mut u8, PhantomPinned)>,
    }
}

/// Available if built with `screen-extensions`.
#[cfg(any(doc, feature = "screen-extensions"))]
pub mod screen {
    use crate::prelude::*;

    #[repr(C)]
    pub struct _screen_buffer {
        _data: (),
        _marker: PhantomData<(*mut u8, PhantomPinned)>,
    }

    #[repr(C)]
    pub struct _screen_context {
        _data: (),
        _marker: PhantomData<(*mut u8, PhantomPinned)>,
    }

    #[repr(C)]
    pub struct _screen_window {
        _data: (),
        _marker: PhantomData<(*mut u8, PhantomPinned)>,
    }
}

/// Available if built with `wayland-extensions`.
#[cfg(any(doc, feature = "wayland-extensions"))]
pub mod wayland {
    use crate::prelude::*;

    #[repr(C)]
    pub struct wl_display {
        _data: (),
        _marker: PhantomData<(*mut u8, PhantomPinned)>,
    }

    #[repr(C)]
    pub struct wl_surface {
        _data: (),
        _marker: PhantomData<(*mut u8, PhantomPinned)>,
    }
}

/// Available if built with `win32-extensions`.
#[cfg(any(doc, feature = "win32-extensions"))]
pub mod win32 {
    use crate::prelude::*;

    #[repr(C)]
    pub struct HINSTANCE__ {
        _data: (),
        _marker: PhantomData<(*mut u8, PhantomPinned)>,
    }

    #[repr(C)]
    pub struct HMONITOR__ {
        _data: (),
        _marker: PhantomData<(*mut u8, PhantomPinned)>,
    }

    #[repr(C)]
    pub struct HWND__ {
        _data: (),
        _marker: PhantomData<(*mut u8, PhantomPinned)>,
    }

    #[repr(C)]
    pub struct _SECURITY_ATTRIBUTES {
        _data: (),
        _marker: PhantomData<(*mut u8, PhantomPinned)>,
    }

    pub type DWORD = u32;
    pub type HANDLE = *mut c_void;
    pub type HINSTANCE = *mut HINSTANCE__;
    pub type HMONITOR = *mut HMONITOR__;
    pub type HWND = *mut HWND__;
    pub type LPCWSTR = *const u16;
    pub type SECURITY_ATTRIBUTES = *mut _SECURITY_ATTRIBUTES;
}

/// Available if built with `xcb-extensions`.
#[cfg(any(doc, feature = "xcb-extensions"))]
pub mod xcb {
    use crate::prelude::*;

    #[repr(C)]
    pub struct xcb_connection_t {
        _data: (),
        _marker: PhantomData<(*mut u8, PhantomPinned)>,
    }

    pub type xcb_visualid_t = u32;
    pub type xcb_window_t = u32;
}

/// Available if built with `xlib-extensions` or `xlib_xrandr-extensions`.
#[cfg(any(doc, feature = "xlib-extensions", feature = "xlib_xrandr-extensions"))]
pub mod xlib {
    use crate::prelude::*;

    #[repr(C)]
    pub struct Display {
        _data: (),
        _marker: PhantomData<(*mut u8, PhantomPinned)>,
    }

    pub type VisualID = c_ulong;
    pub type Window = c_ulong;
}

/// Available if built with `xlib_xrandr-extensions`.
#[cfg(any(doc, feature = "xlib_xrandr-extensions"))]
pub mod xlib_xrandr {
    use crate::prelude::*;

    pub type RROutput = c_ulong;
}
