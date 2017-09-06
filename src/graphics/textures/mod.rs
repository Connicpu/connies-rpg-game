use glium::Display;
use glium::texture::{PixelValue, RawImage2d, SrgbTexture2d, ToClientFormat};
use image;
use index_pool::IndexPool;

use std::borrow::Cow;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TextureId(usize);

#[derive(Default)]
pub struct TextureManager {
    id_pool: IndexPool,
    textures: HashMap<TextureId, Texture>,
    name_to_id: HashMap<String, TextureId>,
    id_to_name: HashMap<TextureId, String>,
}

impl TextureManager {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn load(&mut self, display: &Display, asset: &str) -> TextureId {
        if let Some(id) = self.name_to_id(asset) {
            return id;
        }

        self.insert(asset, Texture::load_from_asset(display, asset))
    }

    pub fn insert(&mut self, name: &str, texture: Texture) -> TextureId {
        let id = TextureId(self.id_pool.new_id());
        self.textures.insert(id, texture);
        self.name_to_id.insert(name.into(), id);
        self.id_to_name.insert(id, name.into());
        id
    }

    pub fn update(&mut self, id: TextureId, texture: Texture) {
        self.textures.insert(id, texture);
    }

    pub fn remove(&mut self, id: TextureId) -> Option<Texture> {
        if self.id_pool.return_id(id.0).is_err() {
            eprintln!("[WARNING] texture was already removed (or never existed?)");
        }

        if let Some(name) = self.id_to_name.get(&id) {
            self.name_to_id.remove(name);
        }
        self.id_to_name.remove(&id);
        self.textures.remove(&id)
    }

    pub fn name_to_id(&self, name: &str) -> Option<TextureId> {
        self.name_to_id.get(name).cloned()
    }

    pub fn id_to_name(&self, id: TextureId) -> Option<&str> {
        self.id_to_name.get(&id).map(|s| &s[..])
    }

    pub fn get(&self, id: TextureId) -> &Texture {
        &self.textures[&id]
    }
}

pub struct Texture {
    pub tex: SrgbTexture2d,
}

impl Texture {
    pub fn from_rgba<'a, C, T>(display: &Display, data: C, dimensions: (u32, u32)) -> Texture
    where
        C: Into<Cow<'a, [T]>>,
        T: ToClientFormat + PixelValue,
    {
        let data = data.into();

        let raw_image = RawImage2d::from_raw_rgba(data.into_owned(), dimensions);
        let texture = SrgbTexture2d::new(display, raw_image).unwrap();

        Texture { tex: texture }
    }

    pub fn load_from_asset(display: &Display, asset: &str) -> Texture {
        let mut path = PathBuf::from("./resources");
        path.push(asset);

        let image = image::open(path)
            .map_err(|e| panic!("Failed to open image {:?}\n    {}", asset, e))
            .unwrap()
            .to_rgba();

        let dimensions = image.dimensions();
        Texture::from_rgba(display, image.into_raw(), dimensions)
    }
}
