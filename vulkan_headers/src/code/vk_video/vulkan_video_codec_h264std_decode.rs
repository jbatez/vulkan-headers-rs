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
    pub bitfields: u32,
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
    pub bitfields: u32,
}

pub const STD_VIDEO_DECODE_H264_FIELD_ORDER_COUNT_BOTTOM: StdVideoDecodeH264FieldOrderCount = 1;
pub const STD_VIDEO_DECODE_H264_FIELD_ORDER_COUNT_INVALID: StdVideoDecodeH264FieldOrderCount = 0x7FFFFFFF;
pub const STD_VIDEO_DECODE_H264_FIELD_ORDER_COUNT_LIST_SIZE: u32 = 2;
pub const STD_VIDEO_DECODE_H264_FIELD_ORDER_COUNT_TOP: StdVideoDecodeH264FieldOrderCount = 0;
pub const VK_STD_VULKAN_VIDEO_CODEC_H264_DECODE_API_VERSION_1_0_0: u32 = VK_MAKE_VIDEO_STD_VERSION(1, 0, 0);
pub const VK_STD_VULKAN_VIDEO_CODEC_H264_DECODE_EXTENSION_NAME: &CStr = c"VK_STD_vulkan_video_codec_h264_decode";
pub const VK_STD_VULKAN_VIDEO_CODEC_H264_DECODE_SPEC_VERSION: u32 = VK_STD_VULKAN_VIDEO_CODEC_H264_DECODE_API_VERSION_1_0_0;

pub type StdVideoDecodeH264FieldOrderCount = i32;
