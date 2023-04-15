use std::f32::consts::PI;
use std::ops::{Index, IndexMut};
use auto_ops::*;
use crate::vec_lib::mat3;
use crate::vec_lib::mat3::Mat3f;
use crate::vec_lib::vec3::Vec3f;
use crate::vec_lib::vec4::Vec4f;

/// Matrix layout
/// ```
///  +----+----+----+----+
///  | 0  | 1  | 2  | 3  |
///  +----+----+----+----+
///  | 4  | 5  | 6  | 7  |
///  +----+----+----+----+
///  | 8  | 9  | 10 | 11 |
///  +----+----+----+----+
///  | 12 | 13 | 14 | 15 |
///  +----+----+----+----+
/// ```
#[derive(Clone, Copy, PartialEq)]
pub struct Mat4f{
    vals: [f32;16]
}

pub static IDENTITY: Mat4f = Mat4f::new([
    1f32, 0f32, 0f32, 0f32,
    0f32, 1f32, 0f32, 0f32,
    0f32, 0f32, 1f32, 0f32,
    0f32, 0f32, 0f32, 1f32,
]);

impl Mat4f {
    pub const fn new(vals: [f32; 16]) -> Self {
        Mat4f { vals }
    }

    pub fn frustum(
        left:   f32,
        right:  f32,
        bottom: f32,
        top:    f32,
        near:   f32,
        far:    f32
    ) -> Self{
        let diff_rl = right - left;
        let diff_tb = top - bottom;
        let diff_fn = far - near;

        Self::new([
            (near * 2.0f32) / diff_rl,
            0.0f32,
            (left + right) / diff_rl,
            0.0f32,

            0.0f32,
            (near * 2.0f32) / diff_tb,
            (top + bottom) / diff_tb,
            0.0f32,

            0.0f32,
            0.0f32,
            -(far + near) / diff_fn,
            -(far * near * 2.0f32) / diff_fn,

            0.0f32,
            0.0f32,
            -1.0f32,
            0.0f32,
        ])
    }

   /// Computes the perspective matrix from the given inputs.
   ///  # Arguments
   ///
   /// * 'fov' - The field of view in degrees
   /// * 'aspect' - The aspect ratio (width/height)
   /// * 'near' - Distance to the near clipping plane
   /// * 'far' - Distance to the far clipping plane
   ///
    pub fn perspective(
        fov:    f32,
        aspect: f32,
        near:   f32,
        far:    f32
    )-> Self{
        let top = near * f32::tan((fov * PI) / 360.0f32);
        let right = top * aspect;

        Self::frustum(-right, right, -top, top, near, far)
    }

    pub fn orthographic(
        left:   f32,
        right:  f32,
        bottom: f32,
        top:    f32,
        near:   f32,
        far:    f32
    ) -> Self{
        let diff_rl = right - left;
        let diff_tb = top - bottom;
        let diff_fn = far - near;

        Self::new([
            2.0f32 / diff_rl,
            0.0f32,
            0.0f32,
            -(left + right) / diff_rl,

            0.0f32,
            2.0f32 / diff_tb,
            0.0f32,
            -(top + bottom) / diff_tb,

            0.0f32,
            0.0f32,
            -2.0f32 / diff_fn,
            -(far + near) / diff_fn,

            0.0f32,
            0.0f32,
            0.0f32,
            1.0f32
        ])
    }

    pub fn look_at(position: &Vec3f, target: &Vec3f, up: &Vec3f) -> Self{
        if position == target {
            let mut out = Self::new([0.0f32;16]);
            out.vals[0] = 1.0f32;
            out.vals[5] = 1.0f32;
            out.vals[10] = 1.0f32;
            out.vals[15] = 1.0f32;
            return out;
        }

        let z = position.sub(target).normalize();
        let x = up.cross(&z).normalize();
        let y = z.cross(&x).normalize();

        Self::new([
            x.x(),
            x.y(),
            x.z(),
            -x.dot(position),

            y.x(),
            y.y(),
            y.z(),
            -y.dot(position),

            z.x(),
            z.y(),
            z.z(),
            -z.dot(position),

            0.0f32,
            0.0f32,
            0.0f32,
            1.0f32,
        ])
    }

    pub fn add_mat4(&self, other: &Self) -> Self{
        let mut new_vals : [f32;16] = [0.0f32;16];
        for i in 0..16{
           new_vals[i] = self[i] + other[i];
        }
        Self::new(new_vals)

    }

    pub fn add_mat4_mut(&mut self, other: &Self) -> &mut Self{
        for i in 0..16{
           self[i] += other[i];
        }
        self
    }

    pub fn multiply_mat4(&self, other: &Self) -> Self {
        Self::new([
        other[0] * self[0] + other[4] * self[1] + other[8] * self[2] + other[12] * self[3],
        other[1] * self[0] + other[5] * self[1] + other[9] * self[2] + other[13] * self[3],
        other[2] * self[0] + other[6] * self[1] + other[10] * self[2] + other[14] * self[3],
        other[3] * self[0] + other[7] * self[1] + other[11] * self[2] + other[15] * self[3],

        other[0] * self[4] + other[4] * self[5] + other[8] * self[6] + other[12] * self[7],
        other[1] * self[4] + other[5] * self[5] + other[9] * self[6] + other[13] * self[7],
        other[2] * self[4] + other[6] * self[5] + other[10] * self[6] + other[14] * self[7],
        other[3] * self[4] + other[7] * self[5] + other[11] * self[6] + other[15] * self[7],

        other[0] * self[8] + other[4] * self[9] + other[8] * self[10] + other[12] * self[11],
        other[1] * self[8] + other[5] * self[9] + other[9] * self[10] + other[13] * self[11],
        other[2] * self[8] + other[6] * self[9] + other[10] * self[10] + other[14] * self[11],
        other[3] * self[8] + other[7] * self[9] + other[11] * self[10] + other[15] * self[11],

        other[0] * self[12] + other[4] * self[13] + other[8] * self[14] + other[12] * self[15],
        other[1] * self[12] + other[5] * self[13] + other[9] * self[14] + other[13] * self[15],
        other[2] * self[12] + other[6] * self[13] + other[10] * self[14] + other[14] * self[15],
        other[3] * self[12] + other[7] * self[13] + other[11] * self[14] + other[15] * self[15],
        ])
    }

    pub fn multiply_mat4_mut(&mut self, other: &Self) -> &mut Self {
        let result = self.multiply_mat4(other);
        *self = result;
        self
    }

    pub fn multiply_vec4(&self, vec: &Vec4f) -> Vec4f{
        Vec4f::new(
        self[3] * vec.w() + self[0] * vec.x() + self[1] * vec.y() + self[2] * vec.z(),
        self[7] * vec.w() + self[4] * vec.x() + self[5] * vec.y() + self[6] * vec.z(),
        self[11] * vec.w() + self[8] * vec.x() + self[9] * vec.y() + self[10] * vec.z(),
        self[15] * vec.w() + self[12] * vec.x() + self[13] * vec.y() + self[14] * vec.z()
        )
    }

    pub fn multiply_vec4_mut<'a>(&'a self, vec: &'a mut Vec4f) -> &mut Vec4f{
        let result = self.multiply_vec4(vec);
        *vec = result;
        vec
    }

    pub fn scale(&self, val: f32) -> Self{
        let mut new_vals: [f32;16] = [0.0f32;16];
        for i in 0..16{
           new_vals[i] = self[i] * val;
        }
        Self::new(new_vals)
    }

    pub fn scale_mut(&mut self, val: f32) -> &mut Self{
        for i in 0..16{
           self[i] *= val;
        }
        self
    }

    pub fn determinant(self) -> f32 {
        let s0 = self[0] * self[5] - self[4] * self[1];
        let s1 = self[0] * self[6] - self[4] * self[2];
        let s2 = self[0] * self[7] - self[4] * self[3];
        let s3 = self[1] * self[6] - self[5] * self[2];
        let s4 = self[1] * self[7] - self[5] * self[3];
        let s5 = self[2] * self[7] - self[6] * self[3];

        let c5 = self[10] * self[15] - self[14] * self[11];
        let c4 = self[9] * self[15] - self[13] * self[11];
        let c3 = self[9] * self[14] - self[13] * self[10];
        let c2 = self[8] * self[15] - self[12] * self[11];
        let c1 = self[8] * self[14] - self[12] * self[10];
        let c0 = self[8] * self[13] - self[12] * self[9];

        s0 * c5 - s1 * c4 + s2 * c3 + s3 * c2 - s4 * c1 + s5 * c0
    }

    pub fn transpose(&self) -> Self{
        Self::new([
            self[0], self[4], self[8],  self[12],
            self[1], self[5], self[9],  self[13],
            self[2], self[6], self[10], self[14],
            self[3], self[7], self[11], self[15]
        ])
    }

    pub fn transpose_mut(&mut self) -> &mut Self {
        let result = self.transpose();
        *self = result;
        self
    }

    // https://stackoverflow.com/questions/2624422/efficient-4x4-matrix-inverse-affine-transform
    pub fn inverse(&self) -> Self {
        let s0 = self[0] * self[5] - self[4] * self[1];
        let s1 = self[0] * self[6] - self[4] * self[2];
        let s2 = self[0] * self[7] - self[4] * self[3];
        let s3 = self[1] * self[6] - self[5] * self[2];
        let s4 = self[1] * self[7] - self[5] * self[3];
        let s5 = self[2] * self[7] - self[6] * self[3];

        let c5 = self[10] * self[15] - self[14] * self[11];
        let c4 = self[9] * self[15] - self[13] * self[11];
        let c3 = self[9] * self[14] - self[13] * self[10];
        let c2 = self[8] * self[15] - self[12] * self[11];
        let c1 = self[8] * self[14] - self[12] * self[10];
        let c0 = self[8] * self[13] - self[12] * self[9];

        // Should check for 0 determinant
        let invdet = 1.0f32 / (s0 * c5 - s1 * c4 + s2 * c3 + s3 * c2 - s4 * c1 + s5 * c0);

        let mut b = IDENTITY;

        b[0] = ( self[5] * c5 - self[6] * c4 + self[7] * c3) * invdet;
        b[1] = (-self[1] * c5 + self[2] * c4 - self[3] * c3) * invdet;
        b[2] = ( self[13] * s5 - self[14] * s4 + self[15] * s3) * invdet;
        b[3] = (-self[9] * s5 + self[10] * s4 - self[11] * s3) * invdet;

        b[4] = (-self[4] * c5 + self[6] * c2 - self[7] * c1) * invdet;
        b[5] = ( self[0] * c5 - self[2] * c2 + self[3] * c1) * invdet;
        b[6] = (-self[12] * s5 + self[14] * s2 - self[15] * s1) * invdet;
        b[7] = ( self[8] * s5 - self[10] * s2 + self[11] * s1) * invdet;

        b[8] = ( self[4] * c4 - self[5] * c2 + self[7] * c0) * invdet;
        b[9] = (-self[0] * c4 + self[1] * c2 - self[3] * c0) * invdet;
        b[10] = ( self[12] * s4 - self[13] * s2 + self[15] * s0) * invdet;
        b[11] = (-self[8] * s4 + self[9] * s2 - self[11] * s0) * invdet;

        b[12] = (-self[4] * c3 + self[5] * c1 - self[6] * c0) * invdet;
        b[13] = ( self[0] * c3 - self[1] * c1 + self[2] * c0) * invdet;
        b[14] = (-self[12] * s3 + self[13] * s1 - self[14] * s0) * invdet;
        b[15] = ( self[8] * s3 - self[9] * s1 + self[10] * s0) * invdet;

        return b;
    }

    pub fn inverse_mut(&mut self) -> &mut Self {
        let inverted = self.inverse();
        *self = inverted;
        self
    }

    pub fn translate(&self, vec: &Vec3f) -> Self{
        let mut out = *self;
        out[3] = self[0] * vec.x() + self[1] * vec.y() + self[2] * vec.z() + self[3];
        out[7] = self[4] * vec.x() + self[5] * vec.y() + self[6] * vec.z() + self[7];
        out[11] = self[8] * vec.x() + self[9] * vec.y() + self[10] * vec.z() + self[11];
        out[15] = self[12] * vec.x() + self[13] * vec.y() + self[14] * vec.z() + self[15];
        out
    }

    pub fn translate_mut(&mut self, vec: &Vec3f) -> &mut Self{
        self[3] = self[0] * vec.x() + self[1] * vec.y() + self[2] * vec.z() + self[3];
        self[7] = self[4] * vec.x() + self[5] * vec.y() + self[6] * vec.z() + self[7];
        self[11] = self[8] * vec.x() + self[9] * vec.y() + self[10] * vec.z() + self[11];
        self[15] = self[12] * vec.x() + self[13] * vec.y() + self[14] * vec.z() + self[15];
        self
    }

    pub fn rotate3d(&self, axis: &Vec3f, radians: f32) -> Self{
        let mut rot_mat = mat3::IDENTITY.rotate(axis, radians).to_mat4();
        rot_mat[15] = 1.0f32;
        self.multiply_mat4(&rot_mat)
    }

    pub fn rotate3d_mut(&mut self, axis: &Vec3f, radians: f32) -> &mut Self {
        let rotated = self.rotate3d(axis, radians);
        *self = rotated;
        self
    }

}

impl Index<usize> for Mat4f{
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.vals[index]
    }
}

impl IndexMut<usize> for Mat4f{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.vals[index]
    }
}
