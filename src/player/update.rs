use DataHelper;

extern crate winit;

use wrapped2d::b2;

#[derive(Default, System)]
#[process(process)]
pub struct PlayerUpdate;

const PLAYER_MOVE_FORCE:f32 = 1.0;
const PLAYER_JUMP_IMPULSE:f32 = 4.0;

fn process (_: &mut PlayerUpdate, data: &mut DataHelper){
	
	match data.services.player
	{
    	Some (ref player) => {
			let dt = data.services.timer.delta_time;
			let mut player_body = data.services.physics.world.body_mut(player.phys_body.handle);
			//fn apply_force_to_center(&mut self, force: &Vec2, wake: bool)
			// LEFT
			if data.services.keyboard.is_held(winit::VirtualKeyCode::A){
				println! ("left");
				player_body.apply_force_to_center(&b2::Vec2{x: - PLAYER_MOVE_FORCE * dt, y: 0.0}, true);
			}
			// RIGHT
			if data.services.keyboard.is_held(winit::VirtualKeyCode::D){
				println! ("right");
				player_body.apply_force_to_center(&b2::Vec2{x: PLAYER_MOVE_FORCE * dt, y: 0.0}, true);
			}
			//fn apply_linear_impulse(&mut self, impulse: &Vec2, point: &Vec2, wake: bool)
			// JUMP
			if data.services.keyboard.is_pressed(winit::VirtualKeyCode::Space){
				println! ("jump");
				let world_center = *player_body.world_center ();
				player_body.apply_linear_impulse(&b2::Vec2{x: 0.0, y: PLAYER_JUMP_IMPULSE}, & world_center, true)
			}
			
		},
		None => (),
	}
	
}