use graphics::textures::TextureId;

#[derive(Copy, Clone)]
pub struct Sprite {
    pub sprite: TextureId,
    pub uv_rect: [f32; 4],
}
