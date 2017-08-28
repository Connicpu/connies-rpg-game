use DataHelper;
use EntityIter;

#[derive(Default, System)]
#[system_type(Entity)]
#[process(process)]
#[aspect(all(body, transform))]
pub struct PhysicsPreUpdate;

fn process(_: &mut PhysicsPreUpdate, entities: EntityIter, data: &mut DataHelper) {
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
