use std::ops::{Index, IndexMut};
use auto_ops::*;
use crate::vec_lib::vec2::Vec2f;

/// Matrix layout
/// ```
///  +-------+
///  | 0 | 1 |
///  +-------+
///  | 2 | 3 |
///  +-------+
/// ```
#[derive(Clone, Copy, PartialEq)]
pub struct Mat2f{
    vals: [f32;4]
}

pub static IDENTITY: Mat2f = Mat2f::new([1f32, 0f32, 0f32, 1f32]);

enum Mat2Pos{
    POS00 = 0,
    POS01 = 1,
    POS10 = 2,
    POS11 = 3,
}

impl Mat2f {
    pub const fn new(vals: [f32; 4]) -> Self {
        Mat2f { vals }
    }

    pub fn add_mat2(&self, other: &Self) -> Self{
        Self::new([
            self[0] + other[0],
            self[1] + other[1],
            self[2] + other[2],
            self[3] + other[3],
        ])
    }

    pub fn add_mat2_mut(&mut self, other: &Self) -> &mut Self{
        self[0] += other[0];
        self[1] += other[1];
        self[2] += other[2];
        self[3] += other[3];
        self
    }

    pub fn multiply_mat2(&self, other: &Self) -> Self {
        Self::new([
            self[0] * other[0] + self[1] * other[2],
            self[0] * other[1] + self[1] * other[3],
            self[2] * other[0] + self[3] * other[2],
            self[2] * other[1] + self[3] * other[3]
        ])
    }

    pub fn multiply_mat2_mut(&mut self, other: &Self) -> &mut Self {
        let result = self.multiply_mat2(other);
        *self = result;
        self
    }

    pub fn multiply_vec2(&self, vec: &Vec2f) -> Vec2f{
        Vec2f::new(
            vec.x() * self[0] + vec.y() * self[1],
            vec.x() * self[2] + vec.y() * self[3],
        )
    }

    pub fn multiply_vec2_mut<'a>(&'a self, vec: &'a mut Vec2f) -> &mut Vec2f{
        let result = self.multiply_vec2(vec);
        *vec = result;
        vec
    }

    pub fn scale(&self, val: f32) -> Self{
        Self::new([
            self[0] * val,
            self[1] * val,
            self[2] * val,
            self[3] * val,
        ])
    }

    pub fn scale_mut(&mut self, val: f32) -> &mut Self{
        self[0] *= val;
        self[1] *= val;
        self[2] *= val;
        self[3] *= val;
        self
    }

    pub fn determinant(&self) -> f32 {
        self[3] * self[0] - self[1] * self[2]
    }

    pub fn transpose(&self) -> Self {
        Self::new([
            self[0],
            self[2],
            self[1],
            self[3]
        ])
    }

    pub fn transpose_mut(&mut self) -> &mut Self {
        let val = self[1];
        self[1] = self[2];
        self[2] = val;
        self
    }

    pub fn inverse(&self) -> Self {
        let denom = self[0] * self[2] - self[3] * self[0];
        Self::new([
            -self[3] / denom,
            self[1] / denom,
            self[2] / denom,
            -self[0] / denom
        ])
    }

    pub fn inverse_mut(&mut self) -> &mut Self {
        let inverted = self.inverse();
        *self = inverted;
        self
    }

    pub fn rotate(&self, radians: f32) -> Self {
        let sin = f32::sin(radians);
        let cos = f32::cos(radians);
        Self::new([
            self[0] * cos + self[1] * sin,
            self[2] * cos + self[3] * sin,
            self[0] * -sin + self[2] * cos,
            self[2] * -sin + self[3] * cos
        ])
    }

    pub fn rotate_mut(&mut self, radians: f32) -> &mut Self {
        let rotated = self.rotate(radians);
        *self = rotated;
        self
    }
}

impl_op_ex!(+ |a: &Mat2f, b: &Mat2f| -> Mat2f {a.add_mat2(b)});
impl_op_ex!(+= |a: &mut Mat2f, b: &Mat2f| {a.add_mat2_mut(b);});

impl_op_ex!(* |a: &Mat2f, b: &Mat2f| -> Mat2f {a.multiply_mat2(b)});
impl_op_ex!(* |a: &Mat2f, b: &Vec2f| -> Vec2f {a.multiply_vec2(b)});
// no *= b/c matrix multiplication not commutative

impl Index<usize> for Mat2f{
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.vals[index]
    }
}

impl IndexMut<usize> for Mat2f{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.vals[index]
    }
}

impl Mat2f{

}

