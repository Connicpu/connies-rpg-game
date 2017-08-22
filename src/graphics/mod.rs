use glium;
use winit;
use glutin;
use windows_dpi;

use glium::index::PrimitiveType::TrianglesList;
use glium::{Blend, Depth, DepthTest, DrawParameters, IndexBuffer, VertexBuffer};
use glium::texture::SrgbTexture2d;
use glium::framebuffer::SimpleFrameBuffer;
use glium::uniforms::{MagnifySamplerFilter, UniformBuffer};

use std::collections::VecDeque;

use graphics::quad_types::{QUAD_INDICES, QUAD_VERTICES};
use graphics::tileset::{TileInstance, TilesetDesc};
use CONFIG;

pub use graphics::quad_types::{Camera, QuadVertex, SpriteInstance};
pub use graphics::textures::TextureId;
pub use graphics::systems::all::*;

pub mod quad_types;
pub mod shaders;
pub mod systems;
pub mod textures;
pub mod tileset;

pub struct System {
    pub events_loop: Option<winit::EventsLoop>,
    pub display: glium::Display,
    pub textures: textures::TextureManager,
    pub dpi: f32,
    pub current_frame: Option<glium::Frame>,

    quad_vertices: glium::VertexBuffer<QuadVertex>,
    quad_indices: glium::IndexBuffer<u32>,
    sprite_shader: glium::Program,
    tile_shader: glium::Program,
    tile_buffers: VecDeque<VertexBuffer<TileInstance>>,

    fxaa_shader: glium::Program,
    fxaa_buffer: Option<SrgbTexture2d>,

    camera_buffer: UniformBuffer<Camera>,
}

impl System {
    pub fn new() -> System {
        windows_dpi::enable_dpi();
        let dpi = windows_dpi::desktop_dpi();

        let events_loop = winit::EventsLoop::new();
        let mut window_builder = winit::WindowBuilder::new()
            .with_dimensions((800.0 * dpi) as u32, (600.0 * dpi) as u32)
            .with_title("Connie's RPG Game");

        if CONFIG.graphics.fullscreen {
            window_builder = window_builder.with_fullscreen(winit::get_primary_monitor());
        }

        let gl_builder = glutin::ContextBuilder::new()
            .with_vsync(CONFIG.graphics.vsync)
            .with_depth_buffer(24)
            .with_srgb(true);
        let display = glium::Display::new(window_builder, gl_builder, &events_loop)
            .expect("Display creation failure");

        let textures = textures::TextureManager::new();

        let quad_vertices = VertexBuffer::new(&display, &QUAD_VERTICES[..]).unwrap();
        let quad_indices = IndexBuffer::new(&display, TrianglesList, &QUAD_INDICES[..]).unwrap();
        let sprite_shader = shaders::load_sprite_shader(&display);
        let tile_shader = shaders::load_tile_shader(&display);
        let tile_buffers = (0..16).map(|_| Self::make_tile_buffer(&display)).collect();

        let fxaa_shader = shaders::load_fxaa_shader(&display);

        let camera_buffer = UniformBuffer::empty_dynamic(&display).unwrap();

        System {
            events_loop: Some(events_loop),
            display,
            textures,
            dpi,
            current_frame: None,

            quad_vertices,
            quad_indices,
            sprite_shader,
            tile_shader,
            tile_buffers,

            fxaa_shader,
            fxaa_buffer: None,

            camera_buffer,
        }
    }

    pub fn draw_sprites<S>(
        &mut self,
        surface: &mut S,
        camera: &Camera,
        instances: &[SpriteInstance],
        texture: TextureId,
    ) where
        S: glium::Surface,
    {
        let instance_buffer = glium::VertexBuffer::new(&self.display, instances)
            .expect("instance buffer creation shouldn't fail");
        let tex = self.textures.get(texture);
        let sampler = tex.tex
            .sampled()
            .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest)
            .minify_filter(glium::uniforms::MinifySamplerFilter::Linear);

        let instanced = instance_buffer.per_instance().unwrap();

        self.camera_buffer.write(camera);

        surface
            .draw(
                (&self.quad_vertices, instanced),
                &self.quad_indices,
                &self.sprite_shader,
                &uniform! {
                    tex: sampler,
                    Camera: &self.camera_buffer,
                },
                &DrawParameters {
                    blend: Blend::alpha_blending(),
                    depth: Depth {
                        test: DepthTest::IfMoreOrEqual,
                        write: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            )
            .expect("draw shouldn't fail");
    }

    pub fn draw_tiles<S>(
        &mut self,
        surface: &mut S,
        camera: &Camera,
        base_pos: [f32; 3],
        tiles: &[u16],
        tileset: &TilesetDesc,
    ) where
        S: glium::Surface,
    {
        let tile_buffer = self.tile_buffers.pop_front().unwrap();

        let tile_data = unsafe { &*(tiles as *const [u16] as *const [TileInstance]) };
        tile_buffer.write(tile_data);

        self.camera_buffer.write(camera);

        let tex = self.textures.get(tileset.texture);
        let tex_sampler = tex.tex
            .sampled()
            .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest)
            .minify_filter(glium::uniforms::MinifySamplerFilter::Linear);

        let tileset_sampler = tileset.tile_uv.sampled();

        surface
            .draw(
                (&self.quad_vertices, tile_buffer.per_instance().unwrap()),
                &self.quad_indices,
                &self.tile_shader,
                &uniform! {
                    tex: tex_sampler,
                    tileset: tileset_sampler,
                    first_gid: tileset.tileset.first_gid,
                    world_base_pos: base_pos,
                    Camera: &self.camera_buffer,
                },
                &DrawParameters {
                    blend: Blend::alpha_blending(),
                    depth: Depth {
                        test: DepthTest::IfMoreOrEqual,
                        write: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            )
            .expect("draw shouldn't fail");

        self.tile_buffers.push_back(tile_buffer);
    }

    pub fn fxaa<S>(&mut self, surface: &mut S)
    where
        S: glium::Surface,
    {
        self.update_fxaa_buffer(surface.get_dimensions());

        surface.fill(&self.bind_fxaa_buffer(), MagnifySamplerFilter::Linear);

        surface
            .draw(
                &self.quad_vertices,
                &self.quad_indices,
                &self.fxaa_shader,
                &uniform! {
                    tex: self.fxaa_color_buffer().sampled(),
                },
                &Default::default(),
            )
            .unwrap();
    }

    pub fn load_texture(&mut self, asset: &str) -> TextureId {
        self.textures.load(&self.display, asset)
    }

    fn update_fxaa_buffer(&mut self, dimensions: (u32, u32)) {
        if let Some(ref tex) = self.fxaa_buffer {
            if tex.dimensions() == dimensions {
                return;
            }
        }

        self.fxaa_buffer = Some(
            SrgbTexture2d::empty(&self.display, dimensions.0, dimensions.1).unwrap(),
        );
    }

    fn fxaa_color_buffer(&self) -> &SrgbTexture2d {
        self.fxaa_buffer.as_ref().unwrap()
    }

    fn bind_fxaa_buffer(&self) -> SimpleFrameBuffer {
        SimpleFrameBuffer::new(&self.display, self.fxaa_color_buffer()).unwrap()
    }

    fn make_tile_buffer(display: &glium::Display) -> VertexBuffer<TileInstance> {
        VertexBuffer::empty_persistent(display, 64)
            .unwrap_or_else(|_| VertexBuffer::empty_dynamic(display, 64).unwrap())
    }
}

impl Default for System {
    fn default() -> Self {
        System::new()
    }
}
