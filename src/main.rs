#![feature(inclusive_range_syntax, range_contains)]
#![feature(get_type_id)]
#![feature(conservative_impl_trait)]
#![feature(plugin)]
#![plugin(clippy)]
#![warn(items_after_statements, nonminimal_bool)]
#![warn(option_map_unwrap_or, option_map_unwrap_or_else)]
#![warn(single_match_else, used_underscore_binding)]
#![warn(pub_enum_variant_names, unicode_not_nfc)]
#![warn(print_stdout)] // Please use log macros instead

extern crate backtrace;
extern crate cgmath;
extern crate fnv;
extern crate image;
extern crate index_pool;
extern crate msgbox;
extern crate tiled;
extern crate time;
extern crate windows_dpi;
extern crate wrapped2d;

#[macro_use]
extern crate log;

#[macro_use]
extern crate glium;
extern crate glutin;
extern crate winit;

extern crate conniecs;
#[macro_use]
extern crate conniecs_derive;

#[macro_use]
extern crate serde_derive;
//extern crate serde;

extern crate toml;

#[macro_use]
extern crate lazy_static;

use conniecs::EntitySystem;
use conniecs::Entity;

pub use config::CONFIG;

pub mod components;
pub mod config;
pub mod graphics;
pub mod input;
pub mod math;
pub mod physics;
pub mod player;
pub mod systems;
pub mod tilemap;
pub mod timer;
pub mod util;

use std::sync::RwLock;
use std::sync::Arc;

pub type World = conniecs::World<Systems>;
pub type DataHelper = conniecs::DataHelper<Components, Services>;
pub type ComponentList<T> = conniecs::ComponentList<Components, T>;
pub type EntityIter<'a> = conniecs::EntityIter<'a, Components>;
pub type EntityData<'a> = conniecs::EntityData<'a, Components>;

#[derive(Default, ServiceManager)]
pub struct Services {
    pub quit: bool,
    pub resized: bool,

    pub timer: timer::Timer,
    pub physics: physics::World,
    pub graphics: graphics::System,

    pub default_texture: Option<graphics::textures::TextureId>,

    pub camera: math::Camera,
    pub keyboard: input::KeyboardState,

    pub current_map: Option<tilemap::Map>,
    pub player: Entity,
}

#[derive(ComponentManager)]
pub struct Components {
    #[hot] pub transform: ComponentList<components::Transform>,
    #[hot] pub sprite: ComponentList<components::Sprite>,
    #[hot] pub body: ComponentList<physics::Body>,

    #[cold] pub player: ComponentList<player::Player>,
}

#[derive(SystemManager)]
pub struct Systems {
    pub update_time: timer::UpdateTime,
    pub update_input: input::UpdateInput,

    pub player_update: EntitySystem<player::PlayerUpdate>,
    pub camera_follow: EntitySystem<systems::CameraFollow>,

    pub physics_run: physics::PhysicsRun,
    pub physics_update: EntitySystem<physics::PhysicsUpdate>,

    pub sprite_watcher: EntitySystem<graphics::SpriteWatcher>,
    pub lock_camera: input::LockCamera,

    pub begin_frame: graphics::BeginFrame,
    pub tile_draw: graphics::TileDraw,
    pub draw_sprites: graphics::DrawSprites,
    pub draw_physics: graphics::box2d::DrawPhysics,
    pub end_frame: graphics::EndFrame,
}

fn main() {
    util::panic_handler::init();

    let mut world = World::new();
    load_test_zone(&mut world);

    while !world.data.services.quit {
        world.update();

        if world
            .data
            .services
            .keyboard
            .is_pressed(winit::VirtualKeyCode::Escape)
        {
            world.wipe();
            world.data.services.physics = Default::default();
            world.data.services.current_map = None;

            load_test_zone(&mut world);
        }
    }
}

fn load_test_zone(world: &mut World) {
    let ground_entity = world.data.create_entity(|_, _, _| {});
    let player_ground_sensor_entity = world.data.create_entity(|_, _, _| {});

    load_test_map(world, ground_entity);

    for i in 0..3 {
        create_scion(world, 3.0 + i as f32, -248.0, Scion::Aymeric);
        create_scion(world, 3.0 + i as f32, -248.5, Scion::Papalymo);
        create_scion(world, 3.0 + i as f32, -249.0, Scion::Yda);
    }

    world.data.services.default_texture = Some(
        world
            .data
            .services
            .graphics
            .load_texture("textures/default.png"),
    );

    let player = create_player(world, player_ground_sensor_entity);
    world.data.services.player = player;
}

fn create_player(
    world: &mut conniecs::World<Systems>,
    player_ground_sensor_entity: Entity,
) -> Entity {
    world.data.create_entity(|e, c, s| {
        let texture = s.graphics.load_texture("textures/player.png");
        //let anim = s.graphics.load_animation("animations/player.toml");
        let mut player_sprite = components::Sprite::new(texture);
        player_sprite.center = [0.5, 1.0];
        player_sprite.uv_rect = [0.0, 0.0, 1.0 / 3.0, 1.0];
        let player_body = player::Player::create_physics(
            &mut s.physics,
            [9.0, -247.0],
            [0.5, 1.25],
            5.0,
            0.0,
            0.05,
            player_ground_sensor_entity,
        );
        let mut player_transform = components::Transform::new();
        player_transform.size = cgmath::Vector2::<f32> { x: 0.75, y: 1.25 };

        c.sprite.add(e, player_sprite);
        c.transform.add(e, player_transform);
        c.body.add(e, player_body);

        let player_ground_detector = player::PlayerGroundDetector::new(player_ground_sensor_entity);
        let player_ground_detector_arc = Arc::new(RwLock::new(player_ground_detector));
        let player_ground_detector_callbacks =
            player::PlayerGroundDetectorCallbacks::new(&player_ground_detector_arc);
        s.physics
            .world
            .set_contact_listener(Box::new(player_ground_detector_callbacks));

        let player = player::Player {
            ground_detector: player_ground_detector_arc,
        };
        c.player.add(e, player);
    })
}

fn load_test_map(world: &mut World, ground_entity: Entity) {
    let tmap = tiled::parse_file(std::path::Path::new("resources/maps/testmap.tmx")).unwrap();
    let map = tilemap::load_map(tmap, &mut world.data.services.graphics);

    map.create_physics(1, &mut world.data.services.physics, ground_entity);

    world.data.services.current_map = Some(map);
}

#[derive(Copy, Clone)]
enum Scion {
    Aymeric,
    Papalymo,
    Yda,
}

fn create_scion(world: &mut World, x: f32, y: f32, scion: Scion) {
    world.data.create_entity(|e, c, s| {
        use wrapped2d::b2;

        let texture = match scion {
            Scion::Aymeric => "textures/aymeric.png",
            Scion::Papalymo => "textures/papalymo.png",
            Scion::Yda => "textures/yda.png",
        };

        let sprite = s.graphics.load_texture(texture);
        let sprite = components::Sprite::new(sprite);
        let mut transform = components::Transform::new();
        transform.pos.x = x;
        transform.pos.y = y;

        c.transform.add(e, transform);
        c.sprite.add(e, sprite);

        let def = b2::BodyDef {
            body_type: b2::BodyType::Dynamic,
            position: b2::Vec2 { x, y },
            angular_velocity: -10.0,
            ..b2::BodyDef::new()
        };
        let body = s.physics.world.create_body(&def);

        let shape = b2::CircleShape::new_with(b2::Vec2 { x: 0.0, y: 0.0 }, 0.48);
        s.physics
            .world
            .body_mut(body)
            .create_fast_fixture(&shape, 1.0);

        let body = physics::Body { handle: body };
        c.body.add(e, body);
    });
}
