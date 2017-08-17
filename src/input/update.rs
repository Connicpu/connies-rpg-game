use ecs::system::{Process, System};
use input::keyboard::KeyboardUpdate;
use winit::WindowEvent::*;
use winit::{self, ElementState};

pub struct UpdateInput;

impl System for UpdateInput {
    type Components = ::Components;
    type Services = ::Services;
}

impl Process for UpdateInput {
    fn process(&mut self, data: &mut ::DataHelper) {
        data.services.keyboard.frame_start();

        let mut ev_loop = data.services.graphics.events_loop.take().unwrap();

        // Handle input
        ev_loop.poll_events(|event| match event {
            winit::Event::WindowEvent { event, .. } => match event {
                Closed => data.services.quit = true,

                Focused(false) => {
                    println!("Focus lost");
                    data.services.keyboard.focus_lost();
                }

                KeyboardInput { input, .. } => match (input.virtual_keycode, input.state) {
                    (Some(vk), ElementState::Pressed) => {
                        data.services.keyboard.key_pressed(vk);
                    }
                    (Some(vk), ElementState::Released) => {
                        data.services.keyboard.key_released(vk);
                    }
                    _ => (),
                },

                _ => (),
            },
            _ => (),
        });

        data.services.graphics.events_loop = Some(ev_loop);
    }
}
