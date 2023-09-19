use crate::ParseError;
use alloc::string::{String, ToString};
use core::iter::from_fn;
use std::println;

#[derive(Debug)]
pub enum TkKind<'a> {
    I8(i8),
    U8(u8),
    I16(i16),
    U16(u16),
    U32(u32),
    Hex8(u32),
    F32(f32),
    String(&'a str),
}

pub struct FetmReader<Data: AsRef<[u8]>> {
    input: Data,
}

impl<Data: AsRef<[u8]>> FetmReader<Data> {
    pub fn new(input: Data) -> Result<Self, ParseError> {
        if input.as_ref().len() < 3 {
            return Err(ParseError::UnexpectedEnd);
        }

        let magic = input.as_ref().get(0..3).ok_or(ParseError::UnexpectedEnd)?;

        if magic != [0x01, 0x7C, 0x07] {
            return Err(ParseError::BadMagic);
        }

        if input
            .as_ref()
            .get(input.as_ref().len() - 4..input.as_ref().len())
            .ok_or(ParseError::UnexpectedEnd)?
            != b"end\0"
        {
            return Err(ParseError::UnexpectedEnd);
        }

        Ok(Self { input })
    }

    pub fn tokens(&self) -> impl Iterator<Item = TkKind> + '_ {
        let mut index = 0;
        from_fn(move || {
            let token_kind = self.input.as_ref().get(index)?;

            match token_kind {
                0 => {
                    let data_start = index + 1;
                    index += 2;
                    Some(TkKind::I8(self.input.as_ref()[data_start] as i8))
                }
                1 => {
                    let data_start = index + 1;
                    index += 2;
                    Some(TkKind::U8(self.input.as_ref()[data_start]))
                }
                2 => {
                    let data_start = index + 1;
                    let data_end = data_start + 2;
                    index += 3;
                    Some(TkKind::I16(i16::from_be_bytes(
                        self.input
                            .as_ref()
                            .get(data_start..data_end)
                            .unwrap()
                            .try_into()
                            .unwrap(),
                    )))
                }
                3 => {
                    let data_start = index + 1;
                    let data_end = data_start + 2;
                    index += 3;
                    Some(TkKind::U16(u16::from_be_bytes(
                        self.input
                            .as_ref()
                            .get(data_start..data_end)
                            .unwrap()
                            .try_into()
                            .unwrap(),
                    )))
                }
                4 => {
                    let data_start = index + 1;
                    let data_end = data_start + 4;
                    index += 5;
                    Some(TkKind::U32(u32::from_be_bytes(
                        self.input
                            .as_ref()
                            .get(data_start..data_end)
                            .unwrap()
                            .try_into()
                            .unwrap(),
                    )))
                }
                5 => {
                    let data_start = index + 1;
                    let data_end = data_start + 4;
                    index += 5;
                    Some(TkKind::Hex8(u32::from_be_bytes(
                        self.input
                            .as_ref()
                            .get(data_start..data_end)
                            .unwrap()
                            .try_into()
                            .unwrap(),
                    )))
                }
                6 => {
                    let data_start = index + 1;
                    let data_end = data_start + 4;
                    index += 5;
                    Some(TkKind::F32(f32::from_be_bytes(
                        self.input
                            .as_ref()
                            .get(data_start..data_end)
                            .unwrap()
                            .try_into()
                            .unwrap(),
                    )))
                }
                7 => {
                    let data_start = index + 1;
                    let string = core::ffi::CStr::from_bytes_until_nul(
                        self.input.as_ref().get(data_start..).unwrap(),
                    )
                    .unwrap()
                    .to_str()
                    .unwrap();

                    index += string.len() + 2;

                    Some(TkKind::String(string))
                }
                _ => panic!(),
            }
        })
    }
}

#[derive(Debug)]
pub enum Error {
    InvalidTokenKind,
    CurrentlyNotSupported,
}

impl TkKind<'_> {
    pub fn extract_int(&self) -> Result<usize, Error> {
        match self {
            Self::I8(val) => Ok(*val as usize),
            Self::U8(val) => Ok(*val as usize),
            Self::I16(val) => Ok(*val as usize),
            Self::U16(val) => Ok(*val as usize),
            Self::U32(val) => Ok(*val as usize),
            _ => Err(Error::InvalidTokenKind),
        }
    }

    pub fn extract_float(&self) -> Result<f32, Error> {
        match self {
            Self::F32(val) => Ok(*val),
            _ => Err(Error::InvalidTokenKind),
        }
    }

    pub fn extract_str(&self) -> Result<String, Error> {
        match self {
            Self::String(val) => Ok(val.to_string()),
            _ => Err(Error::InvalidTokenKind),
        }
    }

    pub fn extract_hex8(&self) -> Result<usize, Error> {
        match self {
            Self::Hex8(val) => Ok(*val as usize),
            _ => Err(Error::InvalidTokenKind),
        }
    }
}
#[derive(Debug)]
pub struct World {
    pub flags: u32,
    pub sound_name: String,
    pub action_count: usize,
    pub one_x: f32,
    pub one_y: f32,
    pub one_z: f32,
    pub two_x: f32,
    pub two_y: f32,
    pub two_z: f32,
    o_104: usize,
    o_154: usize,
    o_158: usize,
    o_164: usize,
    o_168: usize,
    o_16c: usize,
    o_170: usize,
    o_174: usize,
    o_a4: f32,
}

impl World {
    pub fn from_tokens(tokens: &[TkKind]) -> Result<Self, Error> {
        let mut world = Self {
            flags: 0,
            sound_name: String::new(),
            action_count: 0,
            o_104: 0,
            o_154: 0,
            o_158: 0,
            o_168: 0,
            o_164: 0,
            o_16c: 0,
            o_170: 0,
            o_174: 0,
            o_a4: 0.,
            one_x: 0.,
            one_y: 0.,
            one_z: 0.,
            two_x: 0.,
            two_y: 0.,
            two_z: 0.,
        };

        let mut val: usize = tokens[0].extract_int()?;

        world.flags = (val as u32 & 1) << 0x12 | world.flags & 0xFFFB_FFFF;

        val = tokens[1].extract_int()?;

        world.flags = (val as u32 & 1) << 0x14 | world.flags & 0xFFFE_FFFF;

        std::println!(
            "Has 0x400000: {}; Has 0x1000: {}",
            world.flags & 0x400000 != 0,
            world.flags & 0x1000 == 0
        );

        val = tokens[2].extract_int()?;

        world.flags = (val as u32) << 0x1F | world.flags & 0x7FFF_FFFF;

        val = tokens[3].extract_int()?;

        world.flags = (val as u32 & 1) << 0x1B | world.flags & 0xF7FF_FFFF;

        world.o_164 = tokens[4].extract_int()?;
        world.o_168 = tokens[5].extract_int()?;

        world.o_16c = tokens[8].extract_int()?;
        world.o_170 = tokens[9].extract_int()?;
        world.o_174 = tokens[10].extract_int()?;

        std::println!(
            "Unknown 1: {}; Unknown 2: {:X}; Unknown 3: {:X}",
            world.o_16c,
            world.o_170 << 0x10,
            world.o_174 << 0x10,
        );

        println!("{:?}", tokens[11]);
        println!("{:?}", tokens[12]);
        println!("{:?}", tokens[13]);

        world.o_a4 = tokens[14].extract_float()?;

        world.o_154 = tokens[15].extract_int()?;
        world.o_158 = tokens[16].extract_int()?;
        println!(
            "Needs Lighting Rig: {:?}",
            tokens[17].extract_int().unwrap() != 0
        );

        world.sound_name = tokens[18].extract_str()?;

        println!("Needs Audio {:?}", tokens[19].extract_int().unwrap() != 0);

        world.action_count = tokens[20].extract_int()?;

        if world.action_count != 0 {
            return Err(Error::CurrentlyNotSupported);
        }

        println!("{}", world.action_count);

        world.one_x = tokens[21].extract_float()?;
        world.one_y = tokens[22].extract_float()?;
        world.one_z = tokens[23].extract_float()?;
        world.two_x = tokens[24].extract_float()?;
        world.two_y = tokens[25].extract_float()?;
        world.two_z = tokens[26].extract_float()?;

        world.o_104 = tokens[27].extract_int()?;

        println!("{:?}", tokens[28]);

        Ok(world)
    }
}

#[derive(Debug)]
pub struct Sector {
    pub normal_sector: NormalSector,
    pub entity_class: String,
    pub has_entity_class: usize,
    pub token_size: usize,
    pub action_list_size: usize,
    pub node_sprite_batch_size: usize,
    pub node_list_size: usize,
    pub music_name: String,
    pub sector_name: String,
    pub music_crc: usize,
    pub blend_r: f32,
    pub blend_g: f32,
    pub blend_b: f32,
    pub blend_a: f32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub one_x: f32,
    pub one_y: f32,
    pub one_z: f32,
    pub two_x: f32,
    pub two_y: f32,
    pub two_z: f32,

    o_c8: usize,
    o_e8: usize,
    o_e0: usize,
    o_d8: usize,
    o_d0: usize,
    o_f8: f32,
    o_f5: usize,
    o_f4: usize,
    o_f0: usize,
    o_114: usize,
    o_10c: usize,
    o_104: usize,
    o_fc: usize,
    o_11c: usize,
    o_15c: usize,
    o_160: f32,
    o_164: usize,
    o_165: usize,
    o_166: usize,
    o_167: usize,
    o_168: usize,
    o_16a: usize,
    o_178: usize,
    o_17c: usize,
    o_184: usize,
    o_188: usize,
    o_18c: usize,
    o_190: usize,
    o_194: usize,
    o_170: usize,
    o_16c: usize,
    o_bc: usize,
    o_bb: usize,
    o_ba: usize,
    o_b9: usize,
    o_b8: usize,
    flags: usize,
    o_bd: usize,
    o_be: usize,
    o_bf: usize,
    o_c0: usize,
    o_c1: usize,
}

#[derive(Debug)]
pub struct NormalSector {}

impl NormalSector {
    const LENGTH: usize = 62;

    pub fn from_tokens(_tokens: &[TkKind; Self::LENGTH]) -> Self {
        Self {}
    }
}

impl Sector {
    pub fn from_tokens(tokens: &[TkKind]) -> Result<Self, Error> {
        let mut sector = Sector {
            normal_sector: NormalSector {},
            entity_class: String::new(),
            has_entity_class: 0,
            token_size: 0,
            action_list_size: 0,
            music_crc: 0,
            music_name: String::new(),
            sector_name: String::new(),
            flags: 0usize,
            o_b8: 0usize,
            o_b9: 0usize,
            o_ba: 0usize,
            o_bb: 0usize,
            o_bc: 0usize,
            o_bd: 0usize,
            o_be: 0usize,
            o_bf: 0usize,
            o_c0: 0usize,
            o_c1: 0usize,
            o_c8: 0,
            o_d0: 0,
            o_d8: 0,
            o_e0: 0,
            o_e8: 0,
            o_f0: 0,
            o_f4: 0,
            o_f5: 0,
            o_f8: 0.,
            o_fc: 0,
            o_104: 0,
            o_10c: 0,
            o_114: 0,
            o_11c: 0,
            o_15c: 0,
            o_160: 0.,
            blend_r: 0.,
            blend_g: 0.,
            blend_b: 0.,
            blend_a: 0.,
            o_164: 0usize,
            o_165: 0usize,
            o_166: 0usize,
            o_167: 0usize,
            o_168: 0usize,
            o_16a: 0usize,
            o_16c: 0usize,
            o_170: 0usize,
            o_178: 0usize,
            o_17c: 0usize,
            o_184: 0usize,
            o_188: 0usize,
            o_18c: 0usize,
            o_190: 0usize,
            o_194: 0usize,
            x: 0.,
            y: 0.,
            z: 0.,
            one_x: 0.,
            one_y: 0.,
            one_z: 0.,
            two_x: 0.,
            two_y: 0.,
            two_z: 0.,
            node_sprite_batch_size: 0,
            node_list_size: 0,
        };

        println!("{:?}", tokens[0]);
        sector.entity_class = tokens[1].extract_str()?;
        sector.has_entity_class = tokens[2].extract_int()?;

        //UNKNOWN
        println!("{:?}", tokens[3]);
        println!("{:?}", tokens[4]);

        sector.token_size = tokens[5].extract_int()?;

        //CECNormalSector::Parse
        sector.normal_sector =
            NormalSector::from_tokens(tokens[6..6 + NormalSector::LENGTH].try_into().unwrap());

        let curr_pos = 6 + NormalSector::LENGTH;

        sector.o_c8 = tokens[curr_pos].extract_hex8()?;
        sector.o_d0 = tokens[curr_pos + 1].extract_hex8()?;
        sector.o_d8 = tokens[curr_pos + 2].extract_hex8()?;
        sector.o_e0 = tokens[curr_pos + 3].extract_hex8()?;
        sector.o_e8 = tokens[curr_pos + 4].extract_hex8()?;

        sector.o_f0 = tokens[curr_pos + 5].extract_int()?;
        sector.o_f4 = tokens[curr_pos + 6].extract_int()?;
        sector.o_f5 = tokens[curr_pos + 7].extract_int()?;

        sector.o_f8 = tokens[curr_pos + 8].extract_float()?;

        sector.o_fc = tokens[curr_pos + 9].extract_hex8()?;
        sector.o_104 = tokens[curr_pos + 10].extract_hex8()?;
        sector.o_10c = tokens[curr_pos + 11].extract_hex8()?;
        sector.o_114 = tokens[curr_pos + 12].extract_hex8()?;
        sector.o_11c = tokens[curr_pos + 13].extract_hex8()?;

        sector.action_list_size = tokens[curr_pos + 14].extract_int()?;
        println!("{}", sector.action_list_size);

        sector.music_name = tokens[curr_pos + 15].extract_str()?;
        sector.music_crc = tokens[curr_pos + 16].extract_hex8()?;

        sector.blend_r = tokens[curr_pos + 17].extract_float()?;
        sector.blend_g = tokens[curr_pos + 18].extract_float()?;
        sector.blend_b = tokens[curr_pos + 19].extract_float()?;
        sector.blend_a = tokens[curr_pos + 20].extract_float()?;

        sector.o_15c = tokens[curr_pos + 21].extract_int()?;

        sector.o_160 = tokens[curr_pos + 22].extract_float()?;
        sector.o_164 = tokens[curr_pos + 23].extract_int()?;
        sector.o_165 = tokens[curr_pos + 24].extract_int()?;
        sector.o_166 = tokens[curr_pos + 25].extract_int()?;
        sector.o_167 = tokens[curr_pos + 26].extract_int()?;
        sector.o_168 = tokens[curr_pos + 27].extract_int()?;
        sector.o_16a = tokens[curr_pos + 28].extract_int()?;
        sector.o_16c = tokens[curr_pos + 29].extract_int()?;

        sector.o_170 = tokens[curr_pos + 30].extract_hex8()?;

        sector.o_178 = tokens[curr_pos + 31].extract_int()?;

        sector.o_17c = tokens[curr_pos + 32].extract_hex8()?;

        sector.o_184 = tokens[curr_pos + 33].extract_int()?;
        sector.o_188 = tokens[curr_pos + 34].extract_int()?;
        sector.o_18c = tokens[curr_pos + 35].extract_int()?;
        sector.o_190 = tokens[curr_pos + 36].extract_int()?;

        sector.o_194 = tokens[curr_pos + 37].extract_hex8()?;

        //END OF WorldSector::ParseError
        let curr_pos = curr_pos + 38;

        sector.sector_name = tokens[curr_pos].extract_str()?;

        sector.o_b8 = tokens[curr_pos + 1].extract_int()?;
        sector.o_b9 = tokens[curr_pos + 2].extract_int()?;
        sector.o_ba = tokens[curr_pos + 3].extract_int()?;

        let mut val = tokens[curr_pos + 4].extract_int()?;
        sector.flags = (val & 1) << 0x1e | sector.flags & 0xBFFF_FFFF;
        val = tokens[curr_pos + 5].extract_int()?;
        sector.flags = (val & 1) << 0x1d | sector.flags & 0xDFFF_FFFF;

        sector.o_bb = tokens[curr_pos + 6].extract_int()?;
        sector.o_bc = tokens[curr_pos + 7].extract_int()?;
        sector.o_bd = tokens[curr_pos + 8].extract_int()?;

        sector.x = tokens[curr_pos + 9].extract_float()?;
        sector.y = tokens[curr_pos + 10].extract_float()?;
        sector.z = tokens[curr_pos + 11].extract_float()?;

        val = tokens[curr_pos + 12].extract_int()?;
        sector.flags = (val & 1) << 0x1f | sector.flags & 0xFFFF_FFFF;

        sector.o_be = tokens[curr_pos + 13].extract_int()?;
        sector.o_bf = tokens[curr_pos + 14].extract_int()?;
        sector.o_c0 = tokens[curr_pos + 15].extract_int()?;
        sector.o_c1 = tokens[curr_pos + 16].extract_int()?;

        val = tokens[curr_pos + 17].extract_int()?;
        sector.flags = (val & 1) << 0x1b | sector.flags & 0xF7FF_FFFF;

        val = tokens[curr_pos + 18].extract_int()?;
        sector.flags = (val & 1) << 0x1a | sector.flags & 0xFBFF_FFFF;

        val = tokens[curr_pos + 19].extract_int()?;
        sector.flags = (val & 1) << 0x17 | sector.flags & 0xFF7F_FFFF;

        val = tokens[curr_pos + 20].extract_int()?;
        sector.flags = (val & 1) << 0x16 | sector.flags & 0xFFBF_FFFF;

        val = tokens[curr_pos + 21].extract_int()?;
        sector.flags = (val & 1) << 0x15 | sector.flags & 0xFFDF_FFFF;

        val = tokens[curr_pos + 22].extract_int()?;
        sector.flags = (val & 1) << 0x13 | sector.flags & 0xFFF7_FFFF;

        val = tokens[curr_pos + 23].extract_int()?;
        sector.flags = (val & 1) << 0x14 | sector.flags & 0xFFFE_FFFF;

        val = tokens[curr_pos + 24].extract_int()?;
        sector.flags = (val & 1) << 0x11 | sector.flags & 0xFFFD_FFFF;

        val = tokens[curr_pos + 25].extract_int()?;
        sector.flags = (val & 1) << 0x12 | sector.flags & 0xFFFB_FFFF;

        val = tokens[curr_pos + 26].extract_int()?;
        sector.flags = (val & 1) << 0x10 | sector.flags & 0xFFFE_FFFF;

        val = tokens[curr_pos + 26].extract_int()?;
        sector.flags = (val & 1) << 0x10 | sector.flags & 0xFFFE_FFFF;

        if val & 1 != 0 {
            println!("{:?}", tokens[curr_pos + 27]);
        }

        sector.one_x = tokens[curr_pos + 27].extract_float()?;
        sector.one_y = tokens[curr_pos + 28].extract_float()?;
        sector.one_z = tokens[curr_pos + 29].extract_float()?;
        sector.two_x = tokens[curr_pos + 30].extract_float()?;
        sector.two_y = tokens[curr_pos + 31].extract_float()?;
        sector.two_z = tokens[curr_pos + 32].extract_float()?;

        sector.node_sprite_batch_size = tokens[curr_pos + 33].extract_int()?;
        sector.node_list_size = tokens[curr_pos + 34].extract_int()?;

        println!("{:?}", tokens[curr_pos + 35]);
        println!("Node Name: {:?}", tokens[curr_pos + 36]);
        println!("Node Type: {:?}", tokens[curr_pos + 37]);
        println!("Entity Class: {:?}", tokens[curr_pos + 38]);
        println!("{:?}", tokens[curr_pos + 39]);
        println!("{:?}", tokens[curr_pos + 40]);
        println!("Node Name: {:?}", tokens[curr_pos + 41]);
        println!("Node Type: {:?}", tokens[curr_pos + 42]);
        println!("Entity Class: {:?}", tokens[curr_pos + 43]);
        println!("{:?}", tokens[curr_pos + 44]);
        Ok(sector)
    }
}
