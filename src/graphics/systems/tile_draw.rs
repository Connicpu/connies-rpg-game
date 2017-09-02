use cgmath::Vector2;

use std::f32;

use DataHelper;
use tilemap::Map;

#[derive(Default, System)]
#[process(process)]
pub struct TileDraw;

const CHUNK_SCALE: Vector2<f32> = Vector2 {
    x: 0.125,
    y: -0.125,
};

fn process(_: &mut TileDraw, data: &mut DataHelper) {
    let map: &Map = match data.services.current_map {
        Some(ref map) => map,
        None => return,
    };

    let camera_aabb = data.services.camera.aabb();
    let chunk_range = camera_aabb
        .scaled(CHUNK_SCALE)
        .to_int()
        .restricted_min(0, 0)
        .restricted_max(map.h_chunks as i32 - 1, map.v_chunks as i32 - 1)
        .into_iter()
        .to_u32();

    let mut frame = data.services.graphics.current_frame.take().unwrap();

    for i in 0..map.layers.len() {
        let layer = &map.layers[i];
        for (x, y) in chunk_range {
            let pos = [x as f32 * 8.0, y as f32 * -8.0, i as f32];
            let chunk = &layer.chunks.chunks[(x + y * map.h_chunks) as usize];
            data.services
                .graphics
                .draw_tile_chunk(&mut frame, pos, layer.tint, chunk, &map.tilesets);
        }
    }
    
    data.services.graphics.current_frame = Some(frame);
}
