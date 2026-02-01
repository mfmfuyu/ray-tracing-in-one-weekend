use crate::math::color::{write_color, Color};
use crate::math::point3::Point3;
use crate::math::ray::{ray_color, Ray};
use crate::math::vector3::{cross, random_in_unit_disk, unit_vector, Vector3};
use crate::objects::hittable::Hittable;
use rand::Rng;
use std::io;
use std::io::Write;

pub struct Camera {
    aspect_ratio: f64,
    width: u32,
    samples_per_pixel: u32,
    max_depth: u32,

    fov: f64,
    look_from: Point3,
    look_at: Point3,
    up: Vector3,

    defocus_angle: f64,
    focus_dist: f64,

    height: u32,
    pixel_samples_scale: f64,
    center: Point3,
    pixel00_loc: Point3,

    pixel_delta_u: Vector3,
    pixel_delta_v: Vector3,

    u: Vector3,
    v: Vector3,
    w: Vector3,

    defocus_disk_u: Vector3,
    defocus_disk_v: Vector3,
}

impl Camera {
    pub(crate) fn new_from_builder(builder: &CameraBuilder) -> Self {
        let height = (builder.width as f64 / builder.aspect_ratio) as u32;
        let height = height.max(1);

        let center = builder.look_from;

        // let focal_length = (builder.look_from - builder.look_at).length();
        let theta = builder.fov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * builder.focus_dist;
        let viewport_width = viewport_height * (builder.width as f64 / height as f64);

        let pixel_samples_scale = 1.0 / builder.samples_per_pixel as f64;

        let w = unit_vector(&(builder.look_from - builder.look_at));
        let u = unit_vector(&cross(&builder.up, &w));
        let v = cross(&w, &u);

        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        let pixel_delta_u = viewport_u / builder.width as f64;
        let pixel_delta_v = viewport_v / height as f64;

        let viewport_upper_left =
            center - (builder.focus_dist * w) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        let defocus_radius = builder.focus_dist * (builder.defocus_angle / 2.0).to_radians().tan();

        Self {
            aspect_ratio: builder.aspect_ratio,
            width: builder.width,
            samples_per_pixel: builder.samples_per_pixel,
            max_depth: builder.max_depth,
            fov: builder.fov,
            look_from: builder.look_from,
            look_at: builder.look_at,
            up: builder.up,
            defocus_angle: builder.defocus_angle,
            focus_dist: builder.focus_dist,
            height,
            pixel_samples_scale,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            u,
            v,
            w,
            defocus_disk_u: u * defocus_radius,
            defocus_disk_v: v * defocus_radius,
        }
    }

    pub(crate) fn render(&self, world: &dyn Hittable) {
        println!("P3\n{} {}\n255", self.width, self.height);

        for j in 0..self.height {
            eprint!("\rScanlines remaining: {}\x1b[K", self.height - j);
            io::stderr().flush().unwrap();

            for i in 0..self.width {
                let mut pixel_color = Color::black();
                for _sample in 0..self.samples_per_pixel {
                    let ray = self.get_ray(i, j);
                    pixel_color += ray_color(&ray, world, self.max_depth);
                }

                write_color(&pixel_color, self.samples_per_pixel);
            }
        }
        eprintln!("\rDone.");
    }

    pub fn get_ray(&self, i: u32, j: u32) -> Ray {
        let offset = sample_square();
        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.x()) * self.pixel_delta_u)
            + ((j as f64 + offset.y()) * self.pixel_delta_v);

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    pub fn defocus_disk_sample(&self) -> Point3 {
        let p = random_in_unit_disk();

        self.center + (p.x() * self.defocus_disk_u) + (p.y() * self.defocus_disk_v)
    }
}

fn sample_square() -> Vector3 {
    let mut rng = rand::rng();

    Vector3::new(
        rng.random_range(-0.5..0.5),
        rng.random_range(-0.5..0.5),
        0.0,
    )
}

pub struct CameraBuilder {
    aspect_ratio: f64,
    width: u32,
    samples_per_pixel: u32,
    max_depth: u32,

    fov: f64,
    look_from: Point3,
    look_at: Point3,
    up: Vector3,

    defocus_angle: f64,
    focus_dist: f64,
}

impl CameraBuilder {
    pub fn new() -> Self {
        Self {
            aspect_ratio: 1.0,
            width: 100,
            samples_per_pixel: 10,
            max_depth: 10,
            fov: 90.0,
            look_from: Point3::zero(),
            look_at: Point3::new(0.0, 0.0, -1.0),
            up: Vector3::new(0.0, 1.0, 0.0),
            defocus_angle: 0.0,
            focus_dist: 10.0,
        }
    }

    pub fn aspect_ratio(mut self, aspect_ratio: f64) -> Self {
        self.aspect_ratio = aspect_ratio;
        self
    }

    pub fn width(mut self, width: u32) -> Self {
        self.width = width;
        self
    }

    pub fn samples_per_pixel(mut self, samples_per_pixel: u32) -> Self {
        self.samples_per_pixel = samples_per_pixel;
        self
    }

    pub fn max_depth(mut self, max_depth: u32) -> Self {
        self.max_depth = max_depth;
        self
    }

    pub fn fov(mut self, fov: f64) -> Self {
        self.fov = fov;
        self
    }

    pub fn look_from(mut self, look_from: Point3) -> Self {
        self.look_from = look_from;
        self
    }

    pub fn look_at(mut self, look_at: Point3) -> Self {
        self.look_at = look_at;
        self
    }

    pub fn up(mut self, up: Vector3) -> Self {
        self.up = up;
        self
    }

    pub fn defocus_angle(mut self, angle: f64) -> Self {
        self.defocus_angle = angle;
        self
    }

    pub fn focus_dist(mut self, dist: f64) -> Self {
        self.focus_dist = dist;
        self
    }
}
