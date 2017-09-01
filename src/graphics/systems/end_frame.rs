use std::{thread, time};

use CONFIG;
use DataHelper;

#[derive(Default, System)]
#[process(process)]
pub struct EndFrame;

fn process(_: &mut EndFrame, data: &mut DataHelper) {
    data.services
        .graphics
        .current_frame
        .take()
        .map(|f| f.finish().unwrap());
    
    println!("draw count: {}", data.services.graphics.draw_count);
    
    data.services.graphics.draw_count = 0;

    let min_frametime = 1_000_000_000 / CONFIG.graphics.max_fps as u64;
    while data.services.timer.immediate_frametime_ns() < min_frametime {
        thread::sleep(time::Duration::new(0, 0));
    }
}
