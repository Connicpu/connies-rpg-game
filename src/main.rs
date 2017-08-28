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
    pub player: Option<conniecs::Entity>,
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
    pub end_frame: graphics::EndFrame,
}

fn main() {
    util::panic_handler::init();

    let mut world = World::new();
    load_test_map(&mut world);

    while !world.data.services.quit {
        world.update();

        let player = world.data.services.player.unwrap();
        world.data.with_entity_data(player, |e, c, _| {
            println!("{:?}", c.transform[e].pos);
        });
    }
}

fn load_test_map(world: &mut conniecs::World<Systems>) {
    use conniecs::Process;

    let tmap = tiled::parse_file(std::path::Path::new("resources/maps/testmap.tmx")).unwrap();
    let map = tilemap::load_map(tmap, &mut world.data.services.graphics);

    let (hc, vc) = (map.h_chunks, map.v_chunks);
    let coords = (0..hc).flat_map(|y| (0..vc).map(move |x| (x, y)));
    for (chunk, (x, y)) in map.layers[1].chunks.chunks.iter().zip(coords) {
        let pos = [x as f32 * 8.0, y as f32 * -8.0];
        chunk.build_physics(&mut world.data.services.physics, &map.tilesets, pos);
    }

    let player_body = {
        use wrapped2d::b2;
        let p = &mut world.data.services.physics;

        let player_body = p.world.create_body(&b2::BodyDef {
            body_type: b2::BodyType::Dynamic,
            position: b2::Vec2 { x: 4.0, y: -248.0 },
            ..b2::BodyDef::new()
        });

        let player_shape = b2::CircleShape::new_with(b2::Vec2 { x: 0.0, y: 0.0 }, 0.5);
        p.world
            .body_mut(player_body)
            .create_fast_fixture(&player_shape, 1.0);

        player_body
    };

    let player = world.data.create_entity(|e, c, _| {
        let body = physics::Body {
            handle: player_body,
        };
        
        let mut transform = components::Transform::new();
        transform.pos = cgmath::Vector2 { x: 4.0, y: -248.0 };

        c.body.add(e, body);
        c.transform.add(e, transform);
    });

    world.data.services.player = Some(player);
    world.systems.physics_update.process(&mut world.data);
    world.data.services.current_map = Some(map);
}
