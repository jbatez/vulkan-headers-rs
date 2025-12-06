#![cfg(target_pointer_width = "64")]
#![allow(nonstandard_style)]
#![no_std]

pub use code::*;
mod code {
    pub(crate) use core::{ffi::{CStr, c_char, c_int, c_void}, ptr::NonNull};

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

        #[cfg(any(doc, feature = "android_extensions"))]
        pub mod vulkan_android;

        #[cfg(any(doc, feature = "beta_extensions"))]
        pub mod vulkan_beta;

        #[cfg(any(doc, feature = "directfb_extensions"))]
        pub mod vulkan_directfb;

        #[cfg(any(doc, feature = "fuchsia_extensions"))]
        pub mod vulkan_fuchsia;

        #[cfg(any(doc, feature = "ggp_extensions"))]
        pub mod vulkan_ggp;

        #[cfg(any(doc, feature = "ios_extensions"))]
        pub mod vulkan_ios;

        #[cfg(any(doc, feature = "macos_extensions"))]
        pub mod vulkan_macos;

        #[cfg(any(doc, feature = "metal_extensions"))]
        pub mod vulkan_metal;

        #[cfg(any(doc, feature = "ohos_extensions"))]
        pub mod vulkan_ohos;

        #[cfg(any(doc, feature = "screen_extensions"))]
        pub mod vulkan_screen;

        #[cfg(any(doc, feature = "vi_extensions"))]
        pub mod vulkan_vi;

        #[cfg(any(doc, feature = "wayland_extensions"))]
        pub mod vulkan_wayland;

        #[cfg(any(doc, feature = "win32_extensions"))]
        pub mod vulkan_win32;

        #[cfg(any(doc, feature = "xcb_extensions"))]
        pub mod vulkan_xcb;

        #[cfg(any(doc, feature = "xlib_extensions"))]
        pub mod vulkan_xlib;

        #[cfg(any(doc, feature = "xlib_xrandr_extensions"))]
        pub mod vulkan_xlib_xrandr;

        pub(crate) use vulkan::*;
        pub mod vulkan {
            pub use super::vulkan_core::*;

            #[cfg(any(doc, feature = "android_extensions"))]
            pub use super::vulkan_android::*;

            #[cfg(any(doc, feature = "beta_extensions"))]
            pub use super::vulkan_beta::*;

            #[cfg(any(doc, feature = "directfb_extensions"))]
            pub use super::vulkan_directfb::*;

            #[cfg(any(doc, feature = "fuchsia_extensions"))]
            pub use super::vulkan_fuchsia::*;

            #[cfg(any(doc, feature = "ggp_extensions"))]
            pub use super::vulkan_ggp::*;

            #[cfg(any(doc, feature = "ios_extensions"))]
            pub use super::vulkan_ios::*;

            #[cfg(any(doc, feature = "macos_extensions"))]
            pub use super::vulkan_macos::*;

            #[cfg(any(doc, feature = "metal_extensions"))]
            pub use super::vulkan_metal::*;

            #[cfg(any(doc, feature = "ohos_extensions"))]
            pub use super::vulkan_ohos::*;

            #[cfg(any(doc, feature = "screen_extensions"))]
            pub use super::vulkan_screen::*;

            #[cfg(any(doc, feature = "vi_extensions"))]
            pub use super::vulkan_vi::*;

            #[cfg(any(doc, feature = "wayland_extensions"))]
            pub use super::vulkan_wayland::*;

            #[cfg(any(doc, feature = "win32_extensions"))]
            pub use super::vulkan_win32::*;

            #[cfg(any(doc, feature = "xcb_extensions"))]
            pub use super::vulkan_xcb::*;

            #[cfg(any(doc, feature = "xlib_extensions"))]
            pub use super::vulkan_xlib::*;

            #[cfg(any(doc, feature = "xlib_xrandr_extensions"))]
            pub use super::vulkan_xlib_xrandr::*;
        }
    }
}
