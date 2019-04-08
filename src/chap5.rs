use rtweekend::lib::{Hitable, InnerSpace, Ray, Sphere, Vector3};
use std::f32;

fn color<T>(ray: &Ray, world: &[T], t_max: f32) -> Vector3<f32>
where
    T: Hitable,
{
    let unit_direction = ray.direction().normalize();
    let t = 0.5 * (unit_direction.y + 1.0);
    let mut final_color = (1.0 - t)
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

    let mut closest_so_far = t_max;
    for object in world.iter() {
        if let Some(record) = object.hit(ray, 0.0, closest_so_far) {
            closest_so_far = record.t;
            final_color = (record.normal
                + Vector3 {
                    x: 1.0,
                    y: 1.0,
                    z: 1.0,
                })
                * 0.5;
        }
    }

    final_color
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
            let u: f32 = i as f32 / nx as f32;
            let v: f32 = j as f32 / ny as f32;

            let ray = Ray::new(
                origin,
                lower_left_corner + (u * horizontal) + (v * vertical),
            );

            let col = color(&ray, &world, f32::MAX) * 255.99;
            println!("{} {} {}", col.x as i32, col.y as i32, col.z as i32);
        }
    }
}
