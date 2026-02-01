use crate::math::color::Color;
use crate::math::point3::Point3;
use crate::math::vector3::{unit_vector, Vector3};
use crate::objects::hittable::{HitRecord, Hittable};
use rand::Rng;
use std::f64::consts::PI;

pub struct Ray {
    pub(crate) origin: Point3,
    pub(crate) direction: Vector3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vector3) -> Self {
        Self { origin, direction }
    }
    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }
}

pub fn ray_color(ray: &Ray, world: &dyn Hittable, depth: u32) -> Color {
    if depth <= 0 {
        return Color::black();
    }

    let mut record = HitRecord::new();
    if world.hit(ray, 0.001, f64::INFINITY, &mut record) {
        let mut scattered = Ray::new(Vector3::zero(), Point3::zero());
        let mut attenuation = Color::black();

        let mat = record.material.clone();
        if let Some(material) = mat {
            if material.scatter(&ray, &mut record, &mut attenuation, &mut scattered) {
                return attenuation * ray_color(&scattered, world, depth - 1);
            }
            return Color::black();
        }
    };

    let unit_direction = unit_vector(&ray.direction);
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

pub fn random_in_unit_sphere() -> Vector3 {
    loop {
        let p = Vector3::random(-1.0, 1.0);
        if p.length_squared() < 1.0 {
            continue;
        }

        return p;
    }
}

pub(crate) fn random_unit_vector() -> Vector3 {
    let mut rng = rand::rng();
    let a = rng.random_range(0.0f64..(2.0f64 * PI));
    let z = rng.random_range(-1.0f64..1.0f64);
    let r = (1.0 - z * z).sqrt();

    Vector3::new(r * a.cos(), r * a.sin(), z)
}
