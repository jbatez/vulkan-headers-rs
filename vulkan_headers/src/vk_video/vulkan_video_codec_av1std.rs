use crate::prelude::*;

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
    pub bitfields: u32,
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
    pub bitfields: u32,
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
    pub bitfields: u32,
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
    pub bitfields: u32,
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
    pub bitfields: u32,
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
    pub bitfields: u32,
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
    pub bitfields: u32,
}

pub const STD_VIDEO_AV1_CHROMA_SAMPLE_POSITION_COLOCATED: StdVideoAV1ChromaSamplePosition = 2;
pub const STD_VIDEO_AV1_CHROMA_SAMPLE_POSITION_INVALID: StdVideoAV1ChromaSamplePosition = 0x7FFFFFFF;
pub const STD_VIDEO_AV1_CHROMA_SAMPLE_POSITION_RESERVED: StdVideoAV1ChromaSamplePosition = 3;
pub const STD_VIDEO_AV1_CHROMA_SAMPLE_POSITION_UNKNOWN: StdVideoAV1ChromaSamplePosition = 0;
pub const STD_VIDEO_AV1_CHROMA_SAMPLE_POSITION_VERTICAL: StdVideoAV1ChromaSamplePosition = 1;
pub const STD_VIDEO_AV1_COLOR_PRIMARIES_BT_2020: StdVideoAV1ColorPrimaries = 9;
pub const STD_VIDEO_AV1_COLOR_PRIMARIES_BT_470_B_G: StdVideoAV1ColorPrimaries = 5;
pub const STD_VIDEO_AV1_COLOR_PRIMARIES_BT_470_M: StdVideoAV1ColorPrimaries = 4;
pub const STD_VIDEO_AV1_COLOR_PRIMARIES_BT_601: StdVideoAV1ColorPrimaries = 6;
pub const STD_VIDEO_AV1_COLOR_PRIMARIES_BT_709: StdVideoAV1ColorPrimaries = 1;
pub const STD_VIDEO_AV1_COLOR_PRIMARIES_BT_UNSPECIFIED: StdVideoAV1ColorPrimaries = STD_VIDEO_AV1_COLOR_PRIMARIES_UNSPECIFIED;
pub const STD_VIDEO_AV1_COLOR_PRIMARIES_EBU_3213: StdVideoAV1ColorPrimaries = 22;
pub const STD_VIDEO_AV1_COLOR_PRIMARIES_GENERIC_FILM: StdVideoAV1ColorPrimaries = 8;
pub const STD_VIDEO_AV1_COLOR_PRIMARIES_INVALID: StdVideoAV1ColorPrimaries = 0x7FFFFFFF;
pub const STD_VIDEO_AV1_COLOR_PRIMARIES_SMPTE_240: StdVideoAV1ColorPrimaries = 7;
pub const STD_VIDEO_AV1_COLOR_PRIMARIES_SMPTE_431: StdVideoAV1ColorPrimaries = 11;
pub const STD_VIDEO_AV1_COLOR_PRIMARIES_SMPTE_432: StdVideoAV1ColorPrimaries = 12;
pub const STD_VIDEO_AV1_COLOR_PRIMARIES_UNSPECIFIED: StdVideoAV1ColorPrimaries = 2;
pub const STD_VIDEO_AV1_COLOR_PRIMARIES_XYZ: StdVideoAV1ColorPrimaries = 10;
pub const STD_VIDEO_AV1_FRAME_RESTORATION_TYPE_INVALID: StdVideoAV1FrameRestorationType = 0x7FFFFFFF;
pub const STD_VIDEO_AV1_FRAME_RESTORATION_TYPE_NONE: StdVideoAV1FrameRestorationType = 0;
pub const STD_VIDEO_AV1_FRAME_RESTORATION_TYPE_SGRPROJ: StdVideoAV1FrameRestorationType = 2;
pub const STD_VIDEO_AV1_FRAME_RESTORATION_TYPE_SWITCHABLE: StdVideoAV1FrameRestorationType = 3;
pub const STD_VIDEO_AV1_FRAME_RESTORATION_TYPE_WIENER: StdVideoAV1FrameRestorationType = 1;
pub const STD_VIDEO_AV1_FRAME_TYPE_INTER: StdVideoAV1FrameType = 1;
pub const STD_VIDEO_AV1_FRAME_TYPE_INTRA_ONLY: StdVideoAV1FrameType = 2;
pub const STD_VIDEO_AV1_FRAME_TYPE_INVALID: StdVideoAV1FrameType = 0x7FFFFFFF;
pub const STD_VIDEO_AV1_FRAME_TYPE_KEY: StdVideoAV1FrameType = 0;
pub const STD_VIDEO_AV1_FRAME_TYPE_SWITCH: StdVideoAV1FrameType = 3;
pub const STD_VIDEO_AV1_GLOBAL_MOTION_PARAMS: u32 = 6;
pub const STD_VIDEO_AV1_INTERPOLATION_FILTER_BILINEAR: StdVideoAV1InterpolationFilter = 3;
pub const STD_VIDEO_AV1_INTERPOLATION_FILTER_EIGHTTAP: StdVideoAV1InterpolationFilter = 0;
pub const STD_VIDEO_AV1_INTERPOLATION_FILTER_EIGHTTAP_SHARP: StdVideoAV1InterpolationFilter = 2;
pub const STD_VIDEO_AV1_INTERPOLATION_FILTER_EIGHTTAP_SMOOTH: StdVideoAV1InterpolationFilter = 1;
pub const STD_VIDEO_AV1_INTERPOLATION_FILTER_INVALID: StdVideoAV1InterpolationFilter = 0x7FFFFFFF;
pub const STD_VIDEO_AV1_INTERPOLATION_FILTER_SWITCHABLE: StdVideoAV1InterpolationFilter = 4;
pub const STD_VIDEO_AV1_LEVEL_2_0: StdVideoAV1Level = 0;
pub const STD_VIDEO_AV1_LEVEL_2_1: StdVideoAV1Level = 1;
pub const STD_VIDEO_AV1_LEVEL_2_2: StdVideoAV1Level = 2;
pub const STD_VIDEO_AV1_LEVEL_2_3: StdVideoAV1Level = 3;
pub const STD_VIDEO_AV1_LEVEL_3_0: StdVideoAV1Level = 4;
pub const STD_VIDEO_AV1_LEVEL_3_1: StdVideoAV1Level = 5;
pub const STD_VIDEO_AV1_LEVEL_3_2: StdVideoAV1Level = 6;
pub const STD_VIDEO_AV1_LEVEL_3_3: StdVideoAV1Level = 7;
pub const STD_VIDEO_AV1_LEVEL_4_0: StdVideoAV1Level = 8;
pub const STD_VIDEO_AV1_LEVEL_4_1: StdVideoAV1Level = 9;
pub const STD_VIDEO_AV1_LEVEL_4_2: StdVideoAV1Level = 10;
pub const STD_VIDEO_AV1_LEVEL_4_3: StdVideoAV1Level = 11;
pub const STD_VIDEO_AV1_LEVEL_5_0: StdVideoAV1Level = 12;
pub const STD_VIDEO_AV1_LEVEL_5_1: StdVideoAV1Level = 13;
pub const STD_VIDEO_AV1_LEVEL_5_2: StdVideoAV1Level = 14;
pub const STD_VIDEO_AV1_LEVEL_5_3: StdVideoAV1Level = 15;
pub const STD_VIDEO_AV1_LEVEL_6_0: StdVideoAV1Level = 16;
pub const STD_VIDEO_AV1_LEVEL_6_1: StdVideoAV1Level = 17;
pub const STD_VIDEO_AV1_LEVEL_6_2: StdVideoAV1Level = 18;
pub const STD_VIDEO_AV1_LEVEL_6_3: StdVideoAV1Level = 19;
pub const STD_VIDEO_AV1_LEVEL_7_0: StdVideoAV1Level = 20;
pub const STD_VIDEO_AV1_LEVEL_7_1: StdVideoAV1Level = 21;
pub const STD_VIDEO_AV1_LEVEL_7_2: StdVideoAV1Level = 22;
pub const STD_VIDEO_AV1_LEVEL_7_3: StdVideoAV1Level = 23;
pub const STD_VIDEO_AV1_LEVEL_INVALID: StdVideoAV1Level = 0x7FFFFFFF;
pub const STD_VIDEO_AV1_LOOP_FILTER_ADJUSTMENTS: u32 = 2;
pub const STD_VIDEO_AV1_MATRIX_COEFFICIENTS_BT_2020_CL: StdVideoAV1MatrixCoefficients = 10;
pub const STD_VIDEO_AV1_MATRIX_COEFFICIENTS_BT_2020_NCL: StdVideoAV1MatrixCoefficients = 9;
pub const STD_VIDEO_AV1_MATRIX_COEFFICIENTS_BT_470_B_G: StdVideoAV1MatrixCoefficients = 5;
pub const STD_VIDEO_AV1_MATRIX_COEFFICIENTS_BT_601: StdVideoAV1MatrixCoefficients = 6;
pub const STD_VIDEO_AV1_MATRIX_COEFFICIENTS_BT_709: StdVideoAV1MatrixCoefficients = 1;
pub const STD_VIDEO_AV1_MATRIX_COEFFICIENTS_CHROMAT_CL: StdVideoAV1MatrixCoefficients = 13;
pub const STD_VIDEO_AV1_MATRIX_COEFFICIENTS_CHROMAT_NCL: StdVideoAV1MatrixCoefficients = 12;
pub const STD_VIDEO_AV1_MATRIX_COEFFICIENTS_FCC: StdVideoAV1MatrixCoefficients = 4;
pub const STD_VIDEO_AV1_MATRIX_COEFFICIENTS_ICTCP: StdVideoAV1MatrixCoefficients = 14;
pub const STD_VIDEO_AV1_MATRIX_COEFFICIENTS_IDENTITY: StdVideoAV1MatrixCoefficients = 0;
pub const STD_VIDEO_AV1_MATRIX_COEFFICIENTS_INVALID: StdVideoAV1MatrixCoefficients = 0x7FFFFFFF;
pub const STD_VIDEO_AV1_MATRIX_COEFFICIENTS_RESERVED_3: StdVideoAV1MatrixCoefficients = 3;
pub const STD_VIDEO_AV1_MATRIX_COEFFICIENTS_SMPTE_2085: StdVideoAV1MatrixCoefficients = 11;
pub const STD_VIDEO_AV1_MATRIX_COEFFICIENTS_SMPTE_240: StdVideoAV1MatrixCoefficients = 7;
pub const STD_VIDEO_AV1_MATRIX_COEFFICIENTS_SMPTE_YCGCO: StdVideoAV1MatrixCoefficients = 8;
pub const STD_VIDEO_AV1_MATRIX_COEFFICIENTS_UNSPECIFIED: StdVideoAV1MatrixCoefficients = 2;
pub const STD_VIDEO_AV1_MAX_CDEF_FILTER_STRENGTHS: u32 = 8;
pub const STD_VIDEO_AV1_MAX_LOOP_FILTER_STRENGTHS: u32 = 4;
pub const STD_VIDEO_AV1_MAX_NUM_CB_POINTS: u32 = 10;
pub const STD_VIDEO_AV1_MAX_NUM_CR_POINTS: u32 = 10;
pub const STD_VIDEO_AV1_MAX_NUM_PLANES: u32 = 3;
pub const STD_VIDEO_AV1_MAX_NUM_POS_CHROMA: u32 = 25;
pub const STD_VIDEO_AV1_MAX_NUM_POS_LUMA: u32 = 24;
pub const STD_VIDEO_AV1_MAX_NUM_Y_POINTS: u32 = 14;
pub const STD_VIDEO_AV1_MAX_SEGMENTS: u32 = 8;
pub const STD_VIDEO_AV1_MAX_TILE_COLS: u32 = 64;
pub const STD_VIDEO_AV1_MAX_TILE_ROWS: u32 = 64;
pub const STD_VIDEO_AV1_NUM_REF_FRAMES: u32 = 8;
pub const STD_VIDEO_AV1_PRIMARY_REF_NONE: u8 = 7;
pub const STD_VIDEO_AV1_PROFILE_HIGH: StdVideoAV1Profile = 1;
pub const STD_VIDEO_AV1_PROFILE_INVALID: StdVideoAV1Profile = 0x7FFFFFFF;
pub const STD_VIDEO_AV1_PROFILE_MAIN: StdVideoAV1Profile = 0;
pub const STD_VIDEO_AV1_PROFILE_PROFESSIONAL: StdVideoAV1Profile = 2;
pub const STD_VIDEO_AV1_REFERENCE_NAME_ALTREF2_FRAME: StdVideoAV1ReferenceName = 6;
pub const STD_VIDEO_AV1_REFERENCE_NAME_ALTREF_FRAME: StdVideoAV1ReferenceName = 7;
pub const STD_VIDEO_AV1_REFERENCE_NAME_BWDREF_FRAME: StdVideoAV1ReferenceName = 5;
pub const STD_VIDEO_AV1_REFERENCE_NAME_GOLDEN_FRAME: StdVideoAV1ReferenceName = 4;
pub const STD_VIDEO_AV1_REFERENCE_NAME_INTRA_FRAME: StdVideoAV1ReferenceName = 0;
pub const STD_VIDEO_AV1_REFERENCE_NAME_INVALID: StdVideoAV1ReferenceName = 0x7FFFFFFF;
pub const STD_VIDEO_AV1_REFERENCE_NAME_LAST2_FRAME: StdVideoAV1ReferenceName = 2;
pub const STD_VIDEO_AV1_REFERENCE_NAME_LAST3_FRAME: StdVideoAV1ReferenceName = 3;
pub const STD_VIDEO_AV1_REFERENCE_NAME_LAST_FRAME: StdVideoAV1ReferenceName = 1;
pub const STD_VIDEO_AV1_REFS_PER_FRAME: u32 = 7;
pub const STD_VIDEO_AV1_SEG_LVL_MAX: u32 = 8;
pub const STD_VIDEO_AV1_SELECT_INTEGER_MV: u8 = 2;
pub const STD_VIDEO_AV1_SELECT_SCREEN_CONTENT_TOOLS: u32 = 2;
pub const STD_VIDEO_AV1_SKIP_MODE_FRAMES: u32 = 2;
pub const STD_VIDEO_AV1_TOTAL_REFS_PER_FRAME: u32 = 8;
pub const STD_VIDEO_AV1_TRANSFER_CHARACTERISTICS_BT_1361: StdVideoAV1TransferCharacteristics = 12;
pub const STD_VIDEO_AV1_TRANSFER_CHARACTERISTICS_BT_2020_10_BIT: StdVideoAV1TransferCharacteristics = 14;
pub const STD_VIDEO_AV1_TRANSFER_CHARACTERISTICS_BT_2020_12_BIT: StdVideoAV1TransferCharacteristics = 15;
pub const STD_VIDEO_AV1_TRANSFER_CHARACTERISTICS_BT_470_B_G: StdVideoAV1TransferCharacteristics = 5;
pub const STD_VIDEO_AV1_TRANSFER_CHARACTERISTICS_BT_470_M: StdVideoAV1TransferCharacteristics = 4;
pub const STD_VIDEO_AV1_TRANSFER_CHARACTERISTICS_BT_601: StdVideoAV1TransferCharacteristics = 6;
pub const STD_VIDEO_AV1_TRANSFER_CHARACTERISTICS_BT_709: StdVideoAV1TransferCharacteristics = 1;
pub const STD_VIDEO_AV1_TRANSFER_CHARACTERISTICS_HLG: StdVideoAV1TransferCharacteristics = 18;
pub const STD_VIDEO_AV1_TRANSFER_CHARACTERISTICS_IEC_61966: StdVideoAV1TransferCharacteristics = 11;
pub const STD_VIDEO_AV1_TRANSFER_CHARACTERISTICS_INVALID: StdVideoAV1TransferCharacteristics = 0x7FFFFFFF;
pub const STD_VIDEO_AV1_TRANSFER_CHARACTERISTICS_LINEAR: StdVideoAV1TransferCharacteristics = 8;
pub const STD_VIDEO_AV1_TRANSFER_CHARACTERISTICS_LOG_100: StdVideoAV1TransferCharacteristics = 9;
pub const STD_VIDEO_AV1_TRANSFER_CHARACTERISTICS_LOG_100_SQRT10: StdVideoAV1TransferCharacteristics = 10;
pub const STD_VIDEO_AV1_TRANSFER_CHARACTERISTICS_RESERVED_0: StdVideoAV1TransferCharacteristics = 0;
pub const STD_VIDEO_AV1_TRANSFER_CHARACTERISTICS_RESERVED_3: StdVideoAV1TransferCharacteristics = 3;
pub const STD_VIDEO_AV1_TRANSFER_CHARACTERISTICS_SMPTE_2084: StdVideoAV1TransferCharacteristics = 16;
pub const STD_VIDEO_AV1_TRANSFER_CHARACTERISTICS_SMPTE_240: StdVideoAV1TransferCharacteristics = 7;
pub const STD_VIDEO_AV1_TRANSFER_CHARACTERISTICS_SMPTE_428: StdVideoAV1TransferCharacteristics = 17;
pub const STD_VIDEO_AV1_TRANSFER_CHARACTERISTICS_SRGB: StdVideoAV1TransferCharacteristics = 13;
pub const STD_VIDEO_AV1_TRANSFER_CHARACTERISTICS_UNSPECIFIED: StdVideoAV1TransferCharacteristics = 2;
pub const STD_VIDEO_AV1_TX_MODE_INVALID: StdVideoAV1TxMode = 0x7FFFFFFF;
pub const STD_VIDEO_AV1_TX_MODE_LARGEST: StdVideoAV1TxMode = 1;
pub const STD_VIDEO_AV1_TX_MODE_ONLY_4X4: StdVideoAV1TxMode = 0;
pub const STD_VIDEO_AV1_TX_MODE_SELECT: StdVideoAV1TxMode = 2;

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
