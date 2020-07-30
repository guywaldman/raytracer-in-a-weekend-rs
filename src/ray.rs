use crate::vec3::{Scalar, Vec3};

pub(crate) struct Ray {
    origin: Vec3,
    dir: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, dir: Vec3) -> Self {
        Self { origin, dir }
    }

    pub fn origin(&self) -> Vec3 {
        self.origin
    }

    pub fn dir(&self) -> Vec3 {
        self.dir
    }

    pub fn at(&self, t: Scalar) -> Vec3 {
        self.origin + self.dir * t
    }

    pub fn unit(&self) -> Vec3 {
        self.dir.unit()
    }

    /// Checks intersection with the received sphere.
    /// If successful, return the normal.
    pub fn hit_sphere(&self, sphere_center: Vec3, sphere_radius: f64) -> Option<Vec3> {
        let oc = self.origin() - sphere_center;
        let a = self.dir().length_squared();
        let half_b = oc.dot(&self.dir());
        let c = oc.length_squared() - sphere_radius * sphere_radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        }

        Some(self.at((-half_b - discriminant.sqrt()) / a))
    }
}

#[macro_export]
macro_rules! ray {
    ($origin: expr, $dir: expr) => {
        Ray::new($origin, $dir)
    };
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn at() {
        let ray = Ray::new(Vec3::new(2.0, 4.0, 6.0), Vec3::new(1.0, 0.0, 0.0));
        assert_eq!(ray.at(0.0), Vec3::new(2.0, 4.0, 6.0));
        assert_eq!(ray.at(0.5), Vec3::new(2.5, 4.0, 6.0));
        assert_eq!(ray.at(1.0), Vec3::new(3.0, 4.0, 6.0));

        let ray = Ray::new(Vec3::new(2.0, 4.0, 6.0), Vec3::new(0.0, 2.0, 0.0));
        assert_eq!(ray.at(0.0), Vec3::new(2.0, 4.0, 6.0));
        assert_eq!(ray.at(0.5), Vec3::new(2.0, 5.0, 6.0));
        assert_eq!(ray.at(1.0), Vec3::new(2.0, 6.0, 6.0));

        let ray = Ray::new(Vec3::new(2.0, 4.0, 6.0), Vec3::new(0.0, 0.0, 3.0));
        assert_eq!(ray.at(0.0), Vec3::new(2.0, 4.0, 6.0));
        assert_eq!(ray.at(0.5), Vec3::new(2.0, 4.0, 7.5));
        assert_eq!(ray.at(1.0), Vec3::new(2.0, 4.0, 9.0));
    }
}
