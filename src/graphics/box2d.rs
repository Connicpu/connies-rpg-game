use glium::{Display, DrawParameters, Frame, Program, Surface, VertexBuffer};
use glium::index::{NoIndices, PrimitiveType};
use wrapped2d::b2;

use std::f32;

use DataHelper;
use graphics::Camera;
use math::Aabb;

pub struct DebugDraw {
    display: Display,
    line_vertices: Vec<DebugVertex>,
    vertex_buffer: VertexBuffer<DebugVertex>,
    debug_shader: Program,
    aabb: Aabb,
}

impl DebugDraw {
    #[allow(redundant_closure)]
    pub fn new(display: &Display) -> Self {
        DebugDraw {
            display: display.clone(),
            line_vertices: vec![],
            vertex_buffer: VertexBuffer::empty_dynamic(display, 4096).unwrap(),
            debug_shader: program!(
                display,
                330 => {
                    vertex: include_str!("shaders/debugdraw_vs.glsl"),
                    fragment: include_str!("shaders/debugdraw_fs.glsl"),
                }
            ).unwrap(),
            aabb: Aabb::empty(),
        }
    }

    fn draw_lines(&mut self, camera: &Camera, frame: &mut Frame) {
        let vert_count = self.line_vertices.len();
        if vert_count > self.vertex_buffer.len() {
            let new_ct = (vert_count as f64 * 1.25) as usize;
            self.vertex_buffer = VertexBuffer::empty_dynamic(&self.display, new_ct).unwrap();
        }

        let buf_slice = self.vertex_buffer.slice(0..vert_count).unwrap();
        buf_slice.write(&self.line_vertices);

        frame
            .draw(
                buf_slice,
                NoIndices(PrimitiveType::LinesList),
                &self.debug_shader,
                &uniform!{
                    proj: camera.proj,
                    view: camera.view,
                },
                &DrawParameters {
                    line_width: Some(4.0),
                    ..Default::default()
                },
            )
            .unwrap();

        self.line_vertices.clear();
    }

    fn add_line(&mut self, p0: &b2::Vec2, p1: &b2::Vec2, color: &b2::Color) {
        if !self.aabb.contains_point_xy(p0.x, p0.y) && !self.aabb.contains_point_xy(p1.x, p1.y) {
            return;
        }

        self.line_vertices.push(DebugVertex {
            pos: [p0.x, p0.y],
            color: [color.r, color.g, color.b],
        });
        self.line_vertices.push(DebugVertex {
            pos: [p1.x, p1.y],
            color: [color.r, color.g, color.b],
        });
    }
}

impl b2::Draw for DebugDraw {
    fn draw_polygon(&mut self, vertices: &[b2::Vec2], color: &b2::Color) {
        for window in vertices.windows(2) {
            self.add_line(&window[0], &window[1], color);
        }
        if vertices.len() >= 2 {
            self.add_line(&vertices[vertices.len() - 1], &vertices[0], color);
        }
    }

    fn draw_solid_polygon(&mut self, vertices: &[b2::Vec2], color: &b2::Color) {
        self.draw_polygon(vertices, color); // TODO
    }

    fn draw_circle(&mut self, center: &b2::Vec2, radius: f32, color: &b2::Color) {
        // 16 segments is good
        let mut prev = b2::Vec2 {
            x: center.x,
            y: center.y + radius,
        };

        for i in 1...24i32 {
            let theta = i as f32 / 24.0 * 2.0 * f32::consts::PI;
            let x = center.x + theta.sin() * radius;
            let y = center.y + theta.cos() * radius;
            let point = b2::Vec2 { x, y };

            self.add_line(&prev, &point, color);

            prev = point;
        }
    }

    fn draw_solid_circle(
        &mut self,
        center: &b2::Vec2,
        radius: f32,
        _axis: &b2::Vec2,
        color: &b2::Color,
    ) {
        self.draw_circle(center, radius, color); // TODO
    }

    fn draw_segment(&mut self, p1: &b2::Vec2, p2: &b2::Vec2, color: &b2::Color) {
        self.add_line(p1, p2, color);
    }

    fn draw_transform(&mut self, xf: &b2::Transform) {
        let center = xf.pos;

        // Draw up arrow
        {
            let arrow_top = arrow_pt(&center, &xf.rot, 0.0, 0.5);
            let arrow_left = arrow_pt(&center, &xf.rot, -0.05, 0.42);
            let arrow_right = arrow_pt(&center, &xf.rot, 0.05, 0.42);
            self.add_line(&center, &arrow_top, &GREEN);
            self.add_line(&arrow_top, &arrow_left, &GREEN);
            self.add_line(&arrow_top, &arrow_right, &GREEN);
        }

        // Draw right arrow
        {
            let arrow_top = arrow_pt(&center, &xf.rot, 0.5, 0.0);
            let arrow_left = arrow_pt(&center, &xf.rot, 0.42, -0.05);
            let arrow_right = arrow_pt(&center, &xf.rot, 0.42, 0.05);
            self.add_line(&center, &arrow_top, &RED);
            self.add_line(&arrow_top, &arrow_left, &RED);
            self.add_line(&arrow_top, &arrow_right, &RED);
        }
    }
}

fn arrow_pt(center: &b2::Vec2, rot: &b2::Rot, x: f32, y: f32) -> b2::Vec2 {
    b2::Vec2 {
        x: center.x + x * rot.cos - y * rot.sin,
        y: center.y + x * rot.sin + y * rot.cos,
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct DebugVertex {
    pos: [f32; 2],
    color: [f32; 3],
}

mod debugvertex_impl {
    #![allow(forget_copy)]
    use super::DebugVertex;
    implement_vertex!(DebugVertex, pos, color);
}

#[derive(Default, System)]
#[process(draw_physics)]
pub struct DrawPhysics {
    enabled: bool,
    draw_aabb: bool,
}

fn draw_physics(draw: &mut DrawPhysics, data: &mut DataHelper) {
    use winit::VirtualKeyCode as VK;
    if data.services.keyboard.is_pressed(VK::F9) {
        draw.enabled = !draw.enabled;
    }

    if data.services.keyboard.is_pressed(VK::F10) {
        draw.draw_aabb = !draw.draw_aabb;
    }

    if !draw.enabled {
        return;
    }

    let p = &mut data.services.physics;
    let dd = &mut data.services.graphics.debugdraw;

    dd.aabb = data.services.camera.aabb();

    let mut flags = b2::DRAW_SHAPE | b2::DRAW_JOINT | b2::DRAW_CENTER_OF_MASS;
    if draw.draw_aabb {
        flags |= b2::DRAW_AABB;
    }
    p.world.draw_debug_data(dd, flags);

    let mut frame = data.services.graphics.current_frame.take().unwrap();
    dd.draw_lines(&data.services.graphics.camera, &mut frame);
    data.services.graphics.current_frame = Some(frame);
}

const RED: b2::Color = b2::Color {
    r: 1.0,
    g: 0.0,
    b: 0.0,
    a: 1.0,
};

const GREEN: b2::Color = b2::Color {
    r: 0.0,
    g: 1.0,
    b: 0.0,
    a: 1.0,
};
