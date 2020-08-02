use crate::hit_record::HitRecord;
use crate::{ray::Ray, vec3::{Scalar, Vec3}};

pub(crate) struct ScatterRecord {
    pub attenuation: Vec3,
    pub scattered_ray: Ray
}

pub(crate) trait Material {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord>;
}

#[derive(Clone, Copy)]
pub(crate) struct LambertianMaterial {
    albedo: Vec3
}

impl LambertianMaterial {
    pub fn new(color: Vec3) -> Self {
        Self {
            albedo: color
        }
    }
}

impl Material for LambertianMaterial {
    fn scatter(&self, _ray: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
        let scatter_direction = hit_record.normal + Vec3::random_unit_vector();
        let scattered_ray = Ray::new(hit_record.point, scatter_direction);
        let albedo = self.albedo.clone();
        Some(ScatterRecord {
            scattered_ray,
            attenuation: albedo
        })
    }
}

#[derive(Clone, Copy)]
pub(crate) struct MetalMaterial {
    albedo: Vec3
}

impl MetalMaterial {
    pub fn new(color: Vec3) -> Self {
        Self {
            albedo: color
        }
    }
}

impl Material for MetalMaterial {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
        let reflected_ray = ray.dir().unit().reflect(&hit_record.normal);
        let scattered_ray = Ray::new(hit_record.point, reflected_ray);
        if scattered_ray.dir().dot(&hit_record.normal) > 0.0 {
            let attenuation = self.albedo;
            Some(ScatterRecord {
                scattered_ray,
                attenuation
            })
        } else {
            None
        }
    }
}