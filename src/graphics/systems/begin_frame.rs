use glium::Surface;

use DataHelper;
use math::raw::ToRawMath;

#[derive(Default, System)]
#[process(process)]
pub struct BeginFrame;

fn process(_: &mut BeginFrame, data: &mut DataHelper) {
    let mut frame = data.services.graphics.display.draw();
    let (width, height) = frame.get_dimensions();

    // Cornflower Blue
    frame.clear_color_srgb(100.0 / 255.0, 149.0 / 255.0, 237.0 / 255.0, 1.0);
    frame.clear_depth(0.0);

    data.services.graphics.current_frame = Some(frame);

    data.services.camera.aspect_ratio = width as f32 / height as f32;
    data.services.graphics.set_camera(
        &data.services.camera.to_raw(),
    );
}
