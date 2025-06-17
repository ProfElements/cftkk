/*
struct _TBResourceInfo {
    // total size: 0x20
    union {
        struct _TBPackageID packageId; // offset 0x0, size 0x4
        unsigned int packageId32; // offset 0x0, size 0x4
    }; // offset 0x0, size 0x4
    unsigned short groupId; // offset 0x4, size 0x2
    unsigned char type; // offset 0x6, size 0x1
    unsigned char pad1; // offset 0x7, size 0x1
    union {
        int iContext; // offset 0x0, size 0x4
        unsigned int uiContext; // offset 0x0, size 0x4
        short sContext[2]; // offset 0x0, size 0x4
        unsigned short usContext[2]; // offset 0x0, size 0x4
        float fContext; // offset 0x0, size 0x4
        char cContext[4]; // offset 0x0, size 0x4
        unsigned char ucContext[4]; // offset 0x0, size 0x4
        void * pContext; // offset 0x0, size 0x4
    }; // offset 0x8, size 0x4
    unsigned int crc; // offset 0xC, size 0x4
    struct _TBResourceInfo * child1; // offset 0x10, size 0x4
    struct _TBResourceInfo * child2; // offset 0x14, size 0x4
    struct _TBResourceInfo * parent; // offset 0x18, size 0x4
    int pad; // offset 0x1C, size 0x4
};
*/

/*
struct _TBPackageID {
    // total size: 0x4
    unsigned int crc : 31; // offset 0x0, size 0x4
    unsigned int loaded : 1; // offset 0x0, size 0x4
};
*/

#[derive(Copy, Clone, Debug)]
pub struct PackageId(u32);

#[derive(Copy, Clone, Debug)]
pub struct ResourceInfo {
    package_id: PackageId,
    group_id: u16,
    pub resource_kind: Kind,
    context: u32,
    crc: u32,
    child1_offset: u32,
    child2_offset: u32,
    parent_offset: u32,
}

impl ResourceInfo {
    pub const LENGTH: usize = 0x20;

    pub fn from_bytes(data: &[u8; Self::LENGTH]) -> Result<Self, ()> {
        Ok(Self {
            package_id: PackageId(u32::from_be_bytes(data[0..4].try_into().unwrap())),
            group_id: u16::from_be_bytes(data[4..6].try_into().unwrap()),
            resource_kind: Kind::try_from(data[6])?,
            context: u32::from_be_bytes(data[8..12].try_into().unwrap()),
            crc: u32::from_be_bytes(data[12..16].try_into().unwrap()),
            child1_offset: u32::from_be_bytes(data[16..20].try_into().unwrap()),
            child2_offset: u32::from_be_bytes(data[20..24].try_into().unwrap()),
            parent_offset: u32::from_be_bytes(data[24..28].try_into().unwrap()),
        })
    }
}

/*
enum EBResourceType {
    EBRESTYPE_ANY = 22,
    BNOOFRESTYPES = 21,
    EBRESTYPE_PIXELSHADER = 20,
    EBRESTYPE_VERTEXSHADER = 19,
    EBRESTYPE_BLENDER = 18,
    EBRESTYPE_MATERIAL = 17,
    EBRESTYPE_SUBTITLE = 16,
    EBRESTYPE_SIMULATIONDATA = 15,
    EBRESTYPE_LIGHTMATRIX = 14,
    EBRESTYPE_SOUNDPATCH = 13,
    EBRESTYPE_SOUNDBANK = 12,
    EBRESTYPE_FLAREWORLD = 11,
    EBRESTYPE_FLAREMESH = 10,
    EBRESTYPE_LIPSYNC = 9,
    EBRESTYPE_AUDIOSTREAM = 8,
    EBRESTYPE_COLLISIONMESH = 7,
    EBRESTYPE_SET = 6,
    EBRESTYPE_SPLASH = 5,
    EBRESTYPE_STRINGTABLE = 4,
    EBRESTYPE_FONT = 3,
    EBRESTYPE_SAMPLE = 2,
    EBRESTYPE_ACTOR = 1,
    EBRESTYPE_TEXTURE = 0,
    EBRESTYPE_NONE = 255,
};
*/
#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Kind {
    None = 255,
    PixelShader = 20,
    VertexShader = 19,
    Blender = 18,
    Material = 17,
    SubTitle = 16,
    SimulationData = 15,
    LightMatrix = 14,
    SoundPatch = 13,
    SoundBank = 12,
    FlareWorld = 11,
    FlareMesh = 10,
    LipSync = 9,
    AudioStream = 8,
    CollisionMesh = 7,
    Set = 6,
    Splash = 5,
    StringTable = 4,
    Font = 3,
    Sample = 2,
    Actor = 1,
    Texture = 0,
}

impl TryFrom<u8> for Kind {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Texture),
            1 => Ok(Self::Actor),
            2 => Ok(Self::Sample),
            3 => Ok(Self::Font),
            4 => Ok(Self::StringTable),
            5 => Ok(Self::Splash),
            6 => Ok(Self::Set),
            7 => Ok(Self::CollisionMesh),
            8 => Ok(Self::AudioStream),
            9 => Ok(Self::LipSync),
            10 => Ok(Self::FlareMesh),
            11 => Ok(Self::FlareWorld),
            12 => Ok(Self::SoundBank),
            13 => Ok(Self::SoundPatch),
            14 => Ok(Self::LightMatrix),
            15 => Ok(Self::SimulationData),
            16 => Ok(Self::SubTitle),
            17 => Ok(Self::Material),
            18 => Ok(Self::Blender),
            19 => Ok(Self::VertexShader),
            20 => Ok(Self::PixelShader),
            255 => Ok(Self::None),
            _ => Err(()),
        }
    }
}

impl TryFrom<&[u8; 4]> for Kind {
    type Error = ();
    fn try_from(value: &[u8; 4]) -> Result<Self, Self::Error> {
        match value {
            b"TEXR" => Ok(Self::Texture),
            b"ACTR" => Ok(Self::Actor),
            b"SAMP" => Ok(Self::Sample),
            b"FONT" => Ok(Self::Font),
            b"STAB" => Ok(Self::StringTable),
            b"SPLA" => Ok(Self::Splash),
            b"SET " => Ok(Self::Set),
            b"CMES" => Ok(Self::CollisionMesh),
            b"ASTR" | b"_STR" | b"@STR" => Ok(Self::AudioStream),
            b"LIPS" => Ok(Self::LipSync),
            b"FMSH" => Ok(Self::FlareMesh),
            b"FWRL" => Ok(Self::FlareWorld),
            b"SBNK" => Ok(Self::SoundBank),
            b"SPAT" => Ok(Self::SoundPatch),
            b"LMTX" => Ok(Self::LightMatrix),
            b"SIMD" => Ok(Self::SimulationData),
            b"SUBT" => Ok(Self::SubTitle),
            b"MTRL" => Ok(Self::Material),
            b"BLDR" => Ok(Self::Blender),
            b"VSHR" => Ok(Self::VertexShader),
            b"PSHR" => Ok(Self::PixelShader),
            _ => Err(()),
        }
    }
}
