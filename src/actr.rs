use crate::ParseError;

pub struct ActrReader<Data: AsRef<[u8]>> {
    input: Data,
    header: Header,
}
#[derive(Copy, Clone)]
pub struct Header {
    pub mesh_offset: u32,
    pub node_offset: u32,
    pub geometry_offset: u32,
    pub geometry_count: u32,
    pub node_end_offset: u32,
    pub model_name_offset: u32,
    pub node_string_table_offset: u32,
    pub bone_string_table_offset: u32,
}

impl Header {
    pub const LENGTH: usize = 0xF0;

    pub fn from_bytes(input: &[u8; Self::LENGTH]) -> Result<Self, ParseError> {
        let header = Self {
            mesh_offset: u32::from_be_bytes(input[0x78..0x7C].try_into().unwrap()),
            node_offset: u32::from_be_bytes(input[0xA0..0xA4].try_into().unwrap()),
            geometry_offset: u32::from_be_bytes(input[0xB4..0xB8].try_into().unwrap()),
            geometry_count: u32::from_be_bytes(input[0xB8..0xBC].try_into().unwrap()),
            node_end_offset: u32::from_be_bytes(input[0xDC..0xE0].try_into().unwrap()),
            model_name_offset: u32::from_be_bytes(input[0xE0..0xE4].try_into().unwrap()),
            node_string_table_offset: u32::from_be_bytes(input[0xE4..0xE8].try_into().unwrap()),
            bone_string_table_offset: u32::from_be_bytes(input[0xEC..0xF0].try_into().unwrap()),
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
        let node_offset = usize::try_from(self.header().node_offset).unwrap();

        let geo = self.geometry()?;

        let node_end_offset = node_offset + (geo.node_count.max(1) as usize * ActorNode::LENGTH);

        let node_bytes = self
            .input
            .as_ref()
            .get(node_offset..node_end_offset)
            .ok_or(ParseError::UnexpectedEnd)?;

        Ok(node_bytes.chunks_exact(ActorNode::LENGTH).map(|data| {
            let node_data = data.try_into().unwrap();

            ActorNode::from_bytes(node_data)
        }))
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

    pub fn header(&self) -> Header {
        self.header
    }

    pub fn data(&self) -> &[u8] {
        self.input.as_ref()
    }
}
#[derive(Copy, Clone, Debug)]
pub struct ActorNode {
    pub unk1_node_offset: u32,
    pub unk2_node_offset: u32,
    pub prev_node_offset: u32,
    pub next_node_offset: u32,
    pub name_offset: u32,
}

impl ActorNode {
    pub const LENGTH: usize = 0x140;

    pub fn from_bytes(input: &[u8; Self::LENGTH]) -> Self {
        let node = Self {
            unk1_node_offset: u32::from_be_bytes(input[0x110..0x114].try_into().unwrap()),
            unk2_node_offset: u32::from_be_bytes(input[0x114..0x118].try_into().unwrap()),
            prev_node_offset: u32::from_be_bytes(input[0x118..0x11C].try_into().unwrap()),
            next_node_offset: u32::from_be_bytes(input[0x11C..0x120].try_into().unwrap()),
            name_offset: u32::from_be_bytes(input[0x130..0x134].try_into().unwrap()),
        };

        node
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

pub struct ActorMesh {}
