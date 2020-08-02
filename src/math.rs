pub fn clamp<T>(x: T, min: T, max: T) -> T
where
    T: PartialOrd,
{
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

pub fn degrees_to_radians(deg: f64) -> f64 {
    deg * std::f64::consts::PI / 180.0
}
