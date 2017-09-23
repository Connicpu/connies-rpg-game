use fmod_studio;
use wrapped2d::b2;
use winit;

use {DataHelper, EntityIter, Services};
use components::Sprite;
use input::KeyboardState;
use player::{Player, PlayerGroundDetector};
use physics::{Body, EntityUserData};
use physics::ext::BodyExt;
use physics as p;

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
        player_update.jump_sound = data.services.audio.get_id("event:/player/jump").unwrap();
    }

    for e in players {
        let player = &data.components.player[e];
        let body = &data.components.body[e];
        let sprite = &mut data.components.sprite[e];

        update_player(player_update, player, body, sprite, &mut data.services);
    }
}

fn update_player(
    player_update: &mut PlayerUpdate,
    player: &Player,
    body: &Body,
    sprite: &mut Sprite,
    services: &mut Services,
) {
    let jump_detector = player.ground_detector.read().unwrap();
    let dt = services.timer.delta_time;

    let mut body = services.physics.world.body_mut(body.handle);
    let body_mass = body.mass();
    let body_velocity = *body.linear_velocity();

    let (left, right, jump_key) = test_keys(&services.keyboard);
    let (accel_modifier, decel_modifier) = calc_modifiers(&jump_detector);
    let current_time = services.timer.time;
    check_jump_serial(player_update, current_time, jump_key);

    let (late_jump, early_jump) =
        test_offtime_jump(player_update, &jump_detector, current_time, jump_key);

    let jump = (jump_key && jump_detector.grounded) || late_jump || early_jump;

    let accel = body_mass * accel_modifier * TARGET_VELOCITY;
    let decel = body_mass * decel_modifier * TARGET_VELOCITY;

    if left {
        move_left(&mut body, dt, body_mass, accel, body_velocity);
    }

    if right {
        move_right(&mut body, dt, body_mass, accel, body_velocity);
    }

    if jump && try_jump(player_update, &mut body, body_mass) {
        services.audio.play_oneoff(&player_update.jump_sound);
    }

    if !left && !right {
        decelerate(&mut body, dt, body_mass, decel, body_velocity);
    }

    if left && !body_velocity.x.is_sign_positive() {
        sprite.flip_x = true;
    } else if right && body_velocity.x.is_sign_positive() {
        sprite.flip_x = false;
    }
}

fn test_keys(keyboard: &KeyboardState) -> (bool, bool, bool) {
    (
        keyboard.is_held(winit::VirtualKeyCode::A),
        keyboard.is_held(winit::VirtualKeyCode::D),
        keyboard.is_pressed(winit::VirtualKeyCode::Space),
    )
}

fn calc_modifiers(jump_detector: &PlayerGroundDetector) -> (f32, f32) {
    if jump_detector.grounded {
        (1.0, 1.0)
    } else {
        (AIR_CONTROL_ACCEL, AIR_CONTROL_DECEL)
    }
}

fn check_jump_serial(player_update: &mut PlayerUpdate, current_time: u64, jump_key: bool) {
    if jump_key {
        player_update.jump_press_serial += 1;
        player_update.last_jump_press_ns = current_time;
    }
}

fn test_offtime_jump(
    player_update: &PlayerUpdate,
    jump_detector: &PlayerGroundDetector,
    current_time: u64,
    jump_key: bool,
) -> (bool, bool) {
    let last_ground_delay: u64 = current_time - jump_detector.last_unground_time_ns;
    let last_jump_press: u64 = current_time - player_update.last_jump_press_ns;

    let late_jump = jump_key && (last_ground_delay < LATE_JUMP_TOLERANCE_MS * 1_000_000);
    let early_jump =
        jump_detector.grounded && (last_jump_press < EARLY_JUMP_TOLERANCE_MS * 1_000_000);

    (late_jump, early_jump)
}

fn move_left(
    body: &mut b2::MetaBody<EntityUserData>,
    dt: f32,
    body_mass: f32,
    accel: f32,
    body_velocity: b2::Vec2,
) {
    let mut new_force = -accel / ACCEL_TIME;

    if body_velocity.x + new_force * dt < -TARGET_VELOCITY {
        new_force = -(TARGET_VELOCITY + body_velocity.x) * body_mass / dt;
    }

    body.apply_horiz_accel(new_force);
}

fn move_right(
    body: &mut b2::MetaBody<EntityUserData>,
    dt: f32,
    body_mass: f32,
    accel: f32,
    body_velocity: b2::Vec2,
) {
    let mut new_force = accel / ACCEL_TIME;

    if body_velocity.x + new_force * dt > TARGET_VELOCITY {
        new_force = (TARGET_VELOCITY - body_velocity.x) * body_mass / dt;
    }

    body.apply_horiz_accel(new_force);
}

fn try_jump(
    player_update: &mut PlayerUpdate,
    body: &mut b2::MetaBody<EntityUserData>,
    body_mass: f32,
) -> bool {
    if player_update.last_used_jump_press_serial != player_update.jump_press_serial {
        player_update.last_used_jump_press_serial = player_update.jump_press_serial;

        let impulse = (-2.0 * p::GRAVITY.y * JUMP_HEIGHT).sqrt() * body_mass;
        body.apply_vert_impulse(impulse);

        true
    } else {
        false
    }
}

fn decelerate(
    body: &mut b2::MetaBody<EntityUserData>,
    dt: f32,
    body_mass: f32,
    decel: f32,
    body_velocity: b2::Vec2,
) {
    let sign = body_velocity.x.signum();
    let vel_positive = sign.is_sign_positive();
    let mut new_force = -sign * decel / DECEL_TIME;

    if (body_velocity.x + new_force * dt).is_sign_positive() != vel_positive {
        new_force = -body_velocity.x * body_mass / dt;
    }

    body.apply_horiz_accel(new_force);
}
