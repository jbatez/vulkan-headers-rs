use crate::code::*;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct StdVideoEncodeH264PictureInfo {
    pub flags: StdVideoEncodeH264PictureInfoFlags,
    pub seq_parameter_set_id: u8,
    pub pic_parameter_set_id: u8,
    pub idr_pic_id: u16,
    pub primary_pic_type: StdVideoH264PictureType,
    pub frame_num: u32,
    pub PicOrderCnt: i32,
    pub temporal_id: u8,
    pub reserved1: [u8; 3 as usize],
    pub pRefLists: *const StdVideoEncodeH264ReferenceListsInfo,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct StdVideoEncodeH264PictureInfoFlags {
    pub IdrPicFlag: u32,
    pub is_reference: u32,
    pub no_output_of_prior_pics_flag: u32,
    pub long_term_reference_flag: u32,
    pub adaptive_ref_pic_marking_mode_flag: u32,
    pub reserved: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct StdVideoEncodeH264RefListModEntry {
    pub modification_of_pic_nums_idc: StdVideoH264ModificationOfPicNumsIdc,
    pub abs_diff_pic_num_minus1: u16,
    pub long_term_pic_num: u16,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct StdVideoEncodeH264RefPicMarkingEntry {
    pub memory_management_control_operation: StdVideoH264MemMgmtControlOp,
    pub difference_of_pic_nums_minus1: u16,
    pub long_term_pic_num: u16,
    pub long_term_frame_idx: u16,
    pub max_long_term_frame_idx_plus1: u16,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct StdVideoEncodeH264ReferenceInfo {
    pub flags: StdVideoEncodeH264ReferenceInfoFlags,
    pub primary_pic_type: StdVideoH264PictureType,
    pub FrameNum: u32,
    pub PicOrderCnt: i32,
    pub long_term_pic_num: u16,
    pub long_term_frame_idx: u16,
    pub temporal_id: u8,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct StdVideoEncodeH264ReferenceInfoFlags {
    pub used_for_long_term_reference: u32,
    pub reserved: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct StdVideoEncodeH264ReferenceListsInfo {
    pub flags: StdVideoEncodeH264ReferenceListsInfoFlags,
    pub num_ref_idx_l0_active_minus1: u8,
    pub num_ref_idx_l1_active_minus1: u8,
    pub RefPicList0: [u8; STD_VIDEO_H264_MAX_NUM_LIST_REF as usize],
    pub RefPicList1: [u8; STD_VIDEO_H264_MAX_NUM_LIST_REF as usize],
    pub refList0ModOpCount: u8,
    pub refList1ModOpCount: u8,
    pub refPicMarkingOpCount: u8,
    pub reserved1: [u8; 7 as usize],
    pub pRefList0ModOperations: *const StdVideoEncodeH264RefListModEntry,
    pub pRefList1ModOperations: *const StdVideoEncodeH264RefListModEntry,
    pub pRefPicMarkingOperations: *const StdVideoEncodeH264RefPicMarkingEntry,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct StdVideoEncodeH264ReferenceListsInfoFlags {
    pub ref_pic_list_modification_flag_l0: u32,
    pub ref_pic_list_modification_flag_l1: u32,
    pub reserved: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct StdVideoEncodeH264SliceHeader {
    pub flags: StdVideoEncodeH264SliceHeaderFlags,
    pub first_mb_in_slice: u32,
    pub slice_type: StdVideoH264SliceType,
    pub slice_alpha_c0_offset_div2: i8,
    pub slice_beta_offset_div2: i8,
    pub slice_qp_delta: i8,
    pub reserved1: u8,
    pub cabac_init_idc: StdVideoH264CabacInitIdc,
    pub disable_deblocking_filter_idc: StdVideoH264DisableDeblockingFilterIdc,
    pub pWeightTable: *const StdVideoEncodeH264WeightTable,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct StdVideoEncodeH264SliceHeaderFlags {
    pub direct_spatial_mv_pred_flag: u32,
    pub num_ref_idx_active_override_flag: u32,
    pub reserved: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct StdVideoEncodeH264WeightTable {
    pub flags: StdVideoEncodeH264WeightTableFlags,
    pub luma_log2_weight_denom: u8,
    pub chroma_log2_weight_denom: u8,
    pub luma_weight_l0: [i8; STD_VIDEO_H264_MAX_NUM_LIST_REF as usize],
    pub luma_offset_l0: [i8; STD_VIDEO_H264_MAX_NUM_LIST_REF as usize],
    pub chroma_weight_l0: [[i8; STD_VIDEO_H264_MAX_CHROMA_PLANES as usize]; STD_VIDEO_H264_MAX_NUM_LIST_REF as usize],
    pub chroma_offset_l0: [[i8; STD_VIDEO_H264_MAX_CHROMA_PLANES as usize]; STD_VIDEO_H264_MAX_NUM_LIST_REF as usize],
    pub luma_weight_l1: [i8; STD_VIDEO_H264_MAX_NUM_LIST_REF as usize],
    pub luma_offset_l1: [i8; STD_VIDEO_H264_MAX_NUM_LIST_REF as usize],
    pub chroma_weight_l1: [[i8; STD_VIDEO_H264_MAX_CHROMA_PLANES as usize]; STD_VIDEO_H264_MAX_NUM_LIST_REF as usize],
    pub chroma_offset_l1: [[i8; STD_VIDEO_H264_MAX_CHROMA_PLANES as usize]; STD_VIDEO_H264_MAX_NUM_LIST_REF as usize],
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct StdVideoEncodeH264WeightTableFlags {
    pub luma_weight_l0_flag: u32,
    pub chroma_weight_l0_flag: u32,
    pub luma_weight_l1_flag: u32,
    pub chroma_weight_l1_flag: u32,
}

pub const VK_STD_VULKAN_VIDEO_CODEC_H264_ENCODE_API_VERSION_1_0_0: u32 = VK_MAKE_VIDEO_STD_VERSION(1, 0, 0);
pub const VK_STD_VULKAN_VIDEO_CODEC_H264_ENCODE_EXTENSION_NAME: &CStr = c"VK_STD_vulkan_video_codec_h264_encode";
pub const VK_STD_VULKAN_VIDEO_CODEC_H264_ENCODE_SPEC_VERSION: u32 = VK_STD_VULKAN_VIDEO_CODEC_H264_ENCODE_API_VERSION_1_0_0;
