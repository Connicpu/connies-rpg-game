use glium::texture::{MipmapsOption, Texture1d, UncompressedFloatFormat};
use tiled::Tileset;

use std::collections::HashMap;

use graphics::{self, TextureId};

pub struct TilesetDesc {
    pub tileset: Tileset,
    pub tile_gids: HashMap<u16, u16>,
    pub texture: TextureId,
    pub tile_uv: Texture1d,
    pub rows: u32,
    pub cols: u32,
    pub end_gid: u32,
}

impl TilesetDesc {
    pub fn load(graphics: &mut graphics::System, tileset: Tileset) -> TilesetDesc {
        let texture = graphics.load_texture(&format!("tilesets/{}", tileset.images[0].source));

        let iw = tileset.images[0].width;
        let ih = tileset.images[0].height;
        let tw = tileset.tile_width;
        let th = tileset.tile_height;
        let ts = tileset.spacing;

        let cols = (iw as u32 + 1) / (tw + ts);
        let rows = (ih as u32 + 1) / (th + ts);

        let mut data = vec![];
        for y in 0..rows {
            for x in 0..cols {
                let u0 = ((x * (tw + ts)) as f32 + 0.05) / iw as f32;
                let v0 = ((y * (th + ts)) as f32 + 0.05) / ih as f32;
                let u1 = u0 + (tw as f32 - 0.1) / iw as f32;
                let v1 = v0 + (th as f32 - 0.1) / ih as f32;

                data.push((u0, v0, u1, v1));
            }
        }

        let mut tile_gids = HashMap::with_capacity(tileset.tiles.len());
        for (i, tile) in tileset.tiles.iter().enumerate() {
            let id = tile.id + tileset.first_gid;
            tile_gids.insert(id as u16, i as u16);
        }

        let tile_uv = Texture1d::with_format(
            &graphics.display,
            data,
            UncompressedFloatFormat::F32F32F32F32,
            MipmapsOption::NoMipmap,
        ).unwrap();

        let end_gid = tileset.first_gid + rows * cols;

        TilesetDesc {
            tileset,
            tile_gids,
            texture,
            tile_uv,
            rows,
            cols,
            end_gid,
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct TileInstance {
    pub tile_id: u16,
}

implement_vertex!(TileInstance, tile_id);
