use std::process::exit;

use crate::fetm::{Error, TkKind};

#[derive(PartialEq, Debug)]
pub struct ShockwaveEffect {
    field_0x1c: usize, //this is a crc,
    field_0x24: usize,
    field_0x28: f32,
    field_0x2c: usize,
    field_0x30: usize,
    field_0x34: usize,
    field_0x35: usize,
    field_0x36: usize,
    field_0x37: usize,
    field_0x38: usize,
    field_0x3c: f32,
    field_0x40: f32,
    field_0x44: f32,
    field_0x48: f32,
}

impl ShockwaveEffect {
    pub fn from_tokens(data: &[TkKind]) -> Result<Self, Error> {
        Ok(Self {
            field_0x1c: data[0].extract_hex8()?,
            field_0x24: data[1].extract_int()?,
            field_0x28: data[2].extract_float()?,
            field_0x2c: data[3].extract_int()?,
            field_0x30: data[4].extract_int()?,
            field_0x34: data[5].extract_int()?,
            field_0x35: data[6].extract_int()?,
            field_0x36: data[7].extract_int()?,
            field_0x37: data[8].extract_int()?,
            field_0x38: data[9].extract_int()?,
            field_0x3c: data[10].extract_float()?,
            field_0x40: data[11].extract_float()?,
            field_0x44: data[12].extract_float()?,
            field_0x48: data[13].extract_float()?,
        })
    }
}
