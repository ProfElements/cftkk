#![no_std]
#![warn(clippy::std_instead_of_alloc, clippy::std_instead_of_core)]

extern crate alloc;
extern crate std;

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
