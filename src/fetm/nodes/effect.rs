use crate::fetm::{Error, TkKind};

use super::{advanced_node::AdvancedNode, node::Node};

#[derive(Debug)]
pub struct Effect<'a> {
    advanced_node: AdvancedNode<'a>,
    flag_0: usize,
    flag_1: usize,
    flag_2: usize,
    flag_3: usize,
    field_0xdc: f32,
    field_0xe4: f32,
    field_0xe8: f32,
}

impl<'a> Effect<'a> {
    pub fn from_tokens(data: &'a [TkKind]) -> Result<Self, Error> {
        let base = AdvancedNode::from_tokens(data).expect("HURT");
        let base_size = base.size();

        Ok(Self {
            advanced_node: base,
            flag_0: data[base_size].extract_int()?,
            flag_1: data[base_size + 1].extract_int()?,
            flag_2: data[base_size + 2].extract_int()?,
            flag_3: data[base_size + 3].extract_int()?,

            field_0xdc: data[base_size + 4].extract_float()?,
            field_0xe4: data[base_size + 5].extract_float()?,
            field_0xe8: data[base_size + 6].extract_float()?,
        })
    }

    pub fn size(&self) -> usize {
        self.advanced_node.size() + 7
    }
}
