use std::collections::HashSet;
use std::cmp;

use tilemap::MapBuilder;

const MAX_TILESETS: usize = 7;

pub struct Chunks {
    pub chunks: Vec<Chunk>,
    pub chunks_width: usize,
}

pub struct Chunk {
    pub tiles: [u16; 64],
    pub tilesets: [u16; MAX_TILESETS],
    pub tilesets_count: u16,
}

impl Chunks {
    pub fn build(b: &MapBuilder, layer: usize) -> Self {
        let mut chunks = vec![];

        for i in 0..b.v_chunks {
            for j in 0..b.h_chunks {
                chunks.push(Chunk::build(b, layer, j * 8, i * 8));
            }
        }

        Chunks {
            chunks,
            chunks_width: ((b.map.width + 7) / 8) as usize,
        }
    }
}

impl Chunk {
    fn build(b: &MapBuilder, layer: usize, x: u32, y: u32) -> Self {
        let mut tiles = [0; 64];
        let mut tilesets = [0; MAX_TILESETS];
        let mut tilesets_h = HashSet::new();

        for i in 0..8 {
            for j in 0..8 {
                let x = x + j;
                let y = y + i;
                let tile = if x < b.map.width && y < b.map.height {
                    b.map.layers[layer].tiles[y as usize][x as usize]
                } else {
                    0
                };

                tiles[(i * 8 + j) as usize] = tile as u16;

                if tile != 0 {
                    if let Some(tileset) = Chunk::find_tileset(b, tile) {
                        tilesets_h.insert(tileset);
                    }
                }
            }
        }

        let tilesets_count = cmp::min(tilesets_h.len(), MAX_TILESETS) as u16;
        for (i, &tileset) in tilesets_h.iter().take(tilesets_count as usize).enumerate() {
            tilesets[i] = tileset;
        }

        Chunk {
            tiles,
            tilesets,
            tilesets_count,
        }
    }

    fn find_tileset(b: &MapBuilder, gid: u32) -> Option<u16> {
        b.tilesets.tileset_descs.iter().enumerate().filter_map(|(i, t)| {
            let first = t.tileset.first_gid;
            let last = first + t.rows * t.cols;
            if (first..last).contains(gid) {
                Some(i as u16)
            } else {
                None
            }
        }).nth(0)
    }
}
