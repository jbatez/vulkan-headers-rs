use crate::code::*;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct StdVideoVP9ColorConfig {
    pub flags: StdVideoVP9ColorConfigFlags,
    pub BitDepth: u8,
    pub subsampling_x: u8,
    pub subsampling_y: u8,
    pub reserved1: u8,
    pub color_space: StdVideoVP9ColorSpace,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct StdVideoVP9ColorConfigFlags {
    pub bitfields: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct StdVideoVP9LoopFilter {
    pub flags: StdVideoVP9LoopFilterFlags,
    pub loop_filter_level: u8,
    pub loop_filter_sharpness: u8,
    pub update_ref_delta: u8,
    pub loop_filter_ref_deltas: [i8; STD_VIDEO_VP9_MAX_REF_FRAMES as usize],
    pub update_mode_delta: u8,
    pub loop_filter_mode_deltas: [i8; STD_VIDEO_VP9_LOOP_FILTER_ADJUSTMENTS as usize],
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct StdVideoVP9LoopFilterFlags {
    pub bitfields: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct StdVideoVP9Segmentation {
    pub flags: StdVideoVP9SegmentationFlags,
    pub segmentation_tree_probs: [u8; STD_VIDEO_VP9_MAX_SEGMENTATION_TREE_PROBS as usize],
    pub segmentation_pred_prob: [u8; STD_VIDEO_VP9_MAX_SEGMENTATION_PRED_PROB as usize],
    pub FeatureEnabled: [u8; STD_VIDEO_VP9_MAX_SEGMENTS as usize],
    pub FeatureData: [[i16; STD_VIDEO_VP9_SEG_LVL_MAX as usize]; STD_VIDEO_VP9_MAX_SEGMENTS as usize],
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct StdVideoVP9SegmentationFlags {
    pub bitfields: u32,
}

pub const STD_VIDEO_VP9_COLOR_SPACE_BT_2020: StdVideoVP9ColorSpace = 5;
pub const STD_VIDEO_VP9_COLOR_SPACE_BT_601: StdVideoVP9ColorSpace = 1;
pub const STD_VIDEO_VP9_COLOR_SPACE_BT_709: StdVideoVP9ColorSpace = 2;
pub const STD_VIDEO_VP9_COLOR_SPACE_INVALID: StdVideoVP9ColorSpace = 0x7FFFFFFF;
pub const STD_VIDEO_VP9_COLOR_SPACE_RESERVED: StdVideoVP9ColorSpace = 6;
pub const STD_VIDEO_VP9_COLOR_SPACE_RGB: StdVideoVP9ColorSpace = 7;
pub const STD_VIDEO_VP9_COLOR_SPACE_SMPTE_170: StdVideoVP9ColorSpace = 3;
pub const STD_VIDEO_VP9_COLOR_SPACE_SMPTE_240: StdVideoVP9ColorSpace = 4;
pub const STD_VIDEO_VP9_COLOR_SPACE_UNKNOWN: StdVideoVP9ColorSpace = 0;
pub const STD_VIDEO_VP9_FRAME_TYPE_INVALID: StdVideoVP9FrameType = 0x7FFFFFFF;
pub const STD_VIDEO_VP9_FRAME_TYPE_KEY: StdVideoVP9FrameType = 0;
pub const STD_VIDEO_VP9_FRAME_TYPE_NON_KEY: StdVideoVP9FrameType = 1;
pub const STD_VIDEO_VP9_INTERPOLATION_FILTER_BILINEAR: StdVideoVP9InterpolationFilter = 3;
pub const STD_VIDEO_VP9_INTERPOLATION_FILTER_EIGHTTAP: StdVideoVP9InterpolationFilter = 0;
pub const STD_VIDEO_VP9_INTERPOLATION_FILTER_EIGHTTAP_SHARP: StdVideoVP9InterpolationFilter = 2;
pub const STD_VIDEO_VP9_INTERPOLATION_FILTER_EIGHTTAP_SMOOTH: StdVideoVP9InterpolationFilter = 1;
pub const STD_VIDEO_VP9_INTERPOLATION_FILTER_INVALID: StdVideoVP9InterpolationFilter = 0x7FFFFFFF;
pub const STD_VIDEO_VP9_INTERPOLATION_FILTER_SWITCHABLE: StdVideoVP9InterpolationFilter = 4;
pub const STD_VIDEO_VP9_LEVEL_1_0: StdVideoVP9Level = 0;
pub const STD_VIDEO_VP9_LEVEL_1_1: StdVideoVP9Level = 1;
pub const STD_VIDEO_VP9_LEVEL_2_0: StdVideoVP9Level = 2;
pub const STD_VIDEO_VP9_LEVEL_2_1: StdVideoVP9Level = 3;
pub const STD_VIDEO_VP9_LEVEL_3_0: StdVideoVP9Level = 4;
pub const STD_VIDEO_VP9_LEVEL_3_1: StdVideoVP9Level = 5;
pub const STD_VIDEO_VP9_LEVEL_4_0: StdVideoVP9Level = 6;
pub const STD_VIDEO_VP9_LEVEL_4_1: StdVideoVP9Level = 7;
pub const STD_VIDEO_VP9_LEVEL_5_0: StdVideoVP9Level = 8;
pub const STD_VIDEO_VP9_LEVEL_5_1: StdVideoVP9Level = 9;
pub const STD_VIDEO_VP9_LEVEL_5_2: StdVideoVP9Level = 10;
pub const STD_VIDEO_VP9_LEVEL_6_0: StdVideoVP9Level = 11;
pub const STD_VIDEO_VP9_LEVEL_6_1: StdVideoVP9Level = 12;
pub const STD_VIDEO_VP9_LEVEL_6_2: StdVideoVP9Level = 13;
pub const STD_VIDEO_VP9_LEVEL_INVALID: StdVideoVP9Level = 0x7FFFFFFF;
pub const STD_VIDEO_VP9_LOOP_FILTER_ADJUSTMENTS: u32 = 2;
pub const STD_VIDEO_VP9_MAX_REF_FRAMES: u32 = 4;
pub const STD_VIDEO_VP9_MAX_SEGMENTATION_PRED_PROB: u32 = 3;
pub const STD_VIDEO_VP9_MAX_SEGMENTATION_TREE_PROBS: u32 = 7;
pub const STD_VIDEO_VP9_MAX_SEGMENTS: u32 = 8;
pub const STD_VIDEO_VP9_NUM_REF_FRAMES: u32 = 8;
pub const STD_VIDEO_VP9_PROFILE_0: StdVideoVP9Profile = 0;
pub const STD_VIDEO_VP9_PROFILE_1: StdVideoVP9Profile = 1;
pub const STD_VIDEO_VP9_PROFILE_2: StdVideoVP9Profile = 2;
pub const STD_VIDEO_VP9_PROFILE_3: StdVideoVP9Profile = 3;
pub const STD_VIDEO_VP9_PROFILE_INVALID: StdVideoVP9Profile = 0x7FFFFFFF;
pub const STD_VIDEO_VP9_REFERENCE_NAME_ALTREF_FRAME: StdVideoVP9ReferenceName = 3;
pub const STD_VIDEO_VP9_REFERENCE_NAME_GOLDEN_FRAME: StdVideoVP9ReferenceName = 2;
pub const STD_VIDEO_VP9_REFERENCE_NAME_INTRA_FRAME: StdVideoVP9ReferenceName = 0;
pub const STD_VIDEO_VP9_REFERENCE_NAME_INVALID: StdVideoVP9ReferenceName = 0x7FFFFFFF;
pub const STD_VIDEO_VP9_REFERENCE_NAME_LAST_FRAME: StdVideoVP9ReferenceName = 1;
pub const STD_VIDEO_VP9_REFS_PER_FRAME: u32 = 3;
pub const STD_VIDEO_VP9_SEG_LVL_MAX: u32 = 4;

pub type StdVideoVP9ColorSpace = i32;
pub type StdVideoVP9FrameType = i32;
pub type StdVideoVP9InterpolationFilter = i32;
pub type StdVideoVP9Level = i32;
pub type StdVideoVP9Profile = i32;
pub type StdVideoVP9ReferenceName = i32;
