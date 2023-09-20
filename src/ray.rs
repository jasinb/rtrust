use crate::vec3::Vec3;

pub struct Ray {
    pub orig : Vec3,
    pub dir : Vec3,
}

impl Ray {
    pub fn at(&self, t: f32) -> Vec3 {
        self.orig + t * self.dir
    }
}