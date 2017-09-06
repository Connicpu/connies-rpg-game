use std::collections::HashMap;

use DataHelper;
use graphics::{SpriteInstance, TextureId};
use components::{Sprite, Transform};

#[derive(Default, System)]
#[process(process)]
pub struct DrawSprites {
    instance_lists: HashMap<TextureId, Vec<SpriteInstance>>,
}

fn process(sys: &mut DrawSprites, data: &mut DataHelper) {
    let instances = &mut sys.instance_lists;

    let aabb = data.services.camera.aabb();
    for entity in data.services.graphics.scene_grid.entities(aabb) {
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

        let (sin, cos) = (rot.sin, rot.cos);

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
        if instances.is_empty() {
            continue;
        }

        data.services
            .graphics
            .draw_sprites(&mut frame, instances, tex);
        instances.clear();
    }
    data.services.graphics.current_frame = Some(frame);
}
