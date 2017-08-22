use ecs::{Process, System};

use std::{thread, time};

use CONFIG;

pub struct EndFrame;

impl System for EndFrame {
    type Services = ::Services;
    type Components = ::Components;
}

impl Process for EndFrame {
    fn process(&mut self, data: &mut ::DataHelper) {
        data.services.graphics.current_frame.take().map(|f| f.finish().unwrap());

        let min_frametime = 1_000_000_000 / CONFIG.graphics.max_fps as u64;
        while data.services.timer.immediate_frametime_ns() < min_frametime {
            thread::sleep(time::Duration::new(0, 0));
        }
    }
}
