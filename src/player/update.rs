use fmod_studio;
use winit;

use {DataHelper, EntityIter};
use physics as p;
use physics::ext::BodyExt;

use timer;

#[derive(Default, System)]
#[system_type(Entity)]
#[process(process)]
#[aspect(all(player, body))]
pub struct PlayerUpdate {
    pub last_jump_press_ns: u64,
    pub jump_press_serial: u64,
    pub last_used_jump_press_serial: u64,
    pub jump_sound: fmod_studio::Guid,
}

const ACCEL_TIME: f32 = 0.2;
const DECEL_TIME: f32 = 0.2;
const TARGET_VELOCITY: f32 = 4.5;
const AIR_CONTROL_ACCEL: f32 = 0.3;
const AIR_CONTROL_DECEL: f32 = 0.2;
const JUMP_HEIGHT: f32 = 2.3;
const LATE_JUMP_TOLERANCE_MS: u64 = 100;
const EARLY_JUMP_TOLERANCE_MS: u64 = 100;

fn process(player_update: &mut PlayerUpdate, players: EntityIter, data: &mut DataHelper) {
    if player_update.jump_sound == Default::default() {
        player_update.jump_sound = data.services.audio.get_id("event:/jump").unwrap();
    }

    for player in players {
        let (c, s) = (&mut data.components, &mut data.services);
        let jump_detector = &c.player[player].ground_detector.read().unwrap();

        let dt = s.timer.delta_time;

        let mut body = s.physics.world.body_mut(c.body[player].handle);
        let body_mass = body.mass();
        let body_velocity = *body.linear_velocity();

        let (accel_modifier, decel_modifier) = if jump_detector.grounded {
            (1.0, 1.0)
        } else {
            (AIR_CONTROL_ACCEL, AIR_CONTROL_DECEL)
        };

        let (left, right, jump_key) = (
            s.keyboard.is_held(winit::VirtualKeyCode::A),
            s.keyboard.is_held(winit::VirtualKeyCode::D),
            s.keyboard.is_pressed(winit::VirtualKeyCode::Space),
        );

        let current_time: u64 = timer::UPDATE_TIME.with(|update_time| update_time.get().time);

        if jump_key {
            player_update.jump_press_serial += 1;
            player_update.last_jump_press_ns = current_time;
        }

        let (late_jump, early_jump) = {
            let last_ground_delay: u64 = current_time - jump_detector.last_unground_time_ns;
            let last_jump_press: u64 = current_time - player_update.last_jump_press_ns;

            let late_jump = jump_key && (last_ground_delay < LATE_JUMP_TOLERANCE_MS * 1_000_000);
            let early_jump =
                jump_detector.grounded && (last_jump_press < EARLY_JUMP_TOLERANCE_MS * 1_000_000);

            (late_jump, early_jump)
        };

        let jump = (jump_key && jump_detector.grounded) || late_jump || early_jump;

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

        if jump && (player_update.last_used_jump_press_serial != player_update.jump_press_serial) {
            player_update.last_used_jump_press_serial = player_update.jump_press_serial;
            let impulse = (-2.0 * p::GRAVITY.y * JUMP_HEIGHT).sqrt() * body_mass;
            body.apply_vert_impulse(impulse);

            s.audio.play_oneoff(&player_update.jump_sound);
        }

        if !left && !right {
            let sign = body_velocity.x.signum();
            let vel_positive = sign.is_sign_positive();
            let mut new_force = -sign * decel_force_one_second / DECEL_TIME;

            if (body_velocity.x + new_force * dt).is_sign_positive() != vel_positive {
                new_force = -body_velocity.x * body_mass / dt;
            }

            body.apply_horiz_accel(new_force);
        }

        if left && !body_velocity.x.is_sign_positive() {
            c.sprite[player].flip_x = true;
        } else if right && body_velocity.x.is_sign_positive() {
            c.sprite[player].flip_x = false;
        }
    }
}
