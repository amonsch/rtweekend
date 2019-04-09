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

    fn refract(v: Vector3<f32>, n: Vector3<f32>, ni_over_nt: f32) -> Option<Vector3<f32>> {
        let uv = v.normalize();
        let dt = uv.dot(n);
        let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
        if discriminant > 0.0 {
            return Some(ni_over_nt * (uv - n * dt) - n * discriminant.sqrt());
        }

        None
    }

    fn schlick(cosine: f32, ref_idx: f32) -> f32 {
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 *= r0;
        r0 + (1.0 - r0) * f32::powf(1.0 - cosine, 5.0)
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

    pub struct Dielectric {
        pub ref_idx: f32,
    }

    impl PartialEq for Ray {
        fn eq(&self, other: &Ray) -> bool {
            self.a == other.a && self.b == other.b
        }
    }

    impl Material for Dielectric {
        fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<(Vector3<f32>, Ray)> {
            let ni_over_nt: f32;
            let outward_normal: Vector3<f32>;
            let reflected = reflect(ray.direction(), record.normal);
            let cosine: f32;
            let angle = ray.direction().dot(record.normal);
            if angle > 0.0 {
                outward_normal = -record.normal;
                ni_over_nt = self.ref_idx;
                cosine = self.ref_idx * angle / ray.direction().magnitude2();
            } else {
                outward_normal = record.normal;
                ni_over_nt = 1.0 / self.ref_idx;
                cosine = -angle / ray.direction().magnitude2();
            }

            let mut scattered: Option<Ray> = None::<Ray>;
            if let Some(refracted) = refract(ray.direction(), outward_normal, ni_over_nt) {
                if rand::random::<f32>() < schlick(cosine, self.ref_idx) {
                    scattered = Some(Ray {
                        a: record.p,
                        b: refracted,
                    });
                }
            }

            if scattered == None::<Ray> {
                scattered = Some(Ray {
                    a: record.p,
                    b: reflected,
                });
            }

            Some((
                Vector3 {
                    x: 1.0,
                    y: 1.0,
                    z: 1.0,
                },
                scattered.unwrap(),
            ))
        }
    }
}
