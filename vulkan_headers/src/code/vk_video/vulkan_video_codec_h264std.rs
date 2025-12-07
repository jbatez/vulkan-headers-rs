use crate::code::*;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct StdVideoH264HrdParameters {
    pub cpb_cnt_minus1: u8,
    pub bit_rate_scale: u8,
    pub cpb_size_scale: u8,
    pub reserved1: u8,
    pub bit_rate_value_minus1: [u32; STD_VIDEO_H264_CPB_CNT_LIST_SIZE as usize],
    pub cpb_size_value_minus1: [u32; STD_VIDEO_H264_CPB_CNT_LIST_SIZE as usize],
    pub cbr_flag: [u8; STD_VIDEO_H264_CPB_CNT_LIST_SIZE as usize],
    pub initial_cpb_removal_delay_length_minus1: u32,
    pub cpb_removal_delay_length_minus1: u32,
    pub dpb_output_delay_length_minus1: u32,
    pub time_offset_length: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct StdVideoH264PictureParameterSet {
    pub flags: StdVideoH264PpsFlags,
    pub seq_parameter_set_id: u8,
    pub pic_parameter_set_id: u8,
    pub num_ref_idx_l0_default_active_minus1: u8,
    pub num_ref_idx_l1_default_active_minus1: u8,
    pub weighted_bipred_idc: StdVideoH264WeightedBipredIdc,
    pub pic_init_qp_minus26: i8,
    pub pic_init_qs_minus26: i8,
    pub chroma_qp_index_offset: i8,
    pub second_chroma_qp_index_offset: i8,
    pub pScalingLists: *const StdVideoH264ScalingLists,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct StdVideoH264PpsFlags {
    pub bitfields: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct StdVideoH264ScalingLists {
    pub scaling_list_present_mask: u16,
    pub use_default_scaling_matrix_mask: u16,
    pub ScalingList4x4: [[u8; STD_VIDEO_H264_SCALING_LIST_4X4_NUM_ELEMENTS as usize]; STD_VIDEO_H264_SCALING_LIST_4X4_NUM_LISTS as usize],
    pub ScalingList8x8: [[u8; STD_VIDEO_H264_SCALING_LIST_8X8_NUM_ELEMENTS as usize]; STD_VIDEO_H264_SCALING_LIST_8X8_NUM_LISTS as usize],
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct StdVideoH264SequenceParameterSet {
    pub flags: StdVideoH264SpsFlags,
    pub profile_idc: StdVideoH264ProfileIdc,
    pub level_idc: StdVideoH264LevelIdc,
    pub chroma_format_idc: StdVideoH264ChromaFormatIdc,
    pub seq_parameter_set_id: u8,
    pub bit_depth_luma_minus8: u8,
    pub bit_depth_chroma_minus8: u8,
    pub log2_max_frame_num_minus4: u8,
    pub pic_order_cnt_type: StdVideoH264PocType,
    pub offset_for_non_ref_pic: i32,
    pub offset_for_top_to_bottom_field: i32,
    pub log2_max_pic_order_cnt_lsb_minus4: u8,
    pub num_ref_frames_in_pic_order_cnt_cycle: u8,
    pub max_num_ref_frames: u8,
    pub reserved1: u8,
    pub pic_width_in_mbs_minus1: u32,
    pub pic_height_in_map_units_minus1: u32,
    pub frame_crop_left_offset: u32,
    pub frame_crop_right_offset: u32,
    pub frame_crop_top_offset: u32,
    pub frame_crop_bottom_offset: u32,
    pub reserved2: u32,
    pub pOffsetForRefFrame: *const i32,
    pub pScalingLists: *const StdVideoH264ScalingLists,
    pub pSequenceParameterSetVui: *const StdVideoH264SequenceParameterSetVui,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct StdVideoH264SequenceParameterSetVui {
    pub flags: StdVideoH264SpsVuiFlags,
    pub aspect_ratio_idc: StdVideoH264AspectRatioIdc,
    pub sar_width: u16,
    pub sar_height: u16,
    pub video_format: u8,
    pub colour_primaries: u8,
    pub transfer_characteristics: u8,
    pub matrix_coefficients: u8,
    pub num_units_in_tick: u32,
    pub time_scale: u32,
    pub max_num_reorder_frames: u8,
    pub max_dec_frame_buffering: u8,
    pub chroma_sample_loc_type_top_field: u8,
    pub chroma_sample_loc_type_bottom_field: u8,
    pub reserved1: u32,
    pub pHrdParameters: *const StdVideoH264HrdParameters,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct StdVideoH264SpsFlags {
    pub bitfields: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct StdVideoH264SpsVuiFlags {
    pub bitfields: u32,
}

pub const STD_VIDEO_H264_ASPECT_RATIO_IDC_10_11: StdVideoH264AspectRatioIdc = 3;
pub const STD_VIDEO_H264_ASPECT_RATIO_IDC_12_11: StdVideoH264AspectRatioIdc = 2;
pub const STD_VIDEO_H264_ASPECT_RATIO_IDC_15_11: StdVideoH264AspectRatioIdc = 11;
pub const STD_VIDEO_H264_ASPECT_RATIO_IDC_160_99: StdVideoH264AspectRatioIdc = 13;
pub const STD_VIDEO_H264_ASPECT_RATIO_IDC_16_11: StdVideoH264AspectRatioIdc = 4;
pub const STD_VIDEO_H264_ASPECT_RATIO_IDC_18_11: StdVideoH264AspectRatioIdc = 10;
pub const STD_VIDEO_H264_ASPECT_RATIO_IDC_20_11: StdVideoH264AspectRatioIdc = 7;
pub const STD_VIDEO_H264_ASPECT_RATIO_IDC_24_11: StdVideoH264AspectRatioIdc = 6;
pub const STD_VIDEO_H264_ASPECT_RATIO_IDC_2_1: StdVideoH264AspectRatioIdc = 16;
pub const STD_VIDEO_H264_ASPECT_RATIO_IDC_32_11: StdVideoH264AspectRatioIdc = 8;
pub const STD_VIDEO_H264_ASPECT_RATIO_IDC_3_2: StdVideoH264AspectRatioIdc = 15;
pub const STD_VIDEO_H264_ASPECT_RATIO_IDC_40_33: StdVideoH264AspectRatioIdc = 5;
pub const STD_VIDEO_H264_ASPECT_RATIO_IDC_4_3: StdVideoH264AspectRatioIdc = 14;
pub const STD_VIDEO_H264_ASPECT_RATIO_IDC_64_33: StdVideoH264AspectRatioIdc = 12;
pub const STD_VIDEO_H264_ASPECT_RATIO_IDC_80_33: StdVideoH264AspectRatioIdc = 9;
pub const STD_VIDEO_H264_ASPECT_RATIO_IDC_EXTENDED_SAR: StdVideoH264AspectRatioIdc = 255;
pub const STD_VIDEO_H264_ASPECT_RATIO_IDC_INVALID: StdVideoH264AspectRatioIdc = 0x7FFFFFFF;
pub const STD_VIDEO_H264_ASPECT_RATIO_IDC_SQUARE: StdVideoH264AspectRatioIdc = 1;
pub const STD_VIDEO_H264_ASPECT_RATIO_IDC_UNSPECIFIED: StdVideoH264AspectRatioIdc = 0;
pub const STD_VIDEO_H264_CABAC_INIT_IDC_0: StdVideoH264CabacInitIdc = 0;
pub const STD_VIDEO_H264_CABAC_INIT_IDC_1: StdVideoH264CabacInitIdc = 1;
pub const STD_VIDEO_H264_CABAC_INIT_IDC_2: StdVideoH264CabacInitIdc = 2;
pub const STD_VIDEO_H264_CABAC_INIT_IDC_INVALID: StdVideoH264CabacInitIdc = 0x7FFFFFFF;
pub const STD_VIDEO_H264_CHROMA_FORMAT_IDC_420: StdVideoH264ChromaFormatIdc = 1;
pub const STD_VIDEO_H264_CHROMA_FORMAT_IDC_422: StdVideoH264ChromaFormatIdc = 2;
pub const STD_VIDEO_H264_CHROMA_FORMAT_IDC_444: StdVideoH264ChromaFormatIdc = 3;
pub const STD_VIDEO_H264_CHROMA_FORMAT_IDC_INVALID: StdVideoH264ChromaFormatIdc = 0x7FFFFFFF;
pub const STD_VIDEO_H264_CHROMA_FORMAT_IDC_MONOCHROME: StdVideoH264ChromaFormatIdc = 0;
pub const STD_VIDEO_H264_CPB_CNT_LIST_SIZE: u32 = 32;
pub const STD_VIDEO_H264_DISABLE_DEBLOCKING_FILTER_IDC_DISABLED: StdVideoH264DisableDeblockingFilterIdc = 0;
pub const STD_VIDEO_H264_DISABLE_DEBLOCKING_FILTER_IDC_ENABLED: StdVideoH264DisableDeblockingFilterIdc = 1;
pub const STD_VIDEO_H264_DISABLE_DEBLOCKING_FILTER_IDC_INVALID: StdVideoH264DisableDeblockingFilterIdc = 0x7FFFFFFF;
pub const STD_VIDEO_H264_DISABLE_DEBLOCKING_FILTER_IDC_PARTIAL: StdVideoH264DisableDeblockingFilterIdc = 2;
pub const STD_VIDEO_H264_LEVEL_IDC_1_0: StdVideoH264LevelIdc = 0;
pub const STD_VIDEO_H264_LEVEL_IDC_1_1: StdVideoH264LevelIdc = 1;
pub const STD_VIDEO_H264_LEVEL_IDC_1_2: StdVideoH264LevelIdc = 2;
pub const STD_VIDEO_H264_LEVEL_IDC_1_3: StdVideoH264LevelIdc = 3;
pub const STD_VIDEO_H264_LEVEL_IDC_2_0: StdVideoH264LevelIdc = 4;
pub const STD_VIDEO_H264_LEVEL_IDC_2_1: StdVideoH264LevelIdc = 5;
pub const STD_VIDEO_H264_LEVEL_IDC_2_2: StdVideoH264LevelIdc = 6;
pub const STD_VIDEO_H264_LEVEL_IDC_3_0: StdVideoH264LevelIdc = 7;
pub const STD_VIDEO_H264_LEVEL_IDC_3_1: StdVideoH264LevelIdc = 8;
pub const STD_VIDEO_H264_LEVEL_IDC_3_2: StdVideoH264LevelIdc = 9;
pub const STD_VIDEO_H264_LEVEL_IDC_4_0: StdVideoH264LevelIdc = 10;
pub const STD_VIDEO_H264_LEVEL_IDC_4_1: StdVideoH264LevelIdc = 11;
pub const STD_VIDEO_H264_LEVEL_IDC_4_2: StdVideoH264LevelIdc = 12;
pub const STD_VIDEO_H264_LEVEL_IDC_5_0: StdVideoH264LevelIdc = 13;
pub const STD_VIDEO_H264_LEVEL_IDC_5_1: StdVideoH264LevelIdc = 14;
pub const STD_VIDEO_H264_LEVEL_IDC_5_2: StdVideoH264LevelIdc = 15;
pub const STD_VIDEO_H264_LEVEL_IDC_6_0: StdVideoH264LevelIdc = 16;
pub const STD_VIDEO_H264_LEVEL_IDC_6_1: StdVideoH264LevelIdc = 17;
pub const STD_VIDEO_H264_LEVEL_IDC_6_2: StdVideoH264LevelIdc = 18;
pub const STD_VIDEO_H264_LEVEL_IDC_INVALID: StdVideoH264LevelIdc = 0x7FFFFFFF;
pub const STD_VIDEO_H264_MAX_CHROMA_PLANES: u32 = 2;
pub const STD_VIDEO_H264_MAX_NUM_LIST_REF: u32 = 32;
pub const STD_VIDEO_H264_MEM_MGMT_CONTROL_OP_END: StdVideoH264MemMgmtControlOp = 0;
pub const STD_VIDEO_H264_MEM_MGMT_CONTROL_OP_INVALID: StdVideoH264MemMgmtControlOp = 0x7FFFFFFF;
pub const STD_VIDEO_H264_MEM_MGMT_CONTROL_OP_MARK_CURRENT_AS_LONG_TERM: StdVideoH264MemMgmtControlOp = 6;
pub const STD_VIDEO_H264_MEM_MGMT_CONTROL_OP_MARK_LONG_TERM: StdVideoH264MemMgmtControlOp = 3;
pub const STD_VIDEO_H264_MEM_MGMT_CONTROL_OP_SET_MAX_LONG_TERM_INDEX: StdVideoH264MemMgmtControlOp = 4;
pub const STD_VIDEO_H264_MEM_MGMT_CONTROL_OP_UNMARK_ALL: StdVideoH264MemMgmtControlOp = 5;
pub const STD_VIDEO_H264_MEM_MGMT_CONTROL_OP_UNMARK_LONG_TERM: StdVideoH264MemMgmtControlOp = 2;
pub const STD_VIDEO_H264_MEM_MGMT_CONTROL_OP_UNMARK_SHORT_TERM: StdVideoH264MemMgmtControlOp = 1;
pub const STD_VIDEO_H264_MODIFICATION_OF_PIC_NUMS_IDC_END: StdVideoH264ModificationOfPicNumsIdc = 3;
pub const STD_VIDEO_H264_MODIFICATION_OF_PIC_NUMS_IDC_INVALID: StdVideoH264ModificationOfPicNumsIdc = 0x7FFFFFFF;
pub const STD_VIDEO_H264_MODIFICATION_OF_PIC_NUMS_IDC_LONG_TERM: StdVideoH264ModificationOfPicNumsIdc = 2;
pub const STD_VIDEO_H264_MODIFICATION_OF_PIC_NUMS_IDC_SHORT_TERM_ADD: StdVideoH264ModificationOfPicNumsIdc = 1;
pub const STD_VIDEO_H264_MODIFICATION_OF_PIC_NUMS_IDC_SHORT_TERM_SUBTRACT: StdVideoH264ModificationOfPicNumsIdc = 0;
pub const STD_VIDEO_H264_NON_VCL_NALU_TYPE_AUD: StdVideoH264NonVclNaluType = 2;
pub const STD_VIDEO_H264_NON_VCL_NALU_TYPE_END_OF_SEQUENCE: StdVideoH264NonVclNaluType = 4;
pub const STD_VIDEO_H264_NON_VCL_NALU_TYPE_END_OF_STREAM: StdVideoH264NonVclNaluType = 5;
pub const STD_VIDEO_H264_NON_VCL_NALU_TYPE_INVALID: StdVideoH264NonVclNaluType = 0x7FFFFFFF;
pub const STD_VIDEO_H264_NON_VCL_NALU_TYPE_PPS: StdVideoH264NonVclNaluType = 1;
pub const STD_VIDEO_H264_NON_VCL_NALU_TYPE_PRECODED: StdVideoH264NonVclNaluType = 6;
pub const STD_VIDEO_H264_NON_VCL_NALU_TYPE_PREFIX: StdVideoH264NonVclNaluType = 3;
pub const STD_VIDEO_H264_NON_VCL_NALU_TYPE_SPS: StdVideoH264NonVclNaluType = 0;
pub const STD_VIDEO_H264_NO_REFERENCE_PICTURE: u8 = 0xFF;
pub const STD_VIDEO_H264_PICTURE_TYPE_B: StdVideoH264PictureType = 1;
pub const STD_VIDEO_H264_PICTURE_TYPE_I: StdVideoH264PictureType = 2;
pub const STD_VIDEO_H264_PICTURE_TYPE_IDR: StdVideoH264PictureType = 5;
pub const STD_VIDEO_H264_PICTURE_TYPE_INVALID: StdVideoH264PictureType = 0x7FFFFFFF;
pub const STD_VIDEO_H264_PICTURE_TYPE_P: StdVideoH264PictureType = 0;
pub const STD_VIDEO_H264_POC_TYPE_0: StdVideoH264PocType = 0;
pub const STD_VIDEO_H264_POC_TYPE_1: StdVideoH264PocType = 1;
pub const STD_VIDEO_H264_POC_TYPE_2: StdVideoH264PocType = 2;
pub const STD_VIDEO_H264_POC_TYPE_INVALID: StdVideoH264PocType = 0x7FFFFFFF;
pub const STD_VIDEO_H264_PROFILE_IDC_BASELINE: StdVideoH264ProfileIdc = 66;
pub const STD_VIDEO_H264_PROFILE_IDC_HIGH: StdVideoH264ProfileIdc = 100;
pub const STD_VIDEO_H264_PROFILE_IDC_HIGH_444_PREDICTIVE: StdVideoH264ProfileIdc = 244;
pub const STD_VIDEO_H264_PROFILE_IDC_INVALID: StdVideoH264ProfileIdc = 0x7FFFFFFF;
pub const STD_VIDEO_H264_PROFILE_IDC_MAIN: StdVideoH264ProfileIdc = 77;
pub const STD_VIDEO_H264_SCALING_LIST_4X4_NUM_ELEMENTS: u32 = 16;
pub const STD_VIDEO_H264_SCALING_LIST_4X4_NUM_LISTS: u32 = 6;
pub const STD_VIDEO_H264_SCALING_LIST_8X8_NUM_ELEMENTS: u32 = 64;
pub const STD_VIDEO_H264_SCALING_LIST_8X8_NUM_LISTS: u32 = 6;
pub const STD_VIDEO_H264_SLICE_TYPE_B: StdVideoH264SliceType = 1;
pub const STD_VIDEO_H264_SLICE_TYPE_I: StdVideoH264SliceType = 2;
pub const STD_VIDEO_H264_SLICE_TYPE_INVALID: StdVideoH264SliceType = 0x7FFFFFFF;
pub const STD_VIDEO_H264_SLICE_TYPE_P: StdVideoH264SliceType = 0;
pub const STD_VIDEO_H264_WEIGHTED_BIPRED_IDC_DEFAULT: StdVideoH264WeightedBipredIdc = 0;
pub const STD_VIDEO_H264_WEIGHTED_BIPRED_IDC_EXPLICIT: StdVideoH264WeightedBipredIdc = 1;
pub const STD_VIDEO_H264_WEIGHTED_BIPRED_IDC_IMPLICIT: StdVideoH264WeightedBipredIdc = 2;
pub const STD_VIDEO_H264_WEIGHTED_BIPRED_IDC_INVALID: StdVideoH264WeightedBipredIdc = 0x7FFFFFFF;

pub type StdVideoH264AspectRatioIdc = i32;
pub type StdVideoH264CabacInitIdc = i32;
pub type StdVideoH264ChromaFormatIdc = i32;
pub type StdVideoH264DisableDeblockingFilterIdc = i32;
pub type StdVideoH264LevelIdc = i32;
pub type StdVideoH264MemMgmtControlOp = i32;
pub type StdVideoH264ModificationOfPicNumsIdc = i32;
pub type StdVideoH264NonVclNaluType = i32;
pub type StdVideoH264PictureType = i32;
pub type StdVideoH264PocType = i32;
pub type StdVideoH264ProfileIdc = i32;
pub type StdVideoH264SliceType = i32;
pub type StdVideoH264WeightedBipredIdc = i32;
