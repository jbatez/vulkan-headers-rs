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
    pub color_range: u32,
    pub reserved: u32,
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
    pub loop_filter_delta_enabled: u32,
    pub loop_filter_delta_update: u32,
    pub reserved: u32,
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
    pub segmentation_update_map: u32,
    pub segmentation_temporal_update: u32,
    pub segmentation_update_data: u32,
    pub segmentation_abs_or_delta_update: u32,
    pub reserved: u32,
}

pub type StdVideoVP9ColorSpace = i32;
pub type StdVideoVP9FrameType = i32;
pub type StdVideoVP9InterpolationFilter = i32;
pub type StdVideoVP9Level = i32;
pub type StdVideoVP9Profile = i32;
pub type StdVideoVP9ReferenceName = i32;
