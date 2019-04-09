extern crate rand;

use rtweekend::lib::{Camera, Hitable, InnerSpace, Ray, Sphere, Vector3};
use std::f32;

use rand::{thread_rng, Rng, ThreadRng};

fn random_in_unit_sphere(rng: &mut ThreadRng) -> Vector3<f32> {
    let mut p: Vector3<f32>;
    loop {
        p =
            2.0 * Vector3 {
                x: rng.gen_range(0.0, 0.99999) as f32,
                y: rng.gen_range(0.0, 0.99999) as f32,
                z: rng.gen_range(0.0, 0.99999) as f32,
            } - Vector3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            };

        if p.magnitude2() >= 1.0 {
            break;
        }
    }
    p
}

fn color(ray: &Ray, world: &[Sphere]) -> Vector3<f32> {
    let unit_direction = ray.direction().normalize();
    let t = 0.5 * (unit_direction.y + 1.0);
    let final_color = (1.0 - t)
        * Vector3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        }
        + t * Vector3 {
            x: 0.5,
            y: 0.7,
            z: 1.0,
        };

    let mut rng = thread_rng();
    for object in world.iter() {
        if let Some(record) = object.hit(ray, 0.0, f32::MAX) {
            let target = record.normal + record.p + random_in_unit_sphere(&mut rng);
            let ray = Ray {
                a: record.p,
                b: target - record.p,
            };
            return 0.5 * color(&ray, world);
        }
    }

    final_color
}

fn main() {
    let mut rng = thread_rng();

    let nx: f32 = 200.0;
    let ny: f32 = 100.0;
    let ns: f32 = 100.0;

    println!("P3");
    println!("{} {}", nx, ny);
    println!("255");

    let cam = Camera {
        lower_left_corner: Vector3 {
            x: -2.0,
            y: -1.0,
            z: -1.0,
        },
        horizontal: Vector3 {
            x: 4.0,
            y: 0.0,
            z: 0.0,
        },
        vertical: Vector3 {
            x: 0.0,
            y: 2.0,
            z: 0.0,
        },
        origin: Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
    };

    let world = vec![
        Sphere {
            radius: 0.5,
            center: Vector3 {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            },
        },
        Sphere {
            radius: 100.0,
            center: Vector3 {
                x: 0.0,
                y: -100.5,
                z: -1.0,
            },
        },
    ];

    for j in (0..ny as i32).rev() {
        for i in 0..nx as i32 {
            let mut col = Vector3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            };
            for _ in 0..ns as i32 {
                let u: f32 = (i as f32 + rng.gen_range(0.001, 0.99999)) / nx as f32;
                let v: f32 = (j as f32 + rng.gen_range(0.001, 0.99999)) / ny as f32;

                let ray = cam.get_ray(u, v);
                col += color(&ray, &world)
            }
            col /= ns;
            println!(
                "{} {} {}",
                (col.x.sqrt() * 255.99) as i32,
                (col.y.sqrt() * 255.99) as i32,
                (col.z.sqrt() * 255.99) as i32,
            );
        }
    }
}
