use crate::prelude::*;
use super::vulkan_video_codec_av1std::*;
use super::vulkan_video_codecs_common::*;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct StdVideoDecodeAV1PictureInfo {
    pub flags: StdVideoDecodeAV1PictureInfoFlags,
    pub frame_type: StdVideoAV1FrameType,
    pub current_frame_id: u32,
    pub OrderHint: u8,
    pub primary_ref_frame: u8,
    pub refresh_frame_flags: u8,
    pub reserved1: u8,
    pub interpolation_filter: StdVideoAV1InterpolationFilter,
    pub TxMode: StdVideoAV1TxMode,
    pub delta_q_res: u8,
    pub delta_lf_res: u8,
    pub SkipModeFrame: [u8; STD_VIDEO_AV1_SKIP_MODE_FRAMES as usize],
    pub coded_denom: u8,
    pub reserved2: [u8; 3 as usize],
    pub OrderHints: [u8; STD_VIDEO_AV1_NUM_REF_FRAMES as usize],
    pub expectedFrameId: [u32; STD_VIDEO_AV1_NUM_REF_FRAMES as usize],
    pub pTileInfo: *const StdVideoAV1TileInfo,
    pub pQuantization: *const StdVideoAV1Quantization,
    pub pSegmentation: *const StdVideoAV1Segmentation,
    pub pLoopFilter: *const StdVideoAV1LoopFilter,
    pub pCDEF: *const StdVideoAV1CDEF,
    pub pLoopRestoration: *const StdVideoAV1LoopRestoration,
    pub pGlobalMotion: *const StdVideoAV1GlobalMotion,
    pub pFilmGrain: *const StdVideoAV1FilmGrain,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct StdVideoDecodeAV1PictureInfoFlags {
    pub bitfields: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct StdVideoDecodeAV1ReferenceInfo {
    pub flags: StdVideoDecodeAV1ReferenceInfoFlags,
    pub frame_type: u8,
    pub RefFrameSignBias: u8,
    pub OrderHint: u8,
    pub SavedOrderHints: [u8; STD_VIDEO_AV1_NUM_REF_FRAMES as usize],
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct StdVideoDecodeAV1ReferenceInfoFlags {
    pub bitfields: u32,
}

pub const VK_STD_VULKAN_VIDEO_CODEC_AV1_DECODE_API_VERSION_1_0_0: u32 = VK_MAKE_VIDEO_STD_VERSION(1, 0, 0);
pub const VK_STD_VULKAN_VIDEO_CODEC_AV1_DECODE_EXTENSION_NAME: &CStr = c"VK_STD_vulkan_video_codec_av1_decode";
pub const VK_STD_VULKAN_VIDEO_CODEC_AV1_DECODE_SPEC_VERSION: u32 = VK_STD_VULKAN_VIDEO_CODEC_AV1_DECODE_API_VERSION_1_0_0;
