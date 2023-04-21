use std::ops::{Index, IndexMut};
use auto_ops::*;
use crate::vec_lib::mat4::Mat4f;
use crate::vec_lib::vec3::Vec3f;

/// Matrix layout
/// ```
///  +---+---+---+
///  | 0 | 1 | 2 |
///  +---+---+---+
///  | 3 | 4 | 5 |
///  +---+---+---+
///  | 6 | 7 | 8 |
///  +---+---+---+
/// ```
#[derive(Clone, Copy, PartialEq)]
pub struct Mat3f{
    vals: [f32;9]
}

pub static IDENTITY: Mat3f = Mat3f::new([
    1f32, 0f32, 0f32,
    0f32, 1f32, 0f32,
    0f32, 0f32, 1f32,
]);

impl Mat3f{
    pub const fn new(vals: [f32;9]) -> Self{
        Mat3f{vals}
    }

    pub fn to_mat4(&self) -> Mat4f{
        let mut out: Mat4f = Mat4f::new([0.0f32;16]);
        out[0] = self[0];
        out[1] = self[1];
        out[2] = self[2];

        out[4] = self[3];
        out[5] = self[4];
        out[6] = self[5];

        out[8] = self[6];
        out[9] = self[7];
        out[10] = self[8];
        out
    }

    pub fn add_mat3(&self, other: &Self) -> Self{
        Self::new([
            self[0] + other[0],
            self[1] + other[1],
            self[2] + other[2],

            self[3] + other[3],
            self[4] + other[4],
            self[5] + other[5],

            self[6] + other[6],
            self[7] + other[7],
            self[8] + other[8],
        ])
    }

    pub fn add_mat3_mut(&mut self, other: &Self) -> &mut Self{
        self[0] += other[0];
        self[1] += other[1];
        self[2] += other[2];
        self[3] += other[3];
        self[4] += other[4];
        self[5] += other[5];
        self[6] += other[6];
        self[7] += other[7];
        self[8] += other[8];
        self
    }

    pub fn multiply_mat3(&self, other: &Self) -> Self {
        Self::new([
            self[0] * other[0] + self[1] * other[3] + self[2] * other[6],
            self[3] * other[0] + self[4] * other[3] + self[5] * other[6],
            self[6] * other[0] + self[7] * other[3] + self[8] * other[6],

            self[0] * other[1] + self[1] * other[4] + self[2] * other[7],
            self[3] * other[1] + self[4] * other[4] + self[5] * other[7],
            self[6] * other[1] + self[7] * other[4] + self[8] * other[7],

            self[0] * other[2] + self[1] * other[5] + self[2] * other[8],
            self[3] * other[2] + self[4] * other[5] + self[5] * other[8],
            self[6] * other[2] + self[7] * other[5] + self[8] * other[8],
        ])
    }

    pub fn multiply_mat3_mut(&mut self, other: &Self) -> &mut Self {
        let result = self.multiply_mat3(other);
        *self = result;
        self
    }

    pub fn multiply_vec3(&self, vec: &Vec3f) -> Vec3f{
        Vec3f::new(
            vec.x() * self[0] + vec.y() * self[1] + vec.z() * self[2],
            vec.x() * self[3] + vec.y() * self[4] + vec.z() * self[5],
            vec.x() * self[6] + vec.y() * self[7] + vec.z() * self[8],
        )
    }

    pub fn multiply_vec3_mut<'a>(&'a self, vec: &'a mut Vec3f) -> &mut Vec3f{
        let result = self.multiply_vec3(vec);
        *vec = result;
        vec
    }

    pub fn scale(&self, val: f32) -> Self{
        Self::new([
            self[0] * val,
            self[1] * val,
            self[2] * val,
            self[3] * val,
            self[4] * val,
            self[5] * val,
            self[6] * val,
            self[7] * val,
            self[8] * val,
        ])
    }

    pub fn scale_mut(&mut self, val: f32) -> &mut Self{
        self[0] *= val;
        self[1] *= val;
        self[2] *= val;
        self[3] *= val;
        self[4] *= val;
        self[5] *= val;
        self[6] *= val;
        self[7] *= val;
        self[8] *= val;
        self
    }

    pub fn determinant(self) -> f32 {
        -self[2] * self[4] * self[6]
        + self[1] * self[5] * self[6]
        + self[2] * self[3] * self[7]
        - self[0] * self[5] * self[7]
        - self[1] * self[3] * self[8]
        + self[0] * self[4] * self[8]
    }

    pub fn transpose(&self) -> Self{
        Self::new([
            self[0], self[3], self[6],
            self[1], self[4], self[7],
            self[2], self[5], self[8]
        ])
    }

    pub fn transpose_mut(&mut self) -> &mut Self {
        let result = self.transpose();
        *self = result;
        self
    }

    pub fn inverse(&self) -> Self {
        let denom = self[2] * (self[3] * self[7] - self[4] * self[6])
        + self[1] * (self[5] * self[6] - self[3] * self[8])
        + self[0] * (self[4] * self[8] - self[5] * self[7]);

        Self::new([
            self[4] * self[8] - self[5] * self[7],
            self[2] * self[7] - self[1] * self[8],
            self[1] * self[5] - self[2] * self[4],

            self[5] * self[6] - self[3] * self[8],
            self[0] * self[8] - self[2] * self[6],
            self[2] * self[3] - self[0] * self[5],

            self[3] * self[7] - self[4] * self[6],
            self[1] * self[6] - self[0] * self[7],
            self[0] * self[4] - self[1] * self[3],
        ])
    }

    pub fn inverse_mut(&mut self) -> &mut Self {
        let inverted = self.inverse();
        *self = inverted;
        self
    }

    pub fn rotate(&self, axis: &Vec3f, radians: f32) -> Self{
        let mut k = Self::new([
             0.0f32,   -axis.z(), axis.y(),
             axis.z(),  0.0f32,  -axis.x(),
            -axis.y(),  axis.x(), 0.0f32
        ]);
        let mut k2 = k * k;

        let sin = f32::sin(radians);
        let cos = 1.0f32 - f32::cos(radians);

        IDENTITY + k.scale_mut(sin).add_mat3(k2.scale_mut(cos))
    }

    pub fn rotate_mut(&mut self, axis: &Vec3f, radians: f32) -> &mut Self {
        let rotated = self.rotate(axis, radians);
        *self = rotated;
        self
    }
}

impl_op_ex!(+ |a: &Mat3f, b: &Mat3f| -> Mat3f {a.add_mat3(b)});
impl_op_ex!(+= |a: &mut Mat3f, b: &Mat3f| {a.add_mat3_mut(b);});

impl_op_ex!(* |a: &Mat3f, b: &Mat3f| -> Mat3f {a.multiply_mat3(b)});
impl_op_ex!(* |a: &Mat3f, b: &Vec3f| -> Vec3f {a.multiply_vec3(b)});


impl Index<usize> for Mat3f{
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.vals[index]
    }
}

impl IndexMut<usize> for Mat3f{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.vals[index]
    }
}

