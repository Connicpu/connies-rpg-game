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

use ecs::system::EntitySystem;

pub mod components;
pub mod graphics;
pub mod input;
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

fn main() {
    util::panic_handler::init();

    let mut world = ecs::World::<Systems>::with_services(Services::default());

    let overworld = world.data.services.graphics.load_texture("tilesets/overworld.png");

    loop {
        process!(world, update_time);
        process!(world, update_input);

        if world.data.services.quit {
            break;
        }

        world.update();

        {
            use graphics::{Camera,SpriteInstance};
            use glium::Surface;

            fn rect(x: u32, y: u32) -> [f32; 4] {
                const OFFSET: f32 = 15.99 / 271.0;

                let left = x as f32 * 17.0 / 271.0;
                let top = y as f32 * 17.0 / 271.0;
                [left, top, left + OFFSET, top + OFFSET]
            }

            fn sprite(x: f32, y: f32, u: u32, v: u32) -> SpriteInstance {
                SpriteInstance {
                    center: [0.5, 0.5],
                    scale: [1.001, 1.001],
                    rot: [[1.0, 0.0], [0.0, 1.0]],
                    uv_rect: rect(u, v),
                    world_pos: [x, y, 1.0 - y * 0.01],
                }
            }

            let mut frame = world.data.services.graphics.display.draw();
            frame.clear_color(0.5, 0.5, 0.5, 1.0);
            frame.clear_depth(0.0);

            let instances = [
                sprite(0.0, 0.0, 0, 0),
                sprite(1.0, 0.0, 1, 0),
                sprite(2.0, 0.0, 1, 0),
                sprite(3.0, 0.0, 4, 0),
                sprite(3.0, 1.0, 0, 2),
            ];

            let dimensions = frame.get_dimensions();
            let aspect = dimensions.1 as f32 / dimensions.0 as f32;

            let camera = Camera {
                view: [[0.5, 0.0, -0.75],
                       [0.0, 0.5, -0.5],
                       [0.0, 0.0, 1.0]],
                proj: [[aspect, 0.0, 0.0, 0.0],
                       [   0.0, 1.0, 0.0, 0.0],
                       [   0.0, 0.0, 1.0, 0.0],
                       [   0.0, 0.0, 0.0, 1.0]],
            };

            world.data.services.graphics.draw_sprites(&mut frame, &camera, &instances, overworld);

            if !world.data.services.keyboard.is_held(winit::VirtualKeyCode::F) {
                world.data.services.graphics.fxaa(&mut frame);
            }

            frame.finish().unwrap();
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
        }
    }
}
