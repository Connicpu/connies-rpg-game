use cgmath::Vector2;

use graphics;
use math::matrices;
use math::raw::ToRawMath;

#[derive(Copy, Clone)]
pub struct Camera {
    pub pos: Vector2<f32>,
    pub viewport_height: f32,
    pub aspect_ratio: f32,
    pub roll: f32,
    pub near: f32,
    pub far: f32,
}

impl ToRawMath for Camera {
    type Raw = graphics::Camera;
    fn to_raw(self) -> Self::Raw {
        graphics::Camera {
            view: matrices::view(self.pos, self.viewport_height, self.roll).to_raw(),
            proj: matrices::ortho(self.aspect_ratio, self.near, self.far).to_raw(),
        }
    }
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            pos: Vector2 { x: 0.0, y: 0.0 },
            viewport_height: 1.0,
            aspect_ratio: 1.0,
            roll: 0.0,
            near: -100.0,
            far: 100.0,
        }
    }
}
