use {DataHelper, EntityIter};

#[derive(Default, System)]
#[system_type(Entity)]
#[aspect(all(sprite, transform))]
#[process(process)]
pub struct SpriteWatcher;

fn process(_: &mut SpriteWatcher, entities: EntityIter, data: &mut DataHelper) {
    for entity in entities {
        let pos = data.components.transform[entity].pos;
        data.services.graphics.scene_grid.set(entity, pos);
    }
}
