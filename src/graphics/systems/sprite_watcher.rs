use {Components, DataHelper, EntityData, EntityIter, Services};

#[derive(Default, System)]
#[system_type(Entity)]
#[aspect(all(sprite, transform))]
#[process(process)]
#[deactivated(deactivated)]
pub struct SpriteWatcher;

fn process(_: &mut SpriteWatcher, entities: EntityIter, data: &mut DataHelper) {
    for entity in entities {
        let pos = data.components.transform[entity].pos;
        data.services.graphics.scene_grid.set(entity, pos);
    }
}

fn deactivated(_: &mut SpriteWatcher, entity: EntityData, _: &Components, s: &mut Services) {
    s.graphics.scene_grid.remove(entity);
}
