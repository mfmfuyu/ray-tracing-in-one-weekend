use crate::math::color::Color;
use crate::math::ray;
use crate::math::ray::{random_in_unit_sphere, Ray};
use crate::math::vector3::{dot, unit_vector, Vector3};
use crate::objects::hittable::HitRecord;
use rand::Rng;

pub trait Material {
    fn scatter(
        &self,
        ray: &Ray,
        record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}

pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _ray: &Ray,
        record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let scatter_direction = record.normal + ray::random_unit_vector();
        *scattered = Ray::new(record.point, scatter_direction);
        *attenuation = self.albedo;

        true
    }
}

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        ray: &Ray,
        record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = reflect(&unit_vector(&ray.direction), &record.normal);
        *scattered = Ray::new(
            record.point,
            reflected + self.fuzz * random_in_unit_sphere(),
        );
        *attenuation = self.albedo;

        dot(&scattered.direction, &record.normal) > 0.0
    }
}

fn reflect(v: &Vector3, n: &Vector3) -> Vector3 {
    *v - 2.0 * dot(v, n) * *n
}

pub struct Dielectric {
    pub index_of_refraction: f64,
}

impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Self {
        Self {
            index_of_refraction,
        }
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        ray: &Ray,
        record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Color::new(1.0, 1.0, 1.0);
        let etai_over_etat: f64;
        if record.front_face {
            etai_over_etat = 1.0 / self.index_of_refraction;
        } else {
            etai_over_etat = self.index_of_refraction;
        }

        let unit_direction = unit_vector(&ray.direction);
        let cos_theta = f64::min(dot(&-unit_direction, &record.normal), 1.0);
        let sin_theta = (1.0f64 - cos_theta * cos_theta).sqrt();
        if etai_over_etat * sin_theta > 1.0 {
            let reflected = reflect(&unit_direction, &record.normal);
            *scattered = Ray::new(record.point, reflected);

            return true;
        }

        let reflect_prob = schlick(cos_theta, etai_over_etat);
        let mut rng = rand::rng();
        if rng.random_range(0.0..1.0) < reflect_prob {
            let reflected = reflect(&unit_direction, &record.normal);
            *scattered = Ray::new(record.point, reflected);

            return true;
        }

        let refracted = refract(&unit_direction, &record.normal, etai_over_etat);
        *scattered = Ray::new(record.point, refracted);

        true
    }
}

fn refract(uv: &Vector3, n: &Vector3, etai_over_etat: f64) -> Vector3 {
    let cos_theta = dot(&(-*uv), &n);
    let r_out_parallel = etai_over_etat * (*uv + cos_theta * *n);
    let r_out_perp = -(1.0f64 - r_out_parallel.length_squared()).sqrt() * *n;

    r_out_parallel + r_out_perp
}

fn schlick(cosine: f64, refractive_index: f64) -> f64 {
    let mut r0 = (1.0f64 - refractive_index) / (1.0 + refractive_index);
    r0 = r0 * r0;

    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
