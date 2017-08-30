#[derive(Copy, Clone)]
pub struct Rot {
    pub angle: f32,
    pub sin: f32,
    pub cos: f32,
}

impl Rot {
    pub fn new(angle: f32) -> Self {
        let (sin, cos) = (angle.sin(), angle.cos());
        Rot { angle, sin, cos }
    }

    pub fn set(&mut self, angle: f32) {
        *self = Rot::new(angle)
    }
}
