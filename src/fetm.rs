use crate::ParseError;
use alloc::vec::Vec;
use core::ffi::CStr;

#[derive(Clone)]
pub struct Fetm<Data> {
    data: Data,
}

impl<Data: AsRef<[u8]>> Fetm<Data> {
    pub fn new(data: Data) -> Result<Self, crate::ParseError> {
        let bytes = data.as_ref();
        let header = bytes.get(0..3).ok_or(ParseError::UnexpectedEnd)?;
        if &header != &[0x01, 0x7C, 0x07] {
            return Err(ParseError::BadMagic);
        }

        Ok(Self { data })
    }
    pub fn collect_tokens(&self) -> Vec<Token<'_>> {
        let mut tokens = Vec::new();
        let mut idx = 0;
        while idx < self.data.as_ref().len() {
            if let Some(token) = self.get_token(idx) {
                tokens.push(token.clone());
                match token.kind {
                    TokenKind::Char => idx += 2,
                    TokenKind::UnsignedChar => idx += 2,
                    TokenKind::Short => idx += 3,
                    TokenKind::UnsignedShort => idx += 3,
                    TokenKind::Int => idx += 5,
                    TokenKind::Hex8 => idx += 5,
                    TokenKind::Float => idx += 5,
                    TokenKind::Str => idx += token.data.len() + 1,
                }
            }
        }
        tokens
    }

    pub fn get_token(&self, offset: usize) -> Option<Token<'_>> {
        let bytes = self.data.as_ref();
        let token_kind: u8 = bytes[offset];
        let data_start: usize = offset + 1;
        match token_kind {
            0 => Some(Token {
                data: core::slice::from_ref(&bytes[data_start]),
                kind: TokenKind::Char,
            }),
            1 => Some(Token {
                data: core::slice::from_ref(&bytes[data_start]),
                kind: TokenKind::UnsignedChar,
            }),
            2 => Some(Token {
                data: &bytes[data_start..data_start + 2],
                kind: TokenKind::Short,
            }),
            3 => Some(Token {
                data: &bytes[data_start..data_start + 2],
                kind: TokenKind::UnsignedShort,
            }),
            4 => Some(Token {
                data: &bytes[data_start..data_start + 4],
                kind: TokenKind::Int,
            }),
            5 => Some(Token {
                data: &bytes[data_start..data_start + 4],
                kind: TokenKind::Hex8,
            }),
            6 => Some(Token {
                data: &bytes[data_start..data_start + 4],
                kind: TokenKind::Float,
            }),
            7 => Some(Token {
                data: CStr::from_bytes_until_nul(&bytes[data_start..])
                    .unwrap()
                    .to_bytes_with_nul(),
                kind: TokenKind::Str,
            }),
            _ => panic!(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Token<'a> {
    kind: TokenKind,
    data: &'a [u8],
}

#[derive(Copy, Clone, Debug)]
pub enum TokenKind {
    Char,
    UnsignedChar,
    Short,
    UnsignedShort,
    Int,
    Hex8,
    Float,
    Str,
}
