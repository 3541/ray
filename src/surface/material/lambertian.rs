use std::sync::Arc;

use super::{Material, Scatter};
use crate::{surface::Hit, Color, Ray, Vector};

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Arc<dyn Material> {
        Arc::new(Self { albedo })
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit: &Hit) -> Option<Scatter> {
        let mut direction = hit.normal + Vector::random_unit_vector();
        if direction.near_zero() {
            direction = hit.normal;
        }
        Some(Scatter {
            ray: Ray::new(hit.point, direction),
            attenuation: self.albedo,
        })
    }
}
