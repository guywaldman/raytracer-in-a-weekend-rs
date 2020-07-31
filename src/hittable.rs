use crate::{
    ray::Ray,
    vec3::{Scalar, Vec3},
};

#[derive(Debug)]
pub(crate) struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: Scalar,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(point: &Vec3, outward_normal: &Vec3, t: Scalar, ray: &Ray) -> Self {
        let front_face = ray.dir().dot(outward_normal) < 0.0;
        let normal = if front_face {
            *outward_normal
        } else {
            (-1.0) * *outward_normal
        };
        Self {
            point: *point,
            normal,
            t,
            front_face,
        }
    }
}

pub(crate) trait Hittable {
    /// Calculates intersection with the received ray,
    /// and returns the hit record if there was a hit.
    fn hit(&self, ray: &Ray, t_min: Scalar, t_max: Scalar) -> Option<HitRecord>;
}
