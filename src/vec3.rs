pub(crate) type Scalar = f64;

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

    pub fn dot(&self, other: &Self) -> Scalar {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
    }

    pub fn length(&self) -> Scalar {
        self.dot(&self).sqrt()
    }

    pub fn length_squared(&self) -> Scalar {
        self.dot(&self)
    }

    #[allow(dead_code)]
    pub fn cross(&self, other: &Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn unit(&self) -> Self {
        *self / self.length()
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

impl std::ops::Mul<Vec3> for Scalar {
    type Output = Vec3;
    fn mul(self, v: Vec3) -> Self::Output {
        Self::Output {
            x: v.x * self,
            y: v.y * self,
            z: v.z * self,
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

#[macro_export]
macro_rules! vec3 {
    ($x: expr, $y: expr, $z: expr) => {
        Vec3::new($x, $y, $z)
    };
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn add() {
        let a = vec3!(1.0, 2.0, 3.0);
        let b = vec3!(3.0, 11.0, 5.0);
        assert_eq!(a + b, vec3!(4.0, 13.0, 8.0));
    }

    #[test]
    fn sub() {
        let a = vec3!(5.0, 3.0, 12.0);
        let b = vec3!(2.0, 1.0, 2.0);
        assert_eq!(a - b, vec3!(3.0, 2.0, 10.0));
    }

    #[test]
    fn mul() {
        let a = vec3!(5.0, 3.0, 12.0);
        let scalar = 2.0;
        assert_eq!(a * scalar, vec3!(10.0, 6.0, 24.0));
    }

    #[test]
    fn div() {
        let a = vec3!(18.0, 8.0, 4.0);
        let scalar = 2.0;
        assert_eq!(a / scalar, vec3!(9.0, 4.0, 2.0));
    }

    #[test]
    fn dot() {
        let a = vec3!(5.0, 8.0, 1.0);
        let b = vec3!(2.0, 1.0, 5.0);
        assert_eq!(a.dot(&b), 23.0);
    }

    #[test]
    fn cross() {
        let a = vec3!(1.0, 2.0, 3.0);
        let b = vec3!(4.0, 5.0, 6.0);
        assert_eq!(a.cross(&b), vec3!(-3.0, 6.0, -3.0));
    }
}
