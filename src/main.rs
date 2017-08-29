#![feature(inclusive_range_syntax, range_contains)]
#![feature(get_type_id)]

extern crate tiled;
extern crate wrapped2d;
extern crate time;
extern crate cgmath;
extern crate windows_dpi;
extern crate image;
extern crate msgbox;
extern crate backtrace;

#[macro_use]
extern crate glium;
extern crate glutin;
extern crate winit;

#[macro_use]
extern crate conniecs_derive;
extern crate conniecs;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate toml;

#[macro_use]
extern crate lazy_static;

use conniecs::EntitySystem;

pub use config::CONFIG;

pub mod components;
pub mod config;
pub mod graphics;
pub mod input;
pub mod math;
pub mod physics;
pub mod tilemap;
pub mod timer;
pub mod util;

pub type World = conniecs::World<Systems>;
pub type DataHelper = conniecs::DataHelper<Components, Services>;
pub type ComponentList<T> = conniecs::ComponentList<Components, T>;
pub type EntityIter<'a> = conniecs::EntityIter<'a, Components>;

#[derive(Default, ServiceManager)]
pub struct Services {
    pub quit: bool,
    pub resized: bool,

    pub timer: timer::Timer,
    pub physics: physics::World,
    pub graphics: graphics::System,

    pub camera: math::Camera,
    pub keyboard: input::KeyboardState,

    pub current_map: Option<tilemap::Map>,
}

#[derive(ComponentManager)]
pub struct Components {
    #[hot]
    pub transform: ComponentList<components::Transform>,
    #[hot]
    pub sprite: ComponentList<components::Sprite>,
    #[hot]
    pub body: ComponentList<physics::Body>,
}

#[derive(SystemManager)]
pub struct Systems {
    pub update_time: timer::UpdateTime,
    pub update_input: input::UpdateInput,

    pub physics_run: physics::PhysicsRun,
    pub physics_update: EntitySystem<physics::PhysicsUpdate>,

    pub begin_frame: graphics::BeginFrame,
    pub temp_draw: graphics::TempDraw,
    pub draw_sprites: EntitySystem<graphics::DrawSprites>,
    pub end_frame: graphics::EndFrame,
}

fn main() {
    util::panic_handler::init();

    let mut world = World::new();
    load_test_map(&mut world);

    while !world.data.services.quit {
        world.update();
    }
}

fn load_test_map(world: &mut conniecs::World<Systems>) {
    let tmap = tiled::parse_file(std::path::Path::new("resources/maps/testmap.tmx")).unwrap();
    let map = tilemap::load_map(tmap, &mut world.data.services.graphics);

    map.create_physics(1, &mut world.data.services.physics);

    world.data.services.current_map = Some(map);

    let _aymeric = world.data.create_entity(|e, c, s| {
        use wrapped2d::b2;

        let sprite = s.graphics.load_texture("textures/aymeric.png");
        let sprite = components::Sprite::new(sprite);
        let mut transform = components::Transform::new();
        transform.pos.x = 4.0;
        transform.pos.y = -248.0;

        c.transform.add(e, transform);
        c.sprite.add(e, sprite);

        let def = b2::BodyDef {
            body_type: b2::BodyType::Dynamic,
            position: b2::Vec2 { x: 4.0, y: -248.0 },
            angular_velocity: -10.0,
            ..b2::BodyDef::new()
        };
        let body = s.physics.world.create_body(&def);

        let shape = b2::CircleShape::new_with(b2::Vec2 { x: 0.0, y: 0.0 }, 0.5);
        s.physics
            .world
            .body_mut(body)
            .create_fast_fixture(&shape, 1.0);

        let body = physics::Body { handle: body };
        c.body.add(e, body);
    });
}
