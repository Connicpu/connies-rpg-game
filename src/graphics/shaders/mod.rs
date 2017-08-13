use glium::Display;
use glium::Program;

pub fn load_fxaa(display: &Display) -> Program {
    program!(
        display,
        330 => {
            vertex: include_str!("postprocess_vs.glsl"),
            fragment: include_str!("fxaa_fs.glsl"),
        }
    ).unwrap()
}

pub fn load_sprite_shader(display: &Display) -> Program {
    program!(
        display,
        330 => {
            vertex: include_str!("sprite_vs.glsl"),
            fragment: include_str!("sprite_fs.glsl"),
        }
    ).unwrap()
}
