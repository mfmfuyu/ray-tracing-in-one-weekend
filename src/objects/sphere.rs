use crate::materials::material::Material;
use crate::math::ray::Ray;
use crate::math::vector3::{dot, Vector3};
use crate::objects::hittable::{HitRecord, Hittable};
use std::sync::Arc;

pub(crate) struct Sphere {
    pub center: Vector3,
    pub radius: f64,
    pub material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vector3, radius: f64, material: Arc<dyn Material>) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = dot(&oc, &ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            let mut temp = (-half_b - root) / a;
            if temp < t_max && temp > t_min {
                hit_record.t = temp;
                hit_record.point = ray.at(hit_record.t);
                let outward_normal = (hit_record.point - self.center) / self.radius;
                hit_record.set_face_normal(ray, &outward_normal);
                hit_record.material = Some(self.material.clone());

                return true;
            }

            temp = (-half_b + root) / a;
            if temp < t_max && temp > t_min {
                hit_record.t = temp;
                hit_record.point = ray.at(hit_record.t);
                let outward_normal = (hit_record.point - self.center) / self.radius;
                hit_record.set_face_normal(ray, &outward_normal);
                hit_record.material = Some(self.material.clone());

                return true;
            }
        }

        false
    }
}
