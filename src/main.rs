use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

use color::{BLUE, RED, WHITE};
use constants::INFINITY;
use hittable::Hittable;
use ray::Ray;

use crate::{
    color::Color,
    hittable_list::HittableList,
    sphere::Sphere,
    vec3::{Point3, Vec3},
};

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: usize = 400;
const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;
const MAX_COLOR: usize = 255;
const VIEWPORT_HEIGHT: f64 = 2.0;
const VIEWPORT_WIDTH: f64 = VIEWPORT_HEIGHT * ASPECT_RATIO;
const FOCAL_LENGTH: f64 = 1.0;
const SPHERE_POINT: Point3 = Point3::new(0.0, 0.0, -1.0);
const SPHERE_RADIUS: f64 = 0.5;

mod color;
mod constants;
mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod utils;
mod vec3;

fn ray_color(ray: &Ray, world: Rc<dyn Hittable>) -> Color {
    if let Some(res) = world.hit(ray, 0.0, INFINITY) {
        return res.get_normal().add(&WHITE).divide_constant(2.0);
    }

    let unit_direction = ray.get_direction().unit_vector();
    let t = 0.5 * (unit_direction.get_y() + 1.0);
    WHITE
        .multiply_constant(1.0 - t)
        .add(&BLUE.multiply_constant(t))
}

fn main() {
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));
    let world: Rc<dyn Hittable> = Rc::new(world);

    let origin = Point3::empty();
    let horizontal = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
    let vertical = Vec3::new(0.0, VIEWPORT_HEIGHT, 0.0);
    let lower_left_corner = origin
        .subtract(&horizontal.divide_constant(2.0))
        .subtract(&vertical.divide_constant(2.0))
        .subtract(&Vec3::new(0.0, 0.0, FOCAL_LENGTH));

    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("{}", MAX_COLOR);

    for i in (0..IMAGE_HEIGHT).rev() {
        eprintln!("Scanlines remaining {}", i);
        for j in 0..IMAGE_WIDTH {
            let u = j as f64 / (IMAGE_WIDTH - 1) as f64;
            let v = i as f64 / (IMAGE_HEIGHT - 1) as f64;

            let ray = Ray::new(
                origin,
                lower_left_corner
                    .add(&horizontal.multiply_constant(u))
                    .add(&vertical.multiply_constant(v))
                    .subtract(&origin),
            );
            let pixel = ray_color(&ray, Rc::clone(&world));

            println!("{}", pixel);
        }
    }

    eprintln!("Done");
}
