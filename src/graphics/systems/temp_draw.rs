use glium::Surface;

def_system! {
    pub struct TempDraw;

    fn process(&mut self, data: &mut DataHelper) {
        let frame = data.services.graphics.current_frame.as_mut().unwrap();
        frame.clear_color(0.0, 0.0, 0.0, 0.0);
    }
}
