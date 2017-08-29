use wrapped2d::b2;

use std::collections::HashSet;
use std::cmp;

use tilemap::MapBuilder;
use tilemap::Tilesets;
use graphics::tileset::TilesetDesc;
use physics as p;

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
                    if let Some(tileset) = find_tileset(&b.tilesets, tile) {
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

    pub fn build_physics(
        &self,
        world: &mut p::World,
        tilesets: &Tilesets,
        pos: [f32; 2],
    ) -> b2::BodyHandle {
        let desc = b2::BodyDef {
            body_type: b2::BodyType::Static,
            position: b2::Vec2 {
                x: pos[0],
                y: pos[1],
            },

            ..b2::BodyDef::new()
        };
        let handle = world.world.create_body(&desc);

        let coords = (0i32..8).flat_map(|y| (0i32..8).map(move |x| (x, y)));
        for (&tile, (x, y)) in self.tiles.iter().zip(coords) {
            if let Some(tileset_i) = find_tileset(tilesets, tile as u32) {
                let offset = [x as f32, -y as f32];
                let ref tileset = tilesets.tileset_descs[tileset_i as usize];

                fixture_for_tile(world, handle, offset, tile, tileset);
            }
        }

        handle
    }
}

fn find_tileset(tilesets: &Tilesets, gid: u32) -> Option<u16> {
    tilesets
        .tileset_descs
        .iter()
        .enumerate()
        .filter_map(|(i, t)| {
            let first = t.tileset.first_gid;
            let last = first + t.rows * t.cols;
            if (first..last).contains(gid) {
                Some(i as u16)
            } else {
                None
            }
        })
        .nth(0)
}

fn fixture_for_tile(
    world: &mut p::World,
    body: b2::BodyHandle,
    offset: [f32; 2],
    tile: u16,
    tileset: &TilesetDesc,
) {
    let ts = 16.0; // TODO: Don't hardcode this?
    let (ox, oy) = (offset[0], offset[1]);

    if let Some(&tile_i) = tileset.tile_gids.get(&tile) {
        let ref tile = tileset.tileset.tiles[tile_i as usize];
        if let Some(ref objectgroup) = tile.objectgroup {
            for object in objectgroup.objects.iter() {
                use tiled::ObjectShape::*;
                let (x, y) = (object.x / ts, -object.y / ts);
                match object.shape {
                    Rect { width, height } => {
                        let (w, h) = (width / ts, height / ts);
                        let verts = rect_fixture(ox, oy, x, y, w, h);
                        let shape = b2::PolygonShape::new_with(&verts);

                        world.world.body_mut(body).create_fast_fixture(&shape, 1.0);
                    }
                    ref shape => unimplemented!(
                        "Unimplemented Tile ObjectShape in collision definition: {:?}",
                        shape
                    ),
                }
            }
        }
    }
}

fn rect_fixture(ox: f32, oy: f32, x: f32, y: f32, w: f32, h: f32) -> [b2::Vec2; 4] {
    [
        b2::Vec2 {
            x: ox + x,
            y: oy + y,
        },
        b2::Vec2 {
            x: ox + x + w,
            y: oy + y,
        },
        b2::Vec2 {
            x: ox + x + w,
            y: oy + y - h,
        },
        b2::Vec2 {
            x: ox + x,
            y: oy + y - h,
        },
    ]
}
