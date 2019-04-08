use cgmath::Vector3;

fn main() {
    let nx: i32 = 200;
    let ny: i32 = 100;

    println! {"P3"};
    println! {"{} {}", nx, ny};
    println! {"255"};

    for j in (0..ny).rev() {
        for i in 0..nx {
            let vec = Vector3 {
                x: (i as f32 / nx as f32) * 255.99,
                y: (j as f32 / ny as f32) * 255.99,
                z: 0.2 * 255.99,
            };

            println!("{} {} {}", vec.x as i32, vec.y as i32, vec.z as i32,);
        }
    }
}
