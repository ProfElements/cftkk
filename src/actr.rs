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
                node_offset = node_offset + node_size;
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
            if indexes_bytes.get(offset..offset + 2).unwrap_or(&[0, 0]) == &[0x0, 0x99] {
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
                    .map(|data| {
                        let index_data = data.try_into().unwrap();
                        Index::from_bytes(index_data)
                    })
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
            0x8 => return index,
            0x9 => {
                index.tag = input[8];
                return index;
            }
            0x10 => {
                return Self {
                    unk1: u16::from_be_bytes(input[0..2].try_into().unwrap()),
                    pos_idx: u16::from_be_bytes(input[2..4].try_into().unwrap()),
                    normal_idx: u16::from_be_bytes(input[4..8].try_into().unwrap()),
                    color_idx: u16::from_be_bytes(input[8..10].try_into().unwrap()),
                    texcoord_idx: u16::from_be_bytes(input[10..12].try_into().unwrap()),
                    tag: 0,
                }
            }
            _ => return index,
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
        let node = Self {
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
        };

        node
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

        let texcoord_bytes = self
            .data
            .as_ref()
            .get(texcoord_offset..self.next_node_offset as usize)
            .ok_or(ParseError::UnexpectedEnd)?;

        Ok(texcoord_bytes.chunks_exact(Texcoord::LENGTH).map(|data| {
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
            if indexes_bytes.get(offset..offset + 2).unwrap_or(&[0, 0]) == &[0x0, 0x99] {
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
                    .map(|data| {
                        let index_data = data.try_into().unwrap();
                        Index::from_bytes(index_data)
                    })
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
        let geo = Self {
            node_count: u32::from_be_bytes(input[0x30..0x34].try_into().unwrap()),
            index_buffer_offset: u32::from_be_bytes(input[0x38..0x3C].try_into().unwrap()),
            node_offset: u32::from_be_bytes(input[0x3C..0x40].try_into().unwrap()),
        };

        geo
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
