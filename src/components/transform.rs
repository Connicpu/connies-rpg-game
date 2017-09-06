use cgmath::{ElementWise, Matrix2, Vector2};
use math::{Aabb, Rot};
use graphics::quad_types::QUAD_VERTICES;

#[derive(Copy, Clone)]
pub struct Transform {
    pub pos: Vector2<f32>,
    pub rot: Rot,
    pub scale: f32,
    pub size: Vector2<f32>,
}

impl Default for Transform {
    fn default() -> Self {
        Transform::new()
    }
}

impl Transform {
    pub fn new() -> Self {
        Transform {
            pos: Vector2 { x: 0.0, y: 0.0 },
            rot: Rot::new(0.0),
            scale: 1.0,
            size: Vector2 { x: 1.0, y: 1.0 },
        }
    }

    pub fn transform_point(&self, point: Vector2<f32>) -> Vector2<f32> {
        let (sin, cos) = (self.rot.sin, self.rot.cos);
        let rot = Matrix2::new(cos, sin, -sin, cos);
        rot * (point * self.scale).mul_element_wise(self.size) + self.pos
    }

    pub fn sprite_aabb(&self, center: Vector2<f32>) -> Aabb {
        let mut aabb = Aabb::empty();

        for vertex in &QUAD_VERTICES {
            let world_pt = self.transform_point(Vector2 {
                x: vertex.pos[0] - center.x,
                y: vertex.pos[1] + center.y,
            });
            aabb.expand_point(world_pt);
        }

        aabb
    }
}
