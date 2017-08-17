use ecs::system::{System, Process};

pub struct PhysicsRun;

impl System for PhysicsRun {
    type Components = ::Components;
    type Services = ::Services;
}

impl Process for PhysicsRun {
    fn process(&mut self, data: &mut ::DataHelper) {
        let dt = data.services.timer.delta_time;
        data.services.physics.world.step(dt, 8, 3);
    }
}
