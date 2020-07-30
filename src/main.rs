use crate::vec3::Vec3;

mod vec3;
mod ray;

fn vec_to_color(v: Vec3) -> String {
    let x = v.x() * 255.999;
    let y = v.y() * 255.999;
    let z = v.z() * 255.999;
    format!("{} {} {}", x, y, z)
}

fn main() {
    let width = 256;
    let height = 256;

    println!("P3");
    println!("{} {}", width, height);
    println!("255");

    for j in (0..height).rev() {
        eprintln!("Scanlines: {}/{}", (height - j), height);
        for i in 0..width {
            let r = (i as f64) / (f64::from(width) - 1.0);
            let g = (j as f64) / (f64::from(height) - 1.0);
            let b = 0.25;

            let v = Vec3::new(r, g, b);

            println!("{}", vec_to_color(v));
        }
    }
}
