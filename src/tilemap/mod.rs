use tiled;

use std::collections::HashMap;

use tilemap::chunks::Chunks;
use tilemap::tilesets::Tilesets;

pub mod chunks;
pub mod tilesets;

pub struct MapBuilder {
    map: tiled::Map,
    tilesets: Option<Tilesets>,
    chunks: HashMap<usize, Chunks>,
}
