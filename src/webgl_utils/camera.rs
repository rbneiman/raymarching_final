use crate::vec_lib::mat4;
use crate::vec_lib::mat4::Mat4f;
use crate::vec_lib::vec3::Vec3f;

pub struct FPSCamera{
    eye: Vec3f,
    forward: Vec3f,
    up: Vec3f,
    right: Vec3f,

    target_dist: f32,
    fov: f32,
    aspect: f32,
    near: f32,
    far: f32,

    forward_initial: Vec3f,
    up_initial: Vec3f,
}


impl FPSCamera{
    pub fn new(pos: Vec3f, target: Vec3f, up_dir:Vec3f, fov: f32, aspect: f32, near: f32, far:f32)
    -> Self{
        let eye = pos;
        let forward = (pos - target).normalize().negate();
        let right = up_dir.cross(&forward).normalize().negate();
        let target_dist = (eye - target).length();
        FPSCamera{
            eye,
            forward,
            up: up_dir.normalize(),
            right,
            target_dist,
            fov,
            aspect,
            near,
            far,

            forward_initial: forward.clone(),
            up_initial: up_dir.normalize(),
        }
    }

    pub fn target(&self) -> Vec3f{
        self.eye + self.forward.scale(self.target_dist)
    }

    pub fn translate(&mut self, vec: &Vec3f){
        self.eye = self.eye + vec;
    }

    pub fn rotate(&mut self, axis: &Vec3f, radians: f32){
        let norm_axis = axis.normalize();
        let rot_mat = mat4::IDENTITY.rotate3d(&norm_axis, radians);

        self.forward = rot_mat.multiply_vec3(&self.forward);
        self.up = rot_mat.multiply_vec3(&self.up);
        self.right = rot_mat.multiply_vec3(&self.right);
    }

    pub fn position(&self) -> Vec3f{
        self.eye
    }

    pub fn forward(&self) -> Vec3f{
        self.forward
    }

    pub fn right(&self) -> Vec3f{
        self.right
    }

    pub fn up(&self) -> Vec3f{
        self.up
    }

    pub fn up_initial(&self) -> Vec3f{
        self.up_initial
    }

    pub fn view_matrix(&self) -> Mat4f{
        Mat4f::look_at(&self.eye,
        &self.target(),
            &self.up
        )
    }

    pub fn proj_matrix(&self) -> Mat4f{
        Mat4f::perspective(
            self.fov,
            self.aspect,
            self.near,
            self.far
        )
    }

}