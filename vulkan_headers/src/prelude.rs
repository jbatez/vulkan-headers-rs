pub(crate) use core::{
    ffi::{CStr, c_char, c_int, c_void},
    ptr::NonNull,
};

#[cfg(any(doc, feature = "android_extensions"))]
pub(crate) use crate::platform::android::*;

#[cfg(any(doc, feature = "directfb_extensions"))]
pub(crate) use crate::platform::directfb::*;

#[cfg(any(doc, feature = "fuchsia_extensions"))]
pub(crate) use crate::platform::fuchsia::*;

#[cfg(any(doc, feature = "ggp_extensions"))]
pub(crate) use crate::platform::ggp::*;

#[cfg(any(doc, feature = "metal_extensions"))]
pub(crate) use crate::platform::metal::*;

#[cfg(any(doc, feature = "ohos_extensions"))]
pub(crate) use crate::platform::ohos::*;

#[cfg(any(doc, feature = "screen_extensions"))]
pub(crate) use crate::platform::screen::*;

#[cfg(any(doc, feature = "wayland_extensions"))]
pub(crate) use crate::platform::wayland::*;

#[cfg(any(doc, feature = "win32_extensions"))]
pub(crate) use crate::platform::win32::*;

#[cfg(any(doc, feature = "xcb_extensions"))]
pub(crate) use crate::platform::xcb::*;

#[cfg(any(doc, feature = "xlib_extensions"))]
pub(crate) use crate::platform::xlib::*;

#[cfg(any(doc, feature = "xlib_xrandr_extensions"))]
pub(crate) use crate::platform::xlib_xrandr::*;

pub(crate) use crate::vulkan::vulkan::*;
