use crate::ParseError;

pub struct CMesReader<Data: AsRef<[u8]>> {
    input: Data,
    header: Header,
}

impl<Data: AsRef<[u8]>> CMesReader<Data> {
    pub fn new(input: Data) -> Result<Self, ParseError> {
        if input.as_ref().len() < Header::LENGTH {
            return Err(ParseError::UnexpectedEnd);
        }

        let header_data = input
            .as_ref()
            .get(0..Header::LENGTH)
            .ok_or(ParseError::UnexpectedEnd)?;
        let header = if let Ok(header) = Header::from_bytes(header_data.try_into().unwrap()) {
            header
        } else {
            let mesh_offset = input
                .as_ref()
                .get(0x6C..0x70)
                .ok_or(ParseError::UnexpectedEnd)?;
            let mesh_offset = u32::from_be_bytes(mesh_offset.try_into().unwrap());

            let header_data = input
                .as_ref()
                .get(
                    usize::try_from(mesh_offset).unwrap()
                        ..usize::try_from(mesh_offset).unwrap() + Header::LENGTH,
                )
                .ok_or(ParseError::UnexpectedEnd)?;
            Header::from_bytes(header_data.try_into().unwrap())?
        };

        let verticies_end_offset = header
            .vertices_offset
            .checked_add(
                header
                    .vertices_count
                    .checked_mul(u32::try_from(Vertex::LENGTH).unwrap())
                    .ok_or(ParseError::UnexpectedEnd)?,
            )
            .ok_or(ParseError::UnexpectedEnd)?;

        if header.vertices_offset > verticies_end_offset
            || usize::try_from(verticies_end_offset).unwrap() >= input.as_ref().len()
        {
            return Err(ParseError::UnexpectedEnd);
        }

        let normal_end_offset = header
            .normal_maybe_offset
            .checked_add(
                header
                    .normal_count
                    .checked_mul(u32::try_from(Normal::LENGTH).unwrap())
                    .ok_or(ParseError::UnexpectedEnd)?,
            )
            .ok_or(ParseError::UnexpectedEnd)?;

        if header.normal_maybe_offset > normal_end_offset
            || usize::try_from(normal_end_offset).unwrap() >= input.as_ref().len()
        {
            return Err(ParseError::UnexpectedEnd);
        }

        let triangle_end_offset = header
            .triangle_offset
            .checked_add(
                header
                    .triangle_count
                    .checked_mul(u32::try_from(Triangle::LENGTH).unwrap())
                    .ok_or(ParseError::UnexpectedEnd)?,
            )
            .ok_or(ParseError::UnexpectedEnd)?;

        if header.triangle_offset > triangle_end_offset
            || usize::try_from(triangle_end_offset).unwrap() >= input.as_ref().len()
        {
            return Err(ParseError::UnexpectedEnd);
        }

        let node_end_offset = header
            .node_offset
            .checked_add(
                header
                    .triangle_count
                    .checked_mul(u32::try_from(MeshNode::LENGTH).unwrap())
                    .ok_or(ParseError::UnexpectedEnd)?,
            )
            .ok_or(ParseError::UnexpectedEnd)?;

        if header.node_offset > node_end_offset
            || usize::try_from(node_end_offset).unwrap() > input.as_ref().len()
        {
            return Err(ParseError::UnexpectedEnd);
        }

        if header.next_mesh_offset != 0 {
            CMesReader::new(
                input
                    .as_ref()
                    .get(usize::try_from(header.next_mesh_offset).unwrap()..)
                    .unwrap(),
            )?;
        }

        Ok(Self { input, header })
    }

    pub fn header(&self) -> Header {
        self.header
    }

    pub fn vertices(&self) -> Result<impl ExactSizeIterator<Item = Vertex> + '_, ParseError> {
        let vertex_count = self.header().vertices_count;

        let vertices_end_offset = self
            .header()
            .vertices_offset
            .checked_add(
                vertex_count
                    .checked_mul(u32::try_from(Vertex::LENGTH).unwrap())
                    .ok_or(ParseError::UnexpectedEnd)?,
            )
            .ok_or(ParseError::UnexpectedEnd)?;

        let vertices_bytes = self
            .input
            .as_ref()
            .get(
                usize::try_from(self.header().vertices_offset).unwrap()
                    ..usize::try_from(vertices_end_offset).unwrap(),
            )
            .ok_or(ParseError::UnexpectedEnd)?;

        Ok(vertices_bytes.chunks_exact(Vertex::LENGTH).map(|data| {
            let vertex_data = data.try_into().unwrap();
            Vertex::from_bytes(vertex_data)
        }))
    }

    pub fn normals(&self) -> Result<impl ExactSizeIterator<Item = Normal> + '_, ParseError> {
        let normal_count = self.header().normal_count;

        let normal_end_offset = self
            .header()
            .normal_maybe_offset
            .checked_add(
                normal_count
                    .checked_mul(u32::try_from(Normal::LENGTH).unwrap())
                    .ok_or(ParseError::UnexpectedEnd)?,
            )
            .ok_or(ParseError::UnexpectedEnd)?;

        let normal_bytes = self
            .input
            .as_ref()
            .get(
                usize::try_from(self.header().normal_maybe_offset).unwrap()
                    ..usize::try_from(normal_end_offset).unwrap(),
            )
            .ok_or(ParseError::UnexpectedEnd)?;

        Ok(normal_bytes.chunks_exact(Normal::LENGTH).map(|data| {
            let normal_data = data.try_into().unwrap();
            Normal::from_bytes(normal_data)
        }))
    }

    pub fn triangles(&self) -> Result<impl ExactSizeIterator<Item = Triangle> + '_, ParseError> {
        let triangle_count = self.header().triangle_count;

        let triangle_end_offset = self
            .header()
            .triangle_offset
            .checked_add(
                triangle_count
                    .checked_mul(u32::try_from(Triangle::LENGTH).unwrap())
                    .ok_or(ParseError::UnexpectedEnd)?,
            )
            .ok_or(ParseError::UnexpectedEnd)?;

        let triangle_bytes = self
            .input
            .as_ref()
            .get(
                usize::try_from(self.header().triangle_offset).unwrap()
                    ..usize::try_from(triangle_end_offset).unwrap(),
            )
            .ok_or(ParseError::UnexpectedEnd)?;

        Ok(triangle_bytes.chunks_exact(Triangle::LENGTH).map(|data| {
            let triangle_data = data.try_into().unwrap();
            Triangle::from_bytes(triangle_data)
        }))
    }

    pub fn nodes(&self) -> Result<impl ExactSizeIterator<Item = MeshNode> + '_, ParseError> {
        let node_count = self.header().triangle_count;

        let node_end_offset = self
            .header()
            .node_offset
            .checked_add(
                node_count
                    .checked_mul(u32::try_from(MeshNode::LENGTH).unwrap())
                    .ok_or(ParseError::UnexpectedEnd)?,
            )
            .ok_or(ParseError::UnexpectedEnd)?;

        let node_bytes = self
            .input
            .as_ref()
            .get(
                usize::try_from(self.header().node_offset).unwrap()
                    ..usize::try_from(node_end_offset).unwrap(),
            )
            .ok_or(ParseError::UnexpectedEnd)?;

        Ok(node_bytes.chunks_exact(MeshNode::LENGTH).map(|data| {
            let node_data = data.try_into().unwrap();
            MeshNode::from_bytes(node_data)
        }))
    }

    pub fn meshes(
        &self,
    ) -> Result<impl ExactSizeIterator<Item = CMesReader<&[u8]>> + '_, ParseError> {
        if self.header().next_mesh_offset != 0 {
            Ok(core::iter::once(CMesReader::new(
                self.input
                    .as_ref()
                    .get(usize::try_from(self.header().next_mesh_offset).unwrap()..)
                    .unwrap(),
            )?))
        } else {
            return Err(ParseError::ZeroOffset);
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Header {
    pub version_flag: u32,
    pub min_x: f32,
    pub min_y: f32,
    pub min_z: f32,
    pub max_x: f32,
    pub max_y: f32,
    pub max_z: f32,
    pub vertices_offset: u32,
    pub normal_maybe_offset: u32,
    pub triangle_offset: u32,
    pub node_offset: u32,
    pub next_mesh_offset: u32,
    pub vertices_count: u32,
    pub normal_count: u32,
    pub triangle_count: u32,
}

impl Header {
    pub const LENGTH: usize = 140;
    pub fn from_bytes(input: &[u8; 140]) -> Result<Self, ParseError> {
        let header = Self {
            version_flag: u32::from_be_bytes(input[0x3C..0x40].try_into().unwrap()),
            min_x: f32::from_be_bytes(input[0x40..0x44].try_into().unwrap()),
            min_y: f32::from_be_bytes(input[0x44..0x48].try_into().unwrap()),
            min_z: f32::from_be_bytes(input[0x48..0x4C].try_into().unwrap()),
            max_x: f32::from_be_bytes(input[0x50..0x54].try_into().unwrap()),
            max_y: f32::from_be_bytes(input[0x54..0x58].try_into().unwrap()),
            max_z: f32::from_be_bytes(input[0x58..0x5C].try_into().unwrap()),
            vertices_offset: u32::from_be_bytes(input[0x60..0x64].try_into().unwrap()),
            normal_maybe_offset: u32::from_be_bytes(input[0x64..0x68].try_into().unwrap()),
            triangle_offset: u32::from_be_bytes(input[0x68..0x6C].try_into().unwrap()),
            node_offset: u32::from_be_bytes(input[0x6C..0x70].try_into().unwrap()),
            next_mesh_offset: u32::from_be_bytes(input[0x7C..0x80].try_into().unwrap()),
            vertices_count: u32::from_be_bytes(input[0x80..0x84].try_into().unwrap()),
            normal_count: u32::from_be_bytes(input[0x84..0x88].try_into().unwrap()),
            triangle_count: u32::from_be_bytes(input[0x88..0x8C].try_into().unwrap()),
        };

        if header.vertices_offset == 0 {
            return Err(ParseError::ZeroOffset);
        }

        if header.normal_maybe_offset == 0 {
            return Err(ParseError::ZeroOffset);
        }

        if header.triangle_offset == 0 {
            return Err(ParseError::ZeroOffset);
        }

        if header.node_offset == 0 {
            return Err(ParseError::ZeroOffset);
        }

        if header.vertices_count == 0 {
            return Err(ParseError::ZeroVertices);
        }

        if header.normal_count == 0 {
            return Err(ParseError::ZeroNormals);
        }

        if header.triangle_count == 0 {
            return Err(ParseError::ZeroTriangles);
        }

        Ok(header)
    }
}

pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vertex {
    pub const LENGTH: usize = 12;

    pub fn from_bytes(data: &[u8; Self::LENGTH]) -> Self {
        Self {
            x: f32::from_be_bytes(data[0..4].try_into().unwrap()),
            y: f32::from_be_bytes(data[4..8].try_into().unwrap()),
            z: f32::from_be_bytes(data[8..12].try_into().unwrap()),
        }
    }
}

pub struct Normal {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Normal {
    pub const LENGTH: usize = 12;

    pub fn from_bytes(data: &[u8; Self::LENGTH]) -> Self {
        Self {
            x: f32::from_be_bytes(data[0..4].try_into().unwrap()),
            y: f32::from_be_bytes(data[4..8].try_into().unwrap()),
            z: f32::from_be_bytes(data[8..12].try_into().unwrap()),
        }
    }
}

pub struct MeshNode {
    data: [u8; 12],
}

impl MeshNode {
    pub const LENGTH: usize = 12;

    pub fn from_bytes(data: &[u8; Self::LENGTH]) -> Self {
        Self { data: *data }
    }
}

pub struct Triangle {
    pub x_idx: u16,
    pub y_idx: u16,
    pub z_idx: u16,
    pub normal_idx: u16,
}

impl Triangle {
    pub const LENGTH: usize = 16;

    pub fn from_bytes(data: &[u8; Self::LENGTH]) -> Self {
        Self {
            x_idx: u16::from_be_bytes(data[0..2].try_into().unwrap()),
            y_idx: u16::from_be_bytes(data[2..4].try_into().unwrap()),
            z_idx: u16::from_be_bytes(data[4..6].try_into().unwrap()),
            normal_idx: u16::from_be_bytes(data[6..8].try_into().unwrap()),
        }
    }
}
