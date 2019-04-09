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

fn random_scene() -> Vec<Sphere> {
    let mut scene = vec![Sphere {
        center: Vector3 {
            x: 0.0,
            y: -1000.0,
            z: 0.0,
        },
        radius: 1000.0,
        material: Box::new(Lambertian {
            albedo: Vector3 {
                x: 0.5,
                y: 0.5,
                z: 0.5,
            },
        }),
    }];

    for a in -11..11 {
        for b in -11..11 {
            let material = rand::random::<f32>();
            let center = Vector3 {
                x: a as f32 + 0.9 * rand::random::<f32>(),
                y: 0.2,
                z: b as f32 + 0.9 * rand::random::<f32>(),
            };

            if (center
                - Vector3 {
                    x: 4.0,
                    y: 0.2,
                    z: 0.0,
                })
            .magnitude2()
                > 0.9
            {
                if material < 0.8 {
                    scene.push(Sphere {
                        center,
                        radius: 0.2,
                        material: Box::new(Lambertian {
                            albedo: Vector3 {
                                x: rand::random::<f32>().powf(2.0),
                                y: rand::random::<f32>().powf(2.0),
                                z: rand::random::<f32>().powf(2.0),
                            },
                        }),
                    });
                } else if material < 0.95 {
                    scene.push(Sphere {
                        center,
                        radius: 0.2,
                        material: Box::new(Metal {
                            albedo: Vector3 {
                                x: 0.5 * (1.0 + rand::random::<f32>()),
                                y: 0.5 * (1.0 + rand::random::<f32>()),
                                z: 0.5 * (1.0 + rand::random::<f32>()),
                            },
                            fuzz: rand::random::<f32>() * 0.5,
                        }),
                    });
                } else {
                    scene.push(Sphere {
                        center,
                        radius: 0.2,
                        material: Box::new(Dielectric { ref_idx: 1.5 }),
                    });
                }
            }
        }
    }

    scene
}

fn main() {
    let nx: f32 = 200.0;
    let ny: f32 = 100.0;
    let ns: f32 = 100.0;

    println!("P3");
    println!("{} {}", nx, ny);
    println!("255");

    let lookfrom = Vector3 {
        x: 3.0,
        y: 3.0,
        z: 2.0,
    };
    let lookat = Vector3 {
        x: 0.0,
        y: 0.0,
        z: -1.0,
    };

    let cam = Camera::new(
        lookfrom,
        lookat,
        Vector3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        20.0,
        nx / ny,
        2.0,
        (lookfrom - lookat).magnitude(),
    );

    let world = random_scene();

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
