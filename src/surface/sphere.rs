use std::sync::Arc;

use super::{Hit, Material, Surface};
use crate::{Ray, Vector};

pub struct Sphere {
    center: Vector,
    radius: f32,
    material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vector, radius: f32, material: &Arc<dyn Material>) -> Box<dyn Surface> {
        Box::new(Self {
            center,
            radius,
            material: Arc::clone(material),
        })
    }
}

impl Surface for Sphere {
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
                        ((intersection - self.center) / self.radius).unit(),
                        self.material.as_ref(),
                        r,
                    )
                })
        }
    }
}
