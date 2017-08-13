use util::IdPool;

use std::collections::HashMap;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TextureId(usize);

pub struct TextureManager {
    id_pool: IdPool,
    textures: HashMap<TextureId, Texture>,
}

impl TextureManager {
    pub fn insert(&mut self, texture: Texture) -> TextureId {
        let id = TextureId(self.id_pool.new_id());
        self.textures.insert(id, texture);
        id
    }

    pub fn update(&mut self, id: TextureId, texture: Texture) {
        self.textures.insert(id, texture);
    }

    pub fn remove(&mut self, id: TextureId) -> Option<Texture> {
        self.id_pool.return_id(id.0);
        self.textures.remove(&id)
    }
}

pub struct Texture {
    // TODO
}
