use crate::code::*;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct StdVideoAV1CDEF {
    pub cdef_damping_minus_3: u8,
    pub cdef_bits: u8,
    pub cdef_y_pri_strength: [u8; STD_VIDEO_AV1_MAX_CDEF_FILTER_STRENGTHS as usize],
    pub cdef_y_sec_strength: [u8; STD_VIDEO_AV1_MAX_CDEF_FILTER_STRENGTHS as usize],
    pub cdef_uv_pri_strength: [u8; STD_VIDEO_AV1_MAX_CDEF_FILTER_STRENGTHS as usize],
    pub cdef_uv_sec_strength: [u8; STD_VIDEO_AV1_MAX_CDEF_FILTER_STRENGTHS as usize],
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct StdVideoAV1ColorConfig {
    pub flags: StdVideoAV1ColorConfigFlags,
    pub BitDepth: u8,
    pub subsampling_x: u8,
    pub subsampling_y: u8,
    pub reserved1: u8,
    pub color_primaries: StdVideoAV1ColorPrimaries,
    pub transfer_characteristics: StdVideoAV1TransferCharacteristics,
    pub matrix_coefficients: StdVideoAV1MatrixCoefficients,
    pub chroma_sample_position: StdVideoAV1ChromaSamplePosition,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct StdVideoAV1ColorConfigFlags {
    pub mono_chrome: u32,
    pub color_range: u32,
    pub separate_uv_delta_q: u32,
    pub color_description_present_flag: u32,
    pub reserved: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct StdVideoAV1FilmGrain {
    pub flags: StdVideoAV1FilmGrainFlags,
    pub grain_scaling_minus_8: u8,
    pub ar_coeff_lag: u8,
    pub ar_coeff_shift_minus_6: u8,
    pub grain_scale_shift: u8,
    pub grain_seed: u16,
    pub film_grain_params_ref_idx: u8,
    pub num_y_points: u8,
    pub point_y_value: [u8; STD_VIDEO_AV1_MAX_NUM_Y_POINTS as usize],
    pub point_y_scaling: [u8; STD_VIDEO_AV1_MAX_NUM_Y_POINTS as usize],
    pub num_cb_points: u8,
    pub point_cb_value: [u8; STD_VIDEO_AV1_MAX_NUM_CB_POINTS as usize],
    pub point_cb_scaling: [u8; STD_VIDEO_AV1_MAX_NUM_CB_POINTS as usize],
    pub num_cr_points: u8,
    pub point_cr_value: [u8; STD_VIDEO_AV1_MAX_NUM_CR_POINTS as usize],
    pub point_cr_scaling: [u8; STD_VIDEO_AV1_MAX_NUM_CR_POINTS as usize],
    pub ar_coeffs_y_plus_128: [i8; STD_VIDEO_AV1_MAX_NUM_POS_LUMA as usize],
    pub ar_coeffs_cb_plus_128: [i8; STD_VIDEO_AV1_MAX_NUM_POS_CHROMA as usize],
    pub ar_coeffs_cr_plus_128: [i8; STD_VIDEO_AV1_MAX_NUM_POS_CHROMA as usize],
    pub cb_mult: u8,
    pub cb_luma_mult: u8,
    pub cb_offset: u16,
    pub cr_mult: u8,
    pub cr_luma_mult: u8,
    pub cr_offset: u16,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct StdVideoAV1FilmGrainFlags {
    pub chroma_scaling_from_luma: u32,
    pub overlap_flag: u32,
    pub clip_to_restricted_range: u32,
    pub update_grain: u32,
    pub reserved: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct StdVideoAV1GlobalMotion {
    pub GmType: [u8; STD_VIDEO_AV1_NUM_REF_FRAMES as usize],
    pub gm_params: [[i32; STD_VIDEO_AV1_GLOBAL_MOTION_PARAMS as usize]; STD_VIDEO_AV1_NUM_REF_FRAMES as usize],
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct StdVideoAV1LoopFilter {
    pub flags: StdVideoAV1LoopFilterFlags,
    pub loop_filter_level: [u8; STD_VIDEO_AV1_MAX_LOOP_FILTER_STRENGTHS as usize],
    pub loop_filter_sharpness: u8,
    pub update_ref_delta: u8,
    pub loop_filter_ref_deltas: [i8; STD_VIDEO_AV1_TOTAL_REFS_PER_FRAME as usize],
    pub update_mode_delta: u8,
    pub loop_filter_mode_deltas: [i8; STD_VIDEO_AV1_LOOP_FILTER_ADJUSTMENTS as usize],
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct StdVideoAV1LoopFilterFlags {
    pub loop_filter_delta_enabled: u32,
    pub loop_filter_delta_update: u32,
    pub reserved: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct StdVideoAV1LoopRestoration {
    pub FrameRestorationType: [StdVideoAV1FrameRestorationType; STD_VIDEO_AV1_MAX_NUM_PLANES as usize],
    pub LoopRestorationSize: [u16; STD_VIDEO_AV1_MAX_NUM_PLANES as usize],
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct StdVideoAV1Quantization {
    pub flags: StdVideoAV1QuantizationFlags,
    pub base_q_idx: u8,
    pub DeltaQYDc: i8,
    pub DeltaQUDc: i8,
    pub DeltaQUAc: i8,
    pub DeltaQVDc: i8,
    pub DeltaQVAc: i8,
    pub qm_y: u8,
    pub qm_u: u8,
    pub qm_v: u8,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct StdVideoAV1QuantizationFlags {
    pub using_qmatrix: u32,
    pub diff_uv_delta: u32,
    pub reserved: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct StdVideoAV1Segmentation {
    pub FeatureEnabled: [u8; STD_VIDEO_AV1_MAX_SEGMENTS as usize],
    pub FeatureData: [[i16; STD_VIDEO_AV1_SEG_LVL_MAX as usize]; STD_VIDEO_AV1_MAX_SEGMENTS as usize],
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct StdVideoAV1SequenceHeader {
    pub flags: StdVideoAV1SequenceHeaderFlags,
    pub seq_profile: StdVideoAV1Profile,
    pub frame_width_bits_minus_1: u8,
    pub frame_height_bits_minus_1: u8,
    pub max_frame_width_minus_1: u16,
    pub max_frame_height_minus_1: u16,
    pub delta_frame_id_length_minus_2: u8,
    pub additional_frame_id_length_minus_1: u8,
    pub order_hint_bits_minus_1: u8,
    pub seq_force_integer_mv: u8,
    pub seq_force_screen_content_tools: u8,
    pub reserved1: [u8; 5 as usize],
    pub pColorConfig: *const StdVideoAV1ColorConfig,
    pub pTimingInfo: *const StdVideoAV1TimingInfo,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct StdVideoAV1SequenceHeaderFlags {
    pub still_picture: u32,
    pub reduced_still_picture_header: u32,
    pub use_128x128_superblock: u32,
    pub enable_filter_intra: u32,
    pub enable_intra_edge_filter: u32,
    pub enable_interintra_compound: u32,
    pub enable_masked_compound: u32,
    pub enable_warped_motion: u32,
    pub enable_dual_filter: u32,
    pub enable_order_hint: u32,
    pub enable_jnt_comp: u32,
    pub enable_ref_frame_mvs: u32,
    pub frame_id_numbers_present_flag: u32,
    pub enable_superres: u32,
    pub enable_cdef: u32,
    pub enable_restoration: u32,
    pub film_grain_params_present: u32,
    pub timing_info_present_flag: u32,
    pub initial_display_delay_present_flag: u32,
    pub reserved: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct StdVideoAV1TileInfo {
    pub flags: StdVideoAV1TileInfoFlags,
    pub TileCols: u8,
    pub TileRows: u8,
    pub context_update_tile_id: u16,
    pub tile_size_bytes_minus_1: u8,
    pub reserved1: [u8; 7 as usize],
    pub pMiColStarts: *const u16,
    pub pMiRowStarts: *const u16,
    pub pWidthInSbsMinus1: *const u16,
    pub pHeightInSbsMinus1: *const u16,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct StdVideoAV1TileInfoFlags {
    pub uniform_tile_spacing_flag: u32,
    pub reserved: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct StdVideoAV1TimingInfo {
    pub flags: StdVideoAV1TimingInfoFlags,
    pub num_units_in_display_tick: u32,
    pub time_scale: u32,
    pub num_ticks_per_picture_minus_1: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct StdVideoAV1TimingInfoFlags {
    pub equal_picture_interval: u32,
    pub reserved: u32,
}

pub type StdVideoAV1ChromaSamplePosition = i32;
pub type StdVideoAV1ColorPrimaries = i32;
pub type StdVideoAV1FrameRestorationType = i32;
pub type StdVideoAV1FrameType = i32;
pub type StdVideoAV1InterpolationFilter = i32;
pub type StdVideoAV1Level = i32;
pub type StdVideoAV1MatrixCoefficients = i32;
pub type StdVideoAV1Profile = i32;
pub type StdVideoAV1ReferenceName = i32;
pub type StdVideoAV1TransferCharacteristics = i32;
pub type StdVideoAV1TxMode = i32;
