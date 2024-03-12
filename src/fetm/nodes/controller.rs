use alloc::vec::Vec;

use crate::fetm::{BaseNode, Error, TkKind};

use super::node::Node;

#[derive(Debug)]
pub struct Controller<'a> {
    base: Node<'a>,
    unk_count_0: usize,
    channel_count: usize,
    input_channels: Vec<InputChannel<'a>>,
    left_stick_flag_0: usize,
    left_stick_flag_1: usize,
    left_stick_unk_float: f32,
    right_stick_flag_0: usize,
    right_stick_flag_1: usize,
    right_stick_unk_float: f32,
    size: usize,
}

impl<'a> Controller<'a> {
    pub fn from_tokens(data: &'a [TkKind]) -> Result<Self, Error> {
        let base = Node::from_tokens(data)?;
        let offset = base.size();

        let channel_count = data[offset + 1].extract_int()?;

        let mut size = offset + 2;
        let mut channels = Vec::with_capacity(channel_count);
        for _ in 0..channel_count {
            if let Ok(channel) = InputChannel::from_tokens(&data[size..]) {
                size += channel.size();
                channels.push(channel);
            }
        }

        Ok(Self {
            base,
            unk_count_0: data[offset].extract_int()?,
            channel_count,
            input_channels: channels,
            left_stick_flag_0: data[size].extract_int()?,
            left_stick_flag_1: data[size + 1].extract_int()?,
            left_stick_unk_float: data[size + 2].extract_float()?,
            right_stick_flag_0: data[size + 3].extract_int()?,
            right_stick_flag_1: data[size + 4].extract_int()?,
            right_stick_unk_float: data[size + 5].extract_float()?,
            size: size + 6,
        })
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

#[derive(Debug, Clone)]
struct InputChannel<'a> {
    channel_name: &'a str,
    has_unk_float_repeat: usize,
    unk_float_0: f32,
    unk_float_1: f32,
    unk_flag_0: usize,
    input_desc_count: usize,
    input_descs: Vec<InputDesc>,
}

#[derive(Debug, Clone)]
struct InputDesc {
    unk_flag_0: usize,
    unk_flag_1: usize,
    unk_flag_2: usize,
    unk_flag_3: usize,
}

impl InputDesc {
    pub fn from_tokens(data: &[TkKind]) -> Result<Self, Error> {
        Ok(Self {
            unk_flag_0: data[0].extract_int()?,
            unk_flag_1: data[1].extract_int()?,
            unk_flag_2: data[2].extract_int()?,
            unk_flag_3: data[3].extract_int()?,
        })
    }
}

impl<'a> InputChannel<'a> {
    pub fn from_tokens(data: &'a [TkKind]) -> Result<Self, Error> {
        let input_desc_count = data[5].extract_int()?;
        Ok(Self {
            channel_name: data[0].extract_str()?,
            has_unk_float_repeat: data[1].extract_int()?,
            unk_float_0: data[2].extract_float()?,
            unk_float_1: data[3].extract_float()?,
            unk_flag_0: data[4].extract_int()?,
            input_desc_count,
            input_descs: data[6..6 + (input_desc_count * 4)]
                .chunks(4)
                .map(|data| InputDesc::from_tokens(data).unwrap())
                .collect::<Vec<InputDesc>>(),
            /*          input_descs: data[6..6 * data[4].extract_int()? * 4]
                            .chunks(4)
                            .map(|data| InputDesc::from_tokens(data).unwrap())
                            .collect::<Vec<InputDesc>>(),
            */
        })
    }

    pub fn size(&self) -> usize {
        self.input_desc_count * 4 + 6
    }
}
