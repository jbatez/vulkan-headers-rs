use crate::code::*;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct StdVideoDecodeH265PictureInfo {
    pub flags: StdVideoDecodeH265PictureInfoFlags,
    pub sps_video_parameter_set_id: u8,
    pub pps_seq_parameter_set_id: u8,
    pub pps_pic_parameter_set_id: u8,
    pub NumDeltaPocsOfRefRpsIdx: u8,
    pub PicOrderCntVal: i32,
    pub NumBitsForSTRefPicSetInSlice: u16,
    pub reserved: u16,
    pub RefPicSetStCurrBefore: [u8; STD_VIDEO_DECODE_H265_REF_PIC_SET_LIST_SIZE as usize],
    pub RefPicSetStCurrAfter: [u8; STD_VIDEO_DECODE_H265_REF_PIC_SET_LIST_SIZE as usize],
    pub RefPicSetLtCurr: [u8; STD_VIDEO_DECODE_H265_REF_PIC_SET_LIST_SIZE as usize],
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct StdVideoDecodeH265PictureInfoFlags {
    pub IrapPicFlag: u32,
    pub IdrPicFlag: u32,
    pub IsReference: u32,
    pub short_term_ref_pic_set_sps_flag: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct StdVideoDecodeH265ReferenceInfo {
    pub flags: StdVideoDecodeH265ReferenceInfoFlags,
    pub PicOrderCntVal: i32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct StdVideoDecodeH265ReferenceInfoFlags {
    pub used_for_long_term_reference: u32,
    pub unused_for_reference: u32,
}
