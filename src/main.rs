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
extern crate ecs;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate toml;

#[macro_use]
extern crate lazy_static;

use cgmath::Vector2;
use ecs::system::EntitySystem;

use std::path::Path;
use std::fs::File;
use std::io::Read;

use math::ToRawMath;

pub mod components;
pub mod config;
pub mod graphics;
pub mod input;
pub mod math;
pub mod physics;
pub mod timer;
pub mod util;

impl ecs::ServiceManager for Services {}
#[derive(Default)]
pub struct Services {
    pub quit: bool,
    pub resized: bool,

    pub timer: timer::Timer,
    pub physics: physics::World,
    pub graphics: graphics::System,

    pub keyboard: input::KeyboardState,
}

pub type DataHelper = ecs::DataHelper<Components, Services>;

lazy_static! {
    pub static ref CONFIG: config::Config = {
        let mut config_data = vec![];
        File::open("resources/config/config.toml").unwrap().read_to_end(&mut config_data).unwrap();
        toml::from_slice(&config_data).unwrap()
    };
}

fn main() {
    util::panic_handler::init();

    let mut world = ecs::World::<Systems>::with_services(Services::default());

    let mut cam_pos = Vector2 { x: 8.5, y: -4.0 };
    let cam_size = 8.0;

    let tilemap = tiled::parse_file(Path::new("resources/maps/testmap.tmx")).unwrap();
    let tileset = tilemap.tilesets[0].clone();
    let tileset = graphics::tileset::TilesetDesc::load(&mut world.data.services.graphics, tileset);

    loop {
        process!(world, update_time);
        process!(world, update_input);

        let dt = world.data.services.timer.delta_time;
        if world.data.services.keyboard.is_held(winit::VirtualKeyCode::W) {
            cam_pos.y += dt * 4.0;
        }
        if world.data.services.keyboard.is_held(winit::VirtualKeyCode::A) {
            cam_pos.x -= dt * 4.0;
        }
        if world.data.services.keyboard.is_held(winit::VirtualKeyCode::S) {
            cam_pos.y -= dt * 4.0;
        }
        if world.data.services.keyboard.is_held(winit::VirtualKeyCode::D) {
            cam_pos.x += dt * 4.0;
        }

        if world.data.services.quit {
            break;
        }

        world.update();

        {
            use graphics::{Camera};
            use glium::Surface;

            let mut frame = world.data.services.graphics.display.draw();
            frame.clear_color_srgb(100.0/255.0, 149.0/255.0, 237.0/255.0, 1.0);
            frame.clear_depth(0.0);

            let dimensions = frame.get_dimensions();
            let aspect = dimensions.0 as f32 / dimensions.1 as f32;

            let view = math::matrices::view(cam_pos, cam_size, 0.0);
            let ortho = math::matrices::ortho(aspect, 100.0, -100.0);

            let camera = Camera {
                view: view.to_raw(),
                proj: ortho.to_raw(),
            };

            let tiles = [
                 1,  2,  2,  2,  2,  2,  2,  3,
                33, 34, 34, 34, 34, 34, 34, 35,
                33, 34, 34, 34, 34, 34, 34, 35,
                33, 34, 34, 34, 34, 34, 34, 35,
                33, 34, 34, 34, 34, 34, 34, 35,
                33, 34, 34, 34, 34, 34, 34, 35,
                33, 34, 34, 34, 34, 34, 34, 35,
                49, 50, 50, 50, 50, 50, 50, 51,
            ];

            world.data.services.graphics.draw_tiles(&mut frame, &camera, [0.0, 0.0, 0.5], &tiles, &tileset);
            world.data.services.graphics.draw_tiles(&mut frame, &camera, [9.0, 0.0, 0.5], &tiles, &tileset);
            world.data.services.graphics.fxaa(&mut frame);

            frame.finish().unwrap();

            process!(world, end_frame);
        }
    }
}

components! {
    #[builder(EntityBuilder)]
    struct Components {
        #[hot] transform: components::Transform,
        #[hot] sprite: components::Sprite,
        #[hot] body: physics::Body,
    }
}

systems! {
    struct Systems<Components, Services> {
        active: {
            physics_run: physics::PhysicsRun = physics::PhysicsRun,
            physics_update: EntitySystem<physics::PhysicsUpdate> =
                EntitySystem::new(physics::PhysicsUpdate, aspect!(<Components> all: [transform, body])),
        },
        passive: {
            update_time: timer::UpdateTime = timer::UpdateTime,
            update_input: input::UpdateInput = input::UpdateInput,
            end_frame: graphics::EndFrame = graphics::EndFrame,
        }
    }
}
