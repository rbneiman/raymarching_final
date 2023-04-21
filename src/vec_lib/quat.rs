use std::ops::{Index, IndexMut};
use crate::vec_lib::vec3::Vec3f;

pub struct Quat{
    vals: [f32;4]
}

pub const IDENTITY : Quat = Quat::new([0.0, 0.0, 0.0, 1.0]);

impl Quat{
    pub const fn new(vals: [f32;4]) -> Self{
        Quat{vals}
    }

    pub fn from_axis_angle(axis: &Vec3f, radians: f32) -> Self{
        let sin = f32::sin(radians * 0.5);

        Self::new([
            axis.x() * sin,
            axis.y() * sin,
            axis.z() * sin,
            f32::cos(radians * 0.5)
        ])
    }

    pub fn add(&self, other: &Self) -> Self{
        Self::new([
            self[0] + other[0],
            self[1] + other[1],
            self[2] + other[2],
            self[3] + other[3],
        ])
    }

    pub fn mult(&self, other: &Self) -> Self{
        Self::new([
            self[0] * other[3] + self[3] * other[0] + self[1] * self[2] - self[2] * other[1],
            self[1] * other[3] + self[3] * other[1] + self[2] * self[0] - self[0] * other[2],
            self[2] * other[3] + self[3] * other[2] + self[0] * self[1] - self[1] * other[0],
            self[3] * other[3] + self[0] * other[0] + self[1] * self[1] - self[2] * other[2],
        ])
    }

    pub fn dot(&self, other: &Self) -> f32{
        self[0] * other[0]
        + self[1] * other[1]
        + self[2] * other[2]
        + self[3] * other[3]
    }

    pub fn inverse(&self) -> Self{
        let dot = self.dot(&self);

        if dot == 0.0 {
            return Self::new([0.0, 0.0, 0.0, 0.0]);
        }

        let dot_inv = -1.0 / dot;

        Self::new([
            self[0] * dot_inv,
            self[1] * dot_inv,
            self[2] * dot_inv,
            self[3] * dot_inv,
        ])
    }

    pub fn conjugate(&self) -> Self{
        Self::new([
            -self[0],
            -self[1],
            -self[2],
            self[3],
        ])
    }

    pub fn normalize(&self) -> Self{
        let length = f32::sqrt(self.dot(&self));
        if length == 0.0 {
            return Self::new([0.0, 0.0, 0.0, 0.0]);
        }
        let inv_len = 1.0 / length;

        Self::new([
            self[0] * inv_len,
            self[1] * inv_len,
            self[2] * inv_len,
            self[3] * inv_len,
        ])
    }

    pub fn mult_vec3(&self, vec: &Vec3f) -> Vec3f{
        todo!()
    }



    pub fn slerp(&self, other: &Self, time: f32) -> Self{
        todo!()
    }

}


impl Index<usize> for Quat{
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.vals[index]
    }
}

impl IndexMut<usize> for Quat{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.vals[index]
    }
}
