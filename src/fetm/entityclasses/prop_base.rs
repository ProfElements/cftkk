use crate::fetm::{Error, TkKind};

#[derive(Debug, PartialEq)]
pub struct PropBase {
    field_0x1c: usize,
    field_0x20: f32,
    field_0x24: usize,
    field_0x25: usize,
    field_0x28: f32,
    field_0x2c: f32,
    field_0x30: usize,
    field_0x34: usize, //This is actually an EntityKlass?
    field_0x38: usize,
    field_0x3c: f32,
    field_0x40: f32,
    field_0x44: [usize; 4],
    field_0x48: usize, //This is probably a texture crc,
    field_0x50: usize,
    field_0x54: usize, //This is actually an EntityKlass?
    field_0x58: f32,
    field_0x5c: usize, //hex8
    field_0x60: usize, //hex8
    field_0x64: f32,
    field_0x68: usize, //this is actually an EntityKlass?
    field_0x69: usize, //this is actually an EntityKlass?
    field_0x6a: usize, //this is actually an EntityKlass?
    field_0x6c: f32,
    field_0x70: f32,
    field_0x74: usize,
    field_0x75: usize,
    field_0x76: usize,
    field_0x77: usize,
    field_0x78: usize,
    field_0x7c: f32,
    field_0x80: usize, //this is actually an EntityKlass?,
    field_0x81: usize, //this is actually an EntityKlass?,
    field_0x84: usize, //hex8,
    field_0x88: usize, //hex8,
    field_0x8c: usize, //hex8,
    field_0x94: usize,
}

impl PropBase {
    pub const SIZE: usize = 39;
    pub fn from_tokens(data: &[TkKind]) -> Result<Self, Error> {
        Ok(Self {
            field_0x1c: data[0].extract_int()?,
            field_0x20: data[1].extract_float()?,
            field_0x24: data[2].extract_int()?,
            field_0x25: data[3].extract_int()?,
            field_0x28: data[4].extract_float()?,
            field_0x2c: data[5].extract_float()?,
            field_0x30: data[6].extract_int()?,
            field_0x34: data[7].extract_int()?,
            field_0x38: data[8].extract_int()?,
            field_0x3c: data[9].extract_float()?,
            field_0x40: data[10].extract_float()?,
            field_0x44: [
                data[11].extract_int()?,
                data[12].extract_int()?,
                data[13].extract_int()?,
                data[14].extract_int()?,
            ],
            field_0x48: data[15].extract_hex8()?,
            field_0x50: data[16].extract_int()?,
            field_0x54: data[17].extract_int()?,
            field_0x58: data[18].extract_float()?,
            field_0x5c: data[19].extract_hex8()?,
            field_0x60: data[20].extract_hex8()?,
            field_0x64: data[21].extract_float()?,
            field_0x68: data[22].extract_int()?,
            field_0x69: data[23].extract_int()?,
            field_0x6a: data[24].extract_int()?,
            field_0x6c: data[25].extract_float()?,
            field_0x70: data[26].extract_float()?,
            field_0x74: data[27].extract_int()?,
            field_0x75: data[28].extract_int()?,
            field_0x76: data[29].extract_int()?,
            field_0x77: data[30].extract_int()?,
            field_0x78: data[31].extract_int()?,
            field_0x7c: data[32].extract_float()?,
            field_0x80: data[33].extract_int()?,
            field_0x81: data[34].extract_int()?,
            field_0x84: data[35].extract_hex8()?,
            field_0x88: data[36].extract_hex8()?,
            field_0x8c: data[37].extract_hex8()?,
            field_0x94: data[38].extract_int()?,
        })
    }
}
