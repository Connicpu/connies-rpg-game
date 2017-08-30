use winit::WindowEvent::*;
use winit::{self, ElementState};

use util::Mutate;
use input::keyboard::KeyboardUpdate;

#[derive(Default, System)]
#[process(process)]
pub struct UpdateInput;

fn process(_: &mut UpdateInput, data: &mut ::DataHelper) {
    data.services.keyboard.frame_start();

    let mut ev_loop = data.services.graphics.events_loop.take().unwrap();

    // Handle input
    ev_loop.poll_events(|event| match event {
        winit::Event::WindowEvent { event, .. } => {
            match event {
                Closed => data.services.quit = true,

                Focused(false) => {
                    println!("Focus lost");
                    data.services.keyboard.focus_lost();
                }

                KeyboardInput { input, .. } => {
                    match (input.virtual_keycode, input.state) {
                        (Some(vk), ElementState::Pressed) => {
                            data.services.keyboard.key_pressed(vk);
                        }
                        (Some(vk), ElementState::Released) => {
                            data.services.keyboard.key_released(vk);
                        }
                        _ => (),
                    }
                }

                _ => (),
            }
        }
        _ => (),
    });

    data.services.graphics.events_loop = Some(ev_loop);

    let dt = data.services.timer.delta_time;
    if data.services.keyboard.is_held(winit::VirtualKeyCode::D) {
        data.services.camera.pos.x += 6.0 * dt;
    }
    if data.services.keyboard.is_held(winit::VirtualKeyCode::A) {
        data.services.camera.pos.x -= 6.0 * dt;
    }
    if data.services.keyboard.is_held(winit::VirtualKeyCode::W) {
        data.services.camera.pos.y += 6.0 * dt;
    }
    if data.services.keyboard.is_held(winit::VirtualKeyCode::S) {
        data.services.camera.pos.y -= 6.0 * dt;
    }

    let aspect = data.services.camera.aspect_ratio;
    data.services.camera.pos.mutate(|cpos| {
        cpos.y = cpos.y.max(-252.0).min(252.0);
        cpos.x = cpos.x.max(4.0 * aspect).min(256.0 - 4.0 * aspect);
    });
}
