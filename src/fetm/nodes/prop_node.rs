use alloc::vec::Vec;

use crate::fetm::{EntityClassHeader, Error, TkKind};

use super::{advanced_node::AdvancedNode, node::Node};

#[derive(Debug)]
pub struct PropNode<'a> {
    advanced_node: AdvancedNode<'a>,
    skipped_token: usize,
    flag: usize,
    field_0xe0: usize, //this is actor type, actor, animating or animtree
    has_field_0xbc: usize,
    field_0xbc: Option<f32>,
    skipped_token_1: usize,
    token_count: usize,
    token_names: Vec<(&'a str, f32)>,
    field_0xd8: &'a str,
    field_0xf9: usize,
    field_0xd0: usize, //this is a hex8
    flag_1: usize,
    flag_2: usize,
    field_0xdc: &'a str,
    field_0xfb: usize,
    flag_3: usize,
    field_0xe1: usize,
    has_lighting_rig: usize,
    flag_4: usize,
    field_0x54: usize,
    flag_5: usize,
    field_0xf0: &'a str,
    field_0xfa: usize,
    //this is just for my help
    pub size: usize,
}

impl<'a> PropNode<'a> {
    pub fn from_tokens(data: &'a [TkKind]) -> Result<Self, Error> {
        let node = AdvancedNode::from_tokens(data)?;
        let base = node.entity_class.header.class_size + Node::SIZE + 14;

        let float = if data[base + 3].extract_int()? != 0 {
            Some(data[base + 4].extract_float()?)
        } else {
            None
        };

        let count = data[base + 4].extract_int()?;

        let mut vec = Vec::with_capacity(count);

        let mut meshes_offset = 6;
        for n in 0..count {
            vec.push((
                data[base + n + meshes_offset].extract_str()?,
                data[base + n + meshes_offset + 1].extract_float()?,
            ));

            meshes_offset += 2;
        }

        let roughy = base + meshes_offset * count;

        Ok(Self {
            advanced_node: node,
            skipped_token: data[base].extract_int()?,
            flag: data[base + 1].extract_int()?,
            field_0xe0: data[base + 2].extract_int()?,
            has_field_0xbc: data[base + 3].extract_int()?,
            field_0xbc: float,
            skipped_token_1: data[base + 5].extract_int()?,
            token_count: data[base + 4].extract_int()?,
            token_names: vec,
            field_0xd8: data[roughy].extract_str()?,
            field_0xf9: data[roughy + 1].extract_int()?,
            field_0xd0: data[roughy + 2].extract_hex8()?,
            flag_1: data[roughy + 3].extract_int()?,
            flag_2: data[roughy + 4].extract_int()?,
            field_0xdc: data[roughy + 5].extract_str()?,
            field_0xfb: data[roughy + 6].extract_int()?,
            flag_3: data[roughy + 7].extract_int()?,
            field_0xe1: data[roughy + 8].extract_int()?,
            has_lighting_rig: data[roughy + 9].extract_int()?,
            flag_4: data[roughy + 10].extract_int()?,
            field_0x54: data[roughy + 11].extract_int()?,
            flag_5: data[roughy + 12].extract_int()?,
            field_0xf0: data[roughy + 13].extract_str()?,
            field_0xfa: data[roughy + 14].extract_int()?,
            size: roughy + 15,
        })
    }
}
