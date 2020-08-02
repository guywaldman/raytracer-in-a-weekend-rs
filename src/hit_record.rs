use crate::material::Material;
use crate::{
    ray::Ray,
    vec3::{Scalar, Vec3},
};

pub(crate) struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub material: Box<dyn Material>,
    pub t: Scalar,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(point: &Vec3, outward_normal: &Vec3, material: Box<dyn Material>, t: Scalar, ray: &Ray) -> Self {
        let front_face = ray.dir().dot(outward_normal) < 0.0;
        let normal = if front_face { *outward_normal } else { (-1.0) * *outward_normal };
        Self {
            point: *point,
            normal,
            t,
            material,
            front_face,
        }
    }
}
