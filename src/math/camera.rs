use cgmath::{Matrix4, Point3, Vector2};
use cgmath::prelude::*;

use graphics;
use math::{matrices, Aabb, Rot};
use math::raw::ToRawMath;

#[derive(Copy, Clone)]
pub struct Camera {
    pub pos: Vector2<f32>,
    pub viewport_height: f32,
    pub aspect_ratio: f32,
    pub roll: Rot,
    pub near: f32,
    pub far: f32,
}

impl Camera {
    pub fn aabb(&self) -> Aabb {
        let graphics_cam = self.to_raw();
        let camera_view_proj = Matrix4::from(graphics_cam.view) * Matrix4::from(graphics_cam.proj);
        let inverse_camera_view_proj = camera_view_proj.transpose().invert().unwrap();

        let mut aabb = Aabb::new();

        for &point in CAMERA_BOUNDING_POINTS.iter() {
            let point = inverse_camera_view_proj.transform_point(point.clone());
            aabb.expand_point(Vector2 { x: point.x, y: point.y });
        }

        aabb
    }
}

impl ToRawMath for Camera {
    type Raw = graphics::Camera;
    fn to_raw(self) -> Self::Raw {
        graphics::Camera {
            view: matrices::view(self.pos, self.viewport_height, &self.roll).to_raw(),
            proj: matrices::ortho(self.aspect_ratio, self.near, self.far).to_raw(),
        }
    }
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            pos: Vector2 { x: 0.0, y: -256.0 },
            viewport_height: 8.0,
            aspect_ratio: 1.0,
            roll: Rot::new(0.0),
            near: -100.0,
            far: 100.0,
        }
    }
}

static CAMERA_BOUNDING_POINTS: [Point3<f32>; 4] = [
    Point3 {
        x: -1.0,
        y: -1.0,
        z: 0.0,
    },
    Point3 {
        x: 1.0,
        y: -1.0,
        z: 0.0,
    },
    Point3 {
        x: -1.0,
        y: 1.0,
        z: 0.0,
    },
    Point3 {
        x: 1.0,
        y: 1.0,
        z: 0.0,
    },
];
