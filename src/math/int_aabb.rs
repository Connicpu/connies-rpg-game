use std::i32;
use std::cmp::{max, min};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct IntAabb {
    pub min_x: i32,
    pub min_y: i32,
    pub max_x: i32,
    pub max_y: i32,
}

impl IntAabb {
    pub fn empty() -> Self {
        IntAabb {
            min_x: i32::MAX,
            min_y: i32::MAX,
            max_x: i32::MIN,
            max_y: i32::MIN,
        }
    }

    pub fn expand(&mut self, other: IntAabb) -> &mut Self {
        self.expand_point(other.min_x, other.min_y)
            .expand_point(other.max_x, other.max_y)
    }

    pub fn expand_point(&mut self, x: i32, y: i32) -> &mut Self {
        self.min_x = min(self.min_x, x);
        self.min_y = min(self.min_y, y);

        self.max_x = max(self.max_x, x);
        self.max_y = max(self.max_y, y);

        self
    }

    pub fn restrict(&mut self, other: IntAabb) -> &mut Self {
        self.min_x = max(self.min_x, other.min_x);
        self.min_y = max(self.min_y, other.min_y);

        self.max_x = min(self.max_x, other.max_x);
        self.max_y = min(self.max_y, other.max_y);

        self
    }

    pub fn restricted(self, other: IntAabb) -> Self {
        *self.clone().restrict(other)
    }

    pub fn restricted_min(mut self, x: i32, y: i32) -> Self {
        self.min_x = max(self.min_x, x);
        self.min_y = max(self.min_y, y);
        self
    }

    pub fn restricted_max(mut self, x: i32, y: i32) -> Self {
        self.max_x = min(self.max_x, x);
        self.max_y = min(self.max_y, y);
        self
    }
}

#[derive(Copy, Clone)]
pub struct IntAabbIter {
    aabb: IntAabb,
    y: i32,
    x: i32,
}

impl IntAabbIter {
    pub fn to_u32(self) -> IntAabbUIter {
        assert!(self.aabb.min_x >= 0 && self.aabb.min_y >= 0);
        IntAabbUIter { inner: self }
    }
}

impl IntoIterator for IntAabb {
    type Item = (i32, i32);
    type IntoIter = IntAabbIter;
    fn into_iter(self) -> Self::IntoIter {
        IntAabbIter {
            aabb: self,
            y: self.min_y,
            x: self.min_x,
        }
    }
}

impl Iterator for IntAabbIter {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<(i32, i32)> {
        if self.y > self.aabb.max_y {
            return None;
        }

        let result = (self.x, self.y);

        if self.x == self.aabb.max_x {
            self.y += 1;
            self.x = self.aabb.min_x;
        } else {
            self.x += 1;
        }

        Some(result)
    }
}

#[derive(Copy, Clone)]
pub struct IntAabbUIter {
    inner: IntAabbIter,
}

impl Iterator for IntAabbUIter {
    type Item = (u32, u32);

    fn next(&mut self) -> Option<(u32, u32)> {
        self.inner.next().map(|(x, y)| (x as u32, y as u32))
    }
}
