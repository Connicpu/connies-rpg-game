use cgmath::Vector2;
use cgmath::Matrix4;

pub fn view(pos: Vector2<f32>, size: f32, rot: f32) -> Matrix4<f32> {
    let cos = rot.cos();
    let sin = rot.sin();
    let Vector2 { x, y } = pos;

    let m00 = cos / size;
    let m01 = -sin / size;
    let m10 = sin / size;
    let m11 = m00;

    let m03 = y * m10 - x * m00;
    let m13 = x * m01 - y * m00;

    [
        [m00, m01, 0.0, m03],
        [m10, m11, 0.0, m13],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ].into()
}

pub fn ortho(aspect: f32, near: f32, far: f32) -> Matrix4<f32> {
    let n2f = near - far;
    let toa = 2.0 / aspect;

    [
        [toa, 0.0, 0.0, 0.0],
        [0.0, 2.0, 0.0, 0.0],
        [0.0, 0.0, n2f, far],
        [0.0, 0.0, 0.0, 1.0],
    ].into()
}
