use crate::fetm::{Error, TkKind};

use super::advanced_node::AdvancedNode;

#[derive(Debug)]
pub struct DecalSystem<'a> {
    base: AdvancedNode<'a>,
    flag_0: usize,
    field_0x134: usize,
    field_0x138: usize,
    field_0x13c: usize,
    field_0x15c: usize,
    field_0x158: usize,
    field_0x150: usize,
    field_0x151: usize,
    field_0x164: f32,
    field_0x140: usize,
    field_0x170: usize,
    field_0x174: f32,
    field_0x178: f32,
    flag_1: usize,
}

impl<'a> DecalSystem<'a> {
    pub fn from_tokens(data: &'a [TkKind]) -> Result<Self, Error> {
        let base = AdvancedNode::from_tokens(data)?;
        let offset = base.size();
        Ok(Self {
            base,
            flag_0: data[offset].extract_int()?,
            field_0x134: data[offset + 1].extract_int()?,
            field_0x138: data[offset + 2].extract_int()?,
            field_0x13c: data[offset + 3].extract_int()?,
            field_0x15c: data[offset + 4].extract_hex8()?,
            field_0x158: data[offset + 5].extract_int()?,
            field_0x150: data[offset + 6].extract_int()?,
            field_0x151: data[offset + 7].extract_int()?,
            field_0x164: data[offset + 8].extract_float()?,
            field_0x140: data[offset + 9].extract_int()?,
            field_0x170: data[offset + 10].extract_int()?,
            field_0x174: data[offset + 11].extract_float()?,
            field_0x178: data[offset + 12].extract_float()?,
            flag_1: data[offset + 13].extract_int()?,
        })
    }

    pub fn size(&self) -> usize {
        self.base.size() + 14
    }
}
