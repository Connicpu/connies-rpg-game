use wrapped2d::b2;
use physics as p;

pub trait BodyExt {
    fn apply_horiz_accel(&mut self, force: f32);
    fn apply_vert_impulse(&mut self, impulse: f32);
}

impl BodyExt for b2::MetaBody<p::EntityUserData> {
    fn apply_horiz_accel(&mut self, force: f32) {
        self.apply_force_to_center(&b2::Vec2 { x: force, y: 0.0 }, true);
    }

    fn apply_vert_impulse(&mut self, impulse: f32) {
        let world_center = *self.world_center();
        self.apply_linear_impulse(&b2::Vec2 { x: 0.0, y: impulse }, &world_center, true);
    }
}
