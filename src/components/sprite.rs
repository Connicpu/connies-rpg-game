use specs::VecStorage;
use graphics::textures::TextureId;

#[derive(Component)]
#[component(VecStorage)]
#[derive(Copy, Clone)]
pub struct Sprite {
    pub sprite: TextureId,
    pub uv_rect: [f32; 4],
}
