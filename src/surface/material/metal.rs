use super::{Material, Scatter};
use crate::{surface::Hit, Color, Ray, Vector};

pub struct Metal {
    albedo: Color,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f32) -> Self {
        Self {
            albedo,
            fuzz: fuzz.clamp(0.0, 1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<Scatter> {
        let reflected = ray.direction().unit().reflect(&hit.normal);
        if reflected.dot(&hit.normal) <= 0.0 {
            None
        } else {
            Some(Scatter {
                ray: Ray::new(
                    hit.point,
                    reflected + self.fuzz * Vector::random_in_unit_sphere(),
                ),
                attenuation: self.albedo,
            })
        }
    }
}
