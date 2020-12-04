use std::sync::Arc;

use rand::distributions::{Distribution, Uniform};
use rand::seq::SliceRandom;

use crate::surface::{material, Hit, Material, Sphere, Surface, SurfaceList};
use crate::{random_unit, Color, Ray, Vector};

pub struct Scene {
    surfaces: SurfaceList<'static>,
}

impl Scene {
    pub fn field() -> Self {
        let mut surfaces: Vec<Box<dyn Surface>> = Vec::with_capacity(500);

        let metal_color_dist = Uniform::new(0.5, 1.0);
        let fuzz_dist = Uniform::new(0.0, 0.5);

        let mut rng = rand::thread_rng();

        let mat_glass = material::Dielectric::new(1.5);

        let make_materials: [(_, Box<dyn Fn() -> Arc<dyn Material>>); 3] = [
            (
                0.8,
                Box::new(|| {
                    material::Lambertian::new(
                        (Vector::random_in_unit_range() * Vector::random_in_unit_range()).into(),
                    )
                }),
            ),
            (
                0.15,
                Box::new(|| {
                    material::Metal::new(
                        Vector::random(&metal_color_dist).into(),
                        fuzz_dist.sample(&mut rand::thread_rng()),
                    )
                }),
            ),
            (0.05, Box::new(|| Arc::clone(&mat_glass))),
        ];

        surfaces.push(Sphere::new(
            Vector::new(0.0, -1000.0, 0.0),
            1000.0,
            &material::Lambertian::new(Color::new(0.5, 0.5, 0.5)),
        ));

        for x in -11..11 {
            for z in -11..11 {
                let center = Vector::new(
                    x as f32 + 0.9 * random_unit(),
                    0.2,
                    z as f32 + 0.9 * random_unit(),
                );

                if (center - Vector::new(4.0, 0.2, 0.0)).length() <= 0.9 {
                    continue;
                }

                let mat = make_materials.choose_weighted(&mut rng, |m| m.0).unwrap().1();
                surfaces.push(Sphere::new(center, 0.2, &mat));
            }
        }

        surfaces.push(Sphere::new(Vector::new(0.0, 1.0, 0.0), 1.0, &mat_glass));

        surfaces.push(Sphere::new(
            Vector::new(-4.0, 1.0, 0.0),
            1.0,
            &material::Lambertian::new(Color::new(0.4, 0.2, 0.1)),
        ));

        surfaces.push(Sphere::new(
            Vector::new(4.0, 1.0, 0.0),
            1.0,
            &material::Metal::new(Color::new(0.7, 0.6, 0.5), 0.0),
        ));

        Scene {
            surfaces: SurfaceList::new(surfaces),
        }
    }
}

impl Surface for Scene {
    fn hit(&self, ray: &Ray, t_range: (f32, f32)) -> Option<Hit> {
        self.surfaces.hit(ray, t_range)
    }
}
