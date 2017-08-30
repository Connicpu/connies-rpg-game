use wrapped2d::b2;

extern crate winit;

use {DataHelper, EntityIter};

#[derive(Default, System)]
#[system_type(Entity)]
#[process(process)]
#[aspect(all(player, body))]
pub struct PlayerUpdate;

const MOVE_FORCE: f32 = 30.0;
const JUMP_IMPULSE: f32 = 7.0;
const AIR_CONTROL: f32 = 0.6;
const MAX_X_VEL: f32 = 4.0;
const SLOWDOWN: f32 = 5.0;

fn process(_: &mut PlayerUpdate, players: EntityIter, data: &mut DataHelper) {
    for player in players {
        let (c, s) = (&mut data.components, &mut data.services);
        let ref jump_detector = c.player[player].ground_detector.read().unwrap();

        let move_modifier = if jump_detector.grounded {
            1.0
        } else {
            AIR_CONTROL
        };

        let mut body = s.physics.world.body_mut(c.body[player].handle);

        let (left, right, jump) = (
            s.keyboard.is_held(winit::VirtualKeyCode::A),
            s.keyboard.is_held(winit::VirtualKeyCode::D),
            s.keyboard.is_pressed(winit::VirtualKeyCode::Space),
        );

        if left {
            body.apply_force_to_center(
                &b2::Vec2 {
                    x: -MOVE_FORCE * move_modifier,
                    y: 0.0,
                },
                true,
            );
        }

        if right {
            body.apply_force_to_center(
                &b2::Vec2 {
                    x: MOVE_FORCE * move_modifier,
                    y: 0.0,
                },
                true,
            );
        }

        if jump && jump_detector.grounded {
            let world_center = *body.world_center();
            body.apply_linear_impulse(
                &b2::Vec2 {
                    x: 0.0,
                    y: JUMP_IMPULSE,
                },
                &world_center,
                true,
            )
        }

        // Clamp x velocity
        let mut velocity = *body.linear_velocity();
        velocity.x = velocity.x.min(MAX_X_VEL).max(-MAX_X_VEL);

        let dt = s.timer.delta_time;
        if !left && !right {
            let sign = velocity.x.signum();
            velocity.x = (velocity.x.abs() - SLOWDOWN * dt * move_modifier).max(0.0) * sign;
        }

        body.set_linear_velocity(&velocity);
    }
}
