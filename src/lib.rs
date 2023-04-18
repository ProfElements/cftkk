//#![no_std]
extern crate alloc;

pub mod fetm;
pub mod gcp;

#[derive(Copy, Clone, Debug)]
pub enum ParseError {
    UnexpectedEnd,
    BadMagic,
    ZeroFiles,
    ZeroTags,
    ZeroStrings,
}
