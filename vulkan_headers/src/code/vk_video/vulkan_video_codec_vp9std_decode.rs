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
    pub bitfields: u32,
}

pub const VK_STD_VULKAN_VIDEO_CODEC_VP9_DECODE_API_VERSION_1_0_0: u32 = VK_MAKE_VIDEO_STD_VERSION(1, 0, 0);
pub const VK_STD_VULKAN_VIDEO_CODEC_VP9_DECODE_EXTENSION_NAME: &CStr = c"VK_STD_vulkan_video_codec_vp9_decode";
pub const VK_STD_VULKAN_VIDEO_CODEC_VP9_DECODE_SPEC_VERSION: u32 = VK_STD_VULKAN_VIDEO_CODEC_VP9_DECODE_API_VERSION_1_0_0;
