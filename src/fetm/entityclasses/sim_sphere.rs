use crate::fetm::{Error, TkKind};

use super::sim_body::SimBody;

#[derive(Debug, PartialEq)]
pub struct SimSphere {
    body: SimBody,
    field_0x7c: f32,
}

impl SimSphere {
    pub fn from_tokens(data: &[TkKind]) -> Result<Self, Error> {
        let base = SimBody::SIZE;
        Ok(Self {
            body: SimBody::from_tokens(data)?,
            field_0x7c: data[base].extract_float()?,
        })
    }
}
