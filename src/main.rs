fn main() {
    let width = 256;
    let height = 256;

    println!("P3\n{} {}\n255", width, height);

    for i in 0..height {
        eprintln!("Scanlines remaining: {}", height - i);
        for j in 0..width {
            let r = j as f64 / (width - 1) as f64;
            let g = i as f64 / (height - 1) as f64;
            let b = 0.0;

            let ir = (255.99 * r) as u8;
            let ig = (255.99 * g) as u8;
            let ib = (255.99 * b) as u8;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
