use crate::ParseError;

#[derive(Debug, PartialEq, Eq)]
pub enum Tag {
    Splash,
    Texture,
    Actor,
    Sample,
    StringTable,
    Set,
    CollisionMesh,
    FaceMesh,
    AudioStream,
    FaceWorld,
    SoundBank,
    SoundPatch,
    LightMatrix,
    Subtitle,
    Material,
    Bldr,
    VertexShader,
    PixelShader,
    LipSync,
    SimulationData,
    Font,
    None,
}

impl Tag {
    pub const LENGTH: usize = 4;
    pub fn from_str(tag: &str) -> Option<Tag> {
        match tag {
            "SPLA" => Some(Tag::Splash),
            _ => {
                println!("{tag}");
                return None;
            }
        }
    }

    pub fn from_bytes(data: &[u8; Tag::LENGTH]) -> Self {
        match data {
            b"SPLA" => Tag::Splash,
            b"TEXR" => Tag::Texture,
            b"ACTR" => Tag::Actor,
            b"SAMP" => Tag::Sample,
            b"STAB" => Tag::StringTable,
            b"SET " => Tag::Set,
            b"CMES" => Tag::CollisionMesh,
            b"ASTR" | b"_STR" | b"@STR" => Tag::AudioStream,
            b"FMSH" => Tag::FaceMesh,
            b"FWRL" => Tag::FaceWorld,
            b"SBNK" => Tag::SoundBank,
            b"SPAT" => Tag::SoundPatch,
            b"LTMX" => Tag::LightMatrix,
            b"SUBT" => Tag::Subtitle,
            b"MTRL" => Tag::Material,
            b"BLDR" => Tag::Bldr,
            b"VSHR" => Tag::VertexShader,
            b"PSHR" => Tag::PixelShader,
            b"LIPS" => Tag::LipSync,
            b"SIMD" => Tag::SimulationData,
            b"FONT" => Tag::Font,
            _ => Tag::None,
        }
    }
}

#[derive(Debug)]
pub struct FileInfo {
    data_offset: u32,
    crc: u32,
    file_size: u32,
    name_offset: u32,
    is_local: bool,
    resource_tag_offset: u32,
}

impl FileInfo {
    pub const LENGTH: usize = 32;
    pub fn from_be_bytes(bytes: [u8; 0x20]) -> Self {
        Self {
            data_offset: u32::from_be_bytes(bytes[0..4].try_into().unwrap()),
            crc: u32::from_be_bytes(bytes[4..8].try_into().unwrap()),
            file_size: u32::from_be_bytes(bytes[8..12].try_into().unwrap()),
            name_offset: u32::from_be_bytes(bytes[12..16].try_into().unwrap()),
            is_local: u32::from_be_bytes(bytes[16..20].try_into().unwrap()) != 0,
            resource_tag_offset: u32::from_be_bytes(bytes[20..24].try_into().unwrap()),
        }
    }

    pub fn from_bytes(data: &[u8; FileInfo::LENGTH]) -> Self {
        Self {
            data_offset: u32::from_be_bytes(data[0..4].try_into().unwrap()),
            crc: u32::from_be_bytes(data[4..8].try_into().unwrap()),
            file_size: u32::from_be_bytes(data[8..12].try_into().unwrap()),
            name_offset: u32::from_be_bytes(data[12..16].try_into().unwrap()),
            is_local: u32::from_be_bytes(data[16..20].try_into().unwrap()) != 0,
            resource_tag_offset: u32::from_be_bytes(data[20..24].try_into().unwrap()),
        }
    }
}

pub struct ResourceEntry<'a> {
    pub name: &'a str,
    pub data: &'a [u8],
    pub tag: Tag,
    pub is_local: bool,
    pub crc: u32,
}

pub struct GcpReader<Data: AsRef<[u8]>> {
    input: Data,
    header: Header,
}

impl<Data: AsRef<[u8]>> GcpReader<Data> {
    pub fn new(input: Data) -> Result<Self, ParseError> {
        if input.as_ref().len() < Header::LENGTH {
            return Err(ParseError::UnexpectedEnd);
        }

        let header_data = input
            .as_ref()
            .get(0..Header::LENGTH)
            .ok_or(ParseError::UnexpectedEnd)?;
        let header = Header::from_bytes(header_data.try_into().unwrap())?;

        let file_info_start = header
            .align_offset
            .checked_mul(header.file_info_offset)
            .ok_or(ParseError::UnexpectedEnd)?;
        let file_info_size = header
            .file_info_count
            .checked_mul(u32::try_from(FileInfo::LENGTH).unwrap())
            .ok_or(ParseError::UnexpectedEnd)?;
        let file_info_end = file_info_start
            .checked_add(file_info_size)
            .ok_or(ParseError::UnexpectedEnd)?;

        if file_info_start > file_info_end
            || usize::try_from(file_info_end).unwrap() >= input.as_ref().len()
        {
            return Err(ParseError::UnexpectedEnd);
        }

        let file_tag_start = header
            .align_offset
            .checked_mul(header.file_tag_offset)
            .ok_or(ParseError::UnexpectedEnd)?;
        let file_tag_size = header
            .file_tag_count
            .checked_mul(u32::try_from(Tag::LENGTH).unwrap())
            .ok_or(ParseError::UnexpectedEnd)?;
        let file_tag_end = file_tag_start
            .checked_add(file_tag_size)
            .ok_or(ParseError::UnexpectedEnd)?;

        if file_tag_start > file_tag_end || file_tag_end as usize >= input.as_ref().len() {
            return Err(ParseError::UnexpectedEnd);
        }

        let string_table_start = header
            .align_offset
            .checked_mul(header.string_table_offset)
            .ok_or(ParseError::UnexpectedEnd)?;
        let string_table_end = string_table_start
            .checked_add(header.string_table_size)
            .ok_or(ParseError::UnexpectedEnd)?;

        if string_table_start > string_table_end
            || string_table_end as usize >= input.as_ref().len()
        {
            return Err(ParseError::UnexpectedEnd);
        }

        Ok(Self { input, header })
    }

    pub fn tags(&self) -> Result<impl ExactSizeIterator<Item = Tag> + '_, ParseError> {
        let tag_count = self.header().file_tag_count;

        let file_tag_start = self
            .header()
            .align_offset
            .checked_mul(self.header().file_tag_offset)
            .ok_or(ParseError::UnexpectedEnd)?;
        let file_tag_end = file_tag_start
            .checked_add(
                tag_count
                    .checked_mul(u32::try_from(Tag::LENGTH).unwrap())
                    .ok_or(ParseError::UnexpectedEnd)?,
            )
            .ok_or(ParseError::UnexpectedEnd)?;

        let file_tag_bytes = self
            .input
            .as_ref()
            .get(usize::try_from(file_tag_start).unwrap()..usize::try_from(file_tag_end).unwrap())
            .ok_or(ParseError::UnexpectedEnd)?;

        Ok(file_tag_bytes.chunks_exact(Tag::LENGTH).map(|data| {
            let tag_data = data.try_into().unwrap();
            Tag::from_bytes(tag_data)
        }))
    }

    pub fn resource_names(&self) -> Result<impl Iterator<Item = &'_ str> + '_, ParseError> {
        let string_table_start = self
            .header()
            .align_offset
            .checked_mul(self.header().string_table_offset)
            .ok_or(ParseError::UnexpectedEnd)?;
        let string_table_end = string_table_start
            .checked_add(self.header().string_table_size)
            .ok_or(ParseError::UnexpectedEnd)?;

        let string_table_bytes = self
            .input
            .as_ref()
            .get(
                usize::try_from(string_table_start).unwrap()
                    ..usize::try_from(string_table_end).unwrap(),
            )
            .ok_or(ParseError::UnexpectedEnd)?;

        Ok(string_table_bytes
            .split(|byte| *byte == 0x0)
            .map(|bytes| core::str::from_utf8(bytes).unwrap()))
    }

    pub fn resource_infos(
        &self,
    ) -> Result<impl ExactSizeIterator<Item = FileInfo> + '_, ParseError> {
        let file_info_start = self
            .header()
            .align_offset
            .checked_mul(self.header().file_info_offset)
            .ok_or(ParseError::UnexpectedEnd)?;
        let file_info_end = file_info_start
            .checked_add(
                self.header()
                    .file_info_count
                    .checked_mul(u32::try_from(FileInfo::LENGTH).unwrap())
                    .ok_or(ParseError::UnexpectedEnd)?,
            )
            .ok_or(ParseError::UnexpectedEnd)?;

        let file_info_bytes = self
            .input
            .as_ref()
            .get(usize::try_from(file_info_start).unwrap()..usize::try_from(file_info_end).unwrap())
            .ok_or(ParseError::UnexpectedEnd)?;

        Ok(file_info_bytes.chunks_exact(FileInfo::LENGTH).map(|data| {
            let info_data = data.try_into().unwrap();
            FileInfo::from_bytes(info_data)
        }))
    }

    pub fn resource_entries(&self) -> impl ExactSizeIterator<Item = ResourceEntry> + '_ {
        self.resource_infos().unwrap().map(move |info| {
            let string_table_offset =
                self.header().align_offset * self.header().string_table_offset;

            let resource_name = core::ffi::CStr::from_bytes_until_nul(
                &self.input.as_ref()[usize::try_from(string_table_offset).unwrap()
                    + usize::try_from(info.name_offset).unwrap()..],
            )
            .unwrap()
            .to_str()
            .unwrap();

            let data_offset =
                usize::try_from(self.header().align_offset * info.data_offset).unwrap();
            let tag_offset =
                usize::try_from(self.header().align_offset * self.header().file_tag_offset)
                    .unwrap();
            let tag = Tag::from_bytes(
                self.input.as_ref()[tag_offset + usize::try_from(info.resource_tag_offset).unwrap()
                    ..tag_offset
                        + usize::try_from(info.resource_tag_offset).unwrap()
                        + Tag::LENGTH]
                    .try_into()
                    .unwrap(),
            );

            ResourceEntry {
                name: resource_name,
                data: &self.input.as_ref()
                    [data_offset..data_offset + usize::try_from(info.file_size).unwrap()],
                crc: info.crc,
                tag,
                is_local: info.is_local,
            }
        })
    }

    pub fn header(&self) -> Header {
        self.header
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Header {
    align_offset: u32,
    file_info_count: u32,
    file_info_offset: u32,
    file_tag_offset: u32,
    file_tag_count: u32,
    string_table_offset: u32,
    string_table_size: u32,
}

impl Header {
    pub const LENGTH: usize = 64;
    pub fn from_bytes(input: &[u8; Header::LENGTH]) -> Result<Self, ParseError> {
        let header = Self {
            align_offset: u32::from_be_bytes(input[4..8].try_into().unwrap()),
            file_info_count: u32::from_be_bytes(input[12..16].try_into().unwrap()),
            file_info_offset: u32::from_be_bytes(input[16..20].try_into().unwrap()),
            file_tag_offset: u32::from_be_bytes(input[20..24].try_into().unwrap()),
            file_tag_count: u32::from_be_bytes(input[24..28].try_into().unwrap()),
            string_table_offset: u32::from_be_bytes(input[40..44].try_into().unwrap()),
            string_table_size: u32::from_be_bytes(input[44..48].try_into().unwrap()),
        };

        if header.file_info_count == 0 {
            return Err(ParseError::ZeroFiles);
        }

        if header.string_table_size == 0 {
            return Err(ParseError::ZeroStrings);
        }

        Ok(header)
    }
}
