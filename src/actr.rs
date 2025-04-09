use alloc::vec::Vec;

use crate::ParseError;

pub struct ActrReader<Data: AsRef<[u8]>> {
    input: Data,
    header: Header,
}
#[derive(Copy, Clone)]
pub struct Header {
    pub vertex_buffer_offset: u32,
    pub display_list_offset: u32,
    pub display_list_size: u32,
    pub vertex_offset: u32,
    pub normal_offset: u32,
    pub texcoord_offset: u32,
    pub color_offset: u32,
    pub vertex_count: u32,
    pub normal_count: u32,
    pub mesh_offset: u32,
    pub node_offset: u32,
    pub actor_flags: u32,
    pub geometry_offset: u32,
    pub geometry_count: u32,
    pub vertex_size_index: u8,
    pub node_end_offset: u32,
    pub model_name_offset: u32,
    pub node_string_table_offset: u32,
    pub bone_string_table_offset: u32,
    pub node_count: u8,
}

impl Header {
    pub const LENGTH: usize = 0x100;

    pub fn from_bytes(input: &[u8; Self::LENGTH]) -> Result<Self, ParseError> {
        let header = Self {
            vertex_buffer_offset: u32::from_be_bytes(input[0x24..0x28].try_into().unwrap()),
            display_list_offset: u32::from_be_bytes(input[0x58..0x5C].try_into().unwrap()),
            display_list_size: u32::from_be_bytes(input[0x5C..0x60].try_into().unwrap()),
            vertex_offset: u32::from_be_bytes(input[0x60..0x64].try_into().unwrap()),
            normal_offset: u32::from_be_bytes(input[0x64..0x68].try_into().unwrap()),
            texcoord_offset: u32::from_be_bytes(input[0x68..0x6C].try_into().unwrap()),
            color_offset: u32::from_be_bytes(input[0x6C..0x70].try_into().unwrap()),
            vertex_count: u32::from_be_bytes(input[0x70..0x74].try_into().unwrap()),
            normal_count: u32::from_be_bytes(input[0x74..0x78].try_into().unwrap()),
            mesh_offset: u32::from_be_bytes(input[0x78..0x7C].try_into().unwrap()),
            node_offset: u32::from_be_bytes(input[0xA0..0xA4].try_into().unwrap()),
            actor_flags: u32::from_be_bytes(input[0xA4..0xA8].try_into().unwrap()),
            geometry_offset: u32::from_be_bytes(input[0xB4..0xB8].try_into().unwrap()),
            geometry_count: u32::from_be_bytes(input[0xB8..0xBC].try_into().unwrap()),
            vertex_size_index: input[0xD9],
            node_end_offset: u32::from_be_bytes(input[0xDC..0xE0].try_into().unwrap()),
            model_name_offset: u32::from_be_bytes(input[0xE0..0xE4].try_into().unwrap()),
            node_string_table_offset: u32::from_be_bytes(input[0xE4..0xE8].try_into().unwrap()),
            bone_string_table_offset: u32::from_be_bytes(input[0xEC..0xF0].try_into().unwrap()),
            node_count: input[0xF1],
        };
        if header.node_offset == 0 {
            return Err(ParseError::ZeroOffset);
        }

        if header.geometry_offset == 0 {
            //return Err(ParseError::ZeroOffset);
        }

        if header.geometry_count < 1 {
            //return Err(ParseError::ZeroGeometry);
        }

        if header.model_name_offset == 0 {
            //return Err(ParseError::ZeroOffset);
        }

        if header.node_string_table_offset == 0 {
            //return Err(ParseError::ZeroOffset);
        }

        if header.bone_string_table_offset == 0 {
            //return Err(ParseError::ZeroOffset);
        }
        Ok(header)
    }
}

impl<Data: AsRef<[u8]>> ActrReader<Data> {
    pub fn new(input: Data) -> Result<Self, ParseError> {
        if input.as_ref().len() < Header::LENGTH {
            return Err(ParseError::UnexpectedEnd);
        }

        let header_data = input
            .as_ref()
            .get(0..Header::LENGTH)
            .ok_or(ParseError::UnexpectedEnd)?;
        let header = Header::from_bytes(header_data.try_into().unwrap())?;

        Ok(Self { input, header })
    }

    pub fn nodes(&self) -> Result<impl ExactSizeIterator<Item = ActorNode> + '_, ParseError> {
        let mut node_offset = usize::try_from(self.header.node_offset).unwrap();
        let node_count = usize::try_from(self.header.node_count).unwrap();
        let mut node_size = ActorNode::LENGTH;

        let mut nodes = Vec::with_capacity(node_count);
        for _ in 0..node_count {
            let node_bytes = self
                .input
                .as_ref()
                .get(node_offset..(node_offset + ActorNode::LENGTH))
                .ok_or(ParseError::UnexpectedEnd)?;

            let node = ActorNode::from_bytes(
                node_bytes.try_into().unwrap(),
                self.input.as_ref(),
                self.header.vertex_size_index.into(),
            );
            nodes.push(node);

            if node.next_node_offset != 0 {
                node_offset = usize::try_from(node.next_node_offset).unwrap();
                node_size = usize::try_from(node.next_node_offset).unwrap() - node_offset;
            } else if node.unk1_node_offset != 0
                && usize::try_from(node.unk1_node_offset).unwrap() != node_offset
            {
                node_offset = usize::try_from(node.unk1_node_offset).unwrap();
                node_size = usize::try_from(node.unk1_node_offset).unwrap() - node_offset;
            } else {
                node_offset += node_size;
            }
        }

        Ok(nodes.into_iter())
    }

    pub fn geometry(&self) -> Result<ActorGeometry, ParseError> {
        let geo_offset = usize::try_from(self.header.geometry_offset).unwrap();
        let geo_end_offset = geo_offset + ActorGeometry::LENGTH;

        let geo_bytes = self
            .input
            .as_ref()
            .get(geo_offset..geo_end_offset)
            .ok_or(ParseError::UnexpectedEnd)?;

        Ok(ActorGeometry::from_bytes(geo_bytes.try_into().unwrap()))
    }

    pub fn verticies(&self) -> Result<impl ExactSizeIterator<Item = Vertex> + '_, ParseError> {
        let verticies_offset = usize::try_from(self.header.vertex_offset).unwrap();

        if verticies_offset == 0 {
            return Err(ParseError::UnexpectedEnd);
        }

        let verticies_end_offset = verticies_offset
            + (usize::try_from(self.header.vertex_count).unwrap() * Vertex::LENGTH);

        let verticies_bytes = self
            .input
            .as_ref()
            .get(verticies_offset..verticies_end_offset)
            .ok_or(ParseError::UnexpectedEnd)?;

        Ok(verticies_bytes.chunks_exact(Vertex::LENGTH).map(|data| {
            let vertex_data = data.try_into().unwrap();

            Vertex::from_bytes(vertex_data)
        }))
    }

    pub fn normals(&self) -> Result<impl ExactSizeIterator<Item = Normal> + '_, ParseError> {
        let normal_offset = usize::try_from(self.header.normal_offset).unwrap();

        if normal_offset == 0 {
            return Err(ParseError::UnexpectedEnd);
        }

        let normal_end_offset =
            normal_offset + (usize::try_from(self.header.normal_count).unwrap() * Normal::LENGTH);

        let normal_bytes = self
            .input
            .as_ref()
            .get(normal_offset..normal_end_offset)
            .ok_or(ParseError::UnexpectedEnd)?;

        Ok(normal_bytes.chunks_exact(Normal::LENGTH).map(|data| {
            let normal_data = data.try_into().unwrap();

            Normal::from_bytes(normal_data)
        }))
    }

    pub fn texcoords(&self) -> Result<impl Iterator<Item = Texcoord> + '_, ParseError> {
        let texcoord_offset = usize::try_from(self.header.texcoord_offset).unwrap();

        if texcoord_offset == 0 {
            return Err(ParseError::UnexpectedEnd);
        }

        //TODO: Hacky workaround to find size, there is probably a size a somewhere
        let texcoord_end_offset = usize::try_from(self.header.vertex_buffer_offset).unwrap();

        let texcoord_bytes = self
            .input
            .as_ref()
            .get(texcoord_offset..texcoord_end_offset)
            .ok_or(ParseError::UnexpectedEnd)?;

        Ok(texcoord_bytes.chunks_exact(Texcoord::LENGTH).map(|data| {
            let texcoord_data = data.try_into().unwrap();

            Texcoord::from_bytes(texcoord_data)
        }))
    }

    pub fn colors(&self) -> Result<impl Iterator<Item = Color> + '_, ParseError> {
        let color_offset = usize::try_from(self.header.color_offset).unwrap();

        if color_offset == 0 {
            return Err(ParseError::UnexpectedEnd);
        }

        let color_end_offset = usize::try_from(self.header.texcoord_offset).unwrap();

        let color_bytes = self
            .input
            .as_ref()
            .get(color_offset..color_end_offset)
            .ok_or(ParseError::UnexpectedEnd)?;

        Ok(color_bytes.chunks_exact(Color::LENGTH).map(|data| {
            let color_data = data.try_into().unwrap();

            Color::from_bytes(color_data)
        }))
    }

    //Returns Indexes and current group num;
    pub fn indexes(
        &self,
    ) -> Result<impl ExactSizeIterator<Item = (Index, usize)> + '_, ParseError> {
        let mut indicies = Vec::new();
        let indexes_offset = usize::try_from(self.header.display_list_offset).unwrap();

        if indexes_offset == 0 {
            return Err(ParseError::UnexpectedEnd);
        }

        let indexes_end_offset =
            indexes_offset + usize::try_from(self.header.display_list_size).unwrap();

        let indexes_bytes = self
            .input
            .as_ref()
            .get(indexes_offset..indexes_end_offset)
            .ok_or(ParseError::UnexpectedEnd)?;

        let mut offset = 0;
        let mut group: usize = 0;
        while offset < indexes_bytes.len() {
            if offset == indexes_bytes.len() - 1 {
                break;
            }
            if indexes_bytes.get(offset..offset + 2).unwrap_or(&[0, 0]) == [0x0, 0x99] {
                let actual_header_length = match self.header.vertex_size_index {
                    0x15 => IndexHeader::LENGTH + 1,
                    0x10 | 0x18 | 0x19 => IndexHeader::LENGTH,
                    val => panic!("Unkown index size: {val}"),
                };

                let actual_index_length = match self.header.vertex_size_index {
                    0x15 => Index::LENGTH + 1,
                    0x10 | 0x18 => Index::LENGTH,
                    0x19 => 10,
                    val => panic!("Unknown index size: {val}"),
                };

                let header_offset = offset;
                //let header_end_offset = offset + actual_header_length;

                let header = IndexHeader::from_bytes(
                    indexes_bytes[header_offset..header_offset + IndexHeader::LENGTH]
                        .try_into()
                        .unwrap(),
                );

                let indicies_offset = offset + actual_header_length;
                let indicies_end_offset =
                    indicies_offset + (actual_index_length * header.count as usize);

                let indexes = indexes_bytes[indicies_offset..]
                    .chunks_exact(actual_index_length)
                    .map(Index::from_bytes)
                    .take(header.count as usize);

                for index in indexes {
                    indicies.push((index, group))
                }
                group += 1;

                offset = indicies_end_offset;

                let mut count = 0;
                if let Some(array) = indexes_bytes.get(offset..) {
                    for byte in array {
                        if *byte == 0x99 {
                            break;
                        }
                        count += 1;
                    }
                }

                offset = offset + count - 1;
            }
        }
        Ok(indicies.into_iter())
    }

    pub fn header(&self) -> Header {
        self.header
    }

    pub fn data(&self) -> &[u8] {
        self.input.as_ref()
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vertex {
    pub const LENGTH: usize = 12;
    pub fn from_bytes(input: &[u8; Self::LENGTH]) -> Self {
        Self {
            x: f32::from_be_bytes(input[0x0..0x4].try_into().unwrap()),
            y: f32::from_be_bytes(input[0x4..0x8].try_into().unwrap()),
            z: f32::from_be_bytes(input[0x8..0xC].try_into().unwrap()),
        }
    }
}
#[derive(Copy, Clone, Debug)]
pub struct Normal {
    pub x: i8,
    pub y: i8,
    pub z: i8,
}

impl Normal {
    pub const LENGTH: usize = 3;
    pub fn from_bytes(input: &[u8; Self::LENGTH]) -> Self {
        Self {
            x: input[0] as i8,
            y: input[1] as i8,
            z: input[2] as i8,
        }
    }
}

#[derive(PartialEq, PartialOrd, Copy, Clone, Debug)]
pub struct Texcoord {
    pub x: f32,
    pub y: f32,
}

impl Texcoord {
    pub const LENGTH: usize = 8;
    pub fn from_bytes(input: &[u8; Self::LENGTH]) -> Self {
        Self {
            x: f32::from_be_bytes(input[0x0..0x4].try_into().unwrap()),
            y: f32::from_be_bytes(input[0x4..0x8].try_into().unwrap()),
        }
    }
}

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const LENGTH: usize = 0x4;
    pub fn from_bytes(input: &[u8; Self::LENGTH]) -> Self {
        Self {
            r: input[0],
            g: input[1],
            b: input[2],
            a: input[3],
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct IndexHeader {
    pub magic: u16,
    pub count: u16,
}

impl IndexHeader {
    pub const LENGTH: usize = 0x4;
    pub fn from_bytes(input: &[u8; Self::LENGTH]) -> Self {
        Self {
            magic: u16::from_be_bytes(input[0..2].try_into().unwrap()),
            count: u16::from_be_bytes(input[2..4].try_into().unwrap()),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Index {
    pub unk1: u16,
    pub pos_idx: u16,
    pub normal_idx: u16,
    pub texcoord_idx: u16,
    pub color_idx: u16,
    pub tag: u8,
}

impl Index {
    pub const LENGTH: usize = 0x8;
    pub fn from_bytes(input: &[u8]) -> Self {
        let mut index = Self {
            pos_idx: u16::from_be_bytes(input[0..2].try_into().unwrap()),
            normal_idx: u16::from_be_bytes(input[2..4].try_into().unwrap()),
            color_idx: u16::from_be_bytes(input[4..6].try_into().unwrap()),
            texcoord_idx: u16::from_be_bytes(input[6..8].try_into().unwrap()),
            tag: 0,
            unk1: 0,
        };

        match input.len() {
            0x8 => index,
            0x9 => {
                index.tag = input[8];
                index
            }
            0x10 => Self {
                unk1: u16::from_be_bytes(input[0..2].try_into().unwrap()),
                pos_idx: u16::from_be_bytes(input[2..4].try_into().unwrap()),
                normal_idx: u16::from_be_bytes(input[4..8].try_into().unwrap()),
                color_idx: u16::from_be_bytes(input[8..10].try_into().unwrap()),
                texcoord_idx: u16::from_be_bytes(input[10..12].try_into().unwrap()),
                tag: 0,
            },
            _ => index,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct ActorNode<'a> {
    pub data: &'a [u8],
    pub idx: u16,
    pub node_idx: u32,
    pub vertex_offset: u32,
    pub normal_offset: u32,
    pub texcoord_ofset: u32,
    pub color_offset: u32,
    pub display_list_offset: u32,
    pub display_list_size: u32,
    pub vertex_count: u32,
    pub unk1_node_offset: u32,
    pub unk2_node_offset: u32,
    pub prev_node_offset: u32,
    pub next_node_offset: u32,
    pub name_offset: u32,
}

impl<'a> ActorNode<'a> {
    pub const LENGTH: usize = 0x140;

    pub fn from_bytes(input: &[u8; ActorNode::LENGTH], data: &'a [u8], idx: u16) -> ActorNode<'a> {
        Self {
            data,
            idx,
            node_idx: u32::from_be_bytes(input[0x74..0x78].try_into().unwrap()),
            vertex_offset: u32::from_be_bytes(input[0xD0..0xD4].try_into().unwrap()),
            normal_offset: u32::from_be_bytes(input[0xD4..0xD8].try_into().unwrap()),
            texcoord_ofset: u32::from_be_bytes(input[0xD8..0xDC].try_into().unwrap()),
            color_offset: u32::from_be_bytes(input[0xDC..0xE0].try_into().unwrap()),
            display_list_offset: u32::from_be_bytes(input[0xE0..0xE4].try_into().unwrap()),
            display_list_size: u32::from_be_bytes(input[0xE4..0xE8].try_into().unwrap()),
            vertex_count: u32::from_be_bytes(input[0x80..0x84].try_into().unwrap()),
            unk1_node_offset: u32::from_be_bytes(input[0x110..0x114].try_into().unwrap()),
            unk2_node_offset: u32::from_be_bytes(input[0x114..0x118].try_into().unwrap()),
            prev_node_offset: u32::from_be_bytes(input[0x118..0x11C].try_into().unwrap()),
            next_node_offset: u32::from_be_bytes(input[0x11C..0x120].try_into().unwrap()),
            name_offset: u32::from_be_bytes(input[0x130..0x134].try_into().unwrap()),
        }
    }

    pub fn verticies(&self) -> Result<impl ExactSizeIterator<Item = Vertex> + '_, ParseError> {
        let verticies_offset = usize::try_from(self.vertex_offset).unwrap();

        if verticies_offset == 0 {
            return Err(ParseError::UnexpectedEnd);
        }

        let verticies_end_offset = verticies_offset + (self.vertex_count as usize * Vertex::LENGTH);
        let verticies_bytes = self
            .data
            .get(verticies_offset..verticies_end_offset)
            .ok_or(ParseError::UnexpectedEnd)?;

        Ok(verticies_bytes.chunks_exact(Vertex::LENGTH).map(|data| {
            let vertex_data = data.try_into().unwrap();

            Vertex::from_bytes(vertex_data)
        }))
    }

    pub fn normals(&self) -> Result<impl ExactSizeIterator<Item = Normal> + '_, ParseError> {
        let normal_offset = usize::try_from(self.normal_offset).unwrap();

        if normal_offset == 0 {
            return Err(ParseError::UnexpectedEnd);
        }

        let normal_end_offset =
            normal_offset + (usize::try_from(self.vertex_count).unwrap() * Normal::LENGTH);

        let normal_bytes = self
            .data
            .as_ref()
            .get(normal_offset..normal_end_offset)
            .ok_or(ParseError::UnexpectedEnd)?;

        Ok(normal_bytes.chunks_exact(Normal::LENGTH).map(|data| {
            let normal_data = data.try_into().unwrap();

            Normal::from_bytes(normal_data)
        }))
    }

    pub fn texcoords(&self) -> Result<impl Iterator<Item = Texcoord> + '_, ParseError> {
        let texcoord_offset = usize::try_from(self.texcoord_ofset).unwrap();

        if texcoord_offset == 0 {
            return Err(ParseError::UnexpectedEnd);
        }

        //TODO: Hacky workaround to find size, there is probably a size a somewhere
        let bytes = if self.next_node_offset != 0 {
            self.data
                .as_ref()
                .get(texcoord_offset..self.next_node_offset as usize)
                .ok_or(ParseError::UnexpectedEnd)?
        } else if self.unk1_node_offset != 0
            && self.prev_node_offset != self.unk1_node_offset
            && self.unk2_node_offset != self.unk1_node_offset
        {
            self.data
                .as_ref()
                .get(texcoord_offset..self.unk1_node_offset as usize)
                .ok_or(ParseError::UnexpectedEnd)?
        } else {
            self.data
                .as_ref()
                .get(texcoord_offset..texcoord_offset + Texcoord::LENGTH)
                .ok_or(ParseError::UnexpectedEnd)?
        };

        Ok(bytes.chunks_exact(Texcoord::LENGTH).map(|data| {
            let texcoord_data = data.try_into().unwrap();

            Texcoord::from_bytes(texcoord_data)
        }))
    }

    pub fn colors(&self) -> Result<impl Iterator<Item = Color> + '_, ParseError> {
        let color_offset = usize::try_from(self.color_offset).unwrap();

        if color_offset == 0 {
            return Err(ParseError::UnexpectedEnd);
        }

        let color_end_offset = usize::try_from(self.texcoord_ofset).unwrap();

        let color_bytes = self
            .data
            .as_ref()
            .get(color_offset..color_end_offset)
            .ok_or(ParseError::UnexpectedEnd)?;

        Ok(color_bytes.chunks_exact(Color::LENGTH).map(|data| {
            let color_data = data.try_into().unwrap();

            Color::from_bytes(color_data)
        }))
    }

    //Returns Indexes and current group num;
    pub fn indexes(
        &self,
    ) -> Result<impl ExactSizeIterator<Item = (Index, usize)> + '_, ParseError> {
        let mut indicies = Vec::new();
        let indexes_offset = usize::try_from(self.display_list_offset).unwrap();

        if indexes_offset == 0 {
            return Err(ParseError::UnexpectedEnd);
        }
        let indexes_end_offset = indexes_offset + usize::try_from(self.display_list_size).unwrap();

        let indexes_bytes = self
            .data
            .as_ref()
            .get(indexes_offset..indexes_end_offset)
            .ok_or(ParseError::UnexpectedEnd)?;

        let mut offset = 0;
        let mut group: usize = 0;
        while offset < indexes_bytes.len() {
            if offset == indexes_bytes.len() - 1 {
                break;
            }
            if indexes_bytes.get(offset..offset + 2).unwrap_or(&[0, 0]) == [0x0, 0x99] {
                let actual_header_length = match self.idx {
                    0x15 => IndexHeader::LENGTH + 1,
                    0x10 | 0x18 | 0x19 => IndexHeader::LENGTH,
                    val => panic!("Unkown index size: {val}"),
                };

                let actual_index_length = match self.idx {
                    0x15 => Index::LENGTH + 1,
                    0x10 | 0x18 => Index::LENGTH,
                    0x19 => 10,
                    val => panic!("Unknown index size: {val}"),
                };

                let header_offset = offset;
                //let header_end_offset = offset + actual_header_length;

                let header = IndexHeader::from_bytes(
                    indexes_bytes[header_offset..header_offset + IndexHeader::LENGTH]
                        .try_into()
                        .unwrap(),
                );

                let indicies_offset = offset + actual_header_length;
                let indicies_end_offset =
                    indicies_offset + (actual_index_length * header.count as usize);

                let indexes = indexes_bytes[indicies_offset..]
                    .chunks_exact(actual_index_length)
                    .map(Index::from_bytes)
                    .take(header.count as usize);

                for index in indexes {
                    indicies.push((index, group))
                }
                group += 1;

                offset = indicies_end_offset;

                let mut count = 0;
                if let Some(array) = indexes_bytes.get(offset..) {
                    for byte in array {
                        if *byte == 0x99 {
                            break;
                        }
                        count += 1;
                    }
                }

                offset = offset + count - 1;
            }
        }
        Ok(indicies.into_iter())
    }
}

pub struct ActorGeometry {
    pub node_count: u32,
    pub index_buffer_offset: u32,
    pub node_offset: u32,
}

impl ActorGeometry {
    pub const LENGTH: usize = 0x40;

    pub fn from_bytes(input: &[u8; Self::LENGTH]) -> Self {
        Self {
            node_count: u32::from_be_bytes(input[0x30..0x34].try_into().unwrap()),
            index_buffer_offset: u32::from_be_bytes(input[0x38..0x3C].try_into().unwrap()),
            node_offset: u32::from_be_bytes(input[0x3C..0x40].try_into().unwrap()),
        }
    }
}
/*
 #pragma endian big

struct Actor {
   padding[0x24];
   u32 vertex_buffer_offset;
   padding[0x38];
   u32 position_offset;
   u32 normal_offset;
   u32 texcoord_offset;
   u32 color_offset;
 padding[0x8];
   u32 mesh_offset;
   padding[0x24];
   u32 node_offset;
   u32 actor_flags;
   padding[0xC ];
   u32 geometry_offset;
   u32 geometry_count;
   padding[0x1D];
   u8 vertex_size_index;
   padding[0x2];
   u32 mesh_data_len;
   u32 mesh_name_offset;
   u32 node_string_table_offset;
   padding[4];
   u32 bone_string_table_offset;


};

struct Node {
    padding[0x88];
    u32 some_count;
    u32 some_offset;
    u32 some_offset_2;
    padding[0x3C];
    u32 vertex_offset;
    u32 normal_offset;
    u32 texcoord_offset;
    u32 color_offset;
    u32 some_count2;
    padding[4];
    u32 vertex_count;
    padding[0x24];
    u32 unk1_node_offset;
    u32 unk2_node_offset;
    u32 prev_node_offset;
    u32 node_next_offset;
    padding[0x10];
    u32 node_string_offset;
    padding[0xC];
};

struct Geometry {
    padding[0x30];
    u32 node_count;
    padding[0x4];
    u32 some_offset;
    u32 mesh_name_offset;
};

struct something16 {
   u8 padd[0x10];
};

struct VertexBuffer {
    u32 start;
};

Actor actor @ 0x0;
VertexBuffer buffer @ actor.vertex_buffer_offset;
Geometry geo @ actor.geometry_offset;

if (geo.node_count < 1) {
Node node[1] @ actor.node_offset;
else {
Node node[geo.node_count] @ actor.node_offset;

something16 some[node[0].some_count] @ node[0].some_offset;
 */

pub mod experimental {
    use core::ffi::CStr;
    use std::vec::Vec;

    use super::{Normal, Texcoord};

    #[derive(Copy, Clone, Debug)]
    pub struct ResourceInfo {
        package_id: u32,
        group_id: u16,
        kind: u8,
        context_offset: u32,
        crc: u32,
        child_1_resource_offset: u32,
        child_2_resource_offset: u32,
        parent_resource_offset: u32,
    }

    impl ResourceInfo {
        pub const SIZE: usize = 0x20;
        pub fn from_bytes(bytes: &[u8; Self::SIZE]) -> Self {
            Self {
                package_id: u32::from_be_bytes(bytes[0..4].try_into().unwrap()),
                group_id: u16::from_be_bytes(bytes[4..6].try_into().unwrap()),
                kind: bytes[6],
                context_offset: u32::from_be_bytes(bytes[8..12].try_into().unwrap()),
                crc: u32::from_be_bytes(bytes[12..16].try_into().unwrap()),
                child_1_resource_offset: u32::from_be_bytes(bytes[16..20].try_into().unwrap()),
                child_2_resource_offset: u32::from_be_bytes(bytes[20..24].try_into().unwrap()),
                parent_resource_offset: u32::from_be_bytes(bytes[24..28].try_into().unwrap()),
            }
        }
    }

    #[derive(Copy, Clone, Debug)]
    pub struct SoftSkin {
        pub number_of_vertices: u32,
        pub vertex_offset: u32,
        number_of_batches: u32,
        batch_offset: u32,
        batch_primitive_offset: u32,
        number_of_solid_batches: u32,
        number_of_color_key_batches: u32,
        number_of_alpha_batches: u32,
        first_color_key_primitive_offset: i32,
        first_alpha_primitive_offset: i32,
        first_color_key_primitive_vertex_offset: i32,
        first_alpha_primitive_vertex_offset: i32,
        bones_per_pertex: u8,
        flags: u16,
        pub display_segment_offset: u32,
        pub display_list_offset: u32,
        pub display_list_size: u32,
        pub position_offset: u32,
        pub normal_offset: u32,
        pub texture_coord_offset: u32,
        pub color_offset: u32,
        position_count: u32,
        normal_count: u32,
        morph_target_offset: u32,
        patch_offset: u32,
        vertex_normal_extra_offset: u32,
    }

    impl SoftSkin {
        pub const SIZE: usize = 0x80;
        pub fn from_bytes(bytes: &[u8; SoftSkin::SIZE]) -> Self {
            Self {
                number_of_vertices: u32::from_be_bytes(bytes[0..4].try_into().unwrap()),
                vertex_offset: u32::from_be_bytes(bytes[4..8].try_into().unwrap()),
                number_of_batches: u32::from_be_bytes(bytes[8..12].try_into().unwrap()),
                batch_offset: u32::from_be_bytes(bytes[12..16].try_into().unwrap()),
                batch_primitive_offset: u32::from_be_bytes(bytes[16..20].try_into().unwrap()),
                number_of_solid_batches: u32::from_be_bytes(bytes[20..24].try_into().unwrap()),
                number_of_color_key_batches: u32::from_be_bytes(bytes[24..28].try_into().unwrap()),
                number_of_alpha_batches: u32::from_be_bytes(bytes[28..32].try_into().unwrap()),
                first_color_key_primitive_offset: i32::from_be_bytes(
                    bytes[32..36].try_into().unwrap(),
                ),
                first_alpha_primitive_offset: i32::from_be_bytes(bytes[36..40].try_into().unwrap()),
                first_color_key_primitive_vertex_offset: i32::from_be_bytes(
                    bytes[40..44].try_into().unwrap(),
                ),
                first_alpha_primitive_vertex_offset: i32::from_be_bytes(
                    bytes[44..48].try_into().unwrap(),
                ),
                bones_per_pertex: bytes[48],
                flags: u16::from_be_bytes(bytes[50..52].try_into().unwrap()),
                display_segment_offset: u32::from_be_bytes(bytes[52..56].try_into().unwrap()),
                display_list_offset: u32::from_be_bytes(bytes[56..60].try_into().unwrap()),
                display_list_size: u32::from_be_bytes(bytes[60..64].try_into().unwrap()),
                position_offset: u32::from_be_bytes(bytes[64..68].try_into().unwrap()),
                normal_offset: u32::from_be_bytes(bytes[68..72].try_into().unwrap()),
                texture_coord_offset: u32::from_be_bytes(bytes[72..76].try_into().unwrap()),
                color_offset: u32::from_be_bytes(bytes[76..80].try_into().unwrap()),
                position_count: u32::from_be_bytes(bytes[80..84].try_into().unwrap()),
                normal_count: u32::from_be_bytes(bytes[84..88].try_into().unwrap()),
                morph_target_offset: u32::from_be_bytes(bytes[88..92].try_into().unwrap()),
                patch_offset: u32::from_be_bytes(bytes[92..96].try_into().unwrap()),
                vertex_normal_extra_offset: u32::from_be_bytes(bytes[96..100].try_into().unwrap()),
            }
        }

        pub fn positions_from_buffer(&self, buffer: &[u8]) -> Vec<Position> {
            let pos_start_offset = self.position_offset as usize;

            if pos_start_offset == 0 {
                panic!()
            }

            let pos_end_offset =
                pos_start_offset + (Position::LENGTH * self.number_of_vertices as usize);
            buffer[pos_start_offset..pos_end_offset]
                .chunks_exact(Position::LENGTH)
                .map(|bytes| Position::from_bytes(bytes.try_into().unwrap()))
                .collect()
        }

        pub fn normals_from_buffer(&self, buffer: &[u8]) -> Vec<Normal> {
            let norm_start_offset = self.normal_offset as usize;
            let norm_end_offset =
                norm_start_offset + (Normal::LENGTH * self.number_of_vertices as usize);
            buffer[norm_start_offset..norm_end_offset]
                .chunks_exact(Normal::LENGTH)
                .map(|bytes| Normal::from_bytes(bytes.try_into().unwrap()))
                .collect()
        }

        pub fn texcoords_from_buffer(&self, buffer: &[u8], kind: u8) -> Vec<Texcoord> {
            let (_, _, _, number_of_texcoords) = self.max_idx(buffer, kind);

            let norm_start_offset = self.texture_coord_offset as usize;
            if norm_start_offset == 0 {
                panic!()
            }

            let norm_end_offset =
                norm_start_offset + (Texcoord::LENGTH * number_of_texcoords as usize + 1);
            buffer[norm_start_offset..norm_end_offset]
                .chunks_exact(Texcoord::LENGTH)
                .map(|bytes| Texcoord::from_bytes(bytes.try_into().unwrap()))
                .collect()
        }

        pub fn display_segments_from_buffer(&self, buffer: &[u8]) -> Vec<(u32, u32)> {
            let seg_start = self.display_segment_offset as usize;
            let seg_end: usize = self.primitives_from_buffer(buffer).len() * 8;

            buffer[seg_start..seg_start + seg_end as usize]
                .chunks_exact(8)
                .map(|bytes| {
                    (
                        u32::from_be_bytes(bytes[0..4].try_into().unwrap()),
                        u32::from_be_bytes(bytes[4..8].try_into().unwrap()),
                    )
                })
                .collect()
        }

        pub fn max_idx<'a>(&'a self, buffer: &'a [u8], kind: u8) -> (u16, u16, u16, u16) {
            let (mut max_p_idx, mut max_n_idx, mut max_t_idx, mut max_c_idx) = (0, 0, 0, 0);

            let display_list_start = self.display_list_offset;
            let display_list_end = display_list_start + self.display_list_size;

            let display_list = &buffer[display_list_start as usize..display_list_end as usize];

            for (offset, size) in self.display_segments_from_buffer(buffer) {
                let offset = offset as usize;
                let part = DisplayListPart {
                    cmd: u16::from_be_bytes(display_list[offset..offset + 2].try_into().unwrap()),
                    vertex_count: u16::from_be_bytes(
                        display_list[offset + 2..offset + 4].try_into().unwrap(),
                    ),
                };

                let mut start = offset + 5;

                for _ in 0..part.vertex_count {
                    let p_idx =
                        u16::from_be_bytes(display_list[start..start + 2].try_into().unwrap());
                    let n_idx =
                        u16::from_be_bytes(display_list[start + 2..start + 4].try_into().unwrap());
                    let t_idx =
                        u16::from_be_bytes(display_list[start + 4..start + 6].try_into().unwrap());
                    let c_idx =
                        u16::from_be_bytes(display_list[start + 6..start + 8].try_into().unwrap());

                    max_p_idx = p_idx.max(max_p_idx);
                    max_n_idx = n_idx.max(max_n_idx);
                    max_t_idx = t_idx.max(max_t_idx);
                    max_c_idx = c_idx.max(max_c_idx);
                    match kind {
                        21 => start += 1,
                        _ => (),
                    }

                    start += 8;
                }
            }

            (max_p_idx, max_n_idx, max_t_idx, max_c_idx)
        }
        pub fn batches_from_buffer(&self, buffer: &[u8]) -> Vec<MeshBatch> {
            let batches_start_offset = self.batch_offset as usize;
            let batches_end_offset =
                batches_start_offset + (MeshBatch::SIZE * self.number_of_batches as usize);

            buffer[batches_start_offset..batches_end_offset]
                .chunks_exact(MeshBatch::SIZE)
                .map(|bytes| MeshBatch::from_bytes(bytes.try_into().unwrap()))
                .collect()
        }
        pub fn primitives_from_buffer(&self, buffer: &[u8]) -> Vec<SoftSkinPrimitive> {
            let mut primitive_count = 0;
            for batch in self.batches_from_buffer(buffer) {
                primitive_count += batch.number_of_primitives
            }

            let batches_start_offset = self.batch_primitive_offset as usize;
            let batches_end_offset =
                batches_start_offset + (SoftSkinPrimitive::SIZE * primitive_count as usize);

            buffer[batches_start_offset..batches_end_offset]
                .chunks_exact(SoftSkinPrimitive::SIZE)
                .map(|bytes| SoftSkinPrimitive::from_bytes(bytes.try_into().unwrap()))
                .collect()
        }
        pub fn display_list_parts_from_buffer<'a>(
            &'a self,
            buffer: &'a [u8],
            kind: u8,
        ) -> Vec<(DisplayListPart, Vec<(u16, u16, u16, u16)>)> {
            const VERTEX_TYPE_DISPLAYLIST_INDEXED: u8 = 16;
            const VERTEX_TYPE_1BONE_DISPLAYLIST_INDEXED: u8 = 21;

            let (mut max_p_idx, mut max_n_idx, mut max_t_idx, mut max_c_idx) = (0, 0, 0, 0);

            let display_list_start = self.display_list_offset;
            let display_list_end = display_list_start + self.display_list_size;

            let display_list = &buffer[display_list_start as usize..display_list_end as usize];

            let mut display_list_parts = Vec::new();
            for (offset, size) in self.display_segments_from_buffer(buffer) {
                let offset = offset as usize;
                let part = DisplayListPart {
                    cmd: u16::from_be_bytes(display_list[offset..offset + 2].try_into().unwrap()),
                    vertex_count: u16::from_be_bytes(
                        display_list[offset + 2..offset + 4].try_into().unwrap(),
                    ),
                };

                let mut indexes = Vec::new();
                let mut start = offset + 4;
                match kind {
                    VERTEX_TYPE_1BONE_DISPLAYLIST_INDEXED => start += 1,
                    VERTEX_TYPE_DISPLAYLIST_INDEXED | _ => (),
                }

                for _ in 0..part.vertex_count {
                    let p_idx =
                        u16::from_be_bytes(display_list[start..start + 2].try_into().unwrap());
                    let n_idx =
                        u16::from_be_bytes(display_list[start + 2..start + 4].try_into().unwrap());
                    let t_idx =
                        u16::from_be_bytes(display_list[start + 4..start + 6].try_into().unwrap());
                    let c_idx =
                        u16::from_be_bytes(display_list[start + 6..start + 8].try_into().unwrap());

                    max_p_idx = p_idx.max(max_p_idx);
                    max_n_idx = n_idx.max(max_n_idx);
                    max_t_idx = t_idx.max(max_t_idx);
                    max_c_idx = c_idx.max(max_c_idx);

                    indexes.push((p_idx, n_idx, t_idx, c_idx));
                    match kind {
                        VERTEX_TYPE_1BONE_DISPLAYLIST_INDEXED => start += 1,
                        VERTEX_TYPE_DISPLAYLIST_INDEXED | _ => (),
                    }
                    start += 8;
                }
                display_list_parts.push((part, indexes));
            }
            display_list_parts
        }
    }

    #[derive(Copy, Clone, Debug)]
    pub struct Actor<Data> {
        resource_info: ResourceInfo,
        soft_skin: SoftSkin,
        pub root_actor_node_offset: u32,
        flags: u32,
        last_frame: u32,
        max_primitive_vertex_count: i32,
        max_total_primitive_vertex_count: i32,
        anim_segment_offset: u32,
        number_of_anim_segments: u32,
        max_radius: f32,
        x_min: f32,
        x_max: f32,
        y_min: f32,
        y_max: f32,
        z_min: f32,
        z_max: f32,
        matrix_palette_size: u8,
        pub vertex_type: u8,
        draw_sync: u16,
        anim_event_data_offset: u32,
        anim_segment_names_offset: u32,
        node_names_offset: u32,
        last_frame_count: u32,
        actor_nodes_offset: u32,
        light_map_format: u8,
        number_of_nodes: u8,
        blend_mode_flags: u8,
        pub data: Data,
    }

    impl<Data: AsRef<[u8]>> Actor<Data> {
        pub const SIZE: usize = 0xF4;
        pub fn from_bytes(bytes: &[u8; 0xF4], data: Data) -> Self {
            Self {
                resource_info: ResourceInfo::from_bytes(
                    bytes.as_ref()[0..ResourceInfo::SIZE].try_into().unwrap(),
                ),
                soft_skin: SoftSkin::from_bytes(
                    bytes.as_ref()[ResourceInfo::SIZE..ResourceInfo::SIZE + SoftSkin::SIZE]
                        .try_into()
                        .unwrap(),
                ),
                root_actor_node_offset: u32::from_be_bytes(
                    bytes.as_ref()[160..164].try_into().unwrap(),
                ),
                flags: u32::from_be_bytes(bytes.as_ref()[164..168].try_into().unwrap()),
                last_frame: u32::from_be_bytes(bytes.as_ref()[168..172].try_into().unwrap()),
                max_primitive_vertex_count: i32::from_be_bytes(
                    bytes.as_ref()[172..176].try_into().unwrap(),
                ),
                max_total_primitive_vertex_count: i32::from_be_bytes(
                    bytes.as_ref()[176..180].try_into().unwrap(),
                ),
                anim_segment_offset: u32::from_be_bytes(
                    bytes.as_ref()[180..184].try_into().unwrap(),
                ),
                number_of_anim_segments: u32::from_be_bytes(
                    bytes.as_ref()[184..188].try_into().unwrap(),
                ),
                max_radius: f32::from_be_bytes(bytes.as_ref()[188..192].try_into().unwrap()),
                x_min: f32::from_be_bytes(bytes.as_ref()[192..196].try_into().unwrap()),
                x_max: f32::from_be_bytes(bytes.as_ref()[196..200].try_into().unwrap()),
                y_min: f32::from_be_bytes(bytes.as_ref()[200..204].try_into().unwrap()),
                y_max: f32::from_be_bytes(bytes.as_ref()[204..208].try_into().unwrap()),
                z_min: f32::from_be_bytes(bytes.as_ref()[208..212].try_into().unwrap()),
                z_max: f32::from_be_bytes(bytes.as_ref()[212..216].try_into().unwrap()),
                matrix_palette_size: bytes.as_ref()[216],
                vertex_type: bytes.as_ref()[217],
                draw_sync: u16::from_be_bytes(bytes.as_ref()[218..220].try_into().unwrap()),
                anim_event_data_offset: u32::from_be_bytes(
                    bytes.as_ref()[220..224].try_into().unwrap(),
                ),
                anim_segment_names_offset: u32::from_be_bytes(
                    bytes.as_ref()[224..228].try_into().unwrap(),
                ),
                node_names_offset: u32::from_be_bytes(bytes.as_ref()[228..232].try_into().unwrap()),
                last_frame_count: u32::from_be_bytes(bytes.as_ref()[232..236].try_into().unwrap()),
                actor_nodes_offset: u32::from_be_bytes(
                    bytes.as_ref()[236..240].try_into().unwrap(),
                ),
                light_map_format: bytes.as_ref()[240],
                number_of_nodes: bytes.as_ref()[241],
                blend_mode_flags: bytes.as_ref()[242],
                data,
            }
        }

        pub fn soft_skin(&self) -> SoftSkin {
            self.soft_skin
        }

        pub fn nodes(&self) -> Vec<ActorNode> {
            let mut vec = Vec::with_capacity(self.number_of_nodes as usize);

            let root_node = self.root_node();
            vec.push(root_node);

            let mut next_node_offset = root_node.next_actor_node_offset;
            let mut cur_node_offset = 0;
            while next_node_offset != 0 && cur_node_offset != next_node_offset {
                let node = ActorNode::from_bytes(
                    self.data.as_ref()
                        [next_node_offset as usize..next_node_offset as usize + ActorNode::SIZE]
                        .try_into()
                        .unwrap(),
                );
                cur_node_offset = next_node_offset;
                next_node_offset = node.next_actor_node_offset;
            }
            vec
        }

        pub fn root_node(&self) -> ActorNode {
            let node = ActorNode::from_bytes(
                self.data.as_ref()[self.root_actor_node_offset as usize
                    ..self.root_actor_node_offset as usize + 0x134]
                    .try_into()
                    .unwrap(),
            );
            return node;
        }
    }

    /*
        struct _TBAnimQuantisation3 {
        // total size: 0x20
        float xQuantBase; // offset 0x0, size 0x4
        float yQuantBase; // offset 0x4, size 0x4
        float zQuantBase; // offset 0x8, size 0x4
        unsigned short lastKeyOffset; // offset 0xC, size 0x2
        short pad; // offset 0xE, size 0x2
        float xQuantScale; // offset 0x10, size 0x4
        float yQuantScale; // offset 0x14, size 0x4
        float zQuantScale; // offset 0x18, size 0x4
        void * lastAnimTrack; // offset 0x1C, size 0x4
    };
        */
    #[derive(Copy, Clone, Debug)]
    pub struct AnimationQuantisation3 {
        x_quantisation_base: f32,
        y_quantisation_base: f32,
        z_quantisation_base: f32,
        last_animation_key_offset: u16,
        x_quantisation_scale: f32,
        y_quantisation_scale: f32,
        z_quantisation_scale: f32,
        last_animation_track_offset: u32,
    }

    impl AnimationQuantisation3 {
        pub const SIZE: usize = 0x20;
        pub fn from_bytes(bytes: &[u8; Self::SIZE]) -> Self {
            Self {
                x_quantisation_base: f32::from_be_bytes(bytes[0..4].try_into().unwrap()),
                y_quantisation_base: f32::from_be_bytes(bytes[4..8].try_into().unwrap()),
                z_quantisation_base: f32::from_be_bytes(bytes[8..12].try_into().unwrap()),
                last_animation_key_offset: u16::from_be_bytes(bytes[12..14].try_into().unwrap()),
                x_quantisation_scale: f32::from_be_bytes(bytes[16..20].try_into().unwrap()),
                y_quantisation_scale: f32::from_be_bytes(bytes[20..24].try_into().unwrap()),
                z_quantisation_scale: f32::from_be_bytes(bytes[24..28].try_into().unwrap()),
                last_animation_track_offset: u32::from_be_bytes(bytes[28..32].try_into().unwrap()),
            }
        }
    }

    #[derive(Copy, Clone, Debug)]
    pub struct AnimationQuantisation4 {
        x_quantisation_base: f32,
        y_quantisation_base: f32,
        z_quantisation_base: f32,
        w_quantisation_base: f32,
        x_quantisation_scale: f32,
        y_quantisation_scale: f32,
        z_quantisation_scale: f32,
        w_quantisation_scale: f32,
        last_animation_key_offset: u16,
        last_animation_track_offset: u32,
    }

    impl AnimationQuantisation4 {
        pub const SIZE: usize = 0x30;
        pub fn from_bytes(bytes: &[u8; Self::SIZE]) -> Self {
            Self {
                x_quantisation_base: f32::from_be_bytes(bytes[0..4].try_into().unwrap()),
                y_quantisation_base: f32::from_be_bytes(bytes[4..8].try_into().unwrap()),
                z_quantisation_base: f32::from_be_bytes(bytes[8..12].try_into().unwrap()),
                w_quantisation_base: f32::from_be_bytes(bytes[12..16].try_into().unwrap()),
                x_quantisation_scale: f32::from_be_bytes(bytes[16..20].try_into().unwrap()),
                y_quantisation_scale: f32::from_be_bytes(bytes[20..24].try_into().unwrap()),
                z_quantisation_scale: f32::from_be_bytes(bytes[24..28].try_into().unwrap()),
                w_quantisation_scale: f32::from_be_bytes(bytes[28..32].try_into().unwrap()),
                last_animation_key_offset: u16::from_be_bytes(bytes[32..34].try_into().unwrap()),
                last_animation_track_offset: u32::from_be_bytes(bytes[44..48].try_into().unwrap()),
            }
        }
    }

    /*
    struct _TBMesh {
        // total size: 0x70
        int noofVertices; // offset 0x0, size 0x4
        union {
            unsigned char * vertices; // offset 0x0, size 0x4
            struct _TBVertexBuffer * vertexBuffer; // offset 0x0, size 0x4
        }; // offset 0x4, size 0x4
        int noofBatches; // offset 0x8, size 0x4
        struct _TBMeshBatch * batches; // offset 0xC, size 0x4
        struct _TBMeshPrim * primitives; // offset 0x10, size 0x4
        unsigned int noofSolidBatches; // offset 0x14, size 0x4
        unsigned int noofCKeyBatches; // offset 0x18, size 0x4
        unsigned int noofAlphaBatches; // offset 0x1C, size 0x4
        int firstCKeyPrim; // offset 0x20, size 0x4
        int firstAlphaPrim; // offset 0x24, size 0x4
        int firstCKeyPrimVert; // offset 0x28, size 0x4
        int firstAlphaPrimVert; // offset 0x2C, size 0x4
        float * svVerts; // offset 0x30, size 0x4
        struct _TBShadowVolumeFace * svFaces; // offset 0x34, size 0x4
        struct _TBShadowVolumeEdge * svEdges; // offset 0x38, size 0x4
        int svNoofFaces; // offset 0x3C, size 0x4
        unsigned char * svEdgeFlags; // offset 0x40, size 0x4
        int svNoofEdges; // offset 0x44, size 0x4
        int svNoofVerts; // offset 0x48, size 0x4
        unsigned int flags; // offset 0x4C, size 0x4
        unsigned char * positionData; // offset 0x50, size 0x4
        unsigned char * normalData; // offset 0x54, size 0x4
        unsigned char * textureCoordData; // offset 0x58, size 0x4
        unsigned char * colourData; // offset 0x5C, size 0x4
        unsigned char * displayList; // offset 0x60, size 0x4
        unsigned int displayListSize; // offset 0x64, size 0x4
        union {
            struct _TBDLSegment * displaySegments; // offset 0x0, size 0x4
            struct _TBDLTextureOffset * displayTextureOffsets; // offset 0x0, size 0x4
        }; // offset 0x68, size 0x4
        unsigned int pad; // offset 0x6C, size 0x4
    };
        */
    #[derive(Copy, Clone, Debug)]
    pub struct Mesh {
        number_of_vertices: u32,
        vertext_offset: u32,
        number_of_batches: i32,
        batch_offset: u32,
        primitive_offset: u32,
        number_of_solid_batches: u32,
        number_of_color_key_batches: u32,
        number_of_alpha_batches: u32,
        first_color_key_primitive: i32,
        first_alpha_primitive: i32,
        first_color_key_primitive_vertex: i32,
        first_alpha_primitive_vertex: i32,
        shadow_volume_vertex_offset: u32,
        shadow_volume_face_offset: u32,
        shadow_volume_edge_offset: u32,
        number_of_shadow_volume_faces: i32,
        shadow_volume_edge_flags_offset: u32,
        number_of_shadow_volume_edge: i32,
        number_of_shadow_volume_vertices: i32,
        flags: u32,
        position_offset: u32,
        normal_offset: u32,
        pub texture_coord_offset: u32,
        color_offset: u32,
        display_list_offset: u32,
        display_list_size: u32,
        display_segments_offset: u32,
    }

    impl Mesh {
        pub const SIZE: usize = 0x70;
        pub fn from_bytes(bytes: &[u8; Self::SIZE]) -> Self {
            Self {
                number_of_vertices: u32::from_be_bytes(bytes[0..4].try_into().unwrap()),
                vertext_offset: u32::from_be_bytes(bytes[4..8].try_into().unwrap()),
                number_of_batches: i32::from_be_bytes(bytes[8..12].try_into().unwrap()),
                batch_offset: u32::from_be_bytes(bytes[12..16].try_into().unwrap()),
                primitive_offset: u32::from_be_bytes(bytes[16..20].try_into().unwrap()),
                number_of_solid_batches: u32::from_be_bytes(bytes[20..24].try_into().unwrap()),
                number_of_color_key_batches: u32::from_be_bytes(bytes[24..28].try_into().unwrap()),
                number_of_alpha_batches: u32::from_be_bytes(bytes[28..32].try_into().unwrap()),
                first_color_key_primitive: i32::from_be_bytes(bytes[32..36].try_into().unwrap()),
                first_alpha_primitive: i32::from_be_bytes(bytes[36..40].try_into().unwrap()),
                first_color_key_primitive_vertex: i32::from_be_bytes(
                    bytes[40..44].try_into().unwrap(),
                ),
                first_alpha_primitive_vertex: i32::from_be_bytes(bytes[44..48].try_into().unwrap()),
                shadow_volume_vertex_offset: u32::from_be_bytes(bytes[48..52].try_into().unwrap()),
                shadow_volume_face_offset: u32::from_be_bytes(bytes[52..56].try_into().unwrap()),
                shadow_volume_edge_offset: u32::from_be_bytes(bytes[56..60].try_into().unwrap()),
                number_of_shadow_volume_faces: i32::from_be_bytes(
                    bytes[60..64].try_into().unwrap(),
                ),
                shadow_volume_edge_flags_offset: u32::from_be_bytes(
                    bytes[64..68].try_into().unwrap(),
                ),
                number_of_shadow_volume_edge: i32::from_be_bytes(bytes[68..72].try_into().unwrap()),
                number_of_shadow_volume_vertices: i32::from_be_bytes(
                    bytes[72..76].try_into().unwrap(),
                ),
                flags: u32::from_be_bytes(bytes[76..80].try_into().unwrap()),
                position_offset: u32::from_be_bytes(bytes[80..84].try_into().unwrap()),
                normal_offset: u32::from_be_bytes(bytes[84..88].try_into().unwrap()),
                texture_coord_offset: u32::from_be_bytes(bytes[88..92].try_into().unwrap()),
                color_offset: u32::from_be_bytes(bytes[92..96].try_into().unwrap()),
                display_list_offset: u32::from_be_bytes(bytes[96..100].try_into().unwrap()),
                display_list_size: u32::from_be_bytes(bytes[100..104].try_into().unwrap()),
                display_segments_offset: u32::from_be_bytes(bytes[104..108].try_into().unwrap()),
            }
        }

        pub fn batches_from_buffer(&self, buffer: &[u8]) -> Vec<MeshBatch> {
            let batches_start_offset = self.batch_offset as usize;
            let batches_end_offset =
                batches_start_offset + (MeshBatch::SIZE * self.number_of_batches as usize);

            buffer[batches_start_offset..batches_end_offset]
                .chunks_exact(MeshBatch::SIZE)
                .map(|bytes| MeshBatch::from_bytes(bytes.try_into().unwrap()))
                .collect()
        }

        pub fn primitives_from_buffer(&self, buffer: &[u8]) -> Vec<MeshPrimitive> {
            let mut primitive_count = 0;
            for batch in self.batches_from_buffer(buffer) {
                primitive_count += batch.number_of_primitives
            }

            let batches_start_offset = self.primitive_offset as usize;
            let batches_end_offset =
                batches_start_offset + (MeshPrimitive::SIZE * primitive_count as usize);

            buffer[batches_start_offset..batches_end_offset]
                .chunks_exact(MeshPrimitive::SIZE)
                .map(|bytes| MeshPrimitive::from_bytes(bytes.try_into().unwrap()))
                .collect()
        }

        pub fn positions_from_buffer(&self, buffer: &[u8]) -> Vec<Position> {
            let pos_start_offset = self.position_offset as usize;

            if (pos_start_offset == 0) {
                panic!()
            }

            let pos_end_offset =
                pos_start_offset + (Position::LENGTH * self.number_of_vertices as usize);
            buffer[pos_start_offset..pos_end_offset]
                .chunks_exact(Position::LENGTH)
                .map(|bytes| Position::from_bytes(bytes.try_into().unwrap()))
                .collect()
        }

        pub fn normals_from_buffer(&self, buffer: &[u8]) -> Vec<Normal> {
            let norm_start_offset = self.normal_offset as usize;
            let norm_end_offset =
                norm_start_offset + (Normal::LENGTH * self.number_of_vertices as usize);
            buffer[norm_start_offset..norm_end_offset]
                .chunks_exact(Normal::LENGTH)
                .map(|bytes| Normal::from_bytes(bytes.try_into().unwrap()))
                .collect()
        }
        pub fn texcoords_from_buffer(&self, buffer: &[u8]) -> Vec<Texcoord> {
            let (_, _, _, number_of_texcoords) = self.max_idx(buffer);

            let norm_start_offset = self.texture_coord_offset as usize;

            if (norm_start_offset == 0) {
                panic!()
            }

            let norm_end_offset =
                norm_start_offset + (Texcoord::LENGTH * number_of_texcoords as usize + 1);
            buffer[norm_start_offset..norm_end_offset]
                .chunks_exact(Texcoord::LENGTH)
                .map(|bytes| Texcoord::from_bytes(bytes.try_into().unwrap()))
                .collect()
        }

        pub fn display_segments_from_buffer(&self, buffer: &[u8]) -> Vec<(u32, u32)> {
            let seg_start = self.display_segments_offset as usize;
            let seg_end = self.display_list_offset as usize;

            buffer[seg_start..seg_end]
                .chunks_exact(8)
                .map(|bytes| {
                    (
                        u32::from_be_bytes(bytes[0..4].try_into().unwrap()),
                        u32::from_be_bytes(bytes[4..8].try_into().unwrap()),
                    )
                })
                .collect()
        }

        pub fn max_idx<'a>(&'a self, buffer: &'a [u8]) -> (u16, u16, u16, u16) {
            let (mut max_p_idx, mut max_n_idx, mut max_t_idx, mut max_c_idx) = (0, 0, 0, 0);

            let display_list_start = self.display_list_offset;
            let display_list_end = display_list_start + self.display_list_size;

            let display_list = &buffer[display_list_start as usize..display_list_end as usize];

            for (offset, size) in self.display_segments_from_buffer(buffer) {
                let offset = offset as usize;
                let part = DisplayListPart {
                    cmd: u16::from_be_bytes(display_list[offset..offset + 2].try_into().unwrap()),
                    vertex_count: u16::from_be_bytes(
                        display_list[offset + 2..offset + 4].try_into().unwrap(),
                    ),
                };

                let mut start = offset + 4;
                for _ in 0..part.vertex_count {
                    let p_idx =
                        u16::from_be_bytes(display_list[start..start + 2].try_into().unwrap());
                    let n_idx =
                        u16::from_be_bytes(display_list[start + 2..start + 4].try_into().unwrap());
                    let t_idx =
                        u16::from_be_bytes(display_list[start + 4..start + 6].try_into().unwrap());
                    let c_idx =
                        u16::from_be_bytes(display_list[start + 6..start + 8].try_into().unwrap());

                    max_p_idx = p_idx.max(max_p_idx);
                    max_n_idx = n_idx.max(max_n_idx);
                    max_t_idx = t_idx.max(max_t_idx);
                    max_c_idx = c_idx.max(max_c_idx);

                    start += 8;
                }
            }

            (max_p_idx, max_n_idx, max_t_idx, max_c_idx)
        }

        pub fn display_list_parts_from_buffer<'a>(
            &'a self,
            buffer: &'a [u8],
        ) -> Vec<(DisplayListPart, Vec<(u16, u16, u16, u16)>)> {
            let display_list_start = self.display_list_offset;
            let display_list_end = display_list_start + self.display_list_size;

            let display_list = &buffer[display_list_start as usize..display_list_end as usize];

            let mut display_list_parts = Vec::new();
            for (offset, size) in self.display_segments_from_buffer(buffer) {
                let offset = offset as usize;
                let part = DisplayListPart {
                    cmd: u16::from_be_bytes(display_list[offset..offset + 2].try_into().unwrap()),
                    vertex_count: u16::from_be_bytes(
                        display_list[offset + 2..offset + 4].try_into().unwrap(),
                    ),
                };

                let mut indexes = Vec::new();

                let mut start = offset + 4;
                for _ in 0..part.vertex_count {
                    let p_idx =
                        u16::from_be_bytes(display_list[start..start + 2].try_into().unwrap());
                    let n_idx =
                        u16::from_be_bytes(display_list[start + 2..start + 4].try_into().unwrap());
                    let t_idx =
                        u16::from_be_bytes(display_list[start + 4..start + 6].try_into().unwrap());
                    let c_idx =
                        u16::from_be_bytes(display_list[start + 6..start + 8].try_into().unwrap());
                    indexes.push((p_idx, n_idx, t_idx, c_idx));

                    start += 8;
                }
                display_list_parts.push((part, indexes));
            }
            display_list_parts
        }
    }

    #[derive(Copy, Clone, Debug)]
    pub struct DisplayListPart {
        cmd: u16,
        vertex_count: u16,
    }

    type Position = super::Vertex;

    #[derive(Copy, Clone, Debug)]
    pub struct MeshBatch {
        pub number_of_primitives: i32,
        pub texture_1_crc: u32,
        pub texture_2_crc: u32,
        flags: u32,
    }

    impl MeshBatch {
        pub const SIZE: usize = 0x10;
        pub fn from_bytes(bytes: &[u8; Self::SIZE]) -> Self {
            Self {
                number_of_primitives: i32::from_be_bytes(bytes[0..4].try_into().unwrap()),
                texture_1_crc: u32::from_be_bytes(bytes[4..8].try_into().unwrap()),
                texture_2_crc: u32::from_be_bytes(bytes[8..12].try_into().unwrap()),
                flags: u32::from_be_bytes(bytes[12..16].try_into().unwrap()),
            }
        }
    }

    #[derive(Copy, Clone, Debug)]
    pub struct MeshPrimitive {
        primtive_type: u8,
        flags: u8,
        pub number_of_vertices: u16,
        pub number_of_draw_primitives: u16,
    }

    #[derive(Copy, Clone, Debug)]
    pub struct SoftSkinPrimitive {
        primtive_type: u8,
        flags: u8,
        pub number_of_vertices: u16,
        pub number_of_matricies: u8,
        pub number_of_draw_primitives: u8,
        pub matrix_indices: [u8; 10],
    }

    impl SoftSkinPrimitive {
        pub const SIZE: usize = 0x12;
        pub fn from_bytes(bytes: [u8; Self::SIZE]) -> Self {
            Self {
                primtive_type: bytes[0],
                flags: bytes[1],
                number_of_vertices: u16::from_be_bytes(bytes[2..4].try_into().unwrap()),
                number_of_matricies: bytes[4],
                number_of_draw_primitives: bytes[5],
                matrix_indices: bytes[8..18].try_into().unwrap(),
            }
        }
    }

    impl MeshPrimitive {
        pub const SIZE: usize = 0x8;
        pub fn from_bytes(bytes: [u8; Self::SIZE]) -> Self {
            Self {
                primtive_type: bytes[0],
                flags: bytes[1],
                number_of_vertices: u16::from_be_bytes(bytes[2..4].try_into().unwrap()),
                number_of_draw_primitives: u16::from_be_bytes(bytes[4..6].try_into().unwrap()),
            }
        }
    }

    #[derive(Copy, Clone, Debug)]
    pub struct Geometry {
        number_of_vertices: i32,
        number_of_triangles: i32,
        data_stream_1_offset: u32,
        data_stream_2_offset: u32,
        data_stream_3_offset: u32,
        data_stream_4_offset: u32,
    }

    impl Geometry {
        pub const SIZE: usize = 0x20;
        pub fn from_bytes(bytes: &[u8; Self::SIZE]) -> Self {
            Self {
                number_of_vertices: i32::from_be_bytes(bytes[0..4].try_into().unwrap()),
                number_of_triangles: i32::from_be_bytes(bytes[4..8].try_into().unwrap()),
                data_stream_1_offset: u32::from_be_bytes(bytes[16..20].try_into().unwrap()),
                data_stream_2_offset: u32::from_be_bytes(bytes[20..24].try_into().unwrap()),
                data_stream_3_offset: u32::from_be_bytes(bytes[24..28].try_into().unwrap()),
                data_stream_4_offset: u32::from_be_bytes(bytes[28..32].try_into().unwrap()),
            }
        }
    }

    #[derive(Copy, Clone, Debug)]
    pub struct ActorInfo {
        pub mesh: Mesh,
        geometry: Geometry,
    }

    impl ActorInfo {
        pub const SIZE: usize = 0x90;
        pub fn from_bytes(bytes: &[u8; Self::SIZE]) -> Self {
            Self {
                mesh: Mesh::from_bytes(bytes[0..Mesh::SIZE].try_into().unwrap()),
                geometry: Geometry::from_bytes(
                    bytes[Mesh::SIZE..Mesh::SIZE + Geometry::SIZE]
                        .try_into()
                        .unwrap(),
                ),
            }
        }
    }

    /*
        struct _TBActorNode {
        // total size: 0x134
        struct _TBAnimQuantisation3 positionQuantisationNode; // offset 0x0, size 0x20
        struct _TBAnimQuantisation4 orientationQuantisationNode; // offset 0x20, size 0x30
        struct _TBAnimQuantisation3 scaleQuantisationNode; // offset 0x50, size 0x20
        unsigned char type; // offset 0x70, size 0x1
       unsigned char flags; // offset 0x71, size 0x1
        unsigned short pad; // offset 0x72, size 0x2
        unsigned int nodeIndex; // offset 0x74, size 0x4
        unsigned int crc; // offset 0x78, size 0x4
        int noofActAnimEvents; // offset 0x7C, size 0x4
        union {
            struct _TBActorInfo_SoftBone softBone; // offset 0x0, size 0x50
            struct _TBActorInfo_Mesh mesh; // offset 0x0, size 0x90
            struct _TBActorInfo_AsyncMesh asyncMesh; // offset 0x0, size 0x90
            struct _TBActorInfo_Link link; // offset 0x0, size 0x1
        }; // offset 0x80, size 0x90
        struct _TBActorNode * next; // offset 0x110, size 0x4
        struct _TBActorNode * prev; // offset 0x114, size 0x4
        struct _TBActorNode * parent; // offset 0x118, size 0x4
        struct _TBActorNode * children; // offset 0x11C, size 0x4
        struct _TBActorAnimEvent * actAnimEvents; // offset 0x120, size 0x4
        struct _TBMorphHeader * morphTargetData; // offset 0x124, size 0x4
        struct _TBPatchHeader * patchData; // offset 0x128, size 0x4
        struct _TBActor * actor; // offset 0x12C, size 0x4
        char * name; // offset 0x130, size 0x4
    };
        */
    #[derive(Copy, Clone, Debug)]
    pub struct ActorNode {
        position_quantisation_node: AnimationQuantisation3,
        rotation_quantisation_node: AnimationQuantisation4,
        scale_quantisation_node: AnimationQuantisation3,
        kind: u8,
        flags: u8,
        node_index: u32,
        crc: u32,
        number_of_actor_animation_events: i32,
        pub actor_info: ActorInfo,
        pub next_actor_node_offset: u32,
        prev_actor_node_offset: u32,
        parent_actor_node_offset: u32,
        child_actor_node_offset: u32,
        actor_anim_events_offset: u32,
        morph_header_offset: u32,
        patch_header_offset: u32,
        actor_offset: u32,
        name_offset: u32,
    }

    impl ActorNode {
        pub const SIZE: usize = 0x134;
        pub fn from_bytes(bytes: &[u8; 0x134]) -> ActorNode {
            Self {
                position_quantisation_node: AnimationQuantisation3::from_bytes(
                    bytes[0..32].try_into().unwrap(),
                ),
                rotation_quantisation_node: AnimationQuantisation4::from_bytes(
                    bytes[32..80].try_into().unwrap(),
                ),
                scale_quantisation_node: AnimationQuantisation3::from_bytes(
                    bytes[80..112].try_into().unwrap(),
                ),
                kind: bytes[112],
                flags: bytes[113],
                node_index: u32::from_be_bytes(bytes[116..120].try_into().unwrap()),
                crc: u32::from_be_bytes(bytes[120..124].try_into().unwrap()),
                number_of_actor_animation_events: i32::from_be_bytes(
                    bytes[124..128].try_into().unwrap(),
                ),
                actor_info: ActorInfo::from_bytes(bytes[128..272].try_into().unwrap()),
                next_actor_node_offset: u32::from_be_bytes(bytes[272..276].try_into().unwrap()),
                prev_actor_node_offset: u32::from_be_bytes(bytes[276..280].try_into().unwrap()),
                parent_actor_node_offset: u32::from_be_bytes(bytes[280..284].try_into().unwrap()),
                child_actor_node_offset: u32::from_be_bytes(bytes[284..288].try_into().unwrap()),
                actor_anim_events_offset: u32::from_be_bytes(bytes[288..292].try_into().unwrap()),
                morph_header_offset: u32::from_be_bytes(bytes[292..296].try_into().unwrap()),
                patch_header_offset: u32::from_be_bytes(bytes[296..300].try_into().unwrap()),
                actor_offset: u32::from_be_bytes(bytes[300..304].try_into().unwrap()),
                name_offset: u32::from_be_bytes(bytes[304..308].try_into().unwrap()),
            }
        }

        pub fn name_from_buffer<'a>(&'a self, buffer: &'a [u8]) -> &'a str {
            CStr::from_bytes_until_nul(buffer[self.name_offset as usize..].try_into().unwrap())
                .unwrap()
                .to_str()
                .unwrap()
        }
    }
}
