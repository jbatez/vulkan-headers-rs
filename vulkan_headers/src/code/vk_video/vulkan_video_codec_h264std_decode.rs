use crate::code::*;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct StdVideoDecodeH264PictureInfo {
    pub flags: StdVideoDecodeH264PictureInfoFlags,
    pub seq_parameter_set_id: u8,
    pub pic_parameter_set_id: u8,
    pub reserved1: u8,
    pub reserved2: u8,
    pub frame_num: u16,
    pub idr_pic_id: u16,
    pub PicOrderCnt: [i32; STD_VIDEO_DECODE_H264_FIELD_ORDER_COUNT_LIST_SIZE as usize],
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct StdVideoDecodeH264PictureInfoFlags {
    pub field_pic_flag: u32,
    pub is_intra: u32,
    pub IdrPicFlag: u32,
    pub bottom_field_flag: u32,
    pub is_reference: u32,
    pub complementary_field_pair: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct StdVideoDecodeH264ReferenceInfo {
    pub flags: StdVideoDecodeH264ReferenceInfoFlags,
    pub FrameNum: u16,
    pub reserved: u16,
    pub PicOrderCnt: [i32; STD_VIDEO_DECODE_H264_FIELD_ORDER_COUNT_LIST_SIZE as usize],
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct StdVideoDecodeH264ReferenceInfoFlags {
    pub top_field_flag: u32,
    pub bottom_field_flag: u32,
    pub used_for_long_term_reference: u32,
    pub is_non_existing: u32,
}

pub type StdVideoDecodeH264FieldOrderCount = i32;
