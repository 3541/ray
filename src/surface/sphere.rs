use super::{Hit, Material, Surface};
use crate::{Ray, Vector};

pub struct Sphere<'m> {
    center: Vector,
    radius: f32,
    material: &'m dyn Material,
}

impl<'m> Sphere<'m> {
    pub fn new(center: Vector, radius: f32, material: &'m dyn Material) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Surface for Sphere<'_> {
    fn hit(&self, ray: &Ray, t_range: (f32, f32)) -> Option<Hit> {
        let origin_to_center = ray.origin() - &self.center;
        let a = ray.direction().length_squared();
        let half_b = ray.direction().dot(&origin_to_center);
        let c = origin_to_center.dot(&origin_to_center) - &self.radius * &self.radius;

        let delta = half_b * half_b - a * c;
        if delta < 0.0 {
            None
        } else {
            let sqrt_delta = delta.sqrt();

            [-half_b - sqrt_delta, -half_b + sqrt_delta]
                .iter()
                .map(|&r| r / a)
                .filter(|&r| t_range.0 <= r && r <= t_range.1)
                .nth(0)
                .map(|r| {
                    let intersection = ray.at(r);
                    Hit::new(
                        ray,
                        intersection,
                        (intersection - self.center).unit(),
                        self.material,
                        r,
                    )
                })
        }
    }
}
