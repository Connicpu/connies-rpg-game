#[derive(Copy, Clone)]
pub struct QuadVertex {
    pos: [f32; 2],
    uv: [f32; 2],
}

implement_vertex!(QuadVertex, pos, uv);

pub static QUAD_VERTICES: [QuadVertex; 4] = [
    QuadVertex {
        pos: [0.0, 0.0],
        uv: [0.0, 0.0],
    },
    QuadVertex {
        pos: [1.0, 0.0],
        uv: [1.0, 0.0],
    },
    QuadVertex {
        pos: [0.0, -1.0],
        uv: [0.0, 1.0],
    },
    QuadVertex {
        pos: [1.0, -1.0],
        uv: [1.0, 1.0],
    },
];

pub static QUAD_INDICES: [u32; 6] = [
    0, 2, 1,
    1, 3, 2,
];
