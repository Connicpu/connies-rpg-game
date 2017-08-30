
use cgmath::{ElementWise, Vector2};

use std::f32;
use std::cmp::Ordering;
use std::cmp::Ordering::*;

use math::IntAabb;

const EPSILON: f32 = 1e-7;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum AabbRelation {
    Disjoint,
    Intersects,
    Contains,
    ContainedBy,
}

#[derive(Copy, Clone, Debug)]
pub struct Aabb {
    pub min: Vector2<f32>,
    pub max: Vector2<f32>,
}

impl Aabb {
    pub fn empty() -> Self {
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

    pub fn new(min: Vector2<f32>, max: Vector2<f32>) -> Self {
        Aabb { min, max }
    }

    pub fn relation_to(&self, other: &Aabb) -> AabbRelation {
        let cmps = self.gen_comparisons(other);

        if cmps.min_min_x != Greater && cmps.min_min_y != Greater && cmps.max_max_x != Less &&
            cmps.max_max_y != Less
        {
            return AabbRelation::Contains;
        }

        if cmps.min_min_x != Less && cmps.min_min_y != Less && cmps.max_max_x != Greater &&
            cmps.max_max_y != Greater
        {
            return AabbRelation::ContainedBy;
        }

        if cmps.min_max_x == Greater || cmps.min_max_y == Greater {
            return AabbRelation::Disjoint;
        }

        if cmps.max_min_x == Less || cmps.max_min_y == Less {
            return AabbRelation::Disjoint;
        }

        AabbRelation::Intersects
    }

    pub fn disjoint_from(&self, other: &Aabb) -> bool {
        self.relation_to(other) == AabbRelation::Disjoint
    }

    pub fn overlaps(&self, other: &Aabb) -> bool {
        self.relation_to(other) != AabbRelation::Disjoint
    }

    pub fn intersects(&self, other: &Aabb) -> bool {
        self.relation_to(other) == AabbRelation::Intersects
    }

    pub fn contains(&self, other: &Aabb) -> bool {
        self.relation_to(other) == AabbRelation::Contains
    }

    pub fn contained_by(&self, other: &Aabb) -> bool {
        self.relation_to(other) == AabbRelation::ContainedBy
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

    pub fn scaled(self, scale: Vector2<f32>) -> Aabb {
        Aabb {
            min: self.min.mul_element_wise(scale),
            max: self.max.mul_element_wise(scale),
        }
    }

    pub fn to_int(self) -> IntAabb {
        IntAabb {
            min_x: self.min.x.floor() as i32,
            min_y: self.min.y.floor() as i32,
            max_x: self.max.x.ceil() as i32,
            max_y: self.max.y.ceil() as i32,
        }
    }

    fn gen_comparisons(&self, other: &Aabb) -> AabbCmp {
        use self::AabbField::*;
        AabbCmp {
            min_min_x: self.compare_fields(other, MinX, MinX),
            min_max_x: self.compare_fields(other, MinX, MaxX),
            max_min_x: self.compare_fields(other, MaxX, MinX),
            max_max_x: self.compare_fields(other, MaxX, MaxX),

            min_min_y: self.compare_fields(other, MinY, MinY),
            min_max_y: self.compare_fields(other, MinY, MaxY),
            max_min_y: self.compare_fields(other, MaxY, MinY),
            max_max_y: self.compare_fields(other, MaxY, MaxY),
        }
    }

    fn compare_fields(
        &self,
        other: &Aabb,
        left_field: AabbField,
        right_field: AabbField,
    ) -> Ordering {
        let left = self.get_field(left_field);
        let right = other.get_field(right_field);
        if (left - right).abs() <= EPSILON {
            Equal
        } else if left < right {
            Less
        } else {
            Greater
        }
    }

    fn get_field(&self, field: AabbField) -> f32 {
        match field {
            AabbField::MinX => self.min.x,
            AabbField::MinY => self.min.y,
            AabbField::MaxX => self.max.x,
            AabbField::MaxY => self.max.y,
        }
    }
}

enum AabbField {
    MinX,
    MinY,
    MaxX,
    MaxY,
}

struct AabbCmp {
    min_min_x: Ordering,
    min_max_x: Ordering,
    max_min_x: Ordering,
    max_max_x: Ordering,

    min_min_y: Ordering,
    min_max_y: Ordering,
    max_min_y: Ordering,
    max_max_y: Ordering,
}
