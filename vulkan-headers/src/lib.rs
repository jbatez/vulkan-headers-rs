#![allow(nonstandard_style, unused_imports)]
#![doc = include_str!("../README.md")]
#![no_std]

const _: () = assert!(cfg!(target_pointer_width = "64"));

pub mod platform;

mod prelude;

pub mod vk_video {
    pub mod vulkan_video_codec_av1std;
    pub mod vulkan_video_codec_av1std_decode;
    pub mod vulkan_video_codec_av1std_encode;
    pub mod vulkan_video_codec_h264std;
    pub mod vulkan_video_codec_h264std_decode;
    pub mod vulkan_video_codec_h264std_encode;
    pub mod vulkan_video_codec_h265std;
    pub mod vulkan_video_codec_h265std_decode;
    pub mod vulkan_video_codec_h265std_encode;
    pub mod vulkan_video_codec_vp9std;
    pub mod vulkan_video_codec_vp9std_decode;
    pub mod vulkan_video_codecs_common;
}

pub mod vulkan {
    pub mod vulkan;
    pub mod vulkan_core;

    /// Available if built with `android-extensions`.
    #[cfg(any(doc, feature = "android-extensions"))]
    pub mod vulkan_android;

    /// Available if built with `beta-extensions`.
    #[cfg(any(doc, feature = "beta-extensions"))]
    pub mod vulkan_beta;

    /// Available if built with `directfb-extensions`.
    #[cfg(any(doc, feature = "directfb-extensions"))]
    pub mod vulkan_directfb;

    /// Available if built with `fuchsia-extensions`.
    #[cfg(any(doc, feature = "fuchsia-extensions"))]
    pub mod vulkan_fuchsia;

    /// Available if built with `ggp-extensions`.
    #[cfg(any(doc, feature = "ggp-extensions"))]
    pub mod vulkan_ggp;

    /// Available if built with `ios-extensions`.
    #[cfg(any(doc, feature = "ios-extensions"))]
    pub mod vulkan_ios;

    /// Available if built with `macos-extensions`.
    #[cfg(any(doc, feature = "macos-extensions"))]
    pub mod vulkan_macos;

    /// Available if built with `metal-extensions`.
    #[cfg(any(doc, feature = "metal-extensions"))]
    pub mod vulkan_metal;

    /// Available if built with `ohos-extensions`.
    #[cfg(any(doc, feature = "ohos-extensions"))]
    pub mod vulkan_ohos;

    /// Available if built with `screen-extensions`.
    #[cfg(any(doc, feature = "screen-extensions"))]
    pub mod vulkan_screen;

    /// Available if built with `vi-extensions`.
    #[cfg(any(doc, feature = "vi-extensions"))]
    pub mod vulkan_vi;

    /// Available if built with `wayland-extensions`.
    #[cfg(any(doc, feature = "wayland-extensions"))]
    pub mod vulkan_wayland;

    /// Available if built with `win32-extensions`.
    #[cfg(any(doc, feature = "win32-extensions"))]
    pub mod vulkan_win32;

    /// Available if built with `xcb-extensions`.
    #[cfg(any(doc, feature = "xcb-extensions"))]
    pub mod vulkan_xcb;

    /// Available if built with `xlib-extensions`.
    #[cfg(any(doc, feature = "xlib-extensions"))]
    pub mod vulkan_xlib;

    /// Available if built with `xlib_xrandr-extensions`.
    #[cfg(any(doc, feature = "xlib_xrandr-extensions"))]
    pub mod vulkan_xlib_xrandr;
}
