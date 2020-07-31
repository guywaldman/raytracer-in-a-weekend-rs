use crate::vec3::Scalar;
use rand::Rng;

pub fn random_scalar(min: Scalar, max: Scalar) -> Scalar {
    let mut rng = rand::thread_rng();
    rng.gen_range(min, max)
}