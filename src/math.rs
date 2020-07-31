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
