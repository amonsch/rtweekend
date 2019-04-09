pub mod lib {
    pub use cgmath::prelude::{ElementWise, InnerSpace};
    pub use cgmath::Vector3;
    use rand;

    pub struct Ray {
        pub a: Vector3<f32>,
        pub b: Vector3<f32>,
    }

    impl Ray {
        pub fn new(a: Vector3<f32>, b: Vector3<f32>) -> Ray {
            Ray { a, b }
        }

        pub fn origin(&self) -> Vector3<f32> {
            self.a
        }

        pub fn direction(&self) -> Vector3<f32> {
            self.b
        }

        pub fn point_at_parameter(&self, t: f32) -> Vector3<f32> {
            self.a + self.b * t
        }
    }

    pub struct HitRecord<'a> {
        pub t: f32,
        pub p: Vector3<f32>,
        pub normal: Vector3<f32>,
        pub material: &'a Box<Material>,
    }

    pub trait Hitable {
        fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
    }

    pub struct Sphere {
        pub center: Vector3<f32>,
        pub radius: f32,
        pub material: Box<Material>,
    }

    impl Hitable for Sphere {
        fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
            let oc = ray.origin() - self.center;
            let a = ray.direction().dot(ray.direction());
            let b = oc.dot(ray.direction());
            let c = oc.dot(oc) - self.radius * self.radius;
            let discriminant = b * b - a * c;
            if discriminant > 0.0 {
                let mut temp = (-b - discriminant.sqrt()) / a;
                if temp < t_max && temp > t_min {
                    let point = ray.point_at_parameter(temp);
                    let hit = HitRecord {
                        t: temp,
                        p: point,
                        normal: (point - self.center) / self.radius,
                        material: &self.material,
                    };
                    return Some(hit);
                }

                temp = (-b + discriminant.sqrt()) / a;
                if temp < t_max && temp > t_min {
                    let point = ray.point_at_parameter(temp);
                    let hit = HitRecord {
                        t: temp,
                        p: point,
                        normal: (point - self.center) / self.radius,
                        material: &self.material,
                    };
                    return Some(hit);
                }
            }
            None
        }
    }

    pub struct Camera {
        pub origin: Vector3<f32>,
        pub lower_left_corner: Vector3<f32>,
        pub horizontal: Vector3<f32>,
        pub vertical: Vector3<f32>,
    }

    impl Camera {
        pub fn get_ray(&self, u: f32, v: f32) -> Ray {
            Ray {
                a: self.origin,
                b: self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
            }
        }
    }

    fn random_in_unit_sphere() -> Vector3<f32> {
        let mut point: Vector3<f32>;
        loop {
            point =
                2.0 * Vector3 {
                    x: rand::random::<f32>(),
                    y: rand::random::<f32>(),
                    z: rand::random::<f32>(),
                } - Vector3 {
                    x: 1.0,
                    y: 1.0,
                    z: 1.0,
                };

            if point.magnitude2() >= 1.0 {
                break;
            }
        }
        point
    }

    fn reflect(v: Vector3<f32>, n: Vector3<f32>) -> Vector3<f32> {
        v - 2.0 * v.dot(n) * n
    }

    pub trait Material {
        fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<(Vector3<f32>, Ray)>;
    }

    pub struct Lambertian {
        pub albedo: Vector3<f32>,
    }

    impl Material for Lambertian {
        fn scatter(&self, _ray: &Ray, record: &HitRecord) -> Option<(Vector3<f32>, Ray)> {
            // let mut rng = thread_rng();
            let target = record.p + record.normal + random_in_unit_sphere();
            let scattered = Ray {
                a: record.p,
                b: target - record.p,
            };
            if scattered.direction().dot(record.normal) > 0.0 {
                return Some((self.albedo, scattered));
            }
            None
        }
    }

    pub struct Metal {
        pub fuzz: f32,
        pub albedo: Vector3<f32>,
    }

    impl Material for Metal {
        fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<(Vector3<f32>, Ray)> {
            let reflected = reflect(ray.direction().normalize(), record.normal);
            let scattered = Ray {
                a: record.p,
                b: reflected + self.fuzz * random_in_unit_sphere(),
            };
            if scattered.direction().dot(record.normal) > 0.0 {
                return Some((self.albedo, scattered));
            }
            None
        }
    }
}
