use crate::fetm::{Error, TkKind};

use super::sim_item::SimItem;

#[derive(Debug, PartialEq)]
pub struct SimBody {
    sim_item: SimItem,
    field_0x28: usize,
    field_0x2c: f32,
    field_0x30: usize,
    field_0x34: usize,
    field_0x38: usize,
    field_0x3c: f32,
    field_0x40: f32,
    field_0x44: f32,
    field_0x4c: f32,
    field_0x50: f32,
    field_0x54: f32,
    flag_0: usize,
    flag_1: usize,
    flag_2: usize,
    field_0x60: usize,
    field_0x64: usize, //this is a node (CFMode_World) crc,
    flag_3: usize,
    flag_4: usize,
    flag_5: usize,
    field_0x6c: usize,
    flag_6: usize,
    flag_7: usize,
    field_0x70: f32,
    flag_8: usize,
    field_0x74: f32,
    flag_9: usize,
    field_0x78: f32,
    flag_10: usize,
    flag_11: usize,
}

impl SimBody {
    pub const SIZE: usize = SimItem::SIZE + 29;
    pub fn from_tokens(data: &[TkKind]) -> Result<Self, Error> {
        let base = SimItem::SIZE;
        Ok(Self {
            sim_item: SimItem::from_tokens(data)?,
            field_0x28: data[base].extract_int()?,
            field_0x2c: data[base + 1].extract_float()?,
            field_0x30: data[base + 2].extract_int()?,
            field_0x34: data[base + 3].extract_int()?,
            field_0x38: data[base + 4].extract_int()?,
            field_0x3c: data[base + 5].extract_float()?,
            field_0x40: data[base + 6].extract_float()?,
            field_0x44: data[base + 7].extract_float()?,
            field_0x4c: data[base + 8].extract_float()?,
            field_0x50: data[base + 9].extract_float()?,
            field_0x54: data[base + 10].extract_float()?,
            flag_0: data[base + 11].extract_int()?,
            flag_1: data[base + 12].extract_int()?,
            flag_2: data[base + 13].extract_int()?,
            field_0x60: data[base + 14].extract_int()?,
            field_0x64: data[base + 15].extract_hex8()?,
            flag_3: data[base + 16].extract_int()?,
            flag_4: data[base + 17].extract_int()?,
            flag_5: data[base + 18].extract_int()?,
            field_0x6c: data[base + 19].extract_int()?,
            flag_6: data[base + 20].extract_int()?,
            flag_7: data[base + 21].extract_int()?,
            field_0x70: data[base + 22].extract_float()?,
            flag_8: data[base + 23].extract_int()?,
            field_0x74: data[base + 24].extract_float()?,
            flag_9: data[base + 25].extract_int()?,
            field_0x78: data[base + 26].extract_float()?,
            flag_10: data[base + 27].extract_int()?,
            flag_11: data[base + 28].extract_int()?,
        })
    }
}
