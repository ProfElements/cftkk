use crate::{resource::ResourceInfo, ParseError};

/*
struct _TBCollisionMesh {
    // total size: 0x1F0
    struct _TBResourceInfo resInfo; // offset 0x0, size 0x20
    int cellRes[3]; // offset 0x20, size 0xC
    unsigned int flags; // offset 0x2C, size 0x4
    float cellSize[3]; // offset 0x30, size 0xC
    unsigned int type; // offset 0x3C, size 0x4
    float min[3]; // offset 0x40, size 0xC
    unsigned int crc; // offset 0x4C, size 0x4
    float max[3]; // offset 0x50, size 0xC
    union {
        struct _TBActorInstance * actorInstance; // offset 0x0, size 0x4
        struct _TBActorNodeInstance * nodeInstance; // offset 0x0, size 0x4
    }; // offset 0x5C, size 0x4
    struct _TBCollisionMeshVertex * vertices; // offset 0x60, size 0x4
    struct _TBCollisionMeshNormal * normals; // offset 0x64, size 0x4
    struct _TBCollisionMeshTri * tris; // offset 0x68, size 0x4
    union {
        struct _TBCollisionMeshCell * cells; // offset 0x0, size 0x4
        struct _TBCollisionMeshNode * nodes; // offset 0x0, size 0x4
        struct _TBCollisionMesh * root; // offset 0x0, size 0x4
    }; // offset 0x6C, size 0x4
    struct _TBCollisionMesh * next; // offset 0x70, size 0x4
    struct _TBCollisionMesh * prev; // offset 0x74, size 0x4
    struct _TBCollisionMesh * parent; // offset 0x78, size 0x4
    struct _TBCollisionMesh * children; // offset 0x7C, size 0x4
    unsigned int noofVertices; // offset 0x80, size 0x4
    unsigned int noofNormals; // offset 0x84, size 0x4
    unsigned int noofTris; // offset 0x88, size 0x4
    int refCount; // offset 0x8C, size 0x4
    float position[4]; // offset 0x90, size 0x10
    float orientation[4]; // offset 0xA0, size 0x10
    float velocity[4]; // offset 0xB0, size 0x10
    float angularVelocity[4]; // offset 0xC0, size 0x10
    float lastNodeToWorld[4][4]; // offset 0xD0, size 0x40
    float nodeToWorld[4][4]; // offset 0x110, size 0x40
    float invNodeToWorld[4][4]; // offset 0x150, size 0x40
    float momentOfInertia[4][4]; // offset 0x190, size 0x40
    float centreOfGravity[4]; // offset 0x1D0, size 0x10
    float volume; // offset 0x1E0, size 0x4
    int dataSize; // offset 0x1E4, size 0x4
    int pad[2]; // offset 0x1E8, size 0x8
};
*/
#[derive(Copy, Clone, Debug)]
pub struct Header {
    resource_info: ResourceInfo,
    cell_resolution_x: i32,
    cell_resolution_y: i32,
    cell_resolution_z: i32,
    flags: u32,
    cell_size_x: f32,
    cell_size_y: f32,
    cell_size_z: f32,
    pub kind: Kind,
    min_x: f32,
    min_y: f32,
    min_z: f32,
    crc: u32,
    max_x: f32,
    max_y: f32,
    max_z: f32,
    instance_offset: u32,
    vertex_offset: u32,
    normal_offset: u32,
    tri_offset: u32,
    pub cell_node_root_offset: u32,
    pub next_mesh_offset: u32,
    prev_mesh_offset: u32,
    parent_mesh_offset: u32,
    pub child_mesh_offset: u32,
    vertex_count: u32,
    normal_count: u32,
    tri_count: u32,
    ref_count: i32,
    pos_x: f32,
    pos_y: f32,
    pos_z: f32,
    pos_w: f32,
    rot_x: f32,
    rot_y: f32,
    rot_z: f32,
    rot_w: f32,
    vel_x: f32,
    vel_y: f32,
    vel_z: f32,
    vel_w: f32,
    ang_vel_x: f32,
    ang_vel_y: f32,
    ang_vel_z: f32,
    ang_vel_w: f32,
    last_node_to_world: Mat4x4,
    node_to_world: Mat4x4,
    inverse_node_to_world: Mat4x4,
    moment_of_intertia: Mat4x4,
    centre_x: f32,
    centre_y: f32,
    centre_z: f32,
    centre_w: f32,
}

impl Header {
    pub const LENGTH: usize = 0x1F0;

    pub fn from_bytes(data: &[u8; Self::LENGTH]) -> Result<Self, ()> {
        Ok(Self {
            resource_info: ResourceInfo::from_bytes(
                data[0..ResourceInfo::LENGTH].try_into().unwrap(),
            )?,
            cell_resolution_x: i32::from_be_bytes(data[32..36].try_into().unwrap()),
            cell_resolution_y: i32::from_be_bytes(data[36..40].try_into().unwrap()),
            cell_resolution_z: i32::from_be_bytes(data[40..44].try_into().unwrap()),
            flags: u32::from_be_bytes(data[44..48].try_into().unwrap()),
            cell_size_x: f32::from_be_bytes(data[48..52].try_into().unwrap()),
            cell_size_y: f32::from_be_bytes(data[52..56].try_into().unwrap()),
            cell_size_z: f32::from_be_bytes(data[56..60].try_into().unwrap()),
            kind: Kind::try_from(u32::from_be_bytes(data[60..64].try_into().unwrap()))?,
            min_x: f32::from_be_bytes(data[64..68].try_into().unwrap()),
            min_y: f32::from_be_bytes(data[68..72].try_into().unwrap()),
            min_z: f32::from_be_bytes(data[72..76].try_into().unwrap()),
            crc: u32::from_be_bytes(data[76..80].try_into().unwrap()),
            max_x: f32::from_be_bytes(data[80..84].try_into().unwrap()),
            max_y: f32::from_be_bytes(data[84..88].try_into().unwrap()),
            max_z: f32::from_be_bytes(data[88..92].try_into().unwrap()),
            instance_offset: u32::from_be_bytes(data[92..96].try_into().unwrap()),
            vertex_offset: u32::from_be_bytes(data[96..100].try_into().unwrap()),
            normal_offset: u32::from_be_bytes(data[100..104].try_into().unwrap()),
            tri_offset: u32::from_be_bytes(data[104..108].try_into().unwrap()),
            cell_node_root_offset: u32::from_be_bytes(data[108..112].try_into().unwrap()),
            next_mesh_offset: u32::from_be_bytes(data[112..116].try_into().unwrap()),
            prev_mesh_offset: u32::from_be_bytes(data[116..120].try_into().unwrap()),
            parent_mesh_offset: u32::from_be_bytes(data[120..124].try_into().unwrap()),
            child_mesh_offset: u32::from_be_bytes(data[124..128].try_into().unwrap()),
            vertex_count: u32::from_be_bytes(data[128..132].try_into().unwrap()),
            normal_count: u32::from_be_bytes(data[132..136].try_into().unwrap()),
            tri_count: u32::from_be_bytes(data[136..140].try_into().unwrap()),
            ref_count: i32::from_be_bytes(data[140..144].try_into().unwrap()),
            pos_x: f32::from_be_bytes(data[144..148].try_into().unwrap()),
            pos_y: f32::from_be_bytes(data[148..152].try_into().unwrap()),
            pos_z: f32::from_be_bytes(data[152..156].try_into().unwrap()),
            pos_w: f32::from_be_bytes(data[156..160].try_into().unwrap()),
            rot_x: f32::from_be_bytes(data[160..164].try_into().unwrap()),
            rot_y: f32::from_be_bytes(data[164..168].try_into().unwrap()),
            rot_z: f32::from_be_bytes(data[168..172].try_into().unwrap()),
            rot_w: f32::from_be_bytes(data[172..176].try_into().unwrap()),
            vel_x: f32::from_be_bytes(data[176..180].try_into().unwrap()),
            vel_y: f32::from_be_bytes(data[180..184].try_into().unwrap()),
            vel_z: f32::from_be_bytes(data[184..188].try_into().unwrap()),
            vel_w: f32::from_be_bytes(data[188..192].try_into().unwrap()),
            ang_vel_x: f32::from_be_bytes(data[192..196].try_into().unwrap()),
            ang_vel_y: f32::from_be_bytes(data[196..200].try_into().unwrap()),
            ang_vel_z: f32::from_be_bytes(data[200..204].try_into().unwrap()),
            ang_vel_w: f32::from_be_bytes(data[204..208].try_into().unwrap()),
            last_node_to_world: Mat4x4::from_bytes(data[208..272].try_into().unwrap()),
            node_to_world: Mat4x4::from_bytes(data[272..336].try_into().unwrap()),
            inverse_node_to_world: Mat4x4::from_bytes(data[336..400].try_into().unwrap()),
            moment_of_intertia: Mat4x4::from_bytes(data[400..464].try_into().unwrap()),
            centre_x: f32::from_be_bytes(data[464..468].try_into().unwrap()),
            centre_y: f32::from_be_bytes(data[468..472].try_into().unwrap()),
            centre_z: f32::from_be_bytes(data[472..476].try_into().unwrap()),
            centre_w: f32::from_be_bytes(data[476..480].try_into().unwrap()),
        })
    }
}

/*
enum EBCollisionMeshType {
    BCOLLISIONMESHTYPES = 3,
    BCOLLISIONMESHTYPE_HIERARCHICAL = 2,
    BCOLLISIONMESHTYPE_AABBTREE = 1,
    BCOLLISIONMESHTYPE_CELLBASED = 0,
};
*/
#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Kind {
    Cell = 0,
    AabbTree = 1,
    Heirarchy = 2,
}

impl TryFrom<u32> for Kind {
    type Error = ();
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Cell),
            1 => Ok(Self::AabbTree),
            2 => Ok(Self::Heirarchy),
            _ => Err(()),
        }
    }
}

pub struct Reader<Data: AsRef<[u8]>> {
    input: Data,
    header: Header,
}

impl<Data: AsRef<[u8]>> Reader<Data> {
    pub fn new(input: Data) -> Result<Self, crate::ParseError> {
        if input.as_ref().len() < Header::LENGTH {
            return Err(ParseError::UnexpectedEnd);
        }

        let header = Header::from_bytes(
            input
                .as_ref()
                .get(0..Header::LENGTH)
                .ok_or(ParseError::UnexpectedEnd)?
                .try_into()
                .unwrap(),
        )
        .map_err(|_| ParseError::UnexpectedEnd)?;

        Ok(Self { input, header })
    }

    pub fn header(&self) -> Header {
        self.header
    }

    pub fn data(&self) -> &[u8] {
        self.input.as_ref()
    }
}
/*
struct _TBCollisionMeshNode {
    // total size: 0xC
    unsigned char flags; // offset 0x0, size 0x1
    unsigned char pad; // offset 0x1, size 0x1
    unsigned char extentMin[3]; // offset 0x2, size 0x3
    unsigned char extentMax[3]; // offset 0x5, size 0x3
    unsigned short left; // offset 0x8, size 0x2
    unsigned short right; // offset 0xA, size 0x2
};
*/
#[derive(Debug)]
pub struct Node {
    flags: u8,
    min_x: u8,
    min_y: u8,
    min_z: u8,
    max_x: u8,
    max_y: u8,
    max_z: u8,
    left: u16,
    right: u16,
}

impl Node {
    pub const LENGTH: usize = 0xC;
    pub fn from_bytes(data: &[u8; Self::LENGTH]) -> Self {
        Self {
            flags: data[0],
            min_x: data[2],
            min_y: data[3],
            min_z: data[4],
            max_x: data[5],
            max_y: data[6],
            max_z: data[7],
            left: u16::from_be_bytes(data[8..10].try_into().unwrap()),
            right: u16::from_be_bytes(data[10..12].try_into().unwrap()),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Mat4x4([f32; 16]);

impl Mat4x4 {
    pub const LENGTH: usize = 0x40;
    pub fn from_bytes(data: &[u8; Self::LENGTH]) -> Self {
        let mut float = [0f32; 16];

        for (idx, chunk) in data.chunks_exact(core::mem::size_of::<f32>()).enumerate() {
            float[idx] = f32::from_be_bytes(chunk.try_into().unwrap());
        }

        Self(float)
    }
}
