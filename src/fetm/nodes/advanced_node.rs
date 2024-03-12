use crate::fetm::{EntityClass, EntityClassHeader, EntityKlass, Error, TkKind};

use super::node::Node;

#[derive(Debug)]
pub struct AdvancedNode<'a> {
    node: Node<'a>,
    has_0xb0: usize,
    field_0xb0: usize,
    has_0xb1: usize,
    field_0xb1: usize,
    has_0xb2: usize,
    field_0xb2: usize,
    has_0xb3: usize,
    field_0xb3: usize,
    field_0xc0: usize, //this is a hex8,
    field_0x34: usize, //this is a hex8
    has_0xbc: usize,
    field_0xbc: f32,
    pub entity_class: EntityClass<'a>,
}

impl<'a> AdvancedNode<'a> {
    pub fn from_tokens(data: &'a [TkKind]) -> Result<Self, Error> {
        let node = Node::from_tokens(data)?;
        let base = node.size();

        Ok(Self {
            node,
            has_0xb0: data[base].extract_int()?,
            field_0xb0: data[base + 1].extract_int()?,
            has_0xb1: data[base + 2].extract_int()?,
            field_0xb1: data[base + 3].extract_int()?,
            has_0xb2: data[base + 4].extract_int()?,
            field_0xb2: data[base + 5].extract_int()?,
            has_0xb3: data[base + 6].extract_int()?,
            field_0xb3: data[base + 7].extract_int()?,
            field_0xc0: data[base + 8].extract_hex8()?,
            field_0x34: data[base + 9].extract_hex8()?,
            has_0xbc: data[base + 10].extract_int()?,
            field_0xbc: data[base + 11].extract_float()?,
            entity_class: EntityClass::from_tokens(&data[base + 12..])?,
        })
    }

    pub fn size(&self) -> usize {
        let class_size = if self.entity_class.class == EntityKlass::Empty {
            1
        } else {
            self.entity_class.header.class_size + EntityClassHeader::LENGTH
        };

        self.node.size() + 12 + class_size
    }
}
