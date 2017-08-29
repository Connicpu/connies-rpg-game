use graphics::textures::TextureId;

#[derive(Copy, Clone)]
pub struct Sprite {
    pub sprite: TextureId,
    pub center: [f32; 2],
    pub uv_rect: [f32; 4],
    pub layer: f32,
}

impl Sprite {
    pub fn new(sprite: TextureId) -> Self {
        Sprite {
            sprite,
            center: [0.5, 0.5],
            uv_rect: [0.0, 0.0, 1.0, 1.0],
            layer: 2.0,
        }
    }
}
