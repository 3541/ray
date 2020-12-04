mod dielectric;
mod lambertian;
mod metal;

pub use dielectric::Dielectric;
pub use lambertian::Lambertian;
pub use metal::Metal;

use super::Hit;
use crate::{Color, Ray};

pub struct Scatter {
    pub ray: Ray,
    pub attenuation: Color,
}

pub trait Material: Sync + Send {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<Scatter>;
}
