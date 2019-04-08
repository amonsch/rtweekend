fn main() {
    let nx: i32 = 200;
    let ny: i32 = 100;

    println! {"P3"};
    println! {"{} {}", nx, ny};
    println! {"255"};

    for j in (0..ny).rev() {
        for i in 0..nx {
            let r = (i as f32 / nx as f32) * 255.99;
            let g = (j as f32 / ny as f32) * 255.99;
            let b = 0.2 * 255.99;
            println! {"{} {} {}", r as i32, g as i32, b as i32};
        }
    }
}
