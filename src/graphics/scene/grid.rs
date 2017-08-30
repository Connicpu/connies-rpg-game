use conniecs::{Entity, EntityIter, IndexedEntity};
use cgmath::Vector2;
use fnv::FnvHashMap;

use std::collections::HashMap;

use EntityData;
use math::Aabb;

pub struct UniformGrid {
    grid_squares: FnvHashMap<(i32, i32), GridSquare>,
    entity_locations: FnvHashMap<Entity, (i32, i32)>,
    grid_size: f32,
    margin: i32,
}

impl UniformGrid {
    pub fn new(grid_size: f32, margin: i32) -> Self {
        UniformGrid {
            grid_squares: HashMap::with_hasher(Default::default()),
            entity_locations: HashMap::with_hasher(Default::default()),
            grid_size,
            margin,
        }
    }

    pub fn set(&mut self, entity: EntityData, location: Vector2<f32>) {
        let x = (location.x / self.grid_size).floor() as i32;
        let y = (location.y / self.grid_size).floor() as i32;
        let new_pos = (x, y);

        if let Some(pos) = self.entity_locations.get(&entity).cloned() {
            if pos == new_pos {
                return;
            }

            self.entity_locations.remove(&entity);

            let mut remove_square = false;
            if let Some(square) = self.grid_squares.get_mut(&pos) {
                square.sprites.remove(&entity);
                if square.sprites.is_empty() {
                    remove_square = true;
                }
            }

            if remove_square {
                self.grid_squares.remove(&pos);
            }
        }

        self.entity_locations.insert(**entity, (x, y));

        self.grid_squares
            .entry(new_pos)
            .or_insert_with(|| Default::default())
            .sprites
            .insert(**entity, entity.__clone());
    }

    pub fn remove(&mut self, entity: EntityData) {
        if let Some(pos) = self.entity_locations.get(&entity).cloned() {
            self.entity_locations.remove(&entity);

            let mut remove_square = false;
            if let Some(square) = self.grid_squares.get_mut(&pos) {
                square.sprites.remove(&entity);
                if square.sprites.is_empty() {
                    remove_square = true;
                }
            }

            if remove_square {
                self.grid_squares.remove(&pos);
            }
        }
    }

    pub fn entities<'a>(&'a self, region: Aabb) -> impl Iterator<Item = EntityData<'a>> + 'a {
        region
            .scaled(self.grid_scale())
            .to_int()
            .expanded_by(self.margin, self.margin)
            .into_iter()
            .filter_map(move |pos| self.grid_squares.get(&pos))
            .flat_map(|square| EntityIter::Map(square.sprites.values()))
    }

    fn grid_scale(&self) -> Vector2<f32> {
        Vector2 {
            x: 1.0 / self.grid_size,
            y: 1.0 / self.grid_size,
        }
    }
}

#[derive(Default)]
struct GridSquare {
    sprites: HashMap<Entity, IndexedEntity<::Components>>,
}
