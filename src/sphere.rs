use crate::hit_record::HitRecord;
use crate::material::Material;
use crate::vec3::{Scalar, Vec3};
use crate::{hittable::Hittable, ray::Ray};

pub(crate) struct Sphere<T> where T: Material {
    center: Vec3,
    radius: Scalar,
    material: T,
}

impl<T> Sphere<T> where T: Material {
    pub fn new(center: Vec3, material: T, radius: Scalar) -> Self {
        Self {
            center,
            radius,
            material
        }
    }

    #[allow(dead_code)]
    pub fn center(&self) -> Vec3 {
        self.center
    }

    #[allow(dead_code)]
    pub fn radius(&self) -> Scalar {
        self.radius
    }
}

impl<T> Hittable for Sphere<T> where T: Material + Copy + 'static {
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
            return Some(HitRecord::new(&point, &outward_normal, Box::new(self.material), *t, &ray));
        }

        None
    }
}
