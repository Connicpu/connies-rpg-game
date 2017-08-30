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
    let x_low = (camera_aabb.min.x * 0.125) as i32;
    let y_low = (camera_aabb.max.y * - 0.125) as i32;
    let x_high = (camera_aabb.max.x * 0.125) as i32 + 1;
    let y_high = (camera_aabb.min.y * - 0.125) as i32 + 1;
    
    let x_low = if x_low < 0 { 0u32 } else { x_low as u32 };
    let y_low = if y_low < 0 { 0u32 } else { y_low as u32 };
    let x_high = if x_high < 0 { 0u32 } else { x_high as u32 };
    let y_high = if y_high < 0 { 0u32 } else { y_high as u32 };
    
    let x_low = if x_low > map.h_chunks { map.h_chunks } else { x_low };
    let y_low = if y_low > map.v_chunks { map.v_chunks } else { y_low };
    let x_high = if x_high > map.h_chunks { map.h_chunks } else { x_high };
    let y_high = if y_high > map.v_chunks { map.v_chunks } else { y_high };

    for i in 0..map.layers.len() {
        let layer = &map.layers[i];
        for y in y_low..y_high {
            for x in x_low..x_high {
                let pos = [x as f32 * 8.0, y as f32 * -8.0, i as f32];
                let chunk = &layer.chunks.chunks[(x + y * map.h_chunks) as usize];
                data.services.graphics
                    .draw_tile_chunk(&mut frame, pos, layer.tint, chunk, &map.tilesets);
            }
        }
    }

    data.services.graphics.current_frame = Some(frame);
}
