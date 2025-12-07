use crate::code::*;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct StdVideoEncodeH265LongTermRefPics {
    pub num_long_term_sps: u8,
    pub num_long_term_pics: u8,
    pub lt_idx_sps: [u8; STD_VIDEO_H265_MAX_LONG_TERM_REF_PICS_SPS as usize],
    pub poc_lsb_lt: [u8; STD_VIDEO_H265_MAX_LONG_TERM_PICS as usize],
    pub used_by_curr_pic_lt_flag: u16,
    pub delta_poc_msb_present_flag: [u8; STD_VIDEO_H265_MAX_DELTA_POC as usize],
    pub delta_poc_msb_cycle_lt: [u8; STD_VIDEO_H265_MAX_DELTA_POC as usize],
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct StdVideoEncodeH265PictureInfo {
    pub flags: StdVideoEncodeH265PictureInfoFlags,
    pub pic_type: StdVideoH265PictureType,
    pub sps_video_parameter_set_id: u8,
    pub pps_seq_parameter_set_id: u8,
    pub pps_pic_parameter_set_id: u8,
    pub short_term_ref_pic_set_idx: u8,
    pub PicOrderCntVal: i32,
    pub TemporalId: u8,
    pub reserved1: [u8; 7 as usize],
    pub pRefLists: *const StdVideoEncodeH265ReferenceListsInfo,
    pub pShortTermRefPicSet: *const StdVideoH265ShortTermRefPicSet,
    pub pLongTermRefPics: *const StdVideoEncodeH265LongTermRefPics,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct StdVideoEncodeH265PictureInfoFlags {
    pub bitfields: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct StdVideoEncodeH265ReferenceInfo {
    pub flags: StdVideoEncodeH265ReferenceInfoFlags,
    pub pic_type: StdVideoH265PictureType,
    pub PicOrderCntVal: i32,
    pub TemporalId: u8,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct StdVideoEncodeH265ReferenceInfoFlags {
    pub bitfields: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct StdVideoEncodeH265ReferenceListsInfo {
    pub flags: StdVideoEncodeH265ReferenceListsInfoFlags,
    pub num_ref_idx_l0_active_minus1: u8,
    pub num_ref_idx_l1_active_minus1: u8,
    pub RefPicList0: [u8; STD_VIDEO_H265_MAX_NUM_LIST_REF as usize],
    pub RefPicList1: [u8; STD_VIDEO_H265_MAX_NUM_LIST_REF as usize],
    pub list_entry_l0: [u8; STD_VIDEO_H265_MAX_NUM_LIST_REF as usize],
    pub list_entry_l1: [u8; STD_VIDEO_H265_MAX_NUM_LIST_REF as usize],
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct StdVideoEncodeH265ReferenceListsInfoFlags {
    pub bitfields: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct StdVideoEncodeH265SliceSegmentHeader {
    pub flags: StdVideoEncodeH265SliceSegmentHeaderFlags,
    pub slice_type: StdVideoH265SliceType,
    pub slice_segment_address: u32,
    pub collocated_ref_idx: u8,
    pub MaxNumMergeCand: u8,
    pub slice_cb_qp_offset: i8,
    pub slice_cr_qp_offset: i8,
    pub slice_beta_offset_div2: i8,
    pub slice_tc_offset_div2: i8,
    pub slice_act_y_qp_offset: i8,
    pub slice_act_cb_qp_offset: i8,
    pub slice_act_cr_qp_offset: i8,
    pub slice_qp_delta: i8,
    pub reserved1: u16,
    pub pWeightTable: *const StdVideoEncodeH265WeightTable,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct StdVideoEncodeH265SliceSegmentHeaderFlags {
    pub bitfields: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct StdVideoEncodeH265WeightTable {
    pub flags: StdVideoEncodeH265WeightTableFlags,
    pub luma_log2_weight_denom: u8,
    pub delta_chroma_log2_weight_denom: i8,
    pub delta_luma_weight_l0: [i8; STD_VIDEO_H265_MAX_NUM_LIST_REF as usize],
    pub luma_offset_l0: [i8; STD_VIDEO_H265_MAX_NUM_LIST_REF as usize],
    pub delta_chroma_weight_l0: [[i8; STD_VIDEO_H265_MAX_CHROMA_PLANES as usize]; STD_VIDEO_H265_MAX_NUM_LIST_REF as usize],
    pub delta_chroma_offset_l0: [[i8; STD_VIDEO_H265_MAX_CHROMA_PLANES as usize]; STD_VIDEO_H265_MAX_NUM_LIST_REF as usize],
    pub delta_luma_weight_l1: [i8; STD_VIDEO_H265_MAX_NUM_LIST_REF as usize],
    pub luma_offset_l1: [i8; STD_VIDEO_H265_MAX_NUM_LIST_REF as usize],
    pub delta_chroma_weight_l1: [[i8; STD_VIDEO_H265_MAX_CHROMA_PLANES as usize]; STD_VIDEO_H265_MAX_NUM_LIST_REF as usize],
    pub delta_chroma_offset_l1: [[i8; STD_VIDEO_H265_MAX_CHROMA_PLANES as usize]; STD_VIDEO_H265_MAX_NUM_LIST_REF as usize],
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct StdVideoEncodeH265WeightTableFlags {
    pub luma_weight_l0_flag: u16,
    pub chroma_weight_l0_flag: u16,
    pub luma_weight_l1_flag: u16,
    pub chroma_weight_l1_flag: u16,
}

pub const VK_STD_VULKAN_VIDEO_CODEC_H265_ENCODE_API_VERSION_1_0_0: u32 = VK_MAKE_VIDEO_STD_VERSION(1, 0, 0);
pub const VK_STD_VULKAN_VIDEO_CODEC_H265_ENCODE_EXTENSION_NAME: &CStr = c"VK_STD_vulkan_video_codec_h265_encode";
pub const VK_STD_VULKAN_VIDEO_CODEC_H265_ENCODE_SPEC_VERSION: u32 = VK_STD_VULKAN_VIDEO_CODEC_H265_ENCODE_API_VERSION_1_0_0;
