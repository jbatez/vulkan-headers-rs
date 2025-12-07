#![cfg(target_pointer_width = "64")]
#![allow(nonstandard_style, unused_imports)]
#![no_std]

pub use code::*;
mod code {
    pub(crate) use core::{ffi::{CStr, c_char, c_int, c_void}, ptr::NonNull};

    pub(crate) use platform::*;
    pub mod platform;

    pub(crate) use vk_video::*;
    pub mod vk_video {
        pub(crate) use vulkan_video_codec_av1std::*;
        pub mod vulkan_video_codec_av1std;

        pub(crate) use vulkan_video_codec_av1std_decode::*;
        pub mod vulkan_video_codec_av1std_decode;

        pub(crate) use vulkan_video_codec_av1std_encode::*;
        pub mod vulkan_video_codec_av1std_encode;

        pub(crate) use vulkan_video_codec_h264std::*;
        pub mod vulkan_video_codec_h264std;

        pub(crate) use vulkan_video_codec_h264std_decode::*;
        pub mod vulkan_video_codec_h264std_decode;

        pub(crate) use vulkan_video_codec_h264std_encode::*;
        pub mod vulkan_video_codec_h264std_encode;

        pub(crate) use vulkan_video_codec_h265std::*;
        pub mod vulkan_video_codec_h265std;

        pub(crate) use vulkan_video_codec_h265std_decode::*;
        pub mod vulkan_video_codec_h265std_decode;

        pub(crate) use vulkan_video_codec_h265std_encode::*;
        pub mod vulkan_video_codec_h265std_encode;

        pub(crate) use vulkan_video_codec_vp9std::*;
        pub mod vulkan_video_codec_vp9std;

        pub(crate) use vulkan_video_codec_vp9std_decode::*;
        pub mod vulkan_video_codec_vp9std_decode;

        pub(crate) use vulkan_video_codecs_common::*;
        pub mod vulkan_video_codecs_common;
    }

    pub(crate) use vulkan::*;
    pub mod vulkan {
        pub mod vulkan_core;

        /// Available if built with `android_extensions`.
        #[cfg(any(doc, feature = "android_extensions"))]
        pub mod vulkan_android;

        /// Available if built with `beta_extensions`.
        #[cfg(any(doc, feature = "beta_extensions"))]
        pub mod vulkan_beta;

        /// Available if built with `directfb_extensions`.
        #[cfg(any(doc, feature = "directfb_extensions"))]
        pub mod vulkan_directfb;

        /// Available if built with `fuchsia_extensions`.
        #[cfg(any(doc, feature = "fuchsia_extensions"))]
        pub mod vulkan_fuchsia;

        /// Available if built with `ggp_extensions`.
        #[cfg(any(doc, feature = "ggp_extensions"))]
        pub mod vulkan_ggp;

        /// Available if built with `ios_extensions`.
        #[cfg(any(doc, feature = "ios_extensions"))]
        pub mod vulkan_ios;

        /// Available if built with `macos_extensions`.
        #[cfg(any(doc, feature = "macos_extensions"))]
        pub mod vulkan_macos;

        /// Available if built with `metal_extensions`.
        #[cfg(any(doc, feature = "metal_extensions"))]
        pub mod vulkan_metal;

        /// Available if built with `ohos_extensions`.
        #[cfg(any(doc, feature = "ohos_extensions"))]
        pub mod vulkan_ohos;

        /// Available if built with `screen_extensions`.
        #[cfg(any(doc, feature = "screen_extensions"))]
        pub mod vulkan_screen;

        /// Available if built with `vi_extensions`.
        #[cfg(any(doc, feature = "vi_extensions"))]
        pub mod vulkan_vi;

        /// Available if built with `wayland_extensions`.
        #[cfg(any(doc, feature = "wayland_extensions"))]
        pub mod vulkan_wayland;

        /// Available if built with `win32_extensions`.
        #[cfg(any(doc, feature = "win32_extensions"))]
        pub mod vulkan_win32;

        /// Available if built with `xcb_extensions`.
        #[cfg(any(doc, feature = "xcb_extensions"))]
        pub mod vulkan_xcb;

        /// Available if built with `xlib_extensions`.
        #[cfg(any(doc, feature = "xlib_extensions"))]
        pub mod vulkan_xlib;

        /// Available if built with `xlib_xrandr_extensions`.
        #[cfg(any(doc, feature = "xlib_xrandr_extensions"))]
        pub mod vulkan_xlib_xrandr;

        pub(crate) use vulkan::*;
        pub mod vulkan {
            #[doc(no_inline)]
            pub use super::vulkan_core::*;

            #[cfg(any(doc, feature = "android_extensions"))]
            #[doc(no_inline)]
            pub use super::vulkan_android::*;

            #[cfg(any(doc, feature = "beta_extensions"))]
            #[doc(no_inline)]
            pub use super::vulkan_beta::*;

            #[cfg(any(doc, feature = "directfb_extensions"))]
            #[doc(no_inline)]
            pub use super::vulkan_directfb::*;

            #[cfg(any(doc, feature = "fuchsia_extensions"))]
            #[doc(no_inline)]
            pub use super::vulkan_fuchsia::*;

            #[cfg(any(doc, feature = "ggp_extensions"))]
            #[doc(no_inline)]
            pub use super::vulkan_ggp::*;

            #[cfg(any(doc, feature = "ios_extensions"))]
            #[doc(no_inline)]
            pub use super::vulkan_ios::*;

            #[cfg(any(doc, feature = "macos_extensions"))]
            #[doc(no_inline)]
            pub use super::vulkan_macos::*;

            #[cfg(any(doc, feature = "metal_extensions"))]
            #[doc(no_inline)]
            pub use super::vulkan_metal::*;

            #[cfg(any(doc, feature = "ohos_extensions"))]
            #[doc(no_inline)]
            pub use super::vulkan_ohos::*;

            #[cfg(any(doc, feature = "screen_extensions"))]
            #[doc(no_inline)]
            pub use super::vulkan_screen::*;

            #[cfg(any(doc, feature = "vi_extensions"))]
            #[doc(no_inline)]
            pub use super::vulkan_vi::*;

            #[cfg(any(doc, feature = "wayland_extensions"))]
            #[doc(no_inline)]
            pub use super::vulkan_wayland::*;

            #[cfg(any(doc, feature = "win32_extensions"))]
            #[doc(no_inline)]
            pub use super::vulkan_win32::*;

            #[cfg(any(doc, feature = "xcb_extensions"))]
            #[doc(no_inline)]
            pub use super::vulkan_xcb::*;

            #[cfg(any(doc, feature = "xlib_extensions"))]
            #[doc(no_inline)]
            pub use super::vulkan_xlib::*;

            #[cfg(any(doc, feature = "xlib_xrandr_extensions"))]
            #[doc(no_inline)]
            pub use super::vulkan_xlib_xrandr::*;
        }
    }
}
