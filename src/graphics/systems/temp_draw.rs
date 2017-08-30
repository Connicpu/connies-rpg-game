use DataHelper;
use tilemap::Map;

use std::f32;

#[derive(Default, System)]
#[process(process)]
pub struct TempDraw;

fn process(_: &mut TempDraw, data: &mut DataHelper) {
    let mut frame = data.services.graphics.current_frame.take().unwrap();

    let map: &Map = match data.services.current_map {
        Some(ref map) => map,
        None => return,
    };

    let camera_aabb = data.services.camera.aabb();

    for i in 0..map.layers.len() {
        let layer = &map.layers[i];
        for y in 0..map.v_chunks {
            for x in 0..map.h_chunks {
                let pos = [x as f32 * 8.0, y as f32 * -8.0, i as f32];
                let chunk = &layer.chunks.chunks[(x + y * map.h_chunks) as usize];
                if (pos[0] < camera_aabb.max.x) && (pos[0] + 8.0 > camera_aabb.min.x) &&
                    (pos[1] > camera_aabb.min.y) &&
                    (pos[1] - 8.0 < camera_aabb.max.y)
                {
                    data.services.graphics.draw_tile_chunk(
                        &mut frame,
                        pos,
                        layer.tint,
                        chunk,
                        &map.tilesets,
                    );
                }
            }
        }
    }

    data.services.graphics.current_frame = Some(frame);
}
