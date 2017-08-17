use cgmath::Vector2;

#[derive(Copy, Clone)]
pub struct Transform {
    pub pos: Vector2<f32>,
    pub rot: f32,
    pub scale: f32,
    pub size: Vector2<f32>,
}

impl Transform {
    pub fn new() -> Self {
        Transform {
            pos: Vector2 { x: 0.0, y: 0.0 },
            rot: 0.0,
            scale: 1.0,
            size: Vector2 { x: 1.0, y: 1.0 },
        }
    }
}
