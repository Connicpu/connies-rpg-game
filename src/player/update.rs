use winit;

use {DataHelper, EntityIter};
use physics as p;
use physics::ext::BodyExt;

#[derive(Default, System)]
#[system_type(Entity)]
#[process(process)]
#[aspect(all(player, body))]
pub struct PlayerUpdate;

const ACCEL_TIME: f32 = 0.2;
const DECEL_TIME: f32 = 0.2;
const TARGET_VELOCITY: f32 = 4.5;
const AIR_CONTROL_ACCEL: f32 = 0.3;
const AIR_CONTROL_DECEL: f32 = 0.2;
const JUMP_HEIGHT: f32 = 2.3;

fn process(_: &mut PlayerUpdate, players: EntityIter, data: &mut DataHelper) {
    for player in players {
        let (c, s) = (&mut data.components, &mut data.services);
        let ref jump_detector = c.player[player].ground_detector.read().unwrap();

        let dt = s.timer.delta_time;

        let mut body = s.physics.world.body_mut(c.body[player].handle);
        let body_mass = body.mass();
        let body_velocity = *body.linear_velocity();

        let (accel_modifier, decel_modifier) = if jump_detector.grounded {
            (1.0, 1.0)
        } else {
            (AIR_CONTROL_ACCEL, AIR_CONTROL_DECEL)
        };

        let (left, right, jump) = (
            s.keyboard.is_held(winit::VirtualKeyCode::A),
            s.keyboard.is_held(winit::VirtualKeyCode::D),
            s.keyboard.is_pressed(winit::VirtualKeyCode::Space),
        );

        let accel_force_one_second = body_mass * accel_modifier * TARGET_VELOCITY;
        let decel_force_one_second = body_mass * decel_modifier * TARGET_VELOCITY;

        if left {
            let mut new_force = -accel_force_one_second / ACCEL_TIME;

            if body_velocity.x + new_force * dt < -TARGET_VELOCITY {
                new_force = -(TARGET_VELOCITY + body_velocity.x) * body_mass / dt;
            }

            body.apply_horiz_accel(new_force);
        }

        if right {
            let mut new_force = accel_force_one_second / ACCEL_TIME;

            if body_velocity.x + new_force * dt > TARGET_VELOCITY {
                new_force = (TARGET_VELOCITY - body_velocity.x) * body_mass / dt;
            }

            body.apply_horiz_accel(new_force);
        }

        if jump && jump_detector.grounded {
            let impulse = (-2.0 * p::GRAVITY.y * JUMP_HEIGHT).sqrt() * body_mass;
            body.apply_vert_impulse(impulse);
        }

        if !left && !right {
            let sign = body_velocity.x.signum();
            let mut new_force = -sign * decel_force_one_second / DECEL_TIME;

            if (body_velocity.x + new_force * dt).signum() != sign {
                new_force = -body_velocity.x * body_mass / dt;
            }

            body.apply_horiz_accel(new_force);
        }
    }
}
