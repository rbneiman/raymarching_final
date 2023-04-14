use std::ops::{Index, IndexMut};
use auto_ops::*;
use crate::vec_lib::vec2::Vec2f;
use crate::vec_lib::vec3::Vec3f;

#[derive(Clone, Copy, PartialEq)]
pub struct Vec4f{
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

pub static ZERO : Vec4f = Vec4f::new(0f32,0f32,0f32,0f32);

impl Vec4f{
    pub const fn new(x:f32, y:f32, z:f32, w:f32) -> Self{
        return Vec4f{x,y,z,w};
    }

    pub fn add(&self, other: &Self) -> Self{
        return Self::new(
        self.x + other.x,
        self.y + other.y,
        self.z + other.z,
        self.w + other.w)
    }

    pub fn add_mut(&mut self, other: &Self) -> &mut Self{
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
        self.w += other.w;
        self
    }

    pub fn sub(&self, other: &Self) -> Self{
        return Self::new(
        self.x - other.x,
        self.y - other.y,
        self.z - other.z,
        self.w - other.w)
    }

    pub fn sub_mut(&mut self, other: &Self) -> &mut Self{
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
        self.w -= other.w;
        self
    }

    pub fn scale(&self, val: f32) -> Self{
        return Self::new(
        self.x * val,
        self.y * val,
        self.z * val,
        self.w * val)
    }

    pub fn scale_mut(&mut self, val: f32) -> &mut Self{
        self.x *= val;
        self.y *= val;
        self.z *= val;
        self.w *= val;
        self
    }

    pub fn negate(&self) -> Self{
        return Self::new(
            -self.x,
            -self.y,
            -self.z,
            -self.w
        )
    }

    pub fn negate_mut(&mut self) -> &mut Self{
        self.x = -self.x;
        self.y = -self.y;
        self.z = -self.z;
        self.w = -self.w;
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
        f32::sqrt(self.x*self.x + self.y*self.y + self.z*self.z + self.w*self.w)
    }

    pub fn squared_length(&self) -> f32{
        self.x*self.x + self.y*self.y + self.z*self.z + self.w*self.w
    }

    pub fn dot(&self, other: &Self) -> f32{
        self.x * other.x + self.y * other.y + self.z * other.z + self.w*other.w
    }

}

impl Index<isize> for Vec4f{
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
            3=>{
                &self.w
            },
            _ =>{
                panic!();
            }
        }
    }
}

impl IndexMut<isize> for Vec4f{

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
            3 =>{
                &mut self.w
            },
            _ =>{
                panic!();
            }
        }
    }
}

impl_op_ex!(+ |a: &Vec4f, b: &Vec4f| -> Vec4f {a.add(b)});
impl_op_ex!(+= |a: &mut Vec4f, b: &Vec4f| {a.add_mut(b);});

impl_op_ex!(- |a: &Vec4f, b: &Vec4f| -> Vec4f {a.sub(b)});
impl_op_ex!(-= |a: &mut Vec4f, b: &Vec4f| {a.sub_mut(b);});
impl_op_ex!(- |a: &Vec4f| -> Vec4f {a.negate()});

impl_op_ex_commutative!(* |a: &Vec4f, b: f32| -> Vec4f {a.scale(b)});
impl_op_ex!(*= |a: &mut Vec4f, b: f32| {a.scale_mut(b);});

impl Vec4f {
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
    pub fn w(&self) -> f32{
        self.w
    }

    #[inline]
    pub fn w_mut(&mut self) -> &mut f32{
        &mut self.w
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

    #[inline]
    pub fn xw(&self) -> Vec2f{
        Vec2f::new(self.x, self.w)
    }

    #[inline]
    pub fn yw(&self) -> Vec2f{
        Vec2f::new(self.y, self.w)
    }

    #[inline]
    pub fn zw(&self) -> Vec2f{
        Vec2f::new(self.z, self.w)
    }

    #[inline]
    pub fn wx(&self) -> Vec2f{
        Vec2f::new(self.w, self.x)
    }

    #[inline]
    pub fn wy(&self) -> Vec2f{
        Vec2f::new(self.w, self.y)
    }

    #[inline]
    pub fn wz(&self) -> Vec2f{
        Vec2f::new(self.w, self.z)
    }

    #[inline]
    pub fn xyz(&self) -> Vec3f{
        Vec3f::new(self.x, self.y, self.z)
    }

    #[inline]
    pub fn xzy(&self) -> Vec3f{
        Vec3f::new(self.x, self.z, self.y)
    }

    #[inline]
    pub fn yxz(&self) -> Vec3f{
        Vec3f::new(self.y, self.x, self.z)
    }

    #[inline]
    pub fn yzx(&self) -> Vec3f{
        Vec3f::new(self.y, self.z, self.x)
    }

    #[inline]
    pub fn zxy(&self) -> Vec3f{
        Vec3f::new(self.z, self.x, self.y)
    }

    #[inline]
    pub fn zyx(&self) -> Vec3f{
        Vec3f::new(self.z, self.y, self.x)
    }
}