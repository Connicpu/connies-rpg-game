use specs::{System, FetchMut, Fetch, Entity};
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
        let mut world = data.world.lock.lock().unwrap();

        world.step(1.0 / 60.0, 8, 3);
    }
}
