mod color;
mod vector;

fn main() {
    let image_width: i32 = 256;
    let image_height: i32 = 256;

    let _v = vector::Vec3 {
        x: 0.,
        y: 0.,
        z: 0.,
    };

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for j in (0..image_height).rev() {
        eprintln!("Scan lines remaining: {}", j + 1);
        for i in 0..image_width {
            let color = color::Color {
                r: (i as f64) / ((image_width - 1) as f64),
                g: (j as f64) / ((image_height - 1) as f64),
                b: 0.25f64,
            };
            color.write();
        }
    }
    eprintln!("Done.");
}
