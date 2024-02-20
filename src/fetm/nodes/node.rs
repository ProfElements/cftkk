use crate::fetm::{Error, TkKind};

#[derive(Debug)]
pub struct Node {
    transform: Transform,
    flag_0: usize,
    min_x: f32,
    min_y: f32,
    min_z: f32,
    max_x: f32,
    max_y: f32,
    max_z: f32,
    flag_1: usize,
    flag_2: usize,
    flag_3: usize,
    flag_4: usize,
    flag_5: usize,
    flag_6: usize,
    field_0x14: f32,
    field_0x18: usize,
    flag_7: usize,
    node_crc: usize, //this is a world node crc, assuming either `child` or `parent`
    flag_8: usize,
    flag_9: usize,
    flag_10: usize,
    flag_11: usize,
    flag_12: usize,
    flag_13: usize,
    flag_14: usize,
    action_count: usize,
    //actions: [Action; action_count],
    attachment_count: usize,
    //attachments: [Attachment; attachment_count],
    //
}

impl Node {
    pub const SIZE: usize = StaticTransform::SIZE + 1 + 26;
    pub fn from_tokens(data: &[TkKind]) -> Result<Self, Error> {
        let base = StaticTransform::SIZE + 1; //counts for transform_type
        Ok(Self {
            transform: Transform::from_tokens(data)?,
            flag_0: data[base].extract_int()?,
            min_x: data[base + 1].extract_float()?,
            min_y: data[base + 2].extract_float()?,
            min_z: data[base + 3].extract_float()?,
            max_x: data[base + 4].extract_float()?,
            max_y: data[base + 5].extract_float()?,
            max_z: data[base + 6].extract_float()?,
            flag_1: data[base + 7].extract_int()?,
            flag_2: data[base + 8].extract_int()?,
            flag_3: data[base + 9].extract_int()?,
            flag_4: data[base + 10].extract_int()?,
            flag_5: data[base + 11].extract_hex8()?,
            flag_6: data[base + 12].extract_int()?,
            field_0x14: data[base + 13].extract_float()?,
            field_0x18: data[base + 14].extract_int()?,
            flag_7: data[base + 15].extract_int()?,
            node_crc: data[base + 16].extract_hex8()?,
            flag_8: data[base + 17].extract_int()?,
            flag_9: data[base + 18].extract_int()?,
            flag_10: data[base + 19].extract_int()?,
            flag_11: data[base + 20].extract_int()?,
            flag_12: data[base + 21].extract_int()?,
            flag_13: data[base + 22].extract_int()?,
            flag_14: data[base + 23].extract_int()?,
            action_count: data[base + 24].extract_int()?,
            attachment_count: data[base + 25].extract_int()?,
        })
    }
}

#[derive(Debug)]
pub enum Transform {
    Static(StaticTransform),
    Spline(SplineTransform),
}

impl Transform {
    pub fn from_tokens(data: &[TkKind]) -> Result<Self, Error> {
        match data[0].extract_int()? {
            1 => Ok(Transform::Static(StaticTransform::from_tokens(&data[1..])?)),
            //2 => Ok(Transform::Spline(SplineTransform::from_tokens(data[1..]))?),
            _ => panic!(),
        }
    }
}
#[derive(Debug)]
pub struct StaticTransform {
    pos_x: f32,
    pos_y: f32,
    pos_z: f32,
    scale_x: f32,
    scale_y: f32,
    scale_z: f32,
    rot_x: f32,
    rot_y: f32,
    rot_z: f32,
    rot_w: f32,
}

impl StaticTransform {
    pub const SIZE: usize = 10;
    pub fn from_tokens(data: &[TkKind]) -> Result<Self, Error> {
        Ok(Self {
            pos_x: data[0].extract_float()?,
            pos_y: data[1].extract_float()?,
            pos_z: data[2].extract_float()?,
            scale_x: data[3].extract_float()?,
            scale_y: data[4].extract_float()?,
            scale_z: data[5].extract_float()?,
            rot_x: data[6].extract_float()?,
            rot_y: data[7].extract_float()?,
            rot_z: data[8].extract_float()?,
            rot_w: data[9].extract_float()?,
        })
    }
}

#[derive(Debug)]
pub struct SplineTransform;
