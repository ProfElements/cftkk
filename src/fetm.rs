use crate::{fetm::entityclasses::main_character::MainCharacter as Character, ParseError};
use core::iter::from_fn;

use self::nodes::simulation_object::SimulationObject as Node2;

mod entityclasses;
mod nodes;
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
    ParseError(ParseError),
}

impl TkKind<'_> {
    pub fn extract_int(&self) -> Result<usize, Error> {
        match self {
            Self::I8(val) => Ok(*val as usize),
            Self::U8(val) => Ok(*val as usize),
            Self::I16(val) => Ok(*val as usize),
            Self::U16(val) => Ok(*val as usize),
            Self::U32(val) => Ok(*val as usize),
            val => {
                std::println!("{val:?}");
                Err(Error::InvalidTokenKind)
            }
        }
    }

    pub fn extract_float(&self) -> Result<f32, Error> {
        match self {
            Self::F32(val) => Ok(*val),
            val => {
                std::println!("{val:?}");
                Err(Error::InvalidTokenKind)
            }
        }
    }

    pub fn extract_str(&self) -> Result<&str, Error> {
        match self {
            Self::String(val) => Ok(val),
            val => {
                std::println!("{val:?}");
                Err(Error::InvalidTokenKind)
            }
        }
    }

    pub fn extract_hex8(&self) -> Result<usize, Error> {
        match self {
            Self::Hex8(val) => Ok(*val as usize),
            val => {
                std::println!("{val:?}");
                Err(Error::InvalidTokenKind)
            }
        }
    }
}
#[derive(Debug)]
pub struct World<'a> {
    pub flags: u32,
    pub sound_name: &'a str,
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
    pub simulation_scale: f32,
}

impl<'a> World<'a> {
    pub const LENGTH: usize = 28;
    pub fn from_tokens(tokens: &'a [TkKind]) -> Result<Self, Error> {
        let mut world = Self {
            flags: 0,
            sound_name: "",
            action_count: 0,
            o_104: 0,
            o_154: 0,
            o_158: 0,
            o_168: 0,
            o_164: 0,
            o_16c: 0,
            o_170: 0,
            o_174: 0,
            simulation_scale: tokens[14].extract_float()?,
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

        val = tokens[2].extract_int()?;

        world.flags = (val as u32) << 0x1F | world.flags & 0x7FFF_FFFF;

        val = tokens[3].extract_int()?;

        world.flags = (val as u32 & 1) << 0x1B | world.flags & 0xF7FF_FFFF;

        world.o_164 = tokens[4].extract_int()?;
        world.o_168 = tokens[5].extract_int()?;

        world.o_16c = tokens[8].extract_int()?;
        world.o_170 = tokens[9].extract_int()?;
        world.o_174 = tokens[10].extract_int()?;

        world.o_154 = tokens[15].extract_int()?;
        world.o_158 = tokens[16].extract_int()?;

        world.sound_name = tokens[18].extract_str()?;

        world.action_count = tokens[20].extract_int()?;

        if world.action_count != 0 {
            return Err(Error::CurrentlyNotSupported);
        }

        world.one_x = tokens[21].extract_float()?;
        world.one_y = tokens[22].extract_float()?;
        world.one_z = tokens[23].extract_float()?;
        world.two_x = tokens[24].extract_float()?;
        world.two_y = tokens[25].extract_float()?;
        world.two_z = tokens[26].extract_float()?;

        world.o_104 = tokens[27].extract_int()?;

        Ok(world)
    }
}

#[derive(Debug)]
pub struct Sector<'a> {
    pub name: &'a str,
    pub entity_class: EntityClass<'a>,
    pub node_sprite_batch_size: usize,
    pub first_node_offset: usize,
    pub node_list_size: usize,
    pub sector_name: &'a str,
    pub one_x: f32,
    pub one_y: f32,
    pub one_z: f32,
    pub two_x: f32,
    pub two_y: f32,
    pub two_z: f32,
    pub unknown_0: usize,
    pub unknown_1: usize,
    pub unknown_2: usize,
    pub flags: usize,
    pub flip_a: usize,
    pub flip_b: usize,
    pub flip_g: usize,
    pub flip_r: usize,
    pub fog_intensity: f32,
    pub fog_far_dist: f32,
    pub fog_near_dist: f32,
    pub fog_b: usize,
    pub fog_g: usize,
    pub fog_r: usize,
}

impl<'a> Sector<'a> {
    pub const LENGTH: usize = 142;
    pub fn from_tokens(tokens: &'a [TkKind]) -> Result<Self, Error> {
        let mut sector = Sector {
            name: tokens[0].extract_str()?,
            entity_class: EntityClass::from_tokens(&tokens[1..])?,
            sector_name: "",
            flags: 0usize,
            unknown_0: 0usize,
            unknown_1: 0usize,
            unknown_2: 0usize,
            one_x: 0.,
            one_y: 0.,
            one_z: 0.,
            two_x: 0.,
            two_y: 0.,
            two_z: 0.,
            node_sprite_batch_size: 0,
            node_list_size: 0,
            first_node_offset: 0,
            fog_r: 0usize,
            fog_g: 0usize,
            fog_b: 0usize,
            fog_near_dist: 0f32,
            fog_far_dist: 0f32,
            fog_intensity: 0f32,
            flip_r: 0usize,
            flip_g: 0usize,
            flip_b: 0usize,
            flip_a: 0usize,
        };

        sector.entity_class = EntityClass::from_tokens(&tokens[1..])?;
        let curr_pos = 6 + sector.entity_class.header.class_size;

        sector.sector_name = tokens[curr_pos].extract_str()?;

        sector.unknown_0 = tokens[curr_pos + 1].extract_int()?;
        sector.unknown_1 = tokens[curr_pos + 2].extract_int()?;
        sector.unknown_2 = tokens[curr_pos + 3].extract_int()?;

        let mut val = tokens[curr_pos + 4].extract_int()?;
        sector.flags = (val & 1) << 0x1e | sector.flags & 0xBFFF_FFFF;
        val = tokens[curr_pos + 5].extract_int()?;
        sector.flags = (val & 1) << 0x1d | sector.flags & 0xDFFF_FFFF;

        sector.fog_r = tokens[curr_pos + 6].extract_int()?;
        sector.fog_g = tokens[curr_pos + 7].extract_int()?;
        sector.fog_b = tokens[curr_pos + 8].extract_int()?;

        sector.fog_near_dist = tokens[curr_pos + 9].extract_float()?;
        sector.fog_far_dist = tokens[curr_pos + 10].extract_float()?;
        sector.fog_intensity = tokens[curr_pos + 11].extract_float()?;

        val = tokens[curr_pos + 12].extract_int()?;
        sector.flags = (val & 1) << 0x1f | sector.flags & 0xFFFF_FFFF;

        sector.flip_r = tokens[curr_pos + 13].extract_int()?;
        sector.flip_g = tokens[curr_pos + 14].extract_int()?;
        sector.flip_b = tokens[curr_pos + 15].extract_int()?;
        sector.flip_a = tokens[curr_pos + 16].extract_int()?;

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

        sector.one_x = tokens[curr_pos + 27].extract_float()?;
        sector.one_y = tokens[curr_pos + 28].extract_float()?;
        sector.one_z = tokens[curr_pos + 29].extract_float()?;
        sector.two_x = tokens[curr_pos + 30].extract_float()?;
        sector.two_y = tokens[curr_pos + 31].extract_float()?;
        sector.two_z = tokens[curr_pos + 32].extract_float()?;

        sector.node_sprite_batch_size = tokens[curr_pos + 33].extract_int()?;
        sector.first_node_offset = tokens[curr_pos + 34].extract_int()?;
        sector.node_list_size = tokens[curr_pos + 35].extract_int()?;

        Ok(sector)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct EntityClassHeader<'a> {
    pub name: &'a str,
    pub has_entity_class: bool,
    pub class_size: usize,
}

impl<'a> EntityClassHeader<'a> {
    const LENGTH: usize = 5;
    pub fn from_tokens(tokens: &'a [TkKind; EntityClassHeader::LENGTH]) -> Result<Self, Error> {
        if tokens[0].extract_str()?.as_bytes() == b"<noentclass>" {
            return Ok(Self {
                name: tokens[0].extract_str()?,
                has_entity_class: false,
                class_size: 0,
            });
        }
        Ok(Self {
            name: tokens[0].extract_str()?,
            has_entity_class: tokens[1].extract_int()? != 0,
            class_size: tokens[4].extract_int()?,
        })
    }
}
#[derive(Debug)]
pub struct EntityClass<'a> {
    pub header: EntityClassHeader<'a>,
    pub class: EntityKlass<'a>,
}

impl<'a> EntityClass<'a> {
    pub fn from_tokens(tokens: &'a [TkKind]) -> Result<Self, Error> {
        let header_tokens = tokens
            .get(0..EntityClassHeader::LENGTH)
            .ok_or(Error::ParseError(ParseError::UnexpectedEnd))?;
        let header = EntityClassHeader::from_tokens(header_tokens.try_into().unwrap())?;

        let class_tokens = tokens
            .get(EntityClassHeader::LENGTH..EntityClassHeader::LENGTH + header.class_size)
            .ok_or(Error::ParseError(ParseError::UnexpectedEnd))?;

        let class = EntityKlass::from_header(header, class_tokens)?;

        Ok(Self { header, class })
    }
}

#[derive(Debug, PartialEq)]
pub enum EntityKlass<'a> {
    WorldSector(WorldSector<'a>),
    MainCharacter(Character),
    DynamicColllsionNode,
    Empty,
}

impl<'a> EntityKlass<'a> {
    pub fn from_header(header: EntityClassHeader, tokens: &'a [TkKind]) -> Result<Self, Error> {
        match header.name {
            "World Sector" => Ok(Self::WorldSector(WorldSector::from_tokens(tokens)?)),
            "Dynamic Collision Node" => Ok(Self::DynamicColllsionNode),
            "Main Character" => Ok(Self::MainCharacter(Character::from_tokens(tokens)?)),
            "" => Ok(Self::Empty),
            "<noentclass>" => Ok(Self::Empty),
            _ => {
                std::println!("{} isn't supported right now", header.name);
                Ok(Self::Empty)
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct WorldSector<'a> {
    pub normal_sector: NormalSector,
    pub node_crc_0: usize,
    pub node_crc_1: usize,
    pub node_crc_2: usize,
    pub node_crc_3: usize,
    pub node_crc_4: usize,
    pub blend_value_0: usize,
    pub blend_value_1: usize,
    pub blend_value_2: usize,
    pub blend_value_3: f32,
    pub node_crc_5: usize,
    pub node_crc_6: usize,
    pub node_crc_7: usize,
    pub node_crc_8: usize,
    pub node_crc_9: usize,
    pub action_count: usize,
    pub music_name: &'a str,
    pub node_crc_10: usize,
    pub unknown_3: usize,
    pub unknown_2: usize,
    pub unknown_1: usize,
    pub string_table_loc_1: usize,
    pub string_table_crc_1: usize,
    pub music_crc: usize,
    pub blend_value_4: f32,
    pub blend_value_5: f32,
    pub blend_value_6: f32,
    pub blend_value_7: f32,
    pub unknown_0: usize,
    pub blend_value_8: f32,
    pub blend_value_9: usize,
    pub blend_value_10: usize,
    pub blend_value_11: usize,
    pub blend_value_12: usize,
    pub blend_value_13: usize,
    pub blend_value_14: usize,
    pub blend_value_15: usize,
    pub string_table_crc_0: usize,
    pub string_table_loc_0: usize,
}

impl<'a> WorldSector<'a> {
    pub const LENGTH: usize = 100;
    pub fn from_tokens(tokens: &'a [TkKind]) -> Result<Self, Error> {
        let normal_sector = NormalSector::from_tokens(&tokens)?;

        let node_crc_0 = tokens[62].extract_hex8()?;
        let node_crc_1 = tokens[63].extract_hex8()?;
        let node_crc_2 = tokens[64].extract_hex8()?;
        let node_crc_3 = tokens[65].extract_hex8()?;
        let node_crc_4 = tokens[66].extract_hex8()?;

        let blend_value_0 = tokens[67].extract_int()?;
        let blend_value_1 = tokens[68].extract_int()?;
        let blend_value_2 = tokens[69].extract_int()?;
        let blend_value_3 = tokens[70].extract_float()?;

        let node_crc_5 = tokens[71].extract_hex8()?;
        let node_crc_6 = tokens[72].extract_hex8()?;
        let node_crc_7 = tokens[73].extract_hex8()?;
        let node_crc_8 = tokens[74].extract_hex8()?;
        let node_crc_9 = tokens[75].extract_hex8()?;

        let action_count = tokens[76].extract_int()?;

        let music_name = tokens[77].extract_str()?;
        let music_crc = tokens[78].extract_hex8()?;

        let blend_value_4 = tokens[79].extract_float()?;
        let blend_value_5 = tokens[80].extract_float()?;
        let blend_value_6 = tokens[81].extract_float()?;
        let blend_value_7 = tokens[82].extract_float()?;

        let unknown_0 = tokens[83].extract_int()?;

        let blend_value_8 = tokens[84].extract_float()?;
        let blend_value_9 = tokens[85].extract_int()?;
        let blend_value_10 = tokens[86].extract_int()?;
        let blend_value_11 = tokens[87].extract_int()?;
        let blend_value_12 = tokens[88].extract_int()?;
        let blend_value_13 = tokens[89].extract_int()?;
        let blend_value_14 = tokens[90].extract_int()?;
        let blend_value_15 = tokens[91].extract_int()?;

        let string_table_crc_0 = tokens[92].extract_hex8()?;
        let string_table_loc_0 = tokens[93].extract_int()?;

        let string_table_crc_1 = tokens[94].extract_hex8()?;
        let string_table_loc_1 = tokens[95].extract_int()?;

        let unknown_1 = tokens[96].extract_int()?;
        let unknown_2 = tokens[97].extract_int()?;
        let unknown_3 = tokens[98].extract_int()?;

        let node_crc_10 = tokens[99].extract_hex8()?;

        Ok(Self {
            normal_sector,
            node_crc_0,
            node_crc_1,
            node_crc_2,
            node_crc_3,
            node_crc_4,
            blend_value_0,
            blend_value_1,
            blend_value_2,
            blend_value_3,
            node_crc_5,
            node_crc_6,
            node_crc_7,
            node_crc_8,
            node_crc_9,
            action_count,
            music_name,
            music_crc,
            blend_value_4,
            blend_value_5,
            blend_value_6,
            blend_value_7,
            unknown_0,
            blend_value_8,
            blend_value_9,
            blend_value_10,
            blend_value_11,
            blend_value_12,
            blend_value_13,
            blend_value_14,
            blend_value_15,
            string_table_crc_0,
            string_table_loc_0,
            string_table_crc_1,
            string_table_loc_1,
            unknown_1,
            unknown_2,
            unknown_3,
            node_crc_10,
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct NormalSector {
    pub texture_resource_crc: usize,
    pub texture_resource_deploy_location: usize,
    pub blend_value_0: usize,
    pub blend_value_1: f32,
    pub blend_value_2: f32,
    pub blend_value_3: f32,
    pub blend_value_4: f32,
    pub blend_value_5: f32,
    pub blend_value_6: usize,
    pub blend_value_7: f32,
    pub blend_value_8: f32,
    pub blend_value_9: f32,
    pub blend_value_10: f32,
    pub blend_value_11: f32,
    pub blend_value_12: f32,
    pub blend_value_13: f32,
    pub blend_value_14: f32,
    pub blend_value_15: f32,
    pub blend_value_16: f32,
    pub enable_excitement: bool,
    pub blend_value_17: f32,
    pub blend_value_18: f32,
    pub blend_value_19: f32,
    pub blend_value_20: f32,
    pub blend_value_21: f32,
    pub blend_value_22: f32,
    pub blend_value_23: f32,
    pub blend_value_24: f32,
    pub blend_value_25: f32,
    pub blend_value_26: f32,
    pub music_excite_1: usize,
    pub music_excite_2: usize,
    pub blend_value_27: usize,
    pub blend_value_28: usize,
    pub blend_value_29: usize,
    pub blend_value_30: usize,
    pub blend_value_31: usize,
    pub blend_value_32: usize,
    pub blend_value_33: usize,
    pub blend_value_34: usize,
    pub blend_value_35: usize,
    pub blend_value_36: usize,
    pub unknown_0: usize,
    pub blend_value_37: usize,
    pub blend_value_38: usize,
    pub blend_value_39: usize,
    pub blend_value_40: usize,
    pub unknown_1: usize,
    pub blend_value_41: usize,
    pub blend_value_42: usize,
    pub blend_value_43: usize,
    pub blend_value_44: usize,
    pub unknown_2: usize,
    pub blend_value_45: usize,
    pub blend_value_46: usize,
    pub blend_value_47: usize,
    pub blend_value_48: usize,
    pub unknown_3: usize,
    pub blend_value_49: usize,
    pub blend_value_50: usize,
    pub blend_value_51: usize,
    pub blend_value_52: usize,
}

impl NormalSector {
    pub const LENGTH: usize = 62;
    pub fn from_tokens(tokens: &[TkKind]) -> Result<Self, Error> {
        let texture_resource_crc = tokens[0].extract_hex8()?;
        let texture_resource_deploy_location = tokens[1].extract_int()?;

        let blend_value_0 = tokens[2].extract_int()?;

        let blend_value_1 = tokens[3].extract_float()?;
        let blend_value_2 = tokens[4].extract_float()?;
        let blend_value_3 = tokens[5].extract_float()?;
        let blend_value_4 = tokens[6].extract_float()?;
        let blend_value_5 = tokens[7].extract_float()?;

        let blend_value_6 = tokens[8].extract_int()?;

        let blend_value_7 = tokens[9].extract_float()?;
        let blend_value_8 = tokens[10].extract_float()?;
        let blend_value_9 = tokens[11].extract_float()?;
        let blend_value_10 = tokens[12].extract_float()?;
        let blend_value_11 = tokens[13].extract_float()?;
        let blend_value_12 = tokens[14].extract_float()?;
        let blend_value_13 = tokens[15].extract_float()?;
        let blend_value_14 = tokens[16].extract_float()?;
        let blend_value_15 = tokens[17].extract_float()?;

        let enable_excitement = tokens[18].extract_int()? != 0;

        let blend_value_16 = tokens[19].extract_float()?;
        let blend_value_17 = tokens[20].extract_float()?;
        let blend_value_18 = tokens[21].extract_float()?;
        let blend_value_19 = tokens[22].extract_float()?;
        let blend_value_20 = tokens[23].extract_float()?;
        let blend_value_21 = tokens[24].extract_float()?;
        let blend_value_22 = tokens[25].extract_float()?;
        let blend_value_23 = tokens[26].extract_float()?;
        let blend_value_24 = tokens[27].extract_float()?;
        let blend_value_25 = tokens[28].extract_float()?;
        let blend_value_26 = tokens[29].extract_float()?;

        let music_excite_1 = tokens[30].extract_int()?;
        let music_excite_2 = tokens[31].extract_int()?;

        let blend_value_27 = tokens[32].extract_int()?;
        let blend_value_28 = tokens[33].extract_int()?;
        let blend_value_29 = tokens[34].extract_int()?;
        let blend_value_30 = tokens[35].extract_int()?;
        let blend_value_31 = tokens[36].extract_int()?;
        let blend_value_32 = tokens[37].extract_int()?;
        let blend_value_33 = tokens[38].extract_int()?;
        let blend_value_34 = tokens[39].extract_int()?;
        let blend_value_35 = tokens[40].extract_int()?;
        let blend_value_36 = tokens[41].extract_int()?;

        let unknown_0 = tokens[42].extract_int()?;

        let blend_value_37 = tokens[43].extract_int()?;
        let blend_value_38 = tokens[44].extract_int()?;
        let blend_value_39 = tokens[45].extract_int()?;
        let blend_value_40 = tokens[46].extract_int()?;

        let unknown_1 = tokens[47].extract_int()?;

        let blend_value_41 = tokens[48].extract_int()?;
        let blend_value_42 = tokens[49].extract_int()?;
        let blend_value_43 = tokens[50].extract_int()?;
        let blend_value_44 = tokens[51].extract_int()?;

        let unknown_2 = tokens[52].extract_int()?;

        let blend_value_45 = tokens[53].extract_int()?;
        let blend_value_46 = tokens[54].extract_int()?;
        let blend_value_47 = tokens[55].extract_int()?;
        let blend_value_48 = tokens[56].extract_int()?;

        let unknown_3 = tokens[57].extract_int()?;

        let blend_value_49 = tokens[58].extract_int()?;
        let blend_value_50 = tokens[59].extract_int()?;
        let blend_value_51 = tokens[60].extract_int()?;
        let blend_value_52 = tokens[61].extract_int()?;

        Ok(Self {
            texture_resource_crc,
            texture_resource_deploy_location,
            blend_value_0,
            blend_value_1,
            blend_value_2,
            blend_value_3,
            blend_value_4,
            blend_value_5,
            blend_value_6,
            blend_value_7,
            blend_value_8,
            blend_value_9,
            blend_value_10,
            blend_value_11,
            blend_value_12,
            blend_value_13,
            blend_value_14,
            blend_value_15,
            blend_value_16,
            enable_excitement,
            blend_value_17,
            blend_value_18,
            blend_value_19,
            blend_value_20,
            blend_value_21,
            blend_value_22,
            blend_value_23,
            blend_value_24,
            blend_value_25,
            blend_value_26,
            music_excite_1,
            music_excite_2,
            blend_value_27,
            blend_value_28,
            blend_value_29,
            blend_value_30,
            blend_value_31,
            blend_value_32,
            blend_value_33,
            blend_value_34,
            blend_value_35,
            blend_value_36,
            unknown_0,
            blend_value_37,
            blend_value_38,
            blend_value_39,
            blend_value_40,
            unknown_1,
            blend_value_41,
            blend_value_42,
            blend_value_43,
            blend_value_44,
            unknown_2,
            blend_value_45,
            blend_value_46,
            blend_value_47,
            blend_value_48,
            unknown_3,
            blend_value_49,
            blend_value_50,
            blend_value_51,
            blend_value_52,
        })
    }
}

#[derive(Debug)]
pub struct WorldNode<'a> {
    pub node_type: &'a str,
    pub node_name: &'a str,
    pub entity_class: EntityClass<'a>,
    pub node: Node<'a>,
}

impl<'a> WorldNode<'a> {
    pub fn from_tokens(tokens: &'a [TkKind]) -> Result<Self, Error> {
        let node_tokens = if EntityClass::from_tokens(&tokens[2..])?.class != EntityKlass::Empty {
            3 + EntityClassHeader::LENGTH
                + EntityClass::from_tokens(&tokens[2..])?.header.class_size
        } else {
            4
        };

        Ok(WorldNode {
            node_type: tokens[0].extract_str()?,
            node_name: tokens[1].extract_str()?,
            entity_class: EntityClass::from_tokens(&tokens[2..])?,
            node: Node::from_name(tokens[0].extract_str()?, &tokens[node_tokens..])?,
        })
    }
}

#[derive(Debug)]
pub enum Node<'a> {
    Simulation(NodeSimulation),
    Collision(NodeCollision),
    SimulationObject(Node2<'a>),
    Dummy,
}

impl<'a> Node<'a> {
    pub fn from_name(name: &str, tokens: &'a [TkKind]) -> Result<Self, Error> {
        match name {
            "simulation" => Ok(Node::Simulation(NodeSimulation::from_sectors(tokens)?)),
            "collision_node" => Ok(Node::Collision(NodeCollision::from_tokens(tokens)?)),
            "dummy" => Ok(Self::Dummy),
            "simulation_object" => Ok(Node::SimulationObject(Node2::from_tokens(tokens)?)),
            _ => {
                std::println!("{} is not supported", name);
                Ok(Self::Dummy)
            }
        }
    }
}

#[derive(Debug)]
pub struct NodeSimulation {
    pub base_node: BaseNode,
    pub gravity_x: f32,
    pub gravity_y: f32,
    pub gravity_z: f32,
    pub sim_0x140: f32,
    pub minimum_motion: f32,
    pub update_rate: f32,
    pub method_type: usize,
    pub unknown_sim_count_0: usize,
    pub unknown_sim_type: usize,
    pub unknown_flags_0: usize,
    pub unknown_flags_1: usize,
    pub unknown_flags_2: usize,
    pub unknown_sim_count_1: usize,
    pub unknown_sim_count_2: usize,
    pub max_points_per_contact: usize,
    pub unknown_float_0: f32,
    pub sim_0xfc: f32,
    pub max_angular_velocity: f32,
    pub unknown_float_1: f32,
    pub constant_spring_force: f32,
    pub min_0: f32,
    pub min_1: f32,
    pub min_2: f32,
    pub min_3: f32,
    pub unknown_flags_3: usize,
    pub unknown_flags_4: usize,
    pub unknown_float_2: f32,
    pub max_dist: f32,
    pub neg_dist: f32,
    pub unknown_int_0: usize,
    pub unknown_int_1: usize,
    pub unknown_count_0: usize,
    pub unknown_count_1: usize,
}

impl NodeSimulation {
    pub const LENGTH: usize = BaseNode::LENGTH + 32;
    pub fn from_sectors(tokens: &[TkKind]) -> Result<Self, Error> {
        let sim = NodeSimulation {
            base_node: BaseNode::from_tokens(&tokens)?,
            gravity_x: tokens[BaseNode::LENGTH].extract_float()?,
            gravity_y: tokens[BaseNode::LENGTH + 1].extract_float()?,
            gravity_z: tokens[BaseNode::LENGTH + 2].extract_float()?,
            sim_0x140: tokens[BaseNode::LENGTH + 3].extract_float()?,
            minimum_motion: tokens[BaseNode::LENGTH + 4].extract_float()?,
            update_rate: tokens[BaseNode::LENGTH + 5].extract_float()?,
            method_type: tokens[BaseNode::LENGTH + 6].extract_int()?,
            unknown_sim_count_0: tokens[BaseNode::LENGTH + 7].extract_int()?,
            unknown_sim_type: tokens[BaseNode::LENGTH + 8].extract_int()?,
            unknown_flags_0: tokens[BaseNode::LENGTH + 9].extract_int()?,
            unknown_flags_1: tokens[BaseNode::LENGTH + 10].extract_int()?,
            unknown_flags_2: tokens[BaseNode::LENGTH + 11].extract_int()?,
            unknown_sim_count_1: tokens[BaseNode::LENGTH + 12].extract_int()?,
            unknown_sim_count_2: tokens[BaseNode::LENGTH + 13].extract_int()?,
            max_points_per_contact: tokens[BaseNode::LENGTH + 14].extract_int()?,
            unknown_float_0: tokens[BaseNode::LENGTH + 15].extract_float()?,
            sim_0xfc: tokens[BaseNode::LENGTH + 16].extract_float()?,
            max_angular_velocity: tokens[BaseNode::LENGTH + 17].extract_float()?,
            unknown_float_1: tokens[BaseNode::LENGTH + 18].extract_float()?,
            constant_spring_force: tokens[BaseNode::LENGTH + 19].extract_float()?,
            min_0: tokens[BaseNode::LENGTH + 20].extract_float()?,
            min_1: tokens[BaseNode::LENGTH + 21].extract_float()?,
            min_2: tokens[BaseNode::LENGTH + 22].extract_float()?,
            min_3: tokens[BaseNode::LENGTH + 23].extract_float()?,
            unknown_flags_3: tokens[BaseNode::LENGTH + 24].extract_int()?,
            unknown_flags_4: tokens[BaseNode::LENGTH + 25].extract_int()?,
            unknown_float_2: tokens[BaseNode::LENGTH + 26].extract_float()?,
            max_dist: tokens[BaseNode::LENGTH + 27].extract_float()?,
            neg_dist: tokens[BaseNode::LENGTH + 28].extract_float()?,
            unknown_int_0: tokens[BaseNode::LENGTH + 29].extract_int()?,
            unknown_int_1: tokens[BaseNode::LENGTH + 30].extract_int()?,
            unknown_count_0: tokens[BaseNode::LENGTH + 31].extract_int()?,
            unknown_count_1: tokens[BaseNode::LENGTH + 32].extract_int()?,
        };

        Ok(sim)
    }
}

#[derive(Debug)]
pub struct BaseNode {
    pub flags: usize,
    pub transform_type: usize,
    pub pos_x: f32,
    pub pos_y: f32,
    pub pos_z: f32,
    pub scale_x: f32,
    pub scale_y: f32,
    pub scale_z: f32,
    pub rot_x: f32,
    pub rot_y: f32,
    pub rot_z: f32,
    pub rot_w: f32,
    pub min_x: f32,
    pub min_y: f32,
    pub min_z: f32,
    pub max_x: f32,
    pub max_y: f32,
    pub max_z: f32,
    pub unknown_0: f32,
    pub unknown_1: usize,
    pub unknown_2: usize,
    pub sector_crc: usize,
    pub action_count: usize,
    pub attach_count: usize,
}

impl BaseNode {
    const LENGTH: usize = 37;
    pub fn from_tokens(tokens: &[TkKind]) -> Result<Self, Error> {
        let mut node = Self {
            transform_type: tokens[0].extract_int()?,
            pos_x: tokens[1].extract_float()?,
            pos_y: tokens[2].extract_float()?,
            pos_z: tokens[3].extract_float()?,
            scale_x: tokens[4].extract_float()?,
            scale_y: tokens[5].extract_float()?,
            scale_z: tokens[6].extract_float()?,
            rot_x: tokens[7].extract_float()?,
            rot_y: tokens[8].extract_float()?,
            rot_z: tokens[9].extract_float()?,
            rot_w: tokens[10].extract_float()?,
            flags: 0,
            min_x: tokens[12].extract_float()?,
            min_y: tokens[13].extract_float()?,
            min_z: tokens[14].extract_float()?,
            max_x: tokens[15].extract_float()?,
            max_y: tokens[16].extract_float()?,
            max_z: tokens[17].extract_float()?,
            unknown_0: tokens[24].extract_float()?,
            unknown_1: tokens[25].extract_int()?,
            sector_crc: tokens[27].extract_hex8()?,
            unknown_2: tokens[34].extract_int()?,
            action_count: tokens[35].extract_int()?,
            attach_count: tokens[36].extract_int()?,
        };

        if node.transform_type != 1 {
            panic!("transform_type != 1 is not supported");
        }

        node.flags = (tokens[11].extract_int()? & 1) << 0xD | node.flags & 0xFFFF_DFFF;

        node.flags = (tokens[18].extract_int()? & 1) << 0x16 | node.flags & 0xFFBF_FFFF;
        node.flags = (tokens[19].extract_int()? & 1) << 0x17 | node.flags & 0xFF7F_FFFF;
        node.flags = (tokens[20].extract_int()? & 1) << 0x15 | node.flags & 0xFFDF_FFFF;
        node.flags = (tokens[21].extract_int()? & 1) << 0xB | node.flags & 0xFFFF_F7FF;
        node.flags = node.flags | tokens[22].extract_hex8()?;
        node.flags = (tokens[23].extract_int()? & 1) << 10 | node.flags & 0xFFFF_FBFF;

        node.flags = (tokens[26].extract_int()? & 1) << 0x13 | node.flags & 0xFFF7_FFFF;

        node.flags = (tokens[28].extract_int()? & 1) << 0x14 | node.flags & 0xFFEF_FFFF;
        node.flags = (tokens[29].extract_int()? & 1) << 0x12 | node.flags & 0xFFFB_FFFF;
        node.flags = (tokens[30].extract_int()? & 1) << 0x11 | node.flags & 0xFFFD_FFFF;
        node.flags = (tokens[31].extract_int()? & 1) << 0x10 | node.flags & 0xFFFE_FFFF;
        node.flags = (tokens[32].extract_int()? & 1) << 0xF | node.flags & 0xFFFF_7FFF;
        node.flags = (tokens[33].extract_int()? & 1) << 0xE | node.flags & 0xFFFF_BFFF;

        Ok(node)
    }
}

#[derive(Debug)]
pub struct NodeCollision {
    pub base_node: BaseNode,
    pub unknown_flags_0: usize,
    pub unknown_flags_1: usize,
    pub unknown_flags_2: usize,
    pub unknown_flags_3: usize,
    pub unknown_flags_4: usize,
    pub unknown_flags_5: usize,
    pub unknown_flags_6: usize,
}

impl NodeCollision {
    pub const LENGTH: usize = BaseNode::LENGTH + 7;
    pub fn from_tokens(tokens: &[TkKind]) -> Result<Self, Error> {
        Ok(Self {
            base_node: BaseNode::from_tokens(tokens)?,
            unknown_flags_0: tokens[BaseNode::LENGTH + 1].extract_int()?,
            unknown_flags_1: tokens[BaseNode::LENGTH + 2].extract_int()?,
            unknown_flags_2: tokens[BaseNode::LENGTH + 3].extract_int()?,
            unknown_flags_3: tokens[BaseNode::LENGTH + 4].extract_int()?,
            unknown_flags_4: tokens[BaseNode::LENGTH + 5].extract_int()?,
            unknown_flags_5: tokens[BaseNode::LENGTH + 6].extract_int()?,
            unknown_flags_6: tokens[BaseNode::LENGTH + 7].extract_int()?,
        })
    }
}
