use tiled;

use graphics;

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
