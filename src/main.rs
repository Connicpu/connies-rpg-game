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

use ecs::system::EntitySystem;

use std::fs::File;
use std::io::Read;

#[macro_use]
mod macros;

pub mod components;
pub mod config;
pub mod graphics;
pub mod input;
pub mod math;
pub mod physics;
pub mod tilemap;
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

    pub camera: math::Camera,
    pub keyboard: input::KeyboardState,

    pub current_map: Option<tilemap::Map>,
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
    load_test_map(&mut world);

    while !world.data.services.quit {
        world.update();
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
            update_time: timer::UpdateTime = timer::UpdateTime,
            update_input: input::UpdateInput = input::UpdateInput,

            physics_run: physics::PhysicsRun = physics::PhysicsRun,
            physics_update: EntitySystem<physics::PhysicsUpdate> = physics::PhysicsUpdate::new(),

            begin_frame: graphics::BeginFrame = graphics::BeginFrame,
            temp_draw: graphics::TempDraw = graphics::TempDraw,
            end_frame: graphics::EndFrame = graphics::EndFrame,
        },
        passive: {
        }
    }
}

fn load_test_map(world: &mut ecs::World<Systems>) {
    let tmap = tiled::parse_file(std::path::Path::new("resources/maps/testmap.tmx")).unwrap();
    let map = tilemap::load_map(tmap, &mut world.data.services.graphics);
    world.data.services.current_map = Some(map);
}
