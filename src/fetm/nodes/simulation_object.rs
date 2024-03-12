use crate::fetm::{EntityClass, Error, TkKind};

use super::prop_node::PropNode;

#[derive(Debug)]
pub struct SimulationObject<'a> {
    prop_node: PropNode<'a>,
    field_0x140: usize,
    entity_class: EntityClass<'a>,
}

impl<'a> SimulationObject<'a> {
    pub fn from_tokens(data: &'a [TkKind]) -> Result<Self, Error> {
        let node = PropNode::from_tokens(data)?;
        let base = node.size;

        let class = EntityClass::from_tokens(&data[base + 1..])?;

        Ok(Self {
            prop_node: node,
            field_0x140: data[base].extract_int()?,
            entity_class: class,
        })
    }

    pub fn size(&self) -> usize {
        self.prop_node.size + self.entity_class.header.class_size
    }
}
