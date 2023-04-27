use std::{
    fmt,
    ops::{Add, Div, Index, IndexMut, Mul, Sub},
};

pub type Point3 = Vec3;
pub type Color = Vec3;

pub struct Vec3 {
    pub e: [f64; 3],
}

impl Vec3 {
    pub fn new() -> Self {
        Vec3 { e: [0.0, 0.0, 0.0] }
    }

    pub fn build(e0: f64, e1: f64, e2: f64) -> Self {
        Vec3 { e: [e0, e1, e2] }
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }
    pub fn y(&self) -> f64 {
        self.e[1]
    }
    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn dot(&self, other: &Vec3) -> f64 {
        return self.e[0] * other.e[0] + self.e[1] * other.e[1] + self.e[2] * other.e[2];
    }

    pub fn cross(self, other: Vec3) -> Vec3 {
        return Vec3::build(
            self.e[1] * other.e[2] - self.e[2] * other.e[1],
            self.e[2] * other.e[0] - self.e[0] * other.e[2],
            self.e[0] * other.e[1] - self.e[1] * other.e[0],
        );
    }

    pub fn unit_vector(self) -> Vec3 {
        return &self / &self.length();
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.e[0].powi(2) + self.e[1].powi(2) + self.e[2].powi(2)
    }
}

impl<'a, 'b> Add<&'b Vec3> for &'a Vec3 {
    type Output = Vec3;

    fn add(self, other: &Vec3) -> Self::Output {
        return Vec3::build(
            self.x() + other.x(),
            self.y() + other.y(),
            self.z() + other.z(),
        );
    }
}

impl<'a, 'b> Sub<&'b Vec3> for &'a Vec3 {
    type Output = Vec3;

    fn sub(self, other: &Vec3) -> Self::Output {
        return Vec3::build(
            self.x() - other.x(),
            self.y() - other.y(),
            self.z() - other.z(),
        );
    }
}

impl<'a, 'b> Mul<&'b f64> for &'a Vec3 {
    type Output = Vec3;

    fn mul(self, scalar: &f64) -> Self::Output {
        return Vec3::build(self.x() * scalar, self.y() * scalar, self.z() * scalar);
    }
}

impl<'a, 'b> Mul<&'b Vec3> for &'a Vec3 {
    type Output = Vec3;

    fn mul(self, other: &Vec3) -> Self::Output {
        return Vec3::build(
            self.x() * other.x(),
            self.y() * other.y(),
            self.z() * other.z(),
        );
    }
}

impl Div<&f64> for &Vec3 {
    type Output = Vec3;

    fn div(self, scalar: &f64) -> Self::Output {
        return Vec3::build(self.x() / scalar, self.y() / scalar, self.z() / scalar);
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, i: usize) -> &Self::Output {
        return &self.e[i];
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        return &mut self.e[i];
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.x(), self.y(), self.z())
    }
}

// Scalar operations
impl Mul<&Vec3> for &f64 {
    type Output = Vec3;

    fn mul(self, vec: &Vec3) -> Self::Output {
        return Vec3::build(self * vec.x(), self * vec.y(), self * vec.z());
    }
}

impl Div<&Vec3> for &f64 {
    type Output = Vec3;

    fn div(self, vec: &Vec3) -> Self::Output {
        return Vec3::build(self / vec.x(), self / vec.y(), self / vec.z());
    }
}

impl Add<&Vec3> for &f64 {
    type Output = Vec3;

    fn add(self, vec: &Vec3) -> Self::Output {
        return Vec3::build(self + vec.x(), self + vec.y(), self + vec.z());
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_aritmetics() {
        let v1 = super::Vec3::build(1.0, 2.0, 3.0);
        let v2 = super::Vec3::build(4.0, 5.0, 6.0);
        let v3 = &v1 + &v2;
        assert_eq!(v3.x(), 5.0);
        assert_eq!(v3.y(), 7.0);
        assert_eq!(v3.z(), 9.0);

        let v4 = &v1 - &v2;
        assert_eq!(v4.x(), -3.0);
        assert_eq!(v4.y(), -3.0);
        assert_eq!(v4.z(), -3.0);

        let v5 = &v1 * &2.0;
        assert_eq!(v5.x(), 2.0);
        assert_eq!(v5.y(), 4.0);
        assert_eq!(v5.z(), 6.0);

        let v6 = &v1 / &2.0;
        assert_eq!(v6.x(), 0.5);
        assert_eq!(v6.y(), 1.0);
        assert_eq!(v6.z(), 1.5);
    }

    #[test]
    fn test_vector_ops() {
        // Test dot
        let v1 = super::Vec3::build(1.0, 2.0, 3.0);
        let v2 = super::Vec3::build(4.0, 5.0, 6.0);
        assert_eq!(v1.dot(&v2), 32.0);

        // Test cross
        let v3 = v1.cross(v2);
        assert_eq!(v3.x(), -3.0);
        assert_eq!(v3.y(), 6.0);
        assert_eq!(v3.z(), -3.0);
    }
}
