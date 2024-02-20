use crate::fetm::{Error, TkKind};

use super::prop_base::PropBase;

#[derive(Debug, PartialEq)]
pub struct GameObject {
    base: PropBase,
    field_0x98: usize,
    field_0x9c: f32,
    field_0xa0: f32,
    field_0xa4: usize,
    field_0xa8: usize, //crc
    field_0xb0: usize,
    field_0xb4: usize,
}

impl GameObject {
    pub const SIZE: usize = 46;
    pub fn from_tokens(data: &[TkKind]) -> Result<Self, Error> {
        Ok(Self {
            base: PropBase::from_tokens(data)?,
            field_0x98: data[PropBase::SIZE].extract_int()?,
            field_0x9c: data[PropBase::SIZE + 1].extract_float()?,
            field_0xa0: data[PropBase::SIZE + 2].extract_float()?,
            field_0xa4: data[PropBase::SIZE + 3].extract_int()?,
            field_0xa8: data[PropBase::SIZE + 4].extract_hex8()?,
            field_0xb0: data[PropBase::SIZE + 5].extract_int()?,
            field_0xb4: data[PropBase::SIZE + 6].extract_int()?,
        })
    }
}
