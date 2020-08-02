use crate::hit_record::HitRecord;
use crate::ray::Ray;
use crate::vec3::Scalar;

pub(crate) trait Hittable {
    /// Calculates intersection with the received ray,
    /// and returns the hit record if there was a hit.
    fn hit(&self, ray: &Ray, t_min: Scalar, t_max: Scalar) -> Option<HitRecord>;
}
