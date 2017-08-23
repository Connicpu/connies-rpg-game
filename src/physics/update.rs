use ecs::EntityIter;

use Components;
use DataHelper;

def_system! {
    #[entity]
    #[aspect(all: [body, transform])]
    pub struct PhysicsUpdate;
}

fn process(entities: EntityIter<Components>, data: &mut DataHelper) {
    for entity in entities {
        let body_handle = data.components.body[entity].handle;
        let body = data.services.physics.world.body(body_handle);
        let body_pos = body.transform();

        let transform = &mut data.components.transform[entity];
        transform.pos.x = body_pos.pos.x;
        transform.pos.y = body_pos.pos.y;
        transform.rot = body_pos.rot.angle();
    }
}
