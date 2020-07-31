use crate::rand::random_scalar;
use crate::vec3::Vec3;
use camera::Camera;
use hittable::{HitRecord, Hittable};
use math::clamp;
use ray::Ray;
use sphere::Sphere;
use world::World;

#[macro_use]
mod vec3;

mod camera;
mod hittable;
mod math;
mod rand;
mod ray;
mod sphere;
mod world;

const SAMPLES_PER_PIXEL: usize = 30;
const MAX_BOUNCE_DEPTH: usize = 30;

fn color_vec_to_output(color_vec: &Vec3) -> String {
    let mut r = color_vec.x();
    let mut g = color_vec.y();
    let mut b = color_vec.z();

    // Divide the color by the number of samples.
    let scale = 1.0 / (SAMPLES_PER_PIXEL as f64);
    r *= scale;
    g *= scale;
    b *= scale;

    // Write the translated [0,255] value of each color component.
    r = 256.0 * clamp(r, 0.0, 0.999);
    g = 256.0 * clamp(g, 0.0, 0.999);
    b = 256.0 * clamp(b, 0.0, 0.999);
    format!("{} {} {}", r as u64, g as u64, b as u64)
}

/// Returns the background color -
/// a blue to white top-to-bottom gradient, depending on the ray Y coordinate.
fn pixel_color_for_ray(ray: &Ray, world: &World, depth: usize) -> Vec3 {
    // If exceeded ray bounce limit, no more light should be gathered.
    if depth <= 0 {
        return vec3!(0.0, 0.0, 0.0);
    }

    // Check intersection with world.
    if let Some(HitRecord { normal, point, .. }) = world.hit(&ray, 0.0, std::f64::MAX) {
        let target = point + normal + Vec3::random_in_unit_sphere();
        // Shoot a random diffuse bounce ray and recurse.
        return 0.5 * pixel_color_for_ray(&Ray::new(point, target - point), world, depth - 1);
    }

    let unit = ray.unit();
    let t = 0.5 * (unit.y() + 1.0);
    vec3!(1.0, 1.0, 1.0) * (1.0 - t) + vec3!(0.5, 0.7, 1.0) * t
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width: usize = 400;
    let image_height = ((image_width as f64) / aspect_ratio) as usize;

    // Camera
    let camera = Camera::new();

    // World
    let mut world = World::new();
    let sphere_1 = Sphere::new(vec3!(0.0, 0.0, -1.0), 0.5);
    let sphere_2 = Sphere::new(vec3!(0.0, -100.5, -1.0), 100.0);
    world.add_hittable(&sphere_1);
    world.add_hittable(&sphere_2);

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for j in (0..image_height).rev() {
        eprintln!("Scanlines: {}/{}", (image_height - j), image_height);

        for i in 0..image_width {
            // Antialiasing - sample and average color around each pixel.
            let mut pixel_color = vec3!(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = ((i as f64) + random_scalar(0.0, 1.0)) / ((image_width as f64) - 1.0);
                let v = ((j as f64) + random_scalar(0.0, 1.0)) / ((image_height as f64) - 1.0);
                let ray = camera.get_ray(u, v);
                pixel_color += pixel_color_for_ray(&ray, &world, MAX_BOUNCE_DEPTH);
            }

            println!("{}", color_vec_to_output(&pixel_color));
        }
    }
}
