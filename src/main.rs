use std::rc::Rc;

use camera::ASPECT_RATIO;
use color::{BLACK, BLUE, WHITE};
use constants::INFINITY;
use hittable::Hittable;
use material::Material;
use ray::Ray;
use utils::{random_in_range, random_vec3, random_vec_in_range};

use crate::{
    camera::Camera,
    color::{Color, MultipleSamplesColor},
    hittable_list::HittableList,
    material::{Dielectric, Lambertian, Metal},
    sphere::Sphere,
    utils::random_double,
    vec3::{Point3, Vec3},
};

// const IMAGE_WIDTH: usize = 1200;
// const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;
const IMAGE_WIDTH: usize = 300;
const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;
const MAX_COLOR: usize = 255;
const SAMPLES_PER_PIXERL: usize = 50;
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

fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let ground_material = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double();
            let center = Point3::new(
                a as f64 + 0.9 * random_double(),
                0.2,
                b as f64 + 0.9 * random_double(),
            );

            if (center.subtract(&Point3::new(4.0, 0.2, 0.0))).length() > 0.9 {
                let sphere_material: Rc<dyn Material> = if choose_mat < 0.8 {
                    let albedo = random_vec3().multiply(&random_vec3());
                    Rc::new(Lambertian::new(albedo))
                } else if choose_mat < 0.95 {
                    let albedo = random_vec_in_range(0.5, 1.0);
                    let fuzz = random_in_range(0.0, 0.5);
                    Rc::new(Metal::new(albedo, fuzz))
                } else {
                    Rc::new(Dielectric::new(1.5))
                };

                world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
            }
        }
    }

    let material1 = Rc::new(Dielectric::new(1.5));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Box::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    world
}

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

    // let R = (PI / 4.0).cos();

    // let material_left = Rc::new(Lambertian::new(Color::new(0.0, 0.0, 1.0)));
    // let material_right = Rc::new(Lambertian::new(Color::new(1.0, 0.0, 0.0)));

    // world.add(Box::new(Sphere::new(
    //     Point3::new(-R, 0.0, -1.0),
    //     R,
    //     material_left,
    // )));
    // world.add(Box::new(Sphere::new(
    //     Point3::new(R, 0.0, -1.0),
    //     R,
    //     material_right,
    // )));

    // let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    // let material_center = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    // let material_left = Rc::new(Dielectric::new(1.5));
    // let material_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.0));

    // world.add(Box::new(Sphere::new(
    //     Point3::new(0.0, -100.5, -1.0),
    //     100.0,
    //     material_ground,
    // )));
    // world.add(Box::new(Sphere::new(
    //     Point3::new(0.0, 0.0, -1.0),
    //     0.5,
    //     material_center,
    // )));
    // world.add(Box::new(Sphere::new(
    //     Point3::new(-1.0, 0.0, -1.0),
    //     0.5,
    //     material_left.clone(),
    // )));
    // world.add(Box::new(Sphere::new(
    //     Point3::new(-1.0, 0.0, -1.0),
    //     -0.45,
    //     material_left,
    // )));
    // world.add(Box::new(Sphere::new(
    //     Point3::new(1.0, 0.0, -1.0),
    //     0.5,
    //     material_right,
    // )));
    // let world: Rc<dyn Hittable> = Rc::new(world);

    let world: Rc<dyn Hittable> = Rc::new(random_scene());

    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let camera = Camera::new(look_from, look_at, vup, 20.0, aperture, dist_to_focus);

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
