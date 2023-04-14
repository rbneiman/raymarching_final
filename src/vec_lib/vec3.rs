use js_sys::slice;

#[derive(Clone, Copy)]
pub struct Vec3f<>{
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3f{
    pub fn new(x:f32, y:f32, z:f32) -> Self{
        return Vec3f{x,y,z};
    }

    #[inline]
    pub fn x(&self) -> f32{
        return self.x;
    }

    #[inline]
    pub fn x_mut(&mut self) -> &mut f32{
        return &mut self.x;
    }

    #[inline]
    pub fn y(&self) -> f32{
        return self.y;
    }

    #[inline]
    pub fn y_mut(&mut self) -> &mut f32{
        return &mut self.y;
    }

    #[inline]
    pub fn z(&self) -> f32{
        return self.z;
    }

    #[inline]
    pub fn z_mut(&mut self) -> &mut f32{
        return &mut self.z;
    }

    pub fn add(&self, other: &Vec3f) -> Vec3f{
        return Vec3f::new(
        self.x + other.x,
        self.y + other.y,
        self.z + other.z)
    }

    pub fn add_mut(&mut self, other: &Vec3f) -> &mut Self{
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
        self
    }

    pub fn sub(&self, other: &Vec3f) -> Vec3f{
        return Vec3f::new(
        self.x - other.x,
        self.y - other.y,
        self.z - other.z)
    }

    pub fn sub_mut(&mut self, other: &Vec3f) -> &mut Self{
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
        self
    }

    pub fn scale(&self, val: f32) -> Vec3f{
        return Vec3f::new(
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

    pub fn dot(&self, other: &Vec3f) -> f32{
        self.x * other.x + self.y * other.y + self.z * other.z
    }

}