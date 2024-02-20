use crate::fetm::{Error, TkKind};

use super::game_object::GameObject;

#[derive(Debug, PartialEq)]
pub struct Character {
    game_object: GameObject,
    field_0xb8: usize,
    field_0xbc: usize, //this is a crc
    field_0xc4: usize,
    field_0xc5: usize,
    field_0xc6: usize,
    field_0xc7: usize,
    field_0xc8: usize,
    field_0xc9: usize,
    field_0xca: usize,
    field_0xcb: usize,
    field_0xcc: usize,
    field_0xcd: usize,
    field_0xce: usize,
    field_0xcf: usize,
    field_0xd0: usize,
    field_0xd1: usize,
    field_0xd2: usize,
    field_0xd3: usize,
    field_0xd4: usize,
    field_0xd5: usize,
    field_0xd8: f32,
    field_0xdc: f32,
    field_0xe0: f32,
    field_0xe4: f32,
    field_0xe8: usize,
    field_0xec: usize,
    field_0xf0: f32,
    field_0xf4: f32,
    field_0xf8: usize,
    field_0xfc: usize, //this is a crc
}

impl Character {
    pub const SIZE: usize = GameObject::SIZE + 30;
    pub fn from_tokens(data: &[TkKind]) -> Result<Self, Error> {
        let base = GameObject::SIZE;
        Ok(Self {
            game_object: GameObject::from_tokens(data)?,
            field_0xb8: data[base].extract_int()?,
            field_0xbc: data[base + 1].extract_hex8()?,
            field_0xc4: data[base + 2].extract_int()?,
            field_0xc5: data[base + 3].extract_int()?,
            field_0xc6: data[base + 4].extract_int()?,
            field_0xc7: data[base + 5].extract_int()?,
            field_0xc8: data[base + 6].extract_int()?,
            field_0xc9: data[base + 7].extract_int()?,
            field_0xca: data[base + 8].extract_int()?,
            field_0xcb: data[base + 9].extract_int()?,
            field_0xcc: data[base + 10].extract_int()?,
            field_0xcd: data[base + 11].extract_int()?,
            field_0xce: data[base + 12].extract_int()?,
            field_0xcf: data[base + 13].extract_int()?,
            field_0xd0: data[base + 14].extract_int()?,
            field_0xd1: data[base + 15].extract_int()?,
            field_0xd2: data[base + 16].extract_int()?,
            field_0xd3: data[base + 17].extract_int()?,
            field_0xd4: data[base + 18].extract_int()?,
            field_0xd5: data[base + 19].extract_int()?,
            field_0xd8: data[base + 20].extract_float()?,
            field_0xdc: data[base + 21].extract_float()?,
            field_0xe0: data[base + 22].extract_float()?,
            field_0xe4: data[base + 23].extract_float()?,
            field_0xe8: data[base + 24].extract_int()?,
            field_0xec: data[base + 25].extract_int()?,
            field_0xf0: data[base + 26].extract_float()?,
            field_0xf4: data[base + 27].extract_float()?,
            field_0xf8: data[base + 28].extract_int()?,
            field_0xfc: data[base + 29].extract_hex8()?,
        })
    }
}
