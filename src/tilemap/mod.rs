use tiled;

use graphics;
use physics as p;

pub use tilemap::chunks::{Chunk, Chunks};
pub use tilemap::layer::Layer;
pub use tilemap::tilesets::Tilesets;

pub mod chunks;
pub mod layer;
pub mod tilesets;

pub struct Map {
    pub tilesets: Tilesets,
    pub layers: Vec<Layer>,

    pub v_chunks: u32,
    pub h_chunks: u32,
}

impl Map {
    pub fn create_physics(&self, layer: usize, physics: &mut p::World) {
        let (hc, vc) = (self.h_chunks, self.v_chunks);
        let coords = (0..hc).flat_map(|y| (0..vc).map(move |x| (x, y)));
        for (chunk, (x, y)) in self.layers[layer].chunks.chunks.iter().zip(coords) {
            let pos = [x as f32 * 8.0, y as f32 * -8.0];
            chunk.build_physics(physics, &self.tilesets, pos);
        }
    }
}

pub struct MapBuilder<'a> {
    pub graphics: &'a mut graphics::System,
    pub map: tiled::Map,
    pub tilesets: Tilesets,
    pub layers: Vec<Layer>,

    pub v_chunks: u32,
    pub h_chunks: u32,
}

pub fn load_map(map: tiled::Map, graphics: &mut graphics::System) -> Map {
    let v_chunks = (map.height + 7) / 8;
    let h_chunks = (map.width + 7) / 8;

    let mut builder = MapBuilder {
        graphics,
        map,
        tilesets: Tilesets::empty(),
        layers: vec![],

        v_chunks,
        h_chunks,
    };

    let tilesets = Tilesets::build(&mut builder);
    builder.tilesets = tilesets;

    for i in 0..builder.map.layers.len() {
        let layer = Layer::build(&builder, i);
        builder.layers.push(layer);
    }

    let MapBuilder { tilesets, layers, .. } = builder;

    Map {
        tilesets,
        layers,
        
        v_chunks,
        h_chunks,
    }
}
