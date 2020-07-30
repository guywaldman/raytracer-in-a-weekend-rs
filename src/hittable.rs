use crate::{
    ray::Ray,
    vec3::{Scalar, Vec3},
};

#[derive(Debug)]
pub(crate) struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: Scalar,
}

pub(crate) trait Hittable {
    /// Calculates intersection with the received ray,
    /// and returns the hit record if there was a hit.
    fn hit(&self, ray: &Ray, t_min: Scalar, t_max: Scalar) -> Option<HitRecord>;
}
