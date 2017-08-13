#![feature(inclusive_range_syntax, range_contains)]

extern crate specs;
extern crate tiled;
extern crate wrapped2d;
extern crate shred;
extern crate time;
extern crate cgmath;
extern crate winit;
extern crate glutin;
extern crate windows_dpi;

#[macro_use]
extern crate glium;
#[macro_use]
extern crate shred_derive;
#[macro_use]
extern crate specs_derive;

use specs::Join;

pub mod components;
pub mod graphics;
pub mod physics;
pub mod timer;

pub mod util;

fn main() {
    let dpi;
    #[cfg(windows)] {
        windows_dpi::enable_dpi();
        dpi = unsafe { windows_dpi::get_dpi_for(std::ptr::null_mut()) };
    }
    #[cfg(not(windows))] {
        dpi = 1.0; // TODO
    }
    println!("{}", dpi);

    let mut events_loop = winit::EventsLoop::new();
    let window_builder = winit::WindowBuilder::new()
        //.with_fullscreen(winit::get_primary_monitor())
        .with_dimensions((800.0 * dpi) as u32, (600.0 * dpi) as u32)
        .with_title("Connie's RPG Game");
    let context = glutin::ContextBuilder::new()
        .with_vsync(true)
        .with_depth_buffer(24)
        .with_multisampling(4);
    let gl_window = glutin::GlWindow::new(window_builder, context, &events_loop).unwrap();
    let display = glium::Display::from_gl_window(gl_window).unwrap();

    let mut world = specs::World::new();

    world.add_resource(timer::Timer::new());
    world.add_resource(physics::World::new());

    world.register::<components::Transform>();
    world.register::<components::Sprite>();
    world.register::<physics::Body>();

    let mut dispatcher = specs::DispatcherBuilder::new()
        .add(physics::run::PhysicsRun, "physics_run", &[])
        .add(
            physics::update::PhysicsUpdate,
            "physics_update",
            &["physics_run"],
        )
        .build();

    let mut framebuffer = Framebuffer::create(&display);

    let quad_vertices =
        glium::VertexBuffer::new(&display, &graphics::vertex::QUAD_VERTICES).unwrap();
    let quad_indices = glium::IndexBuffer::new(
        &display,
        glium::index::PrimitiveType::TrianglesList,
        &graphics::vertex::QUAD_INDICES,
    ).unwrap();
    let fxaa = graphics::shaders::load_fxaa(&display);

    let mut running = true;
    let mut resized = false;
    let mut i = 0;
    loop {
        use glium::Surface;

        // Update time
        world.write_resource::<timer::Timer>().update();

        // Handle input
        events_loop.poll_events(|event| match event {
            winit::Event::WindowEvent {
                event: winit::WindowEvent::Closed,
                ..
            } => running = false,
            winit::Event::WindowEvent {
                event: winit::WindowEvent::Resized(..),
                ..
            } => resized = true,
            _ => (),
        });

        if !running {
            break;
        }

        // Dispatch systems
        dispatcher.dispatch(&mut world.res);

        if i > 30 {
            println!("{}", 1.0 / world.read_resource::<timer::Timer>().delta_time);
            i = 0;
        } else {
            i += 1;
        }

        // Draw
        let mut frame = display.draw();
        frame.clear_color(0.5, 0.5, 0.5, 0.0);

        // Copy the window to a framebuffer and perform FXAA
        if resized {
            framebuffer = Framebuffer::create(&display);
            resized = false;
        }
        let fbo = framebuffer.to_surface(&display);
        frame.fill(&fbo, glium::uniforms::MagnifySamplerFilter::Linear);
        frame
            .draw(
                &quad_vertices,
                &quad_indices,
                &fxaa,
                &uniform!{
                tex: framebuffer.color.sampled()
            },
                &glium::DrawParameters {
                    ..Default::default()
                },
            )
            .unwrap();
        frame.finish().unwrap();
    }
}

struct Framebuffer {
    color: glium::texture::Texture2d,
}

impl Framebuffer {
    pub fn create(display: &glium::Display) -> Self {
        use glium::texture::Texture2d;

        let (width, height) = display.get_framebuffer_dimensions();

        Framebuffer {
            color: Texture2d::empty(display, width, height).unwrap(),
        }
    }

    pub fn to_surface(&self, display: &glium::Display) -> glium::framebuffer::SimpleFrameBuffer {
        glium::framebuffer::SimpleFrameBuffer::new(display, &self.color).unwrap()
    }
}
