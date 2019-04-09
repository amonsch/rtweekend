extern crate rand;

use rtweekend::lib::{
    Camera, Dielectric, ElementWise, HitRecord, Hitable, InnerSpace, Lambertian, Metal, Ray,
    Sphere, Vector3,
};

use std::f32;

fn closest_hit<'a>(ray: &'a Ray, world: &'a [Sphere], t_max: f32) -> Option<HitRecord<'a>> {
    let mut hit: Option<HitRecord> = None;
    let mut closest_so_far = t_max;
    for object in world.iter() {
        if let Some(record) = object.hit(ray, 0.001, closest_so_far) {
            closest_so_far = record.t;
            hit = Some(record);
        }
    }
    hit
}

fn color(ray: &Ray, world: &[Sphere], depth: i32) -> Vector3<f32> {
    let zero = Vector3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    if let Some(record) = closest_hit(&ray, &world, f32::MAX) {
        if depth < 50 {
            if let Some((attenuation, scattered)) = record.material.scatter(ray, &record) {
                return attenuation.mul_element_wise(color(&scattered, world, depth + 1));
            } else {
                return zero;
            }
        } else {
            return zero;
        }
    } else {
        let unit_direction = ray.direction().normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
        return (1.0 - t)
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
    }
}

fn main() {
    let nx: f32 = 400.0;
    let ny: f32 = 200.0;
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
            material: Box::new(Lambertian {
                albedo: Vector3 {
                    x: 0.1,
                    y: 0.2,
                    z: 0.5,
                },
            }),
        },
        Sphere {
            radius: 100.0,
            center: Vector3 {
                x: 0.0,
                y: -100.5,
                z: -1.0,
            },
            material: Box::new(Lambertian {
                albedo: Vector3 {
                    x: 0.8,
                    y: 0.8,
                    z: 0.0,
                },
            }),
        },
        Sphere {
            radius: 0.5,
            center: Vector3 {
                x: 1.0,
                y: 0.0,
                z: -1.0,
            },
            material: Box::new(Metal {
                albedo: Vector3 {
                    x: 0.8,
                    y: 0.6,
                    z: 0.2,
                },
                fuzz: 0.0,
            }),
        },
        Sphere {
            radius: 0.5,
            center: Vector3 {
                x: -1.0,
                y: 0.0,
                z: -1.0,
            },
            material: Box::new(Dielectric { ref_idx: 1.5 }),
        },
        Sphere {
            radius: -0.45,
            center: Vector3 {
                x: -1.0,
                y: 0.0,
                z: -1.0,
            },
            material: Box::new(Dielectric { ref_idx: 1.5 }),
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
                let u: f32 = (i as f32 + rand::random::<f32>()) / nx as f32;
                let v: f32 = (j as f32 + rand::random::<f32>()) / ny as f32;

                let ray = cam.get_ray(u, v);
                col += color(&ray, &world, 0)
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
