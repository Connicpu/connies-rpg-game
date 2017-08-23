use graphics::tileset::TilesetDesc;
use tilemap::MapBuilder;

pub struct Tilesets {
    pub tileset_descs: Vec<TilesetDesc>,
}

impl Tilesets {
    pub fn empty() -> Self {
        Tilesets { tileset_descs: vec![] }
    }

    pub fn build(builder: &mut MapBuilder) -> Self {
        // Reborrow. `&mut *` can be removed if that's ever made implicit outside
        // of function calls, or if closure captures of structs is ever made
        // more fine-grained
        let graphics = &mut *builder.graphics;

        let tileset_descs = builder
            .map
            .tilesets
            .iter()
            .map(|tileset| {
                TilesetDesc::load(graphics, tileset.clone())
            })
            .collect();

        Tilesets { tileset_descs }
    }
}
