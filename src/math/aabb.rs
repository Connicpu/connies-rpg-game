use cgmath::Vector2;

use std::f32;

#[derive(Copy, Clone, Debug)]
pub struct Aabb {
    pub min: Vector2<f32>,
    pub max: Vector2<f32>,
}

impl Aabb {
    pub fn new() -> Self {
        Aabb {
            min: Vector2 {
                x: f32::MAX,
                y: f32::MAX,
            },
            max: Vector2 {
                x: f32::MIN,
                y: f32::MIN,
            },
        }
    }

    pub fn expand(&mut self, aabb: Aabb) -> &mut Self {
        self.expand_point(aabb.min).expand_point(aabb.max)
    }

    pub fn expand_point(&mut self, point: Vector2<f32>) -> &mut Self {
        self.min.x = self.min.x.min(point.x);
        self.min.y = self.min.y.min(point.y);

        self.max.x = self.max.x.max(point.x);
        self.max.y = self.max.y.max(point.y);

        self
    }
}
