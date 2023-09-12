use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::create("image.ppm")?;

    let image_width = 256;
    let image_height = 256;

    file.write_all(format!("P3\n{} {}\n255\n", image_width, image_height).as_bytes());

    for i in 0..image_height {
        println!("{} scan lines remaining", image_height - i);
        for j in 0..image_width {
            let r: f32 = j as f32 / ((image_width - 1) as f32);
            let g: f32 = i as f32 / ((image_height - 1) as f32);
            let b: f32 = 0.0;

            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;

            file.write_all(format!("{} {} {}\n", ir, ig, ib).as_bytes());
        }
    }
    println!("Done!");
    Ok(())
}
