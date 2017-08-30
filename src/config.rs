use toml;

use std::fs::File;
use std::io::Read;

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

lazy_static! {
    pub static ref CONFIG: Config = {
        let mut config_data = vec![];
        let mut file = File::open("resources/config/config.toml").expect("TODO: fallback to default config");
        file.read_to_end(&mut config_data).expect("TODO: fallback to default config");
        toml::from_slice(&config_data).expect("TODO: fallback to default config")
    };
}
