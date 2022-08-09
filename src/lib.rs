//#![no_std]
#![feature(cstr_from_bytes_until_nul)]

extern crate alloc;

pub mod fetm;
pub mod gcp;

#[derive(Copy, Clone, Debug)]
pub enum ParseError {
    UnexpectedEnd,
    BadMagic,
}
