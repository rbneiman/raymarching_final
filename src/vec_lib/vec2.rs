use std::ops::{Index, IndexMut};
use auto_ops::*;

#[derive(Clone, Copy, PartialEq)]
pub struct Vec2f{
    x: f32,
    y: f32,
}

pub static ZERO : Vec2f = Vec2f::new(0f32,0f32);

impl Vec2f{
    pub const fn new(x:f32, y:f32) -> Self{
        return Vec2f{x,y};
    }

    pub fn add_mut(&mut self, other: &Self) -> &mut Self{
        self.x += other.x;
        self.y += other.y;
        self
    }

    pub fn sub(&self, other: &Self) -> Self{
        return Self::new(
        self.x - other.x,
        self.y - other.y)
    }

    pub fn sub_mut(&mut self, other: &Self) -> &mut Self{
        self.x -= other.x;
        self.y -= other.y;
        self
    }

    pub fn scale(&self, val: f32) -> Self{
        return Self::new(
        self.x * val,
        self.y * val)
    }

    pub fn scale_mut(&mut self, val: f32) -> &mut Self{
        self.x *= val;
        self.y *= val;
        self
    }

    pub fn negate(&self) -> Self{
        return Self::new(
            -self.x,
            -self.y
        )
    }

    pub fn negate_mut(&mut self) -> &mut Self{
        self.x = -self.x;
        self.y = -self.y;
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
        f32::sqrt(self.x*self.x + self.y*self.y)
    }

    pub fn squared_length(&self) -> f32{
        self.x*self.x + self.y*self.y
    }

    pub fn dot(&self, other: &Self) -> f32{
        self.x * other.x + self.y * other.y
    }

}

impl Index<isize> for Vec2f{
    type Output = f32;

    fn index(&self, index: isize) -> &Self::Output {
        match index {
            0 =>{
                &self.x
            },
            1 =>{
                &self.y
            },
            _ =>{
                panic!();
            }
        }
    }
}

impl IndexMut<isize> for Vec2f{

    fn index_mut(&mut self, index: isize) -> &mut Self::Output {
        match index {
            0 =>{
                &mut self.x
            },
            1 =>{
                &mut self.y
            },
            _ =>{
                panic!();
            }
        }
    }
}

impl_op_ex!(+ |a: &Vec2f, b: &Vec2f| -> Vec2f {a.add(b)});
impl_op_ex!(+= |a: &mut Vec2f, b: &Vec2f| {a.add_mut(b);});

impl_op_ex!(- |a: &Vec2f, b: &Vec2f| -> Vec2f {a.sub(b)});
impl_op_ex!(-= |a: &mut Vec2f, b: &Vec2f| {a.sub_mut(b);});
impl_op_ex!(- |a: &Vec2f| -> Vec2f {a.negate()});

impl_op_ex_commutative!(* |a: &Vec2f, b: f32| -> Vec2f {a.scale(b)});
impl_op_ex!(*= |a: &mut Vec2f, b: f32| {a.scale_mut(b);});

impl Vec2f{
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
}