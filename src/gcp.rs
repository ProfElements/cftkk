use crate::ParseError;

pub struct Gcp<Data> {
    data: Data,
}

impl<Data: AsRef<[u8]>> Gcp<Data> {
    pub fn new(data: Data) -> Result<Gcp<Data>, crate::ParseError> {
        let len = data.as_ref().len();
        if len < 0x40 {
            return Err(ParseError::UnexpectedEnd);
        }
        let mut result = Self { data };
        let info_offset = result.alignment() * result.info_offset();
        if info_offset > len.try_into().unwrap() {
            return Err(ParseError::UnexpectedEnd);
        }
        if info_offset + result.info_size() > len.try_into().unwrap() {
            return Err(ParseError::UnexpectedEnd);
        }

        let tag_offset = result.alignment() * result.tag_offset();
        if tag_offset > len.try_into().unwrap() {
            return Err(ParseError::UnexpectedEnd);
        }

        //Each TAG = 4 BYTES
        if tag_offset + (result.tag_count() * 4) > len.try_into().unwrap() {
            return Err(ParseError::UnexpectedEnd);
        }

        let name_offset = result.alignment() * result.name_offset();
        if name_offset > len.try_into().unwrap() {
            return Err(ParseError::UnexpectedEnd);
        }

        if name_offset + result.name_size() > len.try_into().unwrap() {
            return Err(ParseError::UnexpectedEnd);
        }

        println!("Timestamp: {}", result.timestamp());
        println!("Alignment: {}", result.alignment());
        println!("File Count: {}", result.file_count());
        println!(
            "Info Offset: {},Actual Info Offset: {}, Info Size: {}",
            result.info_offset(),
            info_offset,
            result.info_size()
        );

        println!(
            "Tag Offset: {},Actual Tag Offset: {}, Tag Count: {}",
            result.tag_offset(),
            tag_offset,
            result.tag_count()
        );
        println!(
            "Names Offset: {}, Actual Name Offset: {} Names Size: {}",
            result.name_offset(),
            name_offset,
            result.name_size()
        );

        for n in 0..result.tag_count() {
            if let Some(tag) = result.get_tag((tag_offset + (n * 4)).try_into().unwrap()) {
                println!("Found Tag: {:?}", tag);
            }
        }

        Ok(result)
    }

    #[inline]
    fn timestamp(&self) -> u32 {
        u32::from_be_bytes(self.data.as_ref()[0..4].try_into().unwrap())
    }

    #[inline]
    fn alignment(&self) -> u32 {
        u32::from_be_bytes(self.data.as_ref()[4..8].try_into().unwrap())
    }

    #[inline]
    fn file_count(&self) -> u32 {
        u32::from_be_bytes(self.data.as_ref()[12..16].try_into().unwrap())
    }

    #[inline]
    fn info_offset(&self) -> u32 {
        u32::from_be_bytes(self.data.as_ref()[16..20].try_into().unwrap())
    }

    #[inline]
    fn info_size(&self) -> u32 {
        u32::from_be_bytes(self.data.as_ref()[48..52].try_into().unwrap())
    }

    #[inline]
    fn tag_offset(&self) -> u32 {
        u32::from_be_bytes(self.data.as_ref()[20..24].try_into().unwrap())
    }

    #[inline]
    fn tag_count(&self) -> u32 {
        u32::from_be_bytes(self.data.as_ref()[24..28].try_into().unwrap())
    }

    #[inline]
    fn name_offset(&self) -> u32 {
        u32::from_be_bytes(self.data.as_ref()[40..44].try_into().unwrap())
    }

    #[inline]
    fn name_size(&self) -> u32 {
        u32::from_be_bytes(self.data.as_ref()[44..48].try_into().unwrap())
    }

    pub fn get_tag(&self, offset: usize) -> Option<Tag> {
        Tag::from_str(core::str::from_utf8(&self.data.as_ref()[offset..offset + 4]).unwrap())
    }

    pub fn get_file_info(&self, offset: usize) -> FileInfo {
        FileInfo::from_be_bytes(
            self.data.as_ref()[offset..offset + 0x20]
                .try_into()
                .unwrap(),
        )
    }

    pub fn get_files(&self) -> Vec<FileEntry> {
        let info_offset = self.info_offset() * self.alignment();
        let name_offset = self.name_offset() * self.alignment();

        let mut files = Vec::new();
        for n in 0..self.file_count() {
            let file_info = self.get_file_info((info_offset + (n * 32)).try_into().unwrap());
            let file_name = core::ffi::CStr::from_bytes_until_nul(
                &self.data.as_ref()[name_offset as usize + file_info.name_offset as usize..],
            )
            .unwrap();
            let file_data_offset = file_info.data_offset * self.alignment();
            println!("File Name: {:?}", file_name);
            println!(
                "File Offset: {}, File Size: {:?}",
                file_data_offset, file_info.file_size
            );
            println!("Flags: {:X?}", file_info.file_flag);

            files.push(FileEntry {
                name: file_name.to_str().unwrap(),
                data: self.data.as_ref()[file_data_offset as usize
                    ..file_data_offset as usize + file_info.file_size as usize]
                    .to_vec(),
                flag: file_info.file_flag,
            });
        }

        files
    }
}
#[derive(Debug)]
pub enum Tag {
    Splash,
}

impl Tag {
    pub fn from_str(tag: &str) -> Option<Tag> {
        match tag {
            "SPLA" => Some(Tag::Splash),
            _ => {
                println!("{tag}");
                return None;
            }
        }
    }
}

pub struct FileInfo {
    data_offset: u32,
    crc: u32,
    file_size: u32,
    name_offset: u32,
    file_flag: u32,
}

impl FileInfo {
    pub fn from_be_bytes(bytes: [u8; 0x20]) -> Self {
        Self {
            data_offset: u32::from_be_bytes(bytes[0..4].try_into().unwrap()),
            crc: u32::from_be_bytes(bytes[4..8].try_into().unwrap()),
            file_size: u32::from_be_bytes(bytes[8..12].try_into().unwrap()),
            name_offset: u32::from_be_bytes(bytes[12..16].try_into().unwrap()),
            file_flag: u32::from_be_bytes(bytes[16..20].try_into().unwrap()),
        }
    }
}

pub struct FileEntry<'a> {
    pub name: &'a str,
    pub data: Vec<u8>,
    flag: u32,
}
