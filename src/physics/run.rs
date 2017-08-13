use specs::{System, FetchMut, Fetch};
use physics::World;
use timer::Timer;

pub struct PhysicsRun;

#[derive(SystemData)]
pub struct Data<'a> {
    world: FetchMut<'a, World>,
    timer: Fetch<'a, Timer>,
}

impl<'a> System<'a> for PhysicsRun {
    type SystemData = Data<'a>;

    fn run(&mut self, mut data: Data) {
        let world = data.world.access();

        world.step(data.timer.delta_time, 8, 3);
    }
}
