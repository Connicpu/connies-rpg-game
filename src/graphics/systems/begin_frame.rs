use glium::Surface;

def_system! {
    pub struct BeginFrame;

    fn process(&mut self, data: &mut DataHelper) {
        let mut frame = data.services.graphics.display.draw();
        frame.clear_color_srgb(100.0/255.0, 149.0/255.0, 237.0/255.0, 1.0);
        frame.clear_depth(0.0);

        data.services.graphics.current_frame = Some(frame);
    }
}
