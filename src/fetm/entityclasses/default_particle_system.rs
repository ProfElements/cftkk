use crate::fetm::{Error, TkKind};

#[derive(Debug, PartialEq)]
pub struct DefaultParticleSystem {
    field_0x1c: usize,
    field_0x24: usize,
    field_0x28: usize,
    field_0x2c: usize,
    field_0x30: usize,
    field_0x34: f32,
    field_0x38: usize,
    field_0x3c: usize,
    field_0x40: usize,
    field_0x41: usize,
    field_0x42: usize,
    field_0x43: usize,
    field_0x44: usize,
    field_0x45: usize,
    field_0x46: usize,
    field_0x47: usize,
    field_0x48: usize,
    field_0x4c: usize,
    field_0x50: f32,
    field_0x54: f32,
    field_0x58: f32,
    field_0x5c: f32,
    field_0x60: f32,
    field_0x64: usize,
    field_0x68: f32,
    field_0x6c: f32,
    field_0x70: f32,
    field_0x74: f32,
    field_0x78: usize,
    field_0x7c: f32,
    field_0x80: f32,
    field_0x84: f32,
    field_0x88: usize,
    field_0x8c: f32,
    field_0x90: f32,
    field_0x94: usize,
    field_0x98: f32,
    field_0x9c: f32,
    field_0xa0: f32,
    field_0xa8: f32,
    field_0xac: f32,
    field_0xb0: f32,
    field_0xb4: f32,
    field_0xb8: f32,
    field_0xbc: f32,
    field_0xc0: f32,
    field_0xc4: f32,
    field_0xc8: usize,
    field_0xcc: f32,
    field_0xd0: f32,
    field_0xd4: usize,
}

impl DefaultParticleSystem {
    pub const SIZE: usize = 51;
    pub fn from_tokens(data: &[TkKind]) -> Result<Self, Error> {
        Ok(Self {
            field_0x1c: data[0].extract_hex8()?,
            field_0x24: data[1].extract_int()?,
            field_0x28: data[2].extract_int()?,
            field_0x2c: data[3].extract_int()?,
            field_0x30: data[4].extract_int()?,
            field_0x34: data[5].extract_float()?,
            field_0x38: data[6].extract_int()?,
            field_0x3c: data[7].extract_int()?,
            field_0x40: data[8].extract_int()?,
            field_0x41: data[9].extract_int()?,
            field_0x42: data[10].extract_int()?,
            field_0x43: data[11].extract_int()?,
            field_0x44: data[12].extract_int()?,
            field_0x45: data[13].extract_int()?,
            field_0x46: data[14].extract_int()?,
            field_0x47: data[15].extract_int()?,
            field_0x48: data[16].extract_int()?,
            field_0x4c: data[17].extract_int()?,
            field_0x50: data[18].extract_float()?,
            field_0x54: data[19].extract_float()?,
            field_0x58: data[20].extract_float()?,
            field_0x5c: data[21].extract_float()?,
            field_0x60: data[22].extract_float()?,
            field_0x64: data[23].extract_int()?,
            field_0x68: data[24].extract_float()?,
            field_0x6c: data[25].extract_float()?,
            field_0x70: data[26].extract_float()?,
            field_0x74: data[27].extract_float()?,
            field_0x78: data[28].extract_int()?,
            field_0x7c: data[29].extract_float()?,
            field_0x80: data[30].extract_float()?,
            field_0x84: data[31].extract_float()?,
            field_0x88: data[32].extract_int()?,
            field_0x8c: data[33].extract_float()?,
            field_0x90: data[34].extract_float()?,
            field_0x94: data[35].extract_int()?,
            field_0x98: data[36].extract_float()?,
            field_0x9c: data[37].extract_float()?,
            field_0xa0: data[38].extract_float()?,
            field_0xa8: data[39].extract_float()?,
            field_0xac: data[40].extract_float()?,
            field_0xb0: data[41].extract_float()?,
            field_0xb4: data[42].extract_float()?,
            field_0xb8: data[43].extract_float()?,
            field_0xbc: data[44].extract_float()?,
            field_0xc0: data[45].extract_float()?,
            field_0xc4: data[46].extract_float()?,
            field_0xc8: data[47].extract_int()?,
            field_0xcc: data[48].extract_float()?,
            field_0xd0: data[49].extract_float()?,
            field_0xd4: data[50].extract_int()?,
        })
    }
}
