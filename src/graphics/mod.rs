use glium;
use winit;
use glutin;
use windows_dpi;

use glium::index::PrimitiveType::TrianglesList;
use glium::{Blend, Depth, DepthTest, DrawParameters, IndexBuffer, VertexBuffer};
use glium::texture::SrgbTexture2d;
use glium::framebuffer::SimpleFrameBuffer;
use glium::uniforms::MagnifySamplerFilter;

use std::collections::VecDeque;

use CONFIG;
use graphics::quad_types::{QUAD_INDICES, QUAD_VERTICES};
use graphics::tileset::{TileInstance, TilesetDesc};
use graphics::scene::grid::UniformGrid;
use tilemap;

pub use graphics::quad_types::{Camera, QuadVertex, SpriteInstance};
pub use graphics::textures::TextureId;
pub use graphics::systems::all::*;

pub mod box2d;
pub mod scene;
pub mod shaders;
pub mod systems;
pub mod textures;
pub mod tileset;
pub mod quad_types;

const SPRITE_BUFFER_SIZE: usize = 32;

pub struct System {
    pub events_loop: Option<winit::EventsLoop>,
    pub display: glium::Display,
    pub textures: textures::TextureManager,
    pub dpi: f32,
    pub current_frame: Option<glium::Frame>,
    pub scene_grid: UniformGrid,

    quad_vertices: glium::VertexBuffer<QuadVertex>,
    quad_indices: glium::IndexBuffer<u32>,
    sprite_shader: glium::Program,
    sprite_buffers: VecDeque<VertexBuffer<SpriteInstance>>,
    tile_shader: glium::Program,
    tile_buffers: VecDeque<VertexBuffer<TileInstance>>,

    fxaa_shader: glium::Program,
    fxaa_buffer: Option<SrgbTexture2d>,

    camera: Camera,

    debugdraw: box2d::DebugDraw,

    pub draw_count: usize,
    pub tile_draw_count: usize,
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

        let scene_grid = UniformGrid::new(4.0, 0);

        let textures = textures::TextureManager::new();

        let quad_vertices = VertexBuffer::new(&display, &QUAD_VERTICES[..]).unwrap();
        let quad_indices = IndexBuffer::new(&display, TrianglesList, &QUAD_INDICES[..]).unwrap();
        let sprite_shader = shaders::load_sprite_shader(&display);
        let sprite_buffers = (0..32)
            .map(|_| Self::make_sprite_buffer(&display))
            .collect();
        let tile_shader = shaders::load_tile_shader(&display);
        let tile_buffers = (0..32).map(|_| Self::make_tile_buffer(&display)).collect();

        let fxaa_shader = shaders::load_fxaa_shader(&display);

        let debugdraw = box2d::DebugDraw::new(&display);

        System {
            events_loop: Some(events_loop),
            display,
            textures,
            dpi,
            current_frame: None,
            scene_grid,

            quad_vertices,
            quad_indices,
            sprite_shader,
            sprite_buffers,
            tile_shader,
            tile_buffers,

            fxaa_shader,
            fxaa_buffer: None,

            camera: Default::default(),

            debugdraw,

            draw_count: 0,
            tile_draw_count: 0,
        }
    }

    pub fn set_camera(&mut self, camera: &Camera) {
        self.camera = *camera;
    }

    pub fn draw_sprites<S>(
        &mut self,
        surface: &mut S,
        instances: &[SpriteInstance],
        texture: TextureId,
    ) where
        S: glium::Surface,
    {
        let use_sbuffer = instances.len() <= SPRITE_BUFFER_SIZE;

        let base_instance_buffer = if use_sbuffer {
            self.sprite_buffers.pop_front().unwrap()
        } else {
            glium::VertexBuffer::immutable(&self.display, instances)
                .expect("instance buffer creation shouldn't fail")
        };

        {
            let instance_buffer = if use_sbuffer {
                let buf = base_instance_buffer.slice(..instances.len()).unwrap();

                buf.write(instances);
                buf
            } else {
                base_instance_buffer.slice(..).unwrap()
            };

            let tex = self.textures.get(texture);
            let sampler = tex.tex
                .sampled()
                .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest)
                .minify_filter(glium::uniforms::MinifySamplerFilter::Linear);

            surface
                .draw(
                    (&self.quad_vertices, instance_buffer.per_instance().unwrap()),
                    &self.quad_indices,
                    &self.sprite_shader,
                    &uniform! {
                        tex: sampler,
                        proj: self.camera.proj,
                        view: self.camera.view,
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

            self.draw_count += 1;
        }

        if use_sbuffer {
            self.sprite_buffers.push_back(base_instance_buffer);
        }
    }

    pub fn draw_tiles<S>(
        &mut self,
        surface: &mut S,
        base_pos: [f32; 3],
        tint: [f32; 4],
        tiles: &[u16],
        tileset: &TilesetDesc,
    ) where
        S: glium::Surface,
    {
        let tile_buffer = self.tile_buffers.pop_front().unwrap();

        let tile_data = unsafe { &*(tiles as *const [u16] as *const [TileInstance]) };
        tile_buffer.write(tile_data);

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
                    end_gid: tileset.end_gid,
                    world_base_pos: base_pos,
                    tint: tint,
                    proj: self.camera.proj,
                    view: self.camera.view,
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

        self.draw_count += 1;
        self.tile_draw_count += 1;

        self.tile_buffers.push_back(tile_buffer);
    }

    pub fn draw_tile_chunk<S>(
        &mut self,
        surface: &mut S,
        base_pos: [f32; 3],
        tint: [f32; 4],
        chunk: &tilemap::Chunk,
        tilesets: &tilemap::Tilesets,
    ) where
        S: glium::Surface,
    {
        for &set in &chunk.tilesets[0..chunk.tilesets_count as usize] {
            let desc = &tilesets.tileset_descs[set as usize];
            self.draw_tiles(surface, base_pos, tint, &chunk.tiles, desc);
        }
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

        self.draw_count += 1;
    }

    pub fn load_texture(&mut self, asset: &str) -> TextureId {
        self.textures.load(&self.display, asset)
    }

    pub fn load_animation(&mut self, asset: &str) -> () {
        let _ = asset;
        // TODO
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

    fn make_sprite_buffer(display: &glium::Display) -> VertexBuffer<SpriteInstance> {
        VertexBuffer::empty_persistent(display, SPRITE_BUFFER_SIZE).unwrap_or_else(|_| {
            VertexBuffer::empty_dynamic(display, SPRITE_BUFFER_SIZE).unwrap()
        })
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
