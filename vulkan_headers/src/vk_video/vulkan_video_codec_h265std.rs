use crate::prelude::*;
use super::vulkan_video_codecs_common::*;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct StdVideoH265DecPicBufMgr {
    pub max_latency_increase_plus1: [u32; STD_VIDEO_H265_SUBLAYERS_LIST_SIZE as usize],
    pub max_dec_pic_buffering_minus1: [u8; STD_VIDEO_H265_SUBLAYERS_LIST_SIZE as usize],
    pub max_num_reorder_pics: [u8; STD_VIDEO_H265_SUBLAYERS_LIST_SIZE as usize],
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct StdVideoH265HrdFlags {
    pub bitfields: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct StdVideoH265HrdParameters {
    pub flags: StdVideoH265HrdFlags,
    pub tick_divisor_minus2: u8,
    pub du_cpb_removal_delay_increment_length_minus1: u8,
    pub dpb_output_delay_du_length_minus1: u8,
    pub bit_rate_scale: u8,
    pub cpb_size_scale: u8,
    pub cpb_size_du_scale: u8,
    pub initial_cpb_removal_delay_length_minus1: u8,
    pub au_cpb_removal_delay_length_minus1: u8,
    pub dpb_output_delay_length_minus1: u8,
    pub cpb_cnt_minus1: [u8; STD_VIDEO_H265_SUBLAYERS_LIST_SIZE as usize],
    pub elemental_duration_in_tc_minus1: [u16; STD_VIDEO_H265_SUBLAYERS_LIST_SIZE as usize],
    pub reserved: [u16; 3 as usize],
    pub pSubLayerHrdParametersNal: *const StdVideoH265SubLayerHrdParameters,
    pub pSubLayerHrdParametersVcl: *const StdVideoH265SubLayerHrdParameters,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct StdVideoH265LongTermRefPicsSps {
    pub used_by_curr_pic_lt_sps_flag: u32,
    pub lt_ref_pic_poc_lsb_sps: [u32; STD_VIDEO_H265_MAX_LONG_TERM_REF_PICS_SPS as usize],
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct StdVideoH265PictureParameterSet {
    pub flags: StdVideoH265PpsFlags,
    pub pps_pic_parameter_set_id: u8,
    pub pps_seq_parameter_set_id: u8,
    pub sps_video_parameter_set_id: u8,
    pub num_extra_slice_header_bits: u8,
    pub num_ref_idx_l0_default_active_minus1: u8,
    pub num_ref_idx_l1_default_active_minus1: u8,
    pub init_qp_minus26: i8,
    pub diff_cu_qp_delta_depth: u8,
    pub pps_cb_qp_offset: i8,
    pub pps_cr_qp_offset: i8,
    pub pps_beta_offset_div2: i8,
    pub pps_tc_offset_div2: i8,
    pub log2_parallel_merge_level_minus2: u8,
    pub log2_max_transform_skip_block_size_minus2: u8,
    pub diff_cu_chroma_qp_offset_depth: u8,
    pub chroma_qp_offset_list_len_minus1: u8,
    pub cb_qp_offset_list: [i8; STD_VIDEO_H265_CHROMA_QP_OFFSET_LIST_SIZE as usize],
    pub cr_qp_offset_list: [i8; STD_VIDEO_H265_CHROMA_QP_OFFSET_LIST_SIZE as usize],
    pub log2_sao_offset_scale_luma: u8,
    pub log2_sao_offset_scale_chroma: u8,
    pub pps_act_y_qp_offset_plus5: i8,
    pub pps_act_cb_qp_offset_plus5: i8,
    pub pps_act_cr_qp_offset_plus3: i8,
    pub pps_num_palette_predictor_initializers: u8,
    pub luma_bit_depth_entry_minus8: u8,
    pub chroma_bit_depth_entry_minus8: u8,
    pub num_tile_columns_minus1: u8,
    pub num_tile_rows_minus1: u8,
    pub reserved1: u8,
    pub reserved2: u8,
    pub column_width_minus1: [u16; STD_VIDEO_H265_CHROMA_QP_OFFSET_TILE_COLS_LIST_SIZE as usize],
    pub row_height_minus1: [u16; STD_VIDEO_H265_CHROMA_QP_OFFSET_TILE_ROWS_LIST_SIZE as usize],
    pub reserved3: u32,
    pub pScalingLists: *const StdVideoH265ScalingLists,
    pub pPredictorPaletteEntries: *const StdVideoH265PredictorPaletteEntries,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct StdVideoH265PpsFlags {
    pub bitfields: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct StdVideoH265PredictorPaletteEntries {
    pub PredictorPaletteEntries: [[u16; STD_VIDEO_H265_PREDICTOR_PALETTE_COMP_ENTRIES_LIST_SIZE as usize]; STD_VIDEO_H265_PREDICTOR_PALETTE_COMPONENTS_LIST_SIZE as usize],
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct StdVideoH265ProfileTierLevel {
    pub flags: StdVideoH265ProfileTierLevelFlags,
    pub general_profile_idc: StdVideoH265ProfileIdc,
    pub general_level_idc: StdVideoH265LevelIdc,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct StdVideoH265ProfileTierLevelFlags {
    pub bitfields: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct StdVideoH265ScalingLists {
    pub ScalingList4x4: [[u8; STD_VIDEO_H265_SCALING_LIST_4X4_NUM_ELEMENTS as usize]; STD_VIDEO_H265_SCALING_LIST_4X4_NUM_LISTS as usize],
    pub ScalingList8x8: [[u8; STD_VIDEO_H265_SCALING_LIST_8X8_NUM_ELEMENTS as usize]; STD_VIDEO_H265_SCALING_LIST_8X8_NUM_LISTS as usize],
    pub ScalingList16x16: [[u8; STD_VIDEO_H265_SCALING_LIST_16X16_NUM_ELEMENTS as usize]; STD_VIDEO_H265_SCALING_LIST_16X16_NUM_LISTS as usize],
    pub ScalingList32x32: [[u8; STD_VIDEO_H265_SCALING_LIST_32X32_NUM_ELEMENTS as usize]; STD_VIDEO_H265_SCALING_LIST_32X32_NUM_LISTS as usize],
    pub ScalingListDCCoef16x16: [u8; STD_VIDEO_H265_SCALING_LIST_16X16_NUM_LISTS as usize],
    pub ScalingListDCCoef32x32: [u8; STD_VIDEO_H265_SCALING_LIST_32X32_NUM_LISTS as usize],
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct StdVideoH265SequenceParameterSet {
    pub flags: StdVideoH265SpsFlags,
    pub chroma_format_idc: StdVideoH265ChromaFormatIdc,
    pub pic_width_in_luma_samples: u32,
    pub pic_height_in_luma_samples: u32,
    pub sps_video_parameter_set_id: u8,
    pub sps_max_sub_layers_minus1: u8,
    pub sps_seq_parameter_set_id: u8,
    pub bit_depth_luma_minus8: u8,
    pub bit_depth_chroma_minus8: u8,
    pub log2_max_pic_order_cnt_lsb_minus4: u8,
    pub log2_min_luma_coding_block_size_minus3: u8,
    pub log2_diff_max_min_luma_coding_block_size: u8,
    pub log2_min_luma_transform_block_size_minus2: u8,
    pub log2_diff_max_min_luma_transform_block_size: u8,
    pub max_transform_hierarchy_depth_inter: u8,
    pub max_transform_hierarchy_depth_intra: u8,
    pub num_short_term_ref_pic_sets: u8,
    pub num_long_term_ref_pics_sps: u8,
    pub pcm_sample_bit_depth_luma_minus1: u8,
    pub pcm_sample_bit_depth_chroma_minus1: u8,
    pub log2_min_pcm_luma_coding_block_size_minus3: u8,
    pub log2_diff_max_min_pcm_luma_coding_block_size: u8,
    pub reserved1: u8,
    pub reserved2: u8,
    pub palette_max_size: u8,
    pub delta_palette_max_predictor_size: u8,
    pub motion_vector_resolution_control_idc: u8,
    pub sps_num_palette_predictor_initializers_minus1: u8,
    pub conf_win_left_offset: u32,
    pub conf_win_right_offset: u32,
    pub conf_win_top_offset: u32,
    pub conf_win_bottom_offset: u32,
    pub pProfileTierLevel: *const StdVideoH265ProfileTierLevel,
    pub pDecPicBufMgr: *const StdVideoH265DecPicBufMgr,
    pub pScalingLists: *const StdVideoH265ScalingLists,
    pub pShortTermRefPicSet: *const StdVideoH265ShortTermRefPicSet,
    pub pLongTermRefPicsSps: *const StdVideoH265LongTermRefPicsSps,
    pub pSequenceParameterSetVui: *const StdVideoH265SequenceParameterSetVui,
    pub pPredictorPaletteEntries: *const StdVideoH265PredictorPaletteEntries,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct StdVideoH265SequenceParameterSetVui {
    pub flags: StdVideoH265SpsVuiFlags,
    pub aspect_ratio_idc: StdVideoH265AspectRatioIdc,
    pub sar_width: u16,
    pub sar_height: u16,
    pub video_format: u8,
    pub colour_primaries: u8,
    pub transfer_characteristics: u8,
    pub matrix_coeffs: u8,
    pub chroma_sample_loc_type_top_field: u8,
    pub chroma_sample_loc_type_bottom_field: u8,
    pub reserved1: u8,
    pub reserved2: u8,
    pub def_disp_win_left_offset: u16,
    pub def_disp_win_right_offset: u16,
    pub def_disp_win_top_offset: u16,
    pub def_disp_win_bottom_offset: u16,
    pub vui_num_units_in_tick: u32,
    pub vui_time_scale: u32,
    pub vui_num_ticks_poc_diff_one_minus1: u32,
    pub min_spatial_segmentation_idc: u16,
    pub reserved3: u16,
    pub max_bytes_per_pic_denom: u8,
    pub max_bits_per_min_cu_denom: u8,
    pub log2_max_mv_length_horizontal: u8,
    pub log2_max_mv_length_vertical: u8,
    pub pHrdParameters: *const StdVideoH265HrdParameters,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct StdVideoH265ShortTermRefPicSet {
    pub flags: StdVideoH265ShortTermRefPicSetFlags,
    pub delta_idx_minus1: u32,
    pub use_delta_flag: u16,
    pub abs_delta_rps_minus1: u16,
    pub used_by_curr_pic_flag: u16,
    pub used_by_curr_pic_s0_flag: u16,
    pub used_by_curr_pic_s1_flag: u16,
    pub reserved1: u16,
    pub reserved2: u8,
    pub reserved3: u8,
    pub num_negative_pics: u8,
    pub num_positive_pics: u8,
    pub delta_poc_s0_minus1: [u16; STD_VIDEO_H265_MAX_DPB_SIZE as usize],
    pub delta_poc_s1_minus1: [u16; STD_VIDEO_H265_MAX_DPB_SIZE as usize],
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct StdVideoH265ShortTermRefPicSetFlags {
    pub bitfields: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct StdVideoH265SpsFlags {
    pub bitfields: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct StdVideoH265SpsVuiFlags {
    pub bitfields: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct StdVideoH265SubLayerHrdParameters {
    pub bit_rate_value_minus1: [u32; STD_VIDEO_H265_CPB_CNT_LIST_SIZE as usize],
    pub cpb_size_value_minus1: [u32; STD_VIDEO_H265_CPB_CNT_LIST_SIZE as usize],
    pub cpb_size_du_value_minus1: [u32; STD_VIDEO_H265_CPB_CNT_LIST_SIZE as usize],
    pub bit_rate_du_value_minus1: [u32; STD_VIDEO_H265_CPB_CNT_LIST_SIZE as usize],
    pub cbr_flag: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct StdVideoH265VideoParameterSet {
    pub flags: StdVideoH265VpsFlags,
    pub vps_video_parameter_set_id: u8,
    pub vps_max_sub_layers_minus1: u8,
    pub reserved1: u8,
    pub reserved2: u8,
    pub vps_num_units_in_tick: u32,
    pub vps_time_scale: u32,
    pub vps_num_ticks_poc_diff_one_minus1: u32,
    pub reserved3: u32,
    pub pDecPicBufMgr: *const StdVideoH265DecPicBufMgr,
    pub pHrdParameters: *const StdVideoH265HrdParameters,
    pub pProfileTierLevel: *const StdVideoH265ProfileTierLevel,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct StdVideoH265VpsFlags {
    pub bitfields: u32,
}

pub const STD_VIDEO_H265_ASPECT_RATIO_IDC_10_11: StdVideoH265AspectRatioIdc = 3;
pub const STD_VIDEO_H265_ASPECT_RATIO_IDC_12_11: StdVideoH265AspectRatioIdc = 2;
pub const STD_VIDEO_H265_ASPECT_RATIO_IDC_15_11: StdVideoH265AspectRatioIdc = 11;
pub const STD_VIDEO_H265_ASPECT_RATIO_IDC_160_99: StdVideoH265AspectRatioIdc = 13;
pub const STD_VIDEO_H265_ASPECT_RATIO_IDC_16_11: StdVideoH265AspectRatioIdc = 4;
pub const STD_VIDEO_H265_ASPECT_RATIO_IDC_18_11: StdVideoH265AspectRatioIdc = 10;
pub const STD_VIDEO_H265_ASPECT_RATIO_IDC_20_11: StdVideoH265AspectRatioIdc = 7;
pub const STD_VIDEO_H265_ASPECT_RATIO_IDC_24_11: StdVideoH265AspectRatioIdc = 6;
pub const STD_VIDEO_H265_ASPECT_RATIO_IDC_2_1: StdVideoH265AspectRatioIdc = 16;
pub const STD_VIDEO_H265_ASPECT_RATIO_IDC_32_11: StdVideoH265AspectRatioIdc = 8;
pub const STD_VIDEO_H265_ASPECT_RATIO_IDC_3_2: StdVideoH265AspectRatioIdc = 15;
pub const STD_VIDEO_H265_ASPECT_RATIO_IDC_40_33: StdVideoH265AspectRatioIdc = 5;
pub const STD_VIDEO_H265_ASPECT_RATIO_IDC_4_3: StdVideoH265AspectRatioIdc = 14;
pub const STD_VIDEO_H265_ASPECT_RATIO_IDC_64_33: StdVideoH265AspectRatioIdc = 12;
pub const STD_VIDEO_H265_ASPECT_RATIO_IDC_80_33: StdVideoH265AspectRatioIdc = 9;
pub const STD_VIDEO_H265_ASPECT_RATIO_IDC_EXTENDED_SAR: StdVideoH265AspectRatioIdc = 255;
pub const STD_VIDEO_H265_ASPECT_RATIO_IDC_INVALID: StdVideoH265AspectRatioIdc = 0x7FFFFFFF;
pub const STD_VIDEO_H265_ASPECT_RATIO_IDC_SQUARE: StdVideoH265AspectRatioIdc = 1;
pub const STD_VIDEO_H265_ASPECT_RATIO_IDC_UNSPECIFIED: StdVideoH265AspectRatioIdc = 0;
pub const STD_VIDEO_H265_CHROMA_FORMAT_IDC_420: StdVideoH265ChromaFormatIdc = 1;
pub const STD_VIDEO_H265_CHROMA_FORMAT_IDC_422: StdVideoH265ChromaFormatIdc = 2;
pub const STD_VIDEO_H265_CHROMA_FORMAT_IDC_444: StdVideoH265ChromaFormatIdc = 3;
pub const STD_VIDEO_H265_CHROMA_FORMAT_IDC_INVALID: StdVideoH265ChromaFormatIdc = 0x7FFFFFFF;
pub const STD_VIDEO_H265_CHROMA_FORMAT_IDC_MONOCHROME: StdVideoH265ChromaFormatIdc = 0;
pub const STD_VIDEO_H265_CHROMA_QP_OFFSET_LIST_SIZE: u32 = 6;
pub const STD_VIDEO_H265_CHROMA_QP_OFFSET_TILE_COLS_LIST_SIZE: u32 = 19;
pub const STD_VIDEO_H265_CHROMA_QP_OFFSET_TILE_ROWS_LIST_SIZE: u32 = 21;
pub const STD_VIDEO_H265_CPB_CNT_LIST_SIZE: u32 = 32;
pub const STD_VIDEO_H265_LEVEL_IDC_1_0: StdVideoH265LevelIdc = 0;
pub const STD_VIDEO_H265_LEVEL_IDC_2_0: StdVideoH265LevelIdc = 1;
pub const STD_VIDEO_H265_LEVEL_IDC_2_1: StdVideoH265LevelIdc = 2;
pub const STD_VIDEO_H265_LEVEL_IDC_3_0: StdVideoH265LevelIdc = 3;
pub const STD_VIDEO_H265_LEVEL_IDC_3_1: StdVideoH265LevelIdc = 4;
pub const STD_VIDEO_H265_LEVEL_IDC_4_0: StdVideoH265LevelIdc = 5;
pub const STD_VIDEO_H265_LEVEL_IDC_4_1: StdVideoH265LevelIdc = 6;
pub const STD_VIDEO_H265_LEVEL_IDC_5_0: StdVideoH265LevelIdc = 7;
pub const STD_VIDEO_H265_LEVEL_IDC_5_1: StdVideoH265LevelIdc = 8;
pub const STD_VIDEO_H265_LEVEL_IDC_5_2: StdVideoH265LevelIdc = 9;
pub const STD_VIDEO_H265_LEVEL_IDC_6_0: StdVideoH265LevelIdc = 10;
pub const STD_VIDEO_H265_LEVEL_IDC_6_1: StdVideoH265LevelIdc = 11;
pub const STD_VIDEO_H265_LEVEL_IDC_6_2: StdVideoH265LevelIdc = 12;
pub const STD_VIDEO_H265_LEVEL_IDC_INVALID: StdVideoH265LevelIdc = 0x7FFFFFFF;
pub const STD_VIDEO_H265_MAX_CHROMA_PLANES: u32 = 2;
pub const STD_VIDEO_H265_MAX_DELTA_POC: u32 = 48;
pub const STD_VIDEO_H265_MAX_DPB_SIZE: u32 = 16;
pub const STD_VIDEO_H265_MAX_LONG_TERM_PICS: u32 = 16;
pub const STD_VIDEO_H265_MAX_LONG_TERM_REF_PICS_SPS: u32 = 32;
pub const STD_VIDEO_H265_MAX_NUM_LIST_REF: u32 = 15;
pub const STD_VIDEO_H265_MAX_SHORT_TERM_REF_PIC_SETS: u32 = 64;
pub const STD_VIDEO_H265_NO_REFERENCE_PICTURE: u8 = 0xFF;
pub const STD_VIDEO_H265_PICTURE_TYPE_B: StdVideoH265PictureType = 1;
pub const STD_VIDEO_H265_PICTURE_TYPE_I: StdVideoH265PictureType = 2;
pub const STD_VIDEO_H265_PICTURE_TYPE_IDR: StdVideoH265PictureType = 3;
pub const STD_VIDEO_H265_PICTURE_TYPE_INVALID: StdVideoH265PictureType = 0x7FFFFFFF;
pub const STD_VIDEO_H265_PICTURE_TYPE_P: StdVideoH265PictureType = 0;
pub const STD_VIDEO_H265_PREDICTOR_PALETTE_COMPONENTS_LIST_SIZE: u32 = 3;
pub const STD_VIDEO_H265_PREDICTOR_PALETTE_COMP_ENTRIES_LIST_SIZE: u32 = 128;
pub const STD_VIDEO_H265_PROFILE_IDC_FORMAT_RANGE_EXTENSIONS: StdVideoH265ProfileIdc = 4;
pub const STD_VIDEO_H265_PROFILE_IDC_INVALID: StdVideoH265ProfileIdc = 0x7FFFFFFF;
pub const STD_VIDEO_H265_PROFILE_IDC_MAIN: StdVideoH265ProfileIdc = 1;
pub const STD_VIDEO_H265_PROFILE_IDC_MAIN_10: StdVideoH265ProfileIdc = 2;
pub const STD_VIDEO_H265_PROFILE_IDC_MAIN_STILL_PICTURE: StdVideoH265ProfileIdc = 3;
pub const STD_VIDEO_H265_PROFILE_IDC_SCC_EXTENSIONS: StdVideoH265ProfileIdc = 9;
pub const STD_VIDEO_H265_SCALING_LIST_16X16_NUM_ELEMENTS: u32 = 64;
pub const STD_VIDEO_H265_SCALING_LIST_16X16_NUM_LISTS: u32 = 6;
pub const STD_VIDEO_H265_SCALING_LIST_32X32_NUM_ELEMENTS: u32 = 64;
pub const STD_VIDEO_H265_SCALING_LIST_32X32_NUM_LISTS: u32 = 2;
pub const STD_VIDEO_H265_SCALING_LIST_4X4_NUM_ELEMENTS: u32 = 16;
pub const STD_VIDEO_H265_SCALING_LIST_4X4_NUM_LISTS: u32 = 6;
pub const STD_VIDEO_H265_SCALING_LIST_8X8_NUM_ELEMENTS: u32 = 64;
pub const STD_VIDEO_H265_SCALING_LIST_8X8_NUM_LISTS: u32 = 6;
pub const STD_VIDEO_H265_SLICE_TYPE_B: StdVideoH265SliceType = 0;
pub const STD_VIDEO_H265_SLICE_TYPE_I: StdVideoH265SliceType = 2;
pub const STD_VIDEO_H265_SLICE_TYPE_INVALID: StdVideoH265SliceType = 0x7FFFFFFF;
pub const STD_VIDEO_H265_SLICE_TYPE_P: StdVideoH265SliceType = 1;
pub const STD_VIDEO_H265_SUBLAYERS_LIST_SIZE: u32 = 7;

pub type StdVideoH265AspectRatioIdc = i32;
pub type StdVideoH265ChromaFormatIdc = i32;
pub type StdVideoH265LevelIdc = i32;
pub type StdVideoH265PictureType = i32;
pub type StdVideoH265ProfileIdc = i32;
pub type StdVideoH265SliceType = i32;
