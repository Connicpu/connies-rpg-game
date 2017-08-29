use std::collections::HashMap;

use {DataHelper, EntityIter};
use graphics::{SpriteInstance, TextureId};
use components::{Sprite, Transform};

#[derive(Default, System)]
#[system_type(Entity)]
#[aspect(all(transform, sprite))]
#[process(process)]
pub struct DrawSprites {
    instance_lists: HashMap<TextureId, Vec<SpriteInstance>>,
}

fn process(sys: &mut DrawSprites, entities: EntityIter, data: &mut DataHelper) {
    let ref mut instances = sys.instance_lists;

    for entity in entities {
        let transform = data.components.transform[entity];
        let sprite = data.components.sprite[entity];

        let Transform {
            pos,
            rot,
            scale,
            size,
        } = transform;
        let Sprite {
            sprite,
            center,
            uv_rect,
            layer,
        } = sprite;

        let (sin, cos) = (rot.sin(), rot.cos());

        let instance = SpriteInstance {
            center,
            scale: [size[0] * scale, size[1] * scale],
            rot: [[cos, -sin], [sin, cos]],
            uv_rect,
            world_pos: [pos.x, pos.y, layer],
        };

        instances
            .entry(sprite)
            .or_insert_with(|| Vec::with_capacity(4))
            .push(instance);
    }

    let mut frame = data.services.graphics.current_frame.take().unwrap();
    for (&tex, instances) in instances.iter_mut() {
        data.services
            .graphics
            .draw_sprites(&mut frame, instances, tex);
        instances.clear();
    }
    data.services.graphics.current_frame = Some(frame);
}
