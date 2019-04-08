use rtweekend::lib::{InnerSpace, Ray, Vector3};

fn color(ray: &Ray) -> Vector3<f32> {
    let unit_direction = ray.direction().normalize();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t)
        * Vector3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        }
        + t * Vector3 {
            x: 0.5,
            y: 0.7,
            z: 1.0,
        }
}

fn main() {
    let nx: f32 = 200.0;
    let ny: f32 = 100.0;

    println!("P3");
    println!("{} {}", nx, ny);
    println!("255");

    let lower_left_corner = Vector3 {
        x: -2.0,
        y: -1.0,
        z: -1.0,
    };
    let horizontal = Vector3 {
        x: 4.0,
        y: 0.0,
        z: 0.0,
    };
    let vertical = Vector3 {
        x: 0.0,
        y: 2.0,
        z: 0.0,
    };
    let origin = Vector3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    for j in (0..ny as i32).rev() {
        for i in 0..nx as i32 {
            let u: f32 = i as f32 / nx as f32;
            let v: f32 = j as f32 / ny as f32;

            let ray = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical,
            );

            let col = color(&ray) * 255.99;

            println!("{} {} {}", col.x as i32, col.y as i32, col.z as i32);
        }
    }
}
