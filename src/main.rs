mod math {
    pub(crate) mod color;
    pub(crate) mod point3;
    pub(crate) mod ray;
    pub(crate) mod vector3;
}

mod objects {
    pub mod hittable;
    pub mod hittable_list;
    pub mod sphere;
}

mod camera {
    pub mod camera;
}

mod materials {
    pub mod material;
}

use crate::camera::camera::{Camera, CameraBuilder};
use crate::materials::material::{Dielectric, Lambertian, Metal};
use crate::math::color::Color;
use crate::math::point3::Point3;
use crate::objects::hittable_list::HittableList;
use crate::objects::sphere::Sphere;
use rand::Rng;
use std::sync::Arc;

fn main() {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const WIDTH: u32 = 1200;

    const SAMPLES_PER_PIXEL: u32 = 500;
    const MAX_DEPTH: u32 = 50;

    let mut world = HittableList::new();

    let ground_material = Lambertian::new(Color::new(0.5, 0.5, 0.5));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(ground_material),
    )));

    for a in -11..11 {
        for b in -11..11 {
            let mut rng = rand::rng();
            let choose_material = rng.random_range(0.0..1.0);
            let center = Point3::new(
                a as f64 + 0.9 * rng.random_range(0.0..1.0),
                0.2,
                b as f64 + 0.9 * rng.random_range(0.0..1.0),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_material < 0.8 {
                    let albedo = Color::random() * Color::random();
                    let material = Lambertian::new(albedo);
                    world.add(Box::new(Sphere::new(center, 0.2, Arc::new(material))));
                } else if (choose_material < 0.95) {
                    let albedo = Color::random_range(0.5..1.0);
                    let fuzz = rng.random_range(0.0..0.5);
                    let material = Metal::new(albedo, fuzz);
                    world.add(Box::new(Sphere::new(center, 0.2, Arc::new(material))));
                } else {
                    let material = Dielectric::new(1.5);
                    world.add(Box::new(Sphere::new(center, 0.2, Arc::new(material))));
                }
            }
        }
    }

    let material1 = Dielectric::new(1.5);
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        Arc::new(material1),
    )));

    let material2 = Lambertian::new(Color::new(0.4, 0.2, 0.1));
    world.add(Box::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        Arc::new(material2),
    )));

    let material3 = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);
    world.add(Box::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        Arc::new(material3),
    )));

    let builder = CameraBuilder::new()
        .aspect_ratio(ASPECT_RATIO)
        .width(WIDTH)
        .samples_per_pixel(SAMPLES_PER_PIXEL)
        .max_depth(MAX_DEPTH)
        .fov(20.0)
        .look_from(Point3::new(13.0, 2.0, 3.0))
        .look_at(Point3::zero())
        .defocus_angle(0.6)
        .focus_dist(10.0);

    let camera = Camera::new_from_builder(&builder);
    camera.render(&world);
}
