use winit::WindowEvent::*;
use winit::{self, ElementState};

use input::keyboard::KeyboardUpdate;

#[derive(Default, System)]
#[process(process)]
pub struct UpdateInput;

fn process(_: &mut UpdateInput, data: &mut ::DataHelper) {
    data.services.keyboard.frame_start();

    let mut ev_loop = data.services.graphics.events_loop.take().unwrap();

    // Handle input
    ev_loop.poll_events(|event| {
        if let winit::Event::WindowEvent { event, .. } = event {
            match event {
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
            }
        }
    });

    data.services.graphics.events_loop = Some(ev_loop);
}
