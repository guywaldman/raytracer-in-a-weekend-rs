use crate::vec3::Vec3;
use ray::Ray;

mod ray;
mod vec3;

fn vec_to_color(v: &Vec3) -> String {
    let x = v.x() * 255.999;
    let y = v.y() * 255.999;
    let z = v.z() * 255.999;
    format!("{} {} {}", x, y, z)
}

/// Returns the background color -
/// a blue to white top-to-bottom gradient, depending on the ray Y coordinate.
fn ray_rgb_vec(r: &Ray) -> Vec3 {
    // Check intersection with sphere.
    let sphere_center = vec3!(0.0, 0.0, -1.0);
    let normal = r.hit_sphere(sphere_center, 0.5);
    if let Some(n) = normal {
        let n = (n - sphere_center).unit();
        return 0.5 * vec3!(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0);
    }

    let unit = r.unit();
    let t = 0.5 * (unit.y() + 1.0);
    vec3!(1.0, 1.0, 1.0) * (1.0 - t) + vec3!(0.5, 0.7, 1.0) * t
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width: usize = 400;
    let image_height = ((image_width as f64) / aspect_ratio) as usize;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = vec3!(0.0, 0.0, 0.0);
    let horizontal = vec3!(viewport_width, 0.0, 0.0);
    let vertical = vec3!(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - vec3!(0.0, 0.0, focal_length);

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for j in (0..image_height).rev() {
        eprintln!("Scanlines: {}/{}", (image_height - j), image_height);
        for i in 0..image_width {
            let u = (i as f64) / ((image_width as f64) - 1.0);
            let v = (j as f64) / ((image_height as f64) - 1.0);

            let ray = ray!(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin
            );

            let pixel_color = ray_rgb_vec(&ray);

            println!("{}", vec_to_color(&pixel_color));
        }
    }
}
