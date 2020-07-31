use crate::vec3::{Scalar, Vec3};
use crate::{
    hittable::{HitRecord, Hittable},
    ray::Ray,
};

pub(crate) struct Sphere {
    center: Vec3,
    radius: Scalar,
}

impl Sphere {
    pub fn new(center: Vec3, radius: Scalar) -> Self {
        Self { center, radius }
    }

    pub fn center(&self) -> Vec3 {
        self.center
    }

    #[allow(dead_code)]
    pub fn radius(&self) -> Scalar {
        self.radius
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: Scalar, t_max: Scalar) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;
        let a = ray.dir().length_squared();
        let half_b = oc.dot(&ray.dir());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            // No hit
            return None;
        }

        let root = discriminant.sqrt();

        let possible_params = vec![(-half_b - root) / a, (-half_b + root) / a];
        for t in possible_params.iter().find(|t| **t > t_min && **t < t_max) {
            let point = ray.at(*t);
            let outward_normal = (point - self.center) / self.radius;
            return Some(HitRecord::new(
                &point,
                &outward_normal,
                *t,
                &ray
            ));
        }

        None
    }
}
