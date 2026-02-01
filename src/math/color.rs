use crate::math::vector3::Vector3;
use rand::Rng;
use std::ops;
use std::ops::Range;

#[derive(Copy, Clone)]
pub struct Color(Vector3);

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Color(Vector3::new(r, g, b))
    }

    fn r(&self) -> f64 {
        self.0.x()
    }

    fn g(&self) -> f64 {
        self.0.y()
    }

    fn b(&self) -> f64 {
        self.0.z()
    }

    pub fn black() -> Self {
        Color::new(0.0, 0.0, 0.0)
    }

    pub fn white() -> Self {
        Color::new(1.0, 1.0, 1.0)
    }

    pub fn random() -> Self {
        let mut rng = rand::rng();

        Self::new(
            rng.random_range(0.0..1.0),
            rng.random_range(0.0..1.0),
            rng.random_range(0.0..1.0),
        )
    }

    pub fn random_range(range: Range<f64>) -> Self {
        let mut rng = rand::rng();

        Self::new(
            rng.random_range(range.clone()),
            rng.random_range(range.clone()),
            rng.random_range(range),
        )
    }
}

impl ops::Add<Color> for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Self::Output {
        Color::new(self.r() + rhs.r(), self.g() + rhs.g(), self.b() + rhs.b())
    }
}

impl ops::Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Color::new(self * rhs.r(), self * rhs.g(), self * rhs.b())
    }
}

impl ops::Mul<Color> for Color {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Color::new(self.r() * rhs.r(), self.g() * rhs.g(), self.b() * rhs.b())
    }
}

impl ops::Add<Color> for Vector3 {
    type Output = Color;

    fn add(self, rhs: Color) -> Self::Output {
        Color::new(self.x() + rhs.r(), self.y() + rhs.g(), self.z() + rhs.b())
    }
}

impl ops::AddAssign<Color> for Color {
    fn add_assign(&mut self, rhs: Color) {
        *self = Color::new(self.r() + rhs.r(), self.g() + rhs.g(), self.b() + rhs.b())
    }
}

pub fn write_color(color: &Color, samples_per_pixel: u32) {
    let mut r = color.r();
    let mut g = color.g();
    let mut b = color.b();

    let scale = 1.0 / samples_per_pixel as f64;
    r = (scale * r).sqrt();
    g = (scale * g).sqrt();
    b = (scale * b).sqrt();

    let rr = (256.0 * r.clamp(0.0, 0.999)) as u32;
    let gg = (256.0 * g.clamp(0.0, 0.999)) as u32;
    let bb = (256.0 * b.clamp(0.0, 0.999)) as u32;

    println!("{} {} {}", rr, gg, bb);
}
