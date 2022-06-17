use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

use camera::ASPECT_RATIO;
use color::{BLUE, RED, WHITE};
use constants::INFINITY;
use hittable::Hittable;
use ray::Ray;

use crate::{
    camera::Camera,
    color::{Color, MultipleSamplesColor},
    hittable_list::HittableList,
    sphere::Sphere,
    utils::random_double,
    vec3::{Point3, Vec3},
};

const IMAGE_WIDTH: usize = 400;
const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;
const MAX_COLOR: usize = 255;
const SAMPLES_PER_PIXERL: usize = 100;

mod camera;
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

    let camera = Camera::new();

    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("{}", MAX_COLOR);

    for i in (0..IMAGE_HEIGHT).rev() {
        eprintln!("Scanlines remaining {}", i);
        for j in 0..IMAGE_WIDTH {
            let mut color = MultipleSamplesColor::new();

            for _ in 0..SAMPLES_PER_PIXERL {
                let u = (j as f64 + random_double()) / (IMAGE_WIDTH - 1) as f64;
                let v = (i as f64 + random_double()) / (IMAGE_HEIGHT - 1) as f64;
                let ray = camera.get_ray(u, v);
                let pixel = ray_color(&ray, Rc::clone(&world));

                color.add(&pixel);
            }

            println!("{}", color);
        }
    }

    eprintln!("Done");
}
