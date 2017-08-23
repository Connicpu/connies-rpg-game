use glium::{Display, Program};

pub fn load_sprite_shader(display: &Display) -> Program {
    program!(
        display,
        330 => {
            vertex: include_str!("sprite_vs.glsl"),
            fragment: include_str!("sprite_fs.glsl"),
        }
    ).unwrap()
}

pub fn load_tile_shader(display: &Display) -> Program {
    program!(
        display,
        330 => {
            vertex: include_str!("tile_vs.glsl"),
            fragment: include_str!("tile_fs.glsl"),
        }
    ).unwrap()
}

pub fn load_fxaa_shader(display: &Display) -> Program {
    program!(
        display,
        330 => {
            vertex: include_str!("postprocess_vs.glsl"),
            fragment: include_str!("fxaa_fs.glsl"),
        }
    ).unwrap()
}
