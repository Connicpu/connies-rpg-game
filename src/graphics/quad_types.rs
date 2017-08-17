#[derive(Copy, Clone)]
pub struct QuadVertex {
    pub pos: [f32; 2],
    pub uv: [f32; 2],
}

implement_vertex!(QuadVertex, pos, uv);

#[derive(Copy, Clone)]
pub struct SpriteInstance {
    pub center: [f32; 2],
    pub scale: [f32; 2],
    pub rot: [[f32; 2]; 2],
    pub uv_rect: [f32; 4],
    pub world_pos: [f32; 3],
}

implement_vertex!(SpriteInstance, center, scale, rot, uv_rect, world_pos);

#[derive(Copy, Clone)]
pub struct Camera {
    pub view: [[f32; 3]; 3],
    pub proj: [[f32; 4]; 4],
}

pub static QUAD_VERTICES: [QuadVertex; 4] = [
    QuadVertex {
        pos: [0.0, 0.0],
        uv: [0.0, 1.0],
    },
    QuadVertex {
        pos: [1.0, 0.0],
        uv: [1.0, 1.0],
    },
    QuadVertex {
        pos: [0.0, 1.0],
        uv: [0.0, 0.0],
    },
    QuadVertex {
        pos: [1.0, 1.0],
        uv: [1.0, 0.0],
    },
];

pub static QUAD_INDICES: [u32; 6] = [
    0, 2, 1,
    1, 3, 2,
];
