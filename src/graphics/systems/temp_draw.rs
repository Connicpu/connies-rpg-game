use DataHelper;
use tilemap::Map;

use cgmath::prelude::*;
use cgmath::*;

use math::raw::ToRawMath;

use std::f32;

#[derive(Default, System)]
#[process(process)]
pub struct TempDraw;

const CAMERA_BOUNDING_POINTS: [Point3 <f32>; 4] = [Point3::<f32>{x: -1.0, y: -1.0, z: 0.0}, Point3::<f32>{x: 1.0, y: -1.0, z: 0.0}, Point3::<f32>{x: -1.0, y: 1.0, z: 0.0}, Point3::<f32>{x: 1.0, y: 1.0, z: 0.0}];

fn process(_: &mut TempDraw, data: &mut DataHelper) {
    let mut frame = data.services.graphics.current_frame.take().unwrap();

    let map: &Map = match data.services.current_map {
        Some(ref map) => map,
        None => return,
    };
    
    let graphics_cam = data.services.camera.to_raw ();
    let camera_view_proj = Matrix4::<f32>::from (graphics_cam.view) * Matrix4::<f32>::from(graphics_cam.proj);
    let inverse_camera_view_proj: &mut Matrix4 <f32> = &mut camera_view_proj.transpose ().invert ().unwrap ();
    
    let mut camera_aabb_min: Point3<f32> = Point3::<f32> {x: f32::MAX, y: f32::MAX, z: 0.0};
    let mut camera_aabb_max: Point3<f32> = Point3::<f32> {x: f32::MIN, y: f32::MIN, z: 0.0};
    
    for point in CAMERA_BOUNDING_POINTS.into_iter() {
        let transformed_point = inverse_camera_view_proj.transform_point(point.clone());
        camera_aabb_min.x = camera_aabb_min.x.min(transformed_point.x);
        camera_aabb_min.y = camera_aabb_min.y.min(transformed_point.y);
        camera_aabb_max.x = camera_aabb_max.x.max(transformed_point.x);
        camera_aabb_max.y = camera_aabb_max.y.max(transformed_point.y);
    }
    
    for i in 0..map.layers.len() {
        let layer = &map.layers[i];
        for y in 0..map.v_chunks {
            for x in 0..map.h_chunks {
                let pos = [x as f32 * 8.0, y as f32 * -8.0, i as f32];
                let chunk = &layer.chunks.chunks[(x + y * map.h_chunks) as usize];
                if (pos [0] < camera_aabb_max.x) && (pos [0] + 8.0 > camera_aabb_min.x) &&
                 (pos [1] > camera_aabb_min.y) && (pos [1] - 8.0 < camera_aabb_max.y) {
                    data.services
                    .graphics
                    .draw_tile_chunk(&mut frame, pos, layer.tint, chunk, &map.tilesets);
                }
            }
        }
    }

    data.services.graphics.current_frame = Some(frame);
}
