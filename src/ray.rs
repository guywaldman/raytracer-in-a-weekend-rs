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