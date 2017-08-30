use cgmath::{Vector1, Vector2, Vector3, Vector4};
use cgmath::{Matrix2, Matrix3, Matrix4};

use std::mem::transmute;

pub trait ToRawMath {
    type Raw: Copy;
    fn to_raw(self) -> Self::Raw;
}

macro_rules! impl_to_raw {
    ($from:ty, $to:ty) => {
        impl ToRawMath for $from {
            type Raw = $to;
            fn to_raw(self) -> Self::Raw {
                unsafe { transmute(self) }
            }
        }
    }
}

impl_to_raw!(Vector1<f32>, [f32; 1]);
impl_to_raw!(Vector2<f32>, [f32; 2]);
impl_to_raw!(Vector3<f32>, [f32; 3]);
impl_to_raw!(Vector4<f32>, [f32; 4]);

impl_to_raw!(Matrix2<f32>, [[f32; 2]; 2]);
impl_to_raw!(Matrix3<f32>, [[f32; 3]; 3]);
impl_to_raw!(Matrix4<f32>, [[f32; 4]; 4]);
