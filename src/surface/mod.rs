mod list;
pub mod material;
mod sphere;

pub use list::SurfaceList;
pub use material::{Material, Scatter};
pub use sphere::Sphere;

use crate::{Ray, Vector};

pub struct Hit<'m> {
    point: Vector,
    normal: Vector,
    material: &'m dyn Material,
    t: f32,
    front_face: bool,
}

impl<'m> Hit<'m> {
    pub fn new(
        ray: &Ray,
        point: Vector,
        outward_normal: Vector,
        material: &'m dyn Material,
        t: f32,
    ) -> Hit<'m> {
        assert!(outward_normal.length() - 1.0 <= 1e-4);
        let front_face = ray.direction().dot(&outward_normal) < 0.0;
        Hit {
            point,
            normal: if front_face {
                outward_normal
            } else {
                -outward_normal
            },
            material,
            t,
            front_face,
        }
    }

    pub fn material(&self) -> &dyn Material {
        self.material
    }
}

pub trait Surface {
    fn hit(&self, ray: &Ray, t_range: (f32, f32)) -> Option<Hit>;
}
