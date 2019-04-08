pub mod lib {
    pub use cgmath::prelude::InnerSpace;
    pub use cgmath::Vector3;

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
    pub struct HitRecord {
        pub t: f32,
        pub p: Vector3<f32>,
        pub normal: Vector3<f32>,
    }

    pub trait Hitable {
        fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
    }

    pub struct Sphere {
        pub center: Vector3<f32>,
        pub radius: f32,
    }

    impl Hitable for Sphere {
        fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
            let mut result: Option<HitRecord> = None;

            let oc = ray.origin() - self.center;
            let a = ray.direction().dot(ray.direction());
            let b = oc.dot(ray.direction());
            let c = oc.dot(oc) - self.radius * self.radius;
            let discriminant = b * b - a * c;
            if discriminant > 0.0 {
                let mut temp = (-b - discriminant.sqrt()) / a;
                if temp < t_max && temp > t_min {
                    let p = ray.point_at_parameter(temp);
                    let hit = HitRecord {
                        t: temp,
                        p,
                        normal: (p - self.center) / self.radius,
                    };
                    result = Some(hit);
                }

                temp = (-b + discriminant.sqrt()) / a;
                if temp < t_max && temp > t_min {
                    let p = ray.point_at_parameter(temp);
                    let hit = HitRecord {
                        t: temp,
                        p,
                        normal: (p - self.center) / self.radius,
                    };
                    result = Some(hit);
                }
            }

            result
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

}
