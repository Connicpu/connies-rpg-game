use physics as p;

use components::Sprite;

pub mod update;
pub use player::update::PlayerUpdate;

use wrapped2d::b2;

pub struct Player {
	pub phys_body: p::Body,
	pub sprite: Sprite
}

impl Player {
	pub fn new(player_sprite: Sprite, world: &mut p::World, pos: [f32; 2], size: [f32; 2], density: f32) -> Self{
		let body_desc = b2::BodyDef {
            body_type: b2::BodyType::Dynamic,
            position: b2::Vec2 {
                x: pos[0],
                y: pos[1],
            },
			
            ..b2::BodyDef::new()	
		};
		let body_handle = world.world.create_body(&body_desc);
		let mut body_mut = world.world.body_mut(body_handle);
		
		let body_box_shape = b2::PolygonShape::new_oriented_box(size[0], size[1]-size[0], &b2::Vec2{x: 0.0, y: size[1]*0.5}, 0.0);
		let bottom_circle_shape = b2::CircleShape::new_with(b2::Vec2{x: 0.0, y: size[0]*0.5}, size[0]*0.5);
		let top_circle_shape = b2::CircleShape::new_with(b2::Vec2{x: size[1]-size[0]*0.5, y: size[0]*0.5}, size[0]*0.5);
		body_mut.create_fast_fixture(&body_box_shape, density);
		body_mut.create_fast_fixture(&bottom_circle_shape, density);
		body_mut.create_fast_fixture(&top_circle_shape, density);
		body_mut.set_rotation_fixed(true);
		
		Player {
			phys_body: p::Body{handle: body_handle},
			sprite: player_sprite
		}
	}
}
