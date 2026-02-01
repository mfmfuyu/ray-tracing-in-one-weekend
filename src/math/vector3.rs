use rand::Rng;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Copy, Clone)]
pub struct Vector3 {
    e: [f64; 3],
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector3 { e: [x, y, z] }
    }

    pub fn zero() -> Self {
        Vector3 { e: [0.0, 0.0, 0.0] }
    }

    pub fn unit_vector(vector3: Vector3) -> Self {
        vector3 / vector3.length()
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

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z()
    }

    pub fn random(min: f64, max: f64) -> Self {
        let mut rng = rand::rng();

        Self::new(
            rng.random_range(min..max),
            rng.random_range(min..max),
            rng.random_range(min..max),
        )
    }
}

impl Add<Vector3> for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Vector3) -> Self::Output {
        Vector3::new(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}

impl Mul<f64> for Vector3 {
    type Output = Vector3;

    fn mul(mut self, rhs: f64) -> Self::Output {
        self.e[0] *= rhs;
        self.e[1] *= rhs;
        self.e[2] *= rhs;

        self
    }
}

impl Mul<Vector3> for f64 {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Self::Output {
        Vector3::new(self * rhs.x(), self * rhs.y(), self * rhs.z())
    }
}

impl Sub<Vector3> for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Vector3) -> Self::Output {
        Vector3::new(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
    }
}

impl Div<f64> for Vector3 {
    type Output = Vector3;

    fn div(self, rhs: f64) -> Self::Output {
        (1.0 / rhs) * self
    }
}

impl Neg for Vector3 {
    type Output = Vector3;

    fn neg(self) -> Self::Output {
        Vector3::new(-self.x(), -self.y(), -self.z())
    }
}

pub fn dot(u: &Vector3, v: &Vector3) -> f64 {
    u.x() * v.x() + u.y() * v.y() + u.z() * v.z()
}

pub fn cross(u: &Vector3, v: &Vector3) -> Vector3 {
    Vector3::new(
        u.e[1] * v.e[2] - u.e[2] * v.e[1], // x = u.y*v.z - u.z*v.y
        u.e[2] * v.e[0] - u.e[0] * v.e[2], // y = u.z*v.x - u.x*v.z
        u.e[0] * v.e[1] - u.e[1] * v.e[0], // z = u.x*v.y - u.y*v.x
    )
}

pub fn unit_vector(v: &Vector3) -> Vector3 {
    *v / v.length()
}

pub fn random_in_unit_disk() -> Vector3 {
    let mut rng = rand::rng();
    loop {
        let p = Vector3::new(
            rng.random_range(-1.0..1.0),
            rng.random_range(-1.0..1.0),
            0.0,
        );
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}
