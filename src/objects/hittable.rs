use crate::materials::material::Material;
use crate::math::point3::Point3;
use crate::math::ray::Ray;
use crate::math::vector3::{dot, Vector3};
use std::sync::Arc;

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool;
}

#[derive(Clone)]
pub struct HitRecord {
    pub point: Point3,
    pub normal: Vector3,
    pub material: Option<Arc<dyn Material>>,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new() -> Self {
        Self {
            point: Point3::zero(),
            normal: Vector3::zero(),
            material: None,
            t: 0.0,
            front_face: false,
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vector3) {
        self.front_face = dot(&ray.direction, outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
    }
}
