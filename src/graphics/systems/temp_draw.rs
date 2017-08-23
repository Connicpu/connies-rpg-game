use DataHelper;
use tilemap::Map;

def_system! {
    pub struct TempDraw;
}


fn process(data: &mut DataHelper) {
    let mut frame = data.services.graphics.current_frame.take().unwrap();

    let map: &Map = match data.services.current_map {
        Some(ref map) => map,
        None => return,
    };

    for i in 0..map.layers.len() {
        for y in 0..map.v_chunks {
            for x in 0..map.h_chunks {
                let pos = [x as f32 * 8.0, y as f32 * -8.0, i as f32];
                let layer = &map.layers[i];
                let chunk = &layer.chunks.chunks[(x + y * map.h_chunks) as usize];
                data.services.graphics.draw_tile_chunk(&mut frame, pos, layer.tint, chunk, &map.tilesets);
            }
        }
    }

    data.services.graphics.current_frame = Some(frame);
}
