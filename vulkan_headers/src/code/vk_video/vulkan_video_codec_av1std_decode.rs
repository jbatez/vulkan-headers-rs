use crate::code::*;

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
    pub error_resilient_mode: u32,
    pub disable_cdf_update: u32,
    pub use_superres: u32,
    pub render_and_frame_size_different: u32,
    pub allow_screen_content_tools: u32,
    pub is_filter_switchable: u32,
    pub force_integer_mv: u32,
    pub frame_size_override_flag: u32,
    pub buffer_removal_time_present_flag: u32,
    pub allow_intrabc: u32,
    pub frame_refs_short_signaling: u32,
    pub allow_high_precision_mv: u32,
    pub is_motion_mode_switchable: u32,
    pub use_ref_frame_mvs: u32,
    pub disable_frame_end_update_cdf: u32,
    pub allow_warped_motion: u32,
    pub reduced_tx_set: u32,
    pub reference_select: u32,
    pub skip_mode_present: u32,
    pub delta_q_present: u32,
    pub delta_lf_present: u32,
    pub delta_lf_multi: u32,
    pub segmentation_enabled: u32,
    pub segmentation_update_map: u32,
    pub segmentation_temporal_update: u32,
    pub segmentation_update_data: u32,
    pub UsesLr: u32,
    pub usesChromaLr: u32,
    pub apply_grain: u32,
    pub reserved: u32,
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
    pub disable_frame_end_update_cdf: u32,
    pub segmentation_enabled: u32,
    pub reserved: u32,
}

pub const VK_STD_VULKAN_VIDEO_CODEC_AV1_DECODE_EXTENSION_NAME: &CStr = c"VK_STD_vulkan_video_codec_av1_decode";
pub const VK_STD_VULKAN_VIDEO_CODEC_AV1_DECODE_SPEC_VERSION: u32 = VK_STD_VULKAN_VIDEO_CODEC_AV1_DECODE_API_VERSION_1_0_0;
