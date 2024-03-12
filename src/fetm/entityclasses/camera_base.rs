use crate::fetm::{Error, TkKind};

#[derive(PartialEq, Debug)]
pub struct CameraBase {
    field_0x1c: usize, //this is a hex8
    field_0x24: usize,
    field_0x25: usize,
    field_0x26: usize,
    field_0x27: usize,
    field_0x28: usize, //this is a hex8
    action_list_0_count: usize,
    // action_list_0: [ActionEntry]
    action_list_1_count: usize,
    // action_list_1: [ActionEntry]
    field_0x38: usize,
    field_0x39: usize,
    field_0x3a: usize,
    field_0x3b: usize,
    field_0x3c: usize,
    field_0x3d: usize,
    field_0x3e: usize,
    field_0x3f: usize,
    field_0x40: f32,
    field_0x44: usize,
    field_0x48: usize,
    field_0x4c: f32,
    field_0x50: usize,
    field_0x51: usize,
    field_0x54: usize, //this is a hex8,
    field_0x5c: usize,
    field_0x5d: usize,
    field_0x5e: usize,
    field_0x5f: usize,
    field_0x60: usize,
    field_0x61: usize,
}

impl CameraBase {
    pub fn from_tokens(data: &[TkKind]) -> Result<Self, Error> {
        if data[6].extract_int()? != 0 {
            panic!("action list 0 count is greater then 0 ")
        }

        if data[7].extract_int()? != 0 {
            panic!("action list 1 count is greater then 0 ")
        }

        Ok(Self {
            field_0x1c: data[0].extract_hex8()?,
            field_0x24: data[1].extract_int()?,
            field_0x25: data[2].extract_int()?,
            field_0x26: data[3].extract_int()?,
            field_0x27: data[4].extract_int()?,
            field_0x28: data[5].extract_hex8()?,
            action_list_0_count: data[6].extract_int()?,
            action_list_1_count: data[7].extract_int()?,
            field_0x38: data[8].extract_int()?,
            field_0x39: data[9].extract_int()?,
            field_0x3a: data[10].extract_int()?,
            field_0x3b: data[11].extract_int()?,
            field_0x3c: data[12].extract_int()?,
            field_0x3d: data[13].extract_int()?,
            field_0x3e: data[14].extract_int()?,
            field_0x3f: data[15].extract_int()?,
            field_0x40: data[16].extract_float()?,
            field_0x44: data[17].extract_int()?,
            field_0x48: data[18].extract_int()?,
            field_0x4c: data[19].extract_float()?,
            field_0x50: data[20].extract_int()?,
            field_0x51: data[21].extract_int()?,
            field_0x54: data[22].extract_hex8()?,
            field_0x5c: data[23].extract_int()?,
            field_0x5d: data[24].extract_int()?,
            field_0x5e: data[25].extract_int()?,
            field_0x5f: data[26].extract_int()?,
            field_0x60: data[27].extract_int()?,
            field_0x61: data[28].extract_int()?,
        })
    }

    pub fn size(&self) -> usize {
        29
    }
}
