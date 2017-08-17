use glium;
use winit;
use glutin;
use windows_dpi;

use glium::index::PrimitiveType::TrianglesList;
use glium::{Blend, Depth, DepthTest, DrawParameters, IndexBuffer, VertexBuffer};
use glium::texture::SrgbTexture2d;
use glium::framebuffer::SimpleFrameBuffer;
use glium::uniforms::MagnifySamplerFilter;

use graphics::quad_types::{QUAD_INDICES, QUAD_VERTICES};

pub use graphics::quad_types::{Camera, QuadVertex, SpriteInstance};
pub use graphics::textures::TextureId;

pub mod quad_types;
pub mod shaders;
pub mod textures;

pub struct System {
    pub events_loop: Option<winit::EventsLoop>,
    pub display: glium::Display,
    pub textures: textures::TextureManager,
    pub dpi: f32,

    quad_vertices: glium::VertexBuffer<QuadVertex>,
    quad_indices: glium::IndexBuffer<u32>,
    sprite_shader: glium::Program,

    fxaa_shader: glium::Program,
    fxaa_buffer: Option<SrgbTexture2d>,
}

impl System {
    pub fn new() -> System {
        windows_dpi::enable_dpi();
        let dpi = windows_dpi::desktop_dpi();

        let events_loop = winit::EventsLoop::new();
        let window_builder = winit::WindowBuilder::new()
            //.with_fullscreen(winit::get_primary_monitor())
            .with_dimensions((800.0 * dpi) as u32, (600.0 * dpi) as u32)
            .with_title("Connie's RPG Game");
        let gl_builder = glutin::ContextBuilder::new()
            .with_vsync(true)
            .with_depth_buffer(24)
            .with_multisampling(4);
        let display = glium::Display::new(window_builder, gl_builder, &events_loop)
            .expect("Display creation failure");

        let textures = textures::TextureManager::new();

        let quad_vertices = VertexBuffer::new(&display, &QUAD_VERTICES[..]).unwrap();
        let quad_indices = IndexBuffer::new(&display, TrianglesList, &QUAD_INDICES[..]).unwrap();
        let sprite_shader = shaders::load_sprite_shader(&display);

        let fxaa_shader = shaders::load_fxaa_shader(&display);

        System {
            events_loop: Some(events_loop),
            display,
            textures,
            dpi,

            quad_vertices,
            quad_indices,
            sprite_shader,

            fxaa_shader,
            fxaa_buffer: None,
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
            .minify_filter(glium::uniforms::MinifySamplerFilter::Linear)
            .anisotropy(4);

        let instanced = instance_buffer.per_instance().unwrap();

        surface
            .draw(
                (&self.quad_vertices, instanced),
                &self.quad_indices,
                &self.sprite_shader,
                &uniform! {
                    tex: sampler,
                    camera_view: camera.view,
                    camera_proj: camera.proj,
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
}

impl Default for System {
    fn default() -> Self {
        System::new()
    }
}
