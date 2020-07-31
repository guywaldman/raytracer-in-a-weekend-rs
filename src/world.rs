use crate::{
    hittable::{self, Hittable},
    ray, vec3,
};
use hittable::HitRecord;

pub(crate) struct World<'a> {
    hittables: Vec<Box<&'a dyn Hittable>>,
}

impl<'a> World<'a> {
    pub fn new() -> Self {
        World {
            hittables: Vec::new(),
        }
    }

    pub fn add_hittable(&mut self, hittable: &'a impl Hittable) {
        self.hittables.push(Box::new(hittable));
    }
}

impl<'a> Hittable for World<'a>
{
    fn hit(&self, ray: &ray::Ray, t_min: vec3::Scalar, t_max: vec3::Scalar) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut hit_record = None;

        for hittable in &self.hittables {
            if let Some(hr) = hittable.hit(ray, t_min, closest_so_far) {
                closest_so_far = hr.t;
                hit_record = Some(hr);
            }
        }

        hit_record
    }
}
