use crate::code::*;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct StdVideoDecodeVP9PictureInfo {
    pub flags: StdVideoDecodeVP9PictureInfoFlags,
    pub profile: StdVideoVP9Profile,
    pub frame_type: StdVideoVP9FrameType,
    pub frame_context_idx: u8,
    pub reset_frame_context: u8,
    pub refresh_frame_flags: u8,
    pub ref_frame_sign_bias_mask: u8,
    pub interpolation_filter: StdVideoVP9InterpolationFilter,
    pub base_q_idx: u8,
    pub delta_q_y_dc: i8,
    pub delta_q_uv_dc: i8,
    pub delta_q_uv_ac: i8,
    pub tile_cols_log2: u8,
    pub tile_rows_log2: u8,
    pub reserved1: [u16; 3 as usize],
    pub pColorConfig: *const StdVideoVP9ColorConfig,
    pub pLoopFilter: *const StdVideoVP9LoopFilter,
    pub pSegmentation: *const StdVideoVP9Segmentation,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct StdVideoDecodeVP9PictureInfoFlags {
    pub error_resilient_mode: u32,
    pub intra_only: u32,
    pub allow_high_precision_mv: u32,
    pub refresh_frame_context: u32,
    pub frame_parallel_decoding_mode: u32,
    pub segmentation_enabled: u32,
    pub show_frame: u32,
    pub UsePrevFrameMvs: u32,
    pub reserved: u32,
}
