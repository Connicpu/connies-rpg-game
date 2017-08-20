#[derive(Default, Deserialize, Clone)]
pub struct Config {
    pub graphics: Graphics,
}

#[derive(Deserialize, Clone)]
pub struct Graphics {
    pub vsync: bool,
    pub fullscreen: bool,
    pub max_fps: u32,
}

impl Default for Graphics {
    fn default() -> Self {
        Graphics {
            vsync: false,
            fullscreen: false,
            max_fps: 120,
        }
    }
}
