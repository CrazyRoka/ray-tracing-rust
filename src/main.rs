use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

use camera::ASPECT_RATIO;
use color::{BLACK, BLUE, RED, WHITE};
use constants::INFINITY;
use hittable::Hittable;
use ray::Ray;
use utils::{random_in_hemisphere, random_in_unit_sphere, random_unit_vector};

use crate::{
    camera::Camera,
    color::{Color, MultipleSamplesColor},
    hittable_list::HittableList,
    material::{Lambertian, Metal},
    sphere::Sphere,
    utils::random_double,
    vec3::{Point3, Vec3},
};

const IMAGE_WIDTH: usize = 400;
const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;
const MAX_COLOR: usize = 255;
const SAMPLES_PER_PIXERL: usize = 100;
const MAX_DEPTH: usize = 50;

mod camera;
mod color;
mod constants;
mod hittable;
mod hittable_list;
mod material;
mod ray;
mod sphere;
mod utils;
mod vec3;

fn ray_color(ray: &Ray, world: Rc<dyn Hittable>, depth: usize) -> Color {
    if depth == 0 {
        return BLACK;
    }

    if let Some(res) = world.hit(ray, 0.001, INFINITY) {
        let answer = if let Some(scatter_result) = res.get_material().scatter(ray, &res) {
            scatter_result.get_color().multiply(&ray_color(
                scatter_result.get_ray(),
                world,
                depth - 1,
            ))
        } else {
            BLACK
        };

        return answer;
    }

    let unit_direction = ray.get_direction().unit_vector();
    let t = 0.5 * (unit_direction.get_y() + 1.0);
    WHITE
        .multiply_constant(1.0 - t)
        .add(&BLUE.multiply_constant(t))
}

fn main() {
    let mut world = HittableList::new();

    let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(Color::new(0.7, 0.3, 0.3)));
    let material_left = Rc::new(Metal::new(Color::new(0.8, 0.8, 0.8)));
    let material_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2)));

    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        material_center,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));
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
                let pixel = ray_color(&ray, Rc::clone(&world), MAX_DEPTH);

                color.add(&pixel);
            }

            println!("{}", color);
        }
    }

    eprintln!("Done");
}
