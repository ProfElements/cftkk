use crate::fetm::{Error, TkKind};

use super::advanced_node::AdvancedNode;

#[derive(Debug)]
pub struct CameraNode<'a> {
    base: AdvancedNode<'a>,
    actually_advanced_node_flag: usize,
    fov: f32,
    plane_1: f32,
    plane_2: f32,
}

impl<'a> CameraNode<'a> {
    pub fn from_tokens(data: &'a [TkKind]) -> Result<Self, Error> {
        let base = AdvancedNode::from_tokens(data)?;
        let offset = base.size() + 1;

        Ok(Self {
            base,
            actually_advanced_node_flag: data[offset - 1].extract_int()?,
            fov: data[offset].extract_float()?,
            plane_1: data[offset + 1].extract_float()?,
            plane_2: data[offset + 2].extract_float()?,
        })
    }

    pub fn size(&self) -> usize {
        self.base.size() + 1 + 3
    }
}
