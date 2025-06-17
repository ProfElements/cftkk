use super::Token;

#[derive(Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

#[derive(Debug)]
pub struct Quat {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

impl Quat {
    pub fn from_xyzw(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }
}

#[derive(Debug)]
pub struct Transform {
    pub position: Vec3,
    scale: Vec3,
    rotation: Quat,
}

impl Transform {
    pub const TOKEN_SIZE: usize = 10;
    pub fn from_tokens(transform: &[Token]) -> Option<Self> {
        if transform.len() < Self::TOKEN_SIZE {
            return None;
        }

        let Token::F32(pos_x) = transform[0] else {
            return None;
        };
        let Token::F32(pos_y) = transform[1] else {
            return None;
        };
        let Token::F32(pos_z) = transform[2] else {
            return None;
        };

        let Token::F32(scale_x) = transform[3] else {
            return None;
        };
        let Token::F32(scale_y) = transform[4] else {
            return None;
        };
        let Token::F32(scale_z) = transform[5] else {
            return None;
        };

        let Token::F32(quat_x) = transform[6] else {
            return None;
        };
        let Token::F32(quat_y) = transform[7] else {
            return None;
        };
        let Token::F32(quat_z) = transform[8] else {
            return None;
        };

        let Token::F32(quat_w) = transform[9] else {
            return None;
        };

        Some(Transform {
            position: Vec3::new(pos_x, pos_y, pos_z),
            scale: Vec3::new(scale_x, scale_y, scale_z),
            rotation: Quat::from_xyzw(quat_x, quat_y, quat_z, quat_w),
        })
    }

    pub fn token_size(&self) -> usize {
        10
    }
}

#[derive(Debug)]
pub struct Bounds {
    pub min: Vec3,
    pub max: Vec3,
}
impl Bounds {
    pub const TOKEN_SIZE: usize = 6;
    pub fn from_tokens(data: &[Token]) -> Option<Bounds> {
        if data.len() < Self::TOKEN_SIZE {
            return None;
        }

        let Token::F32(min_x) = data[0] else {
            return None;
        };
        let Token::F32(min_y) = data[1] else {
            return None;
        };
        let Token::F32(min_z) = data[2] else {
            return None;
        };

        let Token::F32(max_x) = data[3] else {
            return None;
        };
        let Token::F32(max_y) = data[4] else {
            return None;
        };
        let Token::F32(max_z) = data[5] else {
            return None;
        };

        Some(Bounds {
            min: Vec3::new(min_x, min_y, min_z),
            max: Vec3::new(max_x, max_y, max_z),
        })
    }

    pub fn token_size(&self) -> usize {
        Self::TOKEN_SIZE
    }
}
