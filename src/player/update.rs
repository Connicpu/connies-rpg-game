use DataHelper;

extern crate winit;

use wrapped2d::b2;

#[derive(Default, System)]
#[process(process)]
pub struct PlayerUpdate;

const PLAYER_MOVE_FORCE: f32 = 10.0;
const PLAYER_JUMP_IMPULSE: f32 = 40.0;

fn process(_: &mut PlayerUpdate, data: &mut DataHelper) {
    match data.services.player {
        Some(player_entity) => {
            data.with_entity_data(player_entity, |e, c, s| {

                let jump_detector = match s.player_ground_detector {
                    Some(ref jump_detector_arc) => jump_detector_arc.read().unwrap(),
                    None => return,
                };

                let mut player_body = s.physics.world.body_mut(c.body[e].handle);
                // LEFT
                if s.keyboard.is_held(winit::VirtualKeyCode::A) {
                    player_body.apply_force_to_center(
                        &b2::Vec2 {
                            x: -PLAYER_MOVE_FORCE,
                            y: 0.0,
                        },
                        true,
                    );
                }
                // RIGHT
                if s.keyboard.is_held(winit::VirtualKeyCode::D) {
                    player_body.apply_force_to_center(
                        &b2::Vec2 {
                            x: PLAYER_MOVE_FORCE,
                            y: 0.0,
                        },
                        true,
                    );
                }
                //JUMP
                if s.keyboard.is_pressed(winit::VirtualKeyCode::Space) && jump_detector.grounded {
                    let world_center = *player_body.world_center();
                    player_body.apply_linear_impulse(
                        &b2::Vec2 {
                            x: 0.0,
                            y: PLAYER_JUMP_IMPULSE,
                        },
                        &world_center,
                        true,
                    )
                }
            });
        }
        None => (),
    }

}
