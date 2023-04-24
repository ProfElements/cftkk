use std::println;

use crate::ParseError;

pub struct TexrReader<Data: AsRef<[u8]>> {
    input: Data,
    header: Header,
}

impl<Data: AsRef<[u8]>> TexrReader<Data> {
    pub fn new(input: Data) -> Result<Self, ParseError> {
        if input.as_ref().len() < Header::LENGTH {
            return Err(ParseError::UnexpectedEnd);
        }

        let header_data = input
            .as_ref()
            .get(0..Header::LENGTH)
            .ok_or(ParseError::UnexpectedEnd)?;
        let header = Header::from_bytes(header_data.try_into().unwrap())?;

        if header.img_start_offset > header.img_start_offset + header.img_end_offset
            || usize::try_from(header.img_end_offset).unwrap() > input.as_ref().len()
        {
            return Err(ParseError::UnexpectedEnd);
        }

        if header.tlut_offset != 0
            && header.tlut_offset
                > header.tlut_offset + (header.img_start_offset - header.tlut_offset)
        {
            return Err(ParseError::UnexpectedEnd);
        }

        Ok(Self { input, header })
    }

    pub fn header(&self) -> Header {
        self.header
    }

    pub fn data(&self) -> &[u8] {
        self.input.as_ref()
    }

    pub fn texture_lookup_data(&self) -> Option<&[u8]> {
        if self.header().tlut_offset == 0 {
            return None;
        } else {
            Some(
                self.input
                    .as_ref()
                    .get(
                        usize::try_from(self.header().tlut_offset).unwrap()
                            ..usize::try_from(self.header().img_start_offset).unwrap(),
                    )
                    .unwrap(),
            )
        }
    }

    pub fn image_data(&self) -> &[u8] {
        self.input
            .as_ref()
            .get(
                usize::try_from(self.header().img_start_offset).unwrap()
                    ..usize::try_from(self.header().img_end_offset).unwrap(),
            )
            .unwrap()
    }
}
#[derive(Copy, Clone)]
pub struct Header {
    pub width: u32,
    pub height: u32,
    pub texr_format: Format,
    pub should_mipmap: bool,
    pub frame_count: u8,
    pub size: u32,
    pub flip_count: u32,
    pub tlut_offset: u32,
    pub img_start_offset: u32,
    pub img_end_offset: u32,
}

impl Header {
    pub const LENGTH: usize = 0x80;
    pub fn from_bytes(data: &[u8; Self::LENGTH]) -> Result<Self, ParseError> {
        let header = Self {
            width: u32::from_be_bytes(data[0x20..0x24].try_into().unwrap()),
            height: u32::from_be_bytes(data[0x24..0x28].try_into().unwrap()),
            texr_format: Format::new(u32::from_be_bytes(data[0x28..0x2C].try_into().unwrap())),
            should_mipmap: data[0x2E] < 1,
            frame_count: data[0x2F],
            size: u32::from_be_bytes(data[0x38..0x3C].try_into().unwrap()),
            flip_count: u32::from_be_bytes(data[0x3C..0x40].try_into().unwrap()),
            tlut_offset: u32::from_be_bytes(data[0x6C..0x70].try_into().unwrap()),
            img_start_offset: u32::from_be_bytes(data[0x70..0x74].try_into().unwrap()),
            img_end_offset: u32::from_be_bytes(data[0x74..0x78].try_into().unwrap()),
        };

        if header.width == 0 {
            return Err(ParseError::ZeroWidth);
        }

        if header.height == 0 {
            return Err(ParseError::ZeroHeight);
        }

        if header.size == 0 {
            return Err(ParseError::ZeroSize);
        }

        if header.img_start_offset == 0 {
            return Err(ParseError::ZeroOffset);
        }

        if header.img_end_offset == 0 {
            return Err(ParseError::ZeroOffset);
        }

        Ok(header)
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Format {
    Rgba8 = 0xF,
    Rgb5a3 = 0x10,
    Ci8Rgb565 = 0x11,
    Ci8Rgb5a3 = 0x12,
    Ci4Rgb565 = 0x13,
    Ci4Rgb5a3 = 0x14,
    Cmpr = 0x15,
    I4 = 0x16,
    Rgb565 = 0x17,
    I8,
}

impl Format {
    pub fn new(format: u32) -> Self {
        match format {
            0xF => Self::Rgba8,
            0x10 => Self::Rgb5a3,
            0x11 => Self::Ci8Rgb565,
            0x12 => Self::Ci8Rgb5a3,
            0x13 => Self::Ci4Rgb565,
            0x14 => Self::Ci4Rgb5a3,
            0x15 => Self::Cmpr,
            0x16 => Self::I4,
            0x17 => Self::Rgb565,
            _ => Self::I8,
        }
    }

    pub fn bits_per_pixel(&self) -> usize {
        match self {
            Self::Rgba8 => 32,
            Self::Rgb5a3 => 16,
            Self::Ci8Rgb565 => 8,
            Self::Ci8Rgb5a3 => 8,
            Self::Ci4Rgb565 => 4,
            Self::Ci4Rgb5a3 => 4,
            Self::Cmpr => 4,
            Self::Rgb565 => 16,
            Self::I4 => 4,
            Self::I8 => 8,
        }
    }

    pub fn block_dimensions(&self) -> (usize, usize) {
        match self {
            Self::Rgba8 => (4, 4),
            Self::Rgb5a3 => (4, 4),
            Self::Ci8Rgb565 => (8, 4),
            Self::Ci8Rgb5a3 => (8, 4),
            Self::Ci4Rgb565 => (8, 8),
            Self::Ci4Rgb5a3 => (8, 8),
            Self::Cmpr => (8, 8),
            Self::Rgb565 => (4, 4),
            Self::I4 => (8, 8),
            Self::I8 => (8, 4),
        }
    }
}
