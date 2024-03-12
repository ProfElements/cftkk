use crate::fetm::{Error, TkKind};

#[derive(PartialEq, Debug)]
pub struct SwipeEffect {
    field_0x1c: usize,
    field_0x24: usize,
    field_0x28: usize,
    field_0x2c: usize,
    field_0x30: usize,
    field_0x34: f32,
    field_0x38: f32,
    field_0x3c: f32,
    field_0x44: f32,
    field_0x48: f32,
    field_0x4c: f32,
    field_0x54: usize,
    field_0x5c: usize,
    filed_0x60: f32,
    field_0x64: usize,
    field_0x65: usize,
    field_0x66: usize,
    field_0x67: usize,
    field_0x68: usize,
    field_0x6c: usize,
    field_0x6d: usize,
    field_0x6e: usize,
    field_0x70: f32,
    field_0x74: usize,
    field_0x78: f32,
}

impl SwipeEffect {
    pub fn from_tokens(tokens: &[TkKind]) -> Result<Self, Error> {
        Ok(Self {
            field_0x1c: tokens[0].extract_hex8()?,
            field_0x24: tokens[1].extract_hex8()?,
            field_0x28: tokens[2].extract_hex8()?,
            field_0x2c: tokens[3].extract_hex8()?,
            field_0x30: tokens[4].extract_hex8()?,
            field_0x34: tokens[5].extract_float()?,
            field_0x38: tokens[6].extract_float()?,
            field_0x3c: tokens[7].extract_float()?,
            field_0x44: tokens[8].extract_float()?,
            field_0x48: tokens[9].extract_float()?,
            field_0x4c: tokens[10].extract_float()?,
            field_0x54: tokens[11].extract_hex8()?,
            field_0x5c: tokens[12].extract_int()?,
            filed_0x60: tokens[13].extract_float()?,
            field_0x64: tokens[14].extract_int()?,
            field_0x65: tokens[15].extract_int()?,
            field_0x66: tokens[16].extract_int()?,
            field_0x67: tokens[17].extract_int()?,
            field_0x68: tokens[18].extract_int()?,
            field_0x6c: tokens[19].extract_int()?,
            field_0x6d: tokens[20].extract_int()?,
            field_0x6e: tokens[21].extract_int()?,
            field_0x70: tokens[22].extract_float()?,
            field_0x74: tokens[23].extract_int()?,
            field_0x78: tokens[24].extract_float()?,
        })
    }
}
