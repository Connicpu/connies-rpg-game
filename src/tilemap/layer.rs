use tilemap::Chunks;
use tilemap::MapBuilder;

pub struct Layer {
    pub chunks: Chunks,
    pub tint: [f32; 4],
    pub collision: bool,
}

impl Layer {
    pub fn build(b: &MapBuilder, layer: usize) -> Layer {
        let mut tint = [1.0; 4];
        let mut collision = true;

        let chunks = Chunks::build(b, layer);

        for (key, value) in &b.map.layers[layer].properties {
            use tiled::PropertyValue::*;
            match (&key[..], value) {
                ("tint", &ColorValue(color)) => {
                    tint = [
                        ((color & 0x00FF0000) >> 0x10) as f32 / 255.0,
                        ((color & 0x0000FF00) >> 0x08) as f32 / 255.0,
                        ((color & 0x000000FF) >> 0x00) as f32 / 255.0,
                        
                        ((color & 0xFF000000) >> 0x18) as f32 / 255.0,
                    ];
                }
                ("nocollide", &BoolValue(true)) => {
                    collision = false;
                }
                _ => (),
            }
        }

        Layer {
            chunks,
            tint,
            collision,
        }
    }
}
