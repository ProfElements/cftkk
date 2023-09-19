#![no_std]

extern crate std;

extern crate alloc;

pub mod actr;
pub mod cmes;
pub mod fetm;
pub mod gcp;
pub mod texr;

#[derive(Copy, Clone, Debug)]
pub enum ParseError {
    UnexpectedEnd,
    BadMagic,
    ZeroFiles,
    ZeroTags,
    ZeroStrings,
    ZeroWidth,
    ZeroHeight,
    ZeroOffset,
    ZeroSize,
    ZeroTriangles,
    ZeroNormals,
    ZeroVertices,
    ZeroGeometry,
}
