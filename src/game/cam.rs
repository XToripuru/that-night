use super::*;

pub struct Camera {
    pub x: f32,
    pub y: f32,
}

impl Camera {
    pub fn new(x: f32, y: f32) -> Self {
        Camera { x, y }
    }
}
