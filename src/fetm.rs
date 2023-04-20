use crate::ParseError;
use std::iter::from_fn;

#[derive(Debug)]
pub enum TkKind<'a> {
    I8(i8),
    U8(u8),
    I16(i16),
    U16(u16),
    U32(u32),
    Hex8(u32),
    F32(f32),
    String(&'a str),
}

pub struct FetmReader<Data: AsRef<[u8]>> {
    input: Data,
}

impl<Data: AsRef<[u8]>> FetmReader<Data> {
    pub fn new(input: Data) -> Result<Self, ParseError> {
        if input.as_ref().len() < 3 {
            return Err(ParseError::UnexpectedEnd);
        }

        let magic = input.as_ref().get(0..3).ok_or(ParseError::UnexpectedEnd)?;

        if magic != &[0x01, 0x7C, 0x07] {
            return Err(ParseError::BadMagic);
        }

        if input
            .as_ref()
            .get(input.as_ref().len() - 4..input.as_ref().len())
            .ok_or(ParseError::UnexpectedEnd)?
            != b"end\0"
        {
            return Err(ParseError::UnexpectedEnd);
        }

        Ok(Self { input })
    }

    pub fn tokens(&self) -> impl Iterator<Item = TkKind> + '_ {
        let mut index = 0;
        from_fn(move || {
            let token_kind = if let Some(kind) = self.input.as_ref().get(index) {
                kind
            } else {
                return None;
            };

            match token_kind {
                0 => {
                    let data_start = index + 1;
                    index += 2;
                    Some(TkKind::I8(self.input.as_ref()[data_start] as i8))
                }
                1 => {
                    let data_start = index + 1;
                    index += 2;
                    Some(TkKind::U8(self.input.as_ref()[data_start]))
                }
                2 => {
                    let data_start = index + 1;
                    let data_end = data_start + 2;
                    index += 3;
                    Some(TkKind::I16(i16::from_be_bytes(
                        self.input
                            .as_ref()
                            .get(data_start..data_end)
                            .unwrap()
                            .try_into()
                            .unwrap(),
                    )))
                }
                3 => {
                    let data_start = index + 1;
                    let data_end = data_start + 2;
                    index += 3;
                    Some(TkKind::U16(u16::from_be_bytes(
                        self.input
                            .as_ref()
                            .get(data_start..data_end)
                            .unwrap()
                            .try_into()
                            .unwrap(),
                    )))
                }
                4 => {
                    let data_start = index + 1;
                    let data_end = data_start + 4;
                    index += 5;
                    Some(TkKind::U32(u32::from_be_bytes(
                        self.input
                            .as_ref()
                            .get(data_start..data_end)
                            .unwrap()
                            .try_into()
                            .unwrap(),
                    )))
                }
                5 => {
                    let data_start = index + 1;
                    let data_end = data_start + 4;
                    index += 5;
                    Some(TkKind::Hex8(u32::from_be_bytes(
                        self.input
                            .as_ref()
                            .get(data_start..data_end)
                            .unwrap()
                            .try_into()
                            .unwrap(),
                    )))
                }
                6 => {
                    let data_start = index + 1;
                    let data_end = data_start + 4;
                    index += 5;
                    Some(TkKind::F32(f32::from_be_bytes(
                        self.input
                            .as_ref()
                            .get(data_start..data_end)
                            .unwrap()
                            .try_into()
                            .unwrap(),
                    )))
                }
                7 => {
                    let data_start = index + 1;
                    let string = core::ffi::CStr::from_bytes_until_nul(
                        self.input.as_ref().get(data_start..).unwrap(),
                    )
                    .unwrap()
                    .to_str()
                    .unwrap();

                    index += string.len() + 2;

                    Some(TkKind::String(string))
                }
                _ => panic!(),
            }
        })
    }
}
