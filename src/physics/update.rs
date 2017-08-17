use ecs::system::{System, EntityProcess};
use ecs::EntityIter;

pub struct PhysicsUpdate;

impl System for PhysicsUpdate {
    type Components = ::Components;
    type Services = ::Services;
}

impl EntityProcess for PhysicsUpdate {
    fn process<'a>(&mut self, entities: EntityIter<'a, ::Components>, data: &mut ::DataHelper) {
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
}
