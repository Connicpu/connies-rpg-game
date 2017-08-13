use specs::{System, FetchMut, Fetch, ReadStorage, WriteStorage, Join};
use physics::{World, Body};
use components::Transform;

pub struct PhysicsUpdate;

#[derive(SystemData)]
pub struct Data<'a> {
    world: FetchMut<'a, World>,
    body: ReadStorage<'a, Body>,
    transform: WriteStorage<'a, Transform>,
}

impl<'a> System<'a> for PhysicsUpdate {
    type SystemData = Data<'a>;

    fn run(&mut self, mut data: Data) {
        let world = data.world.access();

        for (transform, body) in (&mut data.transform, &data.body).join() {
            let body = world.body(body.handle);
            let body_pos = body.transform();
            transform.pos.x = body_pos.pos.x;
            transform.pos.x = body_pos.pos.y;
            transform.rot = body_pos.rot.angle();
        }
    }
}

