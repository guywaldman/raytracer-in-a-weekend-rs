use crate::vec3::Scalar;
use crate::{ray::Ray, vec3::Vec3};

const ASPECT_RATIO: Scalar = 16.0 / 9.0;
const VIEWPORT_HEIGHT: Scalar = 2.0;
const VIEWPORT_WIDTH: Scalar = ASPECT_RATIO * VIEWPORT_HEIGHT;
const FOCAL_LENGTH: Scalar = 0.9;

pub(crate) struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new() -> Self {
        let origin = vec3!(0.0, 0.0, 0.0);
        let horizontal = vec3!(VIEWPORT_WIDTH, 0.0, 0.0);
        let vertical = vec3!(0.0, VIEWPORT_HEIGHT, 0.0);
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - vec3!(0.0, 0.0, FOCAL_LENGTH);

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, u: Scalar, v: Scalar) -> Ray {
        Ray::new(self.origin, self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin)
    }
}
