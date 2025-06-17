use core::ffi::CStr;

#[derive(Copy, Clone)]
pub struct Header {
    pub crc: u32,
    offset_size: u32,
    flags: u32,
    file_count: i32,
    index_offset: u32,
    tag_offset: u32,
    tag_count: i32,
    block_map_offset: u32,
    block_map_size: u32,
    filename_table_offset: u32,
    filename_table_size: u32,
    index_size: u32,
    start_sector: u32,
    build_number: u32,
    file_count_using_dma: u32,
}

impl Header {
    pub const LENGTH: usize = 0x40;

    pub fn from_bytes(data: &[u8; Self::LENGTH]) -> Result<Self, ParseError> {
        let header = Self {
            crc: u32::from_be_bytes(data[0..4].try_into().unwrap()),
            offset_size: u32::from_be_bytes(data[4..8].try_into().unwrap()),
            flags: u32::from_be_bytes(data[8..12].try_into().unwrap()),
            file_count: i32::from_be_bytes(data[12..16].try_into().unwrap()),
            index_offset: u32::from_be_bytes(data[16..20].try_into().unwrap()),
            tag_offset: u32::from_be_bytes(data[20..24].try_into().unwrap()),
            tag_count: i32::from_be_bytes(data[24..28].try_into().unwrap()),
            //padding[4]
            block_map_offset: u32::from_be_bytes(data[32..36].try_into().unwrap()),
            block_map_size: u32::from_be_bytes(data[36..40].try_into().unwrap()),
            filename_table_offset: u32::from_be_bytes(data[40..44].try_into().unwrap()),
            filename_table_size: u32::from_be_bytes(data[44..48].try_into().unwrap()),
            index_size: u32::from_be_bytes(data[48..52].try_into().unwrap()),
            start_sector: u32::from_be_bytes(data[52..56].try_into().unwrap()),
            build_number: u32::from_be_bytes(data[56..60].try_into().unwrap()),
            file_count_using_dma: u32::from_be_bytes(data[60..64].try_into().unwrap()),
        };

        if header.file_count == 0 {
            return Err(ParseError::ZeroFileCount);
        }

        // if header.tag_count == 0 {
        //     return Err(ParseError::ZeroTagCount);
        // }

        return Ok(header);
    }
}

pub struct Reader<Data: AsRef<[u8]>> {
    input: Data,
    header: Header,
}

#[derive(Debug)]
pub enum ParseError {
    UnexpectedEnd,
    ZeroFileCount,
    ZeroTagCount,
}

impl<Data: AsRef<[u8]>> Reader<Data> {
    pub fn new(input: Data) -> Result<Self, ParseError> {
        if input.as_ref().len() < Header::LENGTH {
            return Err(ParseError::UnexpectedEnd);
        }

        let header_data = input
            .as_ref()
            .get(0..Header::LENGTH)
            .ok_or(ParseError::UnexpectedEnd)?;
        let header = Header::from_bytes(header_data.try_into().unwrap())?;

        //Check index bounds
        let index_start = header
            .index_offset
            .checked_mul(header.offset_size)
            .ok_or(ParseError::UnexpectedEnd)?;
        let index_end = header
            .index_offset
            .checked_mul(header.offset_size)
            .ok_or(ParseError::UnexpectedEnd)?
            .checked_add(header.index_size)
            .ok_or(ParseError::UnexpectedEnd)?;
        if index_end < index_start || index_end as usize >= input.as_ref().len() {
            return Err(ParseError::UnexpectedEnd);
        }

        //Check tag bounds
        let tag_size: u32 = header
            .tag_count
            .checked_mul(4)
            .ok_or(ParseError::UnexpectedEnd)?
            .try_into()
            .unwrap();
        let tag_start = header
            .tag_offset
            .checked_mul(header.offset_size)
            .ok_or(ParseError::UnexpectedEnd)?;
        let tag_end = header
            .tag_offset
            .checked_mul(header.offset_size)
            .ok_or(ParseError::UnexpectedEnd)?
            .checked_add(tag_size)
            .ok_or(ParseError::UnexpectedEnd)?;
        if tag_end < tag_start || tag_end as usize >= input.as_ref().len() {
            return Err(ParseError::UnexpectedEnd);
        }

        //Check block map bounds
        let block_map_start = header
            .block_map_offset
            .checked_mul(header.offset_size)
            .ok_or(ParseError::UnexpectedEnd)?;
        let block_map_end = header
            .block_map_offset
            .checked_mul(header.offset_size)
            .ok_or(ParseError::UnexpectedEnd)?
            .checked_add(header.block_map_size)
            .ok_or(ParseError::UnexpectedEnd)?;
        if block_map_end < block_map_start || block_map_end as usize >= input.as_ref().len() {
            return Err(ParseError::UnexpectedEnd);
        }

        //Check filename table bounds
        let filename_table_start = header
            .filename_table_offset
            .checked_mul(header.offset_size)
            .ok_or(ParseError::UnexpectedEnd)?;
        let filename_table_end = header
            .filename_table_offset
            .checked_mul(header.offset_size)
            .ok_or(ParseError::UnexpectedEnd)?
            .checked_add(header.filename_table_size)
            .ok_or(ParseError::UnexpectedEnd)?;
        if filename_table_end < filename_table_start
            || filename_table_end as usize >= input.as_ref().len()
        {
            return Err(ParseError::UnexpectedEnd);
        }

        Ok(Self { header, input })
    }

    pub fn file_index(&self) -> Result<impl ExactSizeIterator<Item = FileIndex> + '_, ParseError> {
        let index_start = self
            .header()
            .index_offset
            .checked_mul(self.header().offset_size)
            .ok_or(ParseError::UnexpectedEnd)?;

        let index_end = self
            .header()
            .index_offset
            .checked_mul(self.header().offset_size)
            .ok_or(ParseError::UnexpectedEnd)?
            .checked_add(self.header().index_size)
            .ok_or(ParseError::UnexpectedEnd)?;

        let index_bytes = self
            .input
            .as_ref()
            .get(index_start as usize..index_end as usize)
            .ok_or(ParseError::UnexpectedEnd)?;

        Ok(index_bytes.chunks_exact(FileIndex::LENGTH).map(|data| {
            let file_data = data.try_into().unwrap();
            FileIndex::from_bytes(file_data)
        }))
    }

    pub fn tag_index(&self) -> Result<impl ExactSizeIterator<Item = TagIndex> + '_, ParseError> {
        let tag_count: u32 = self.header().tag_count.try_into().unwrap();
        let tag_size = tag_count
            .checked_mul(u32::try_from(TagIndex::LENGTH).unwrap())
            .ok_or(ParseError::UnexpectedEnd)?;

        let tag_start = self
            .header()
            .tag_offset
            .checked_mul(self.header().offset_size)
            .ok_or(ParseError::UnexpectedEnd)?;

        let tag_end = self
            .header()
            .tag_offset
            .checked_mul(self.header().offset_size)
            .ok_or(ParseError::UnexpectedEnd)?
            .checked_add(tag_size)
            .ok_or(ParseError::UnexpectedEnd)?;

        let tag_bytes = self
            .input
            .as_ref()
            .get(tag_start as usize..tag_end as usize)
            .ok_or(ParseError::UnexpectedEnd)?;

        Ok(tag_bytes.chunks_exact(TagIndex::LENGTH).map(|data| {
            let tag_data = data.try_into().unwrap();
            TagIndex::from_bytes(tag_data)
        }))
    }

    pub fn filename_table(&self) -> Result<&[u8], ParseError> {
        let filename_start = self
            .header()
            .filename_table_offset
            .checked_mul(self.header().offset_size)
            .ok_or(ParseError::UnexpectedEnd)?;

        let filename_end = self
            .header()
            .filename_table_offset
            .checked_mul(self.header().offset_size)
            .ok_or(ParseError::UnexpectedEnd)?
            .checked_add(self.header().filename_table_size)
            .ok_or(ParseError::UnexpectedEnd)?;

        let filename_bytes = self
            .input
            .as_ref()
            .get(filename_start as usize..filename_end as usize)
            .ok_or(ParseError::UnexpectedEnd)?;

        Ok(filename_bytes)
    }

    pub fn tag_table(&self) -> Result<&[u8], ParseError> {
        let tag_count: u32 = self.header().tag_count.try_into().unwrap();
        let tag_size = tag_count
            .checked_mul(u32::try_from(TagIndex::LENGTH).unwrap())
            .ok_or(ParseError::UnexpectedEnd)?;

        let tag_start = self
            .header()
            .tag_offset
            .checked_mul(self.header().offset_size)
            .ok_or(ParseError::UnexpectedEnd)?;

        let tag_end = self
            .header()
            .tag_offset
            .checked_mul(self.header().offset_size)
            .ok_or(ParseError::UnexpectedEnd)?
            .checked_add(tag_size)
            .ok_or(ParseError::UnexpectedEnd)?;

        let tag_bytes = self
            .input
            .as_ref()
            .get(tag_start as usize..tag_end as usize)
            .ok_or(ParseError::UnexpectedEnd)?;

        Ok(tag_bytes)
    }

    pub fn files(&self) -> impl ExactSizeIterator<Item = File> + '_ {
        let name_table = self.filename_table().unwrap();
        let mut tag_table = self.tag_table().unwrap();

        let tag_table = if tag_table.len() == 0 {
            b".EXT"
        } else {
            tag_table
        };

        self.file_index().unwrap().map(move |file| File {
            name: CStr::from_bytes_until_nul(&name_table[file.filename_offset as usize..]).unwrap(),
            data: &self.input.as_ref()[((file.offset as u32) * self.header().offset_size) as usize
                ..(((file.offset as u32) * self.header().offset_size) + (file.size as u32))
                    as usize],
            tag: TagIndex::from_bytes(
                &tag_table[file.tag_offset as usize..file.tag_offset as usize + 4]
                    .try_into()
                    .unwrap(),
            ),
            crc: file.crc,
            file_time: file.file_time,
        })
    }

    pub fn header(&self) -> Header {
        self.header
    }

    pub fn data(&self) -> &[u8] {
        self.input.as_ref()
    }
}

#[derive(Debug)]
pub struct File<'a> {
    pub name: &'a CStr,
    pub data: &'a [u8],
    pub tag: TagIndex,
    pub crc: u32,
    file_time: u64,
}

pub struct FileIndex {
    offset: i32,
    crc: u32,
    size: i32,
    filename_offset: u32,
    tag_count: u32,
    tag_offset: u32,
    file_time: u64,
}

impl FileIndex {
    pub const LENGTH: usize = 0x20;

    pub fn from_bytes(data: &[u8; Self::LENGTH]) -> Self {
        Self {
            offset: i32::from_be_bytes(data[0..4].try_into().unwrap()),
            crc: u32::from_be_bytes(data[4..8].try_into().unwrap()),
            size: i32::from_be_bytes(data[8..12].try_into().unwrap()),
            filename_offset: u32::from_be_bytes(data[12..16].try_into().unwrap()),
            tag_count: u32::from_be_bytes(data[16..20].try_into().unwrap()),
            tag_offset: u32::from_be_bytes(data[20..24].try_into().unwrap()),
            file_time: u64::from_be_bytes(data[24..32].try_into().unwrap()),
        }
    }
}

#[derive(Debug)]
pub struct TagIndex {
    pub tag: [u8; 4],
}

impl TagIndex {
    pub const LENGTH: usize = 4;

    pub fn from_bytes(data: &[u8; Self::LENGTH]) -> Self {
        TagIndex { tag: *data }
    }
}
