use super::{Material, Scatter};
use crate::{surface::Hit, Color, Ray};

pub struct Dielectric {
    refractive_index: f32,
}

impl Dielectric {
    pub fn new(refractive_index: f32) -> Self {
        Self { refractive_index }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<Scatter> {
        let cosine = hit.normal.dot(&-ray.direction().unit()).min(1.0);
        let sine = (1.0 - cosine * cosine).sqrt();
        let index_ratio = if hit.front_face {
            1.0 / self.refractive_index
        } else {
            self.refractive_index
        };

        Some(Scatter {
            ray: Ray::new(
                hit.point,
                if index_ratio * sine > 1.0 {
                    // Total internal reflection
                    ray.direction().unit().reflect(&hit.normal)
                } else {
                    ray.direction().unit().refract(&hit.normal, index_ratio)
                },
            ),
            attenuation: Color::new(1.0, 1.0, 1.0),
        })
    }
}
