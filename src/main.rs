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

            let r_col = (r * 255.999) as u8;
            let g_col = (g * 255.999) as u8;
            let b_col = (b * 255.999) as u8;

            println!("{} {} {}", r_col, g_col, b_col);
        }
    }
}
