use crate::fetm::{Error, TkKind};

use super::camera_base::CameraBase;

#[derive(PartialEq, Debug)]
pub struct ChaseCamera {
    base: CameraBase,
    field_0x64: usize,
    field_0x68: f32,
    field_0x6c: f32,
    field_0x70: f32,
    field_0x74: f32,
    field_0x78: f32,
    field_0x7c: f32,
    field_0x80: f32,
    field_0x84: f32,
}

impl ChaseCamera {
    pub fn from_tokens(data: &[TkKind]) -> Result<Self, Error> {
        let base = CameraBase::from_tokens(data)?;
        let offset = base.size();

        Ok(Self {
            base,
            field_0x64: data[offset].extract_int()?,
            field_0x68: data[offset + 1].extract_float()?,
            field_0x6c: data[offset + 2].extract_float()?,
            field_0x70: data[offset + 3].extract_float()?,
            field_0x74: data[offset + 4].extract_float()?,
            field_0x78: data[offset + 5].extract_float()?,
            field_0x7c: data[offset + 6].extract_float()?,
            field_0x80: data[offset + 7].extract_float()?,
            field_0x84: data[offset + 8].extract_float()?,
        })
    }
}
