use std::ops::{Index, IndexMut};
use auto_ops::*;
use crate::vec_lib::vec2::Vec2f;
use crate::vec_lib::vec4::Vec4f;

#[derive(Clone, Copy, PartialEq)]
pub struct Vec3f{
    x: f32,
    y: f32,
    z: f32,
}

pub static ZERO : Vec3f = Vec3f::new(0f32,0f32,0f32);

impl Vec3f{
    pub const fn new(x:f32, y:f32, z:f32) -> Self{
        return Vec3f{x,y,z};
    }

    pub fn to_vec4(&self, w:f32) -> Vec4f{
        Vec4f::new(self.x, self.y, self.z, w)
    }

    pub fn add(&self, other: &Self) -> Self{
        return Self::new(
        self.x + other.x,
        self.y + other.y,
        self.z + other.z)
    }

    pub fn add_mut(&mut self, other: &Self) -> &mut Self{
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
        self
    }

    pub fn sub(&self, other: &Self) -> Self{
        return Self::new(
        self.x - other.x,
        self.y - other.y,
        self.z - other.z)
    }

    pub fn sub_mut(&mut self, other: &Self) -> &mut Self{
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
        self
    }

    pub fn scale(&self, val: f32) -> Self{
        return Self::new(
        self.x * val,
        self.y * val,
        self.z * val)
    }

    pub fn scale_mut(&mut self, val: f32) -> &mut Self{
        self.x *= val;
        self.y *= val;
        self.z *= val;
        self
    }

    pub fn negate(&self) -> Self{
        return Self::new(
            -self.x,
            -self.y,
            -self.z
        )
    }

    pub fn negate_mut(&mut self) -> &mut Self{
        self.x = -self.x;
        self.y = -self.y;
        self.z = -self.z;
        self
    }

    pub fn normalize(&self) -> Self{
        let length = self.length();
        return self.scale(1.0f32 / length);
    }

    pub fn normalize_mut(&mut self) -> &mut Self{
        let length = self.length();
        self.scale_mut(1.0f32 / length)
    }

    pub fn length(&self) -> f32{
        f32::sqrt(self.x*self.x + self.y*self.y + self.z*self.z)
    }

    pub fn squared_length(&self) -> f32{
        self.x*self.x + self.y*self.y + self.z*self.z
    }

    pub fn dot(&self, other: &Self) -> f32{
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Self) -> Self{
        Self::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x
        )
    }

    pub fn cross_mut(&mut self, other: &Self) -> &mut Self{
        let res = self.cross(other);
        *self = res;
        self
    }
}

impl Index<isize> for Vec3f{
    type Output = f32;

    fn index(&self, index: isize) -> &Self::Output {
        match index {
            0 =>{
                &self.x
            },
            1 =>{
                &self.y
            },
            2=>{
                &self.z
            },
            _ =>{
                panic!();
            }
        }
    }
}

impl IndexMut<isize> for Vec3f{

    fn index_mut(&mut self, index: isize) -> &mut Self::Output {
        match index {
            0 =>{
                &mut self.x
            },
            1 =>{
                &mut self.y
            },
            2 =>{
                &mut self.z
            },
            _ =>{
                panic!();
            }
        }
    }
}

impl_op_ex!(+ |a: &Vec3f, b: &Vec3f| -> Vec3f {a.add(b)});
impl_op_ex!(+= |a: &mut Vec3f, b: &Vec3f| {a.add_mut(b);});

impl_op_ex!(- |a: &Vec3f, b: &Vec3f| -> Vec3f {a.sub(b)});
impl_op_ex!(-= |a: &mut Vec3f, b: &Vec3f| {a.sub_mut(b);});
impl_op_ex!(- |a: &Vec3f| -> Vec3f {a.negate()});

impl_op_ex_commutative!(* |a: &Vec3f, b: f32| -> Vec3f {a.scale(b)});
impl_op_ex!(*= |a: &mut Vec3f, b: f32| {a.scale_mut(b);});

impl Vec3f{
    #[inline]
    pub fn x(&self) -> f32{
        self.x
    }

    #[inline]
    pub fn x_mut(&mut self) -> &mut f32{
        &mut self.x
    }

    #[inline]
    pub fn y(&self) -> f32{
        self.y
    }

    #[inline]
    pub fn y_mut(&mut self) -> &mut f32{
        &mut self.y
    }

    #[inline]
    pub fn z(&self) -> f32{
        self.z
    }

    #[inline]
    pub fn z_mut(&mut self) -> &mut f32{
        &mut self.z
    }

    #[inline]
    pub fn xy(&self) -> Vec2f{
        Vec2f::new(self.x, self.y)
    }

    #[inline]
    pub fn yx(&self) -> Vec2f{
        Vec2f::new(self.y, self.x)
    }

    #[inline]
    pub fn xz(&self) -> Vec2f{
        Vec2f::new(self.x, self.z)
    }

    #[inline]
    pub fn zx(&self) -> Vec2f{
        Vec2f::new(self.z, self.x)
    }

    #[inline]
    pub fn yz(&self) -> Vec2f{
        Vec2f::new(self.y, self.z)
    }

    #[inline]
    pub fn zy(&self) -> Vec2f{
        Vec2f::new(self.z, self.y)
    }
}
