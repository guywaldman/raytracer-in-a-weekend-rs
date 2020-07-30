type Scalar = f64;

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) struct Vec3 {
    x: Scalar,
    y: Scalar,
    z: Scalar,
}

impl Vec3 {
    pub fn new(x: Scalar, y: Scalar, z: Scalar) -> Self {
        Vec3 { x, y, z }
    }

    pub fn x(&self) -> Scalar {
        self.x
    }

    pub fn y(&self) -> Scalar {
        self.y
    }

    pub fn z(&self) -> Scalar {
        self.z
    }

    pub fn length(&self) -> Scalar {
        ((self.x * self.x) + (self.y * self.y) + (self.z * self.z)).sqrt()
    }

    pub fn dot(&self, other: &Self) -> Scalar {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
    }

    pub fn cross(&self, other: &Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

impl std::ops::Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl std::ops::Mul<Scalar> for Vec3 {
    type Output = Self;
    fn mul(self, scalar: Scalar) -> Self::Output {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl std::ops::Div<Scalar> for Vec3 {
    type Output = Self;
    fn div(self, scalar: Scalar) -> Self::Output {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn add() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(3.0, 11.0, 5.0);
        assert_eq!(a + b, Vec3::new(4.0, 13.0, 8.0));
    }

    #[test]
    fn sub() {
        let a = Vec3::new(5.0, 3.0, 12.0);
        let b = Vec3::new(2.0, 1.0, 2.0);
        assert_eq!(a - b, Vec3::new(3.0, 2.0, 10.0));
    }

    #[test]
    fn mul() {
        let a = Vec3::new(5.0, 3.0, 12.0);
        let scalar = 2.0;
        assert_eq!(a * scalar, Vec3::new(10.0, 6.0, 24.0));
    }

    #[test]
    fn div() {
        let a = Vec3::new(18.0, 8.0, 4.0);
        let scalar = 2.0;
        assert_eq!(a / scalar, Vec3::new(9.0, 4.0, 2.0));
    }

    #[test]
    fn dot() {
        let a = Vec3::new(5.0, 8.0, 1.0);
        let b = Vec3::new(2.0, 1.0, 5.0);
        assert_eq!(a.dot(&b), 23.0);
    }

    #[test]
    fn cross() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);
        assert_eq!(a.cross(&b), Vec3::new(-3.0, 6.0, -3.0));
    }
}
