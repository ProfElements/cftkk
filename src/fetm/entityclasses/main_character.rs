use std::process::exit;

use crate::fetm::{Error, TkKind};

use super::character::Character;

#[derive(Debug, PartialEq)]
pub struct MainCharacter {
    character: Character,
    field_0x104: usize, //this is a crc,
    field_0x10c: usize,
    field_0x110: usize, //this is a crc,
    field_0x118: usize,
    field_0x11c: usize, //this is a crc,
    field_0x124: usize, //this is a crc,
    field_0x12c: usize, //this is a crc,
    field_0x134: usize, //this is a crc,
    field_0x13c: usize, //this is a crc,
    field_0x144: usize, //this is a crc,
    field_0x14c: usize, //this is a crc,
    field_0x154: usize, //this is a crc,
    field_0x15c: usize, //this is a crc,
    field_0x164: usize, //this is a crc,
    field_0x16c: usize, //this is a crc,
    field_0x174: usize,
    field_0x178: usize, //this is a crc,
    field_0x17c: usize, //this is a crc,
    field_0x180: usize, //this is a crc,
    field_0x188: f32,
    field_0x18c: usize,
    field_0x190: usize,
    field_0x191: usize,
    field_0x194: usize, //this is a crc
    field_0x198: usize, //this is a crc,
    field_0x19c: usize, //this is a crc,
    field_0x1ac: usize, //this is a crc,
    field_0x1b4: usize, //this is a crc,
    field_0x1bc: usize, // this is a crc,
    field_0x1c4: usize, //this is a crc,
    field_0x1cc: usize, //this is a crc,
    field_0x1d4: usize, //this is a crc,
    field_0x1dc: usize, //this is a crc,
    field_0x1e4: usize,
    field_0x1e8: usize, //this is a crc,
    field_0x1f0: usize,
    field_0x1f4: usize, //this is a crc,
    field_0x1fc: usize,
    field_0x200: usize, //this is a crc,
    field_0x208: usize,
    field_0x20c: usize, //this is a crc,
    field_0x214: usize,
    field_0x218: usize, //this is a crc,
    field_0x220: usize, //this is a crc,
    field_0x228: usize, //this is a crc,
    field_0x230: usize, // this is a crc,
    field_0x238: usize, //this is a crc,
    field_0x240: usize, //this is a crc,
    field_0x248: usize, //this is a crc,
    field_0x250: usize, //this is a crc,
    field_0x258: usize, // this is a crc,
    field_0x260: usize, // this is a crc,
    field_0x268: usize, //this is a crc,
    field_0x270: usize, //this is a crc,
    field_0x278: usize, //this is a crc,
    field_0x280: usize, //this is a crc,
    field_0x288: usize, //this is a crc,
    field_0x290: usize, //this is a crc,
    field_0x298: usize, //this is a crc,
    field_0x2a0: usize, //this is a crc,
    field_0x2a8: usize, //this is a crc,
    field_0x2b0: usize, //this is a crc,
    field_0x2b8: usize, //this is a crc,
    field_0x2c0: usize, //this is a crc,
    field_0x2c8: usize, //this is a crc,
    field_0x2d0: usize, //this is a crc,
    field_0x2d8: usize, //this is a crc,
    field_0x2dc: usize, //this is a crc,
    field_0x2e0: usize, //this is a crc,
    field_0x2e4: usize, //this is a crc,
    field_0x2e8: usize, //this is a crc,
    field_0x2ec: usize, //this is a crc,
    field_0x2f0: usize, //this is a crc,
    field_0x2f4: usize, //this is a crc,
    field_0x2f8: usize, //this is a crc,
    field_0x2fc: usize, //this is a crc,
    field_0x300: usize, //this is a crc,
    field_0x304: usize, //this is a crc,
    field_0x308: usize, //this is a crc,
    field_0x30c: usize, //this is a crc,
    field_0x310: usize, //this is a crc,
    field_0x318: usize, //this is a crc,
    field_0x320: usize, //this is a crc,
    field_0x328: usize, //this is a crc,
    field_0x330: usize, //this is a crc,
    field_0x338: usize,
    field_0x33c: usize, //this is a crc,
    field_0x344: usize, //this is a crc,
    field_0x34c: usize, //this is a crc,
    field_0x354: usize, //this is a crc,
    field_0x35c: f32,
    field_0x360: usize, //this is  a crc,
    field_0x368: usize, //this is a crc,
    field_0x370: usize, //this is a crc,
    field_0x378: usize, //this is a crc,
    field_0x380: usize, //this is a crc,
    field_0x388: usize, //this is a crc,
    field_0x390: usize, //this is a crc,
    field_0x398: usize, //this is a crc,
    field_0x3a0: usize, //this is a crc,
    field_0x3a8: usize,
    field_0x3ac: usize, //this is a crc,
    field_0x3b0: usize, //this is a crc,
    field_0x3b4: usize,
    field_0x3b8: usize, //this is a crc,
    field_0x3bc: usize, //this is a crc,
    field_0x3c0: usize,
}

impl MainCharacter {
    pub const SIZE: usize = Character::SIZE + 107;
    pub fn from_tokens(data: &[TkKind]) -> Result<Self, Error> {
        let base = Character::SIZE;
        Ok(Self {
            character: Character::from_tokens(data)?,
            field_0x104: data[base].extract_hex8()?,
            field_0x10c: data[base + 1].extract_int()?,
            field_0x110: data[base + 2].extract_hex8()?,
            field_0x118: data[base + 3].extract_int()?,
            field_0x11c: data[base + 4].extract_hex8()?,
            field_0x124: data[base + 5].extract_hex8()?,
            field_0x12c: data[base + 6].extract_hex8()?,
            field_0x134: data[base + 7].extract_hex8()?,
            field_0x13c: data[base + 8].extract_hex8()?,
            field_0x144: data[base + 9].extract_hex8()?,
            field_0x14c: data[base + 10].extract_hex8()?,
            field_0x154: data[base + 11].extract_hex8()?,
            field_0x15c: data[base + 12].extract_hex8()?,
            field_0x164: data[base + 13].extract_hex8()?,
            field_0x16c: data[base + 14].extract_hex8()?,
            field_0x174: data[base + 15].extract_int()?,
            field_0x178: data[base + 16].extract_hex8()?,
            field_0x17c: data[base + 17].extract_hex8()?,
            field_0x180: data[base + 18].extract_hex8()?,
            field_0x188: data[base + 19].extract_float()?,
            field_0x18c: data[base + 20].extract_int()?,
            field_0x190: data[base + 21].extract_int()?,
            field_0x191: data[base + 22].extract_int()?,
            field_0x194: data[base + 23].extract_hex8()?,
            field_0x198: data[base + 24].extract_hex8()?,
            field_0x19c: data[base + 25].extract_hex8()?,
            field_0x1ac: data[base + 26].extract_hex8()?,
            field_0x1b4: data[base + 27].extract_hex8()?,
            field_0x1bc: data[base + 28].extract_hex8()?,
            field_0x1c4: data[base + 29].extract_hex8()?,
            field_0x1cc: data[base + 30].extract_hex8()?,
            field_0x1d4: data[base + 31].extract_hex8()?,
            field_0x1dc: data[base + 32].extract_hex8()?,
            field_0x1e4: data[base + 33].extract_int()?,
            field_0x1e8: data[base + 34].extract_hex8()?,
            field_0x1f0: data[base + 35].extract_int()?,
            field_0x1f4: data[base + 36].extract_hex8()?,
            field_0x1fc: data[base + 37].extract_int()?,
            field_0x200: data[base + 38].extract_hex8()?,
            field_0x208: data[base + 39].extract_int()?,
            field_0x20c: data[base + 40].extract_hex8()?,
            field_0x214: data[base + 41].extract_int()?,
            field_0x218: data[base + 42].extract_hex8()?,
            field_0x220: data[base + 43].extract_hex8()?,
            field_0x228: data[base + 44].extract_hex8()?,
            field_0x230: data[base + 45].extract_hex8()?,
            field_0x238: data[base + 46].extract_hex8()?,
            field_0x240: data[base + 47].extract_hex8()?,
            field_0x248: data[base + 48].extract_hex8()?,
            field_0x250: data[base + 49].extract_hex8()?,
            field_0x258: data[base + 50].extract_hex8()?,
            field_0x260: data[base + 51].extract_hex8()?,
            field_0x268: data[base + 52].extract_hex8()?,
            field_0x270: data[base + 53].extract_hex8()?,
            field_0x278: data[base + 54].extract_hex8()?,
            field_0x280: data[base + 55].extract_hex8()?,
            field_0x288: data[base + 56].extract_hex8()?,
            field_0x290: data[base + 57].extract_hex8()?,
            field_0x298: data[base + 58].extract_hex8()?,
            field_0x2a0: data[base + 59].extract_hex8()?,
            field_0x2a8: data[base + 60].extract_hex8()?,
            field_0x2b0: data[base + 61].extract_hex8()?,
            field_0x2b8: data[base + 62].extract_hex8()?,
            field_0x2c0: data[base + 63].extract_hex8()?,
            field_0x2c8: data[base + 64].extract_hex8()?,
            field_0x2d0: data[base + 65].extract_hex8()?,
            field_0x2d8: data[base + 66].extract_hex8()?,
            field_0x2dc: data[base + 67].extract_hex8()?,
            field_0x2e0: data[base + 68].extract_hex8()?,
            field_0x2e4: data[base + 69].extract_hex8()?,
            field_0x2e8: data[base + 70].extract_hex8()?,
            field_0x2ec: data[base + 71].extract_hex8()?,
            field_0x2f0: data[base + 72].extract_hex8()?,
            field_0x2f4: data[base + 73].extract_hex8()?,
            field_0x2f8: data[base + 74].extract_hex8()?,
            field_0x2fc: data[base + 75].extract_hex8()?,
            field_0x300: data[base + 76].extract_hex8()?,
            field_0x304: data[base + 77].extract_hex8()?,
            field_0x308: data[base + 78].extract_hex8()?,
            field_0x30c: data[base + 79].extract_hex8()?,
            field_0x310: data[base + 80].extract_hex8()?,
            field_0x318: data[base + 81].extract_hex8()?,
            field_0x320: data[base + 82].extract_hex8()?,
            field_0x328: data[base + 83].extract_hex8()?,
            field_0x330: data[base + 84].extract_hex8()?,
            field_0x338: data[base + 85].extract_int()?,
            field_0x33c: data[base + 86].extract_hex8()?,
            field_0x344: data[base + 87].extract_hex8()?,
            field_0x34c: data[base + 88].extract_hex8()?,
            field_0x354: data[base + 89].extract_hex8()?,
            field_0x35c: data[base + 90].extract_float()?,
            field_0x360: data[base + 91].extract_hex8()?,
            field_0x368: data[base + 92].extract_hex8()?,
            field_0x370: data[base + 93].extract_hex8()?,
            field_0x378: data[base + 94].extract_hex8()?,
            field_0x380: data[base + 95].extract_hex8()?,
            field_0x388: data[base + 96].extract_hex8()?,
            field_0x390: data[base + 97].extract_hex8()?,
            field_0x398: data[base + 98].extract_hex8()?,
            field_0x3a0: data[base + 99].extract_hex8()?,
            field_0x3a8: data[base + 100].extract_int()?,
            field_0x3ac: data[base + 101].extract_hex8()?,
            field_0x3b0: data[base + 102].extract_hex8()?,
            field_0x3b4: data[base + 103].extract_int()?,
            field_0x3b8: data[base + 104].extract_hex8()?,
            field_0x3bc: data[base + 105].extract_hex8()?,
            field_0x3c0: data[base + 106].extract_int()?,
        })
    }
}