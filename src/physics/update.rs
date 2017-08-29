use DataHelper;
use EntityIter;

#[derive(Default, System)]
#[system_type(Entity)]
#[process(process)]
#[aspect(all(body, transform))]
pub struct PhysicsUpdate;

fn process(_: &mut PhysicsUpdate, entities: EntityIter, data: &mut DataHelper) {
    for entity in entities {
        let body_handle = data.components.body[entity].handle;
        let body = data.services.physics.world.body(body_handle);
        let body_pos = body.transform();

        let transform = &mut data.components.transform[entity];
        transform.pos.x = body_pos.pos.x;
        transform.pos.y = body_pos.pos.y;
        transform.rot.angle = body_pos.rot.angle();
        transform.rot.sin = body_pos.rot.sin;
        transform.rot.cos = body_pos.rot.cos;
    }
}
