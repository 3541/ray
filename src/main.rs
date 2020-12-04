mod camera;
mod color;
mod ray;
mod surface;
mod vector;

use std::io::{self, BufWriter, Write};

use once_cell::sync::OnceCell;
use rand::distributions::{Distribution, Uniform};
use rand::Rng;

use camera::Camera;
use color::Color;
use ray::Ray;
use surface::{material, Scatter, Sphere, Surface, SurfaceList};
use vector::Vector;

fn random_unit() -> f32 {
    static DIST: OnceCell<Uniform<f32>> = OnceCell::new();
    DIST.get_or_init(|| Uniform::new(0.0, 1.0))
        .sample(&mut rand::thread_rng())
}

fn ray_color(ray: &Ray, world: &dyn Surface, depth: usize) -> Color {
    if depth == 0 {
        Color::new(0.0, 0.0, 0.0)
    } else if let Some(hit) = world.hit(ray, (1e-3, f32::INFINITY)) {
        if let Some(Scatter {
            ray: ref scattered,
            attenuation,
        }) = hit.material().scatter(ray, &hit)
        {
            (*ray_color(scattered, world, depth - 1) * *attenuation).into()
        } else {
            Color::new(0.0, 0.0, 0.0)
        }
    } else {
        let direction = ray.direction().unit();
        let t = (direction[1] + 1.0) / 2.0;

        (Vector::new(1.0, 1.0, 1.0) * (1.0 - t) + Vector::new(0.5, 0.7, 1.0) * t).into()
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const IMAGE_WIDTH: usize = 400;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as usize;

    const SAMPLES_PER_PIXEL: u32 = 100;
    const MAX_REFLECTION_DEPTH: usize = 50;

    let mut rng = rand::thread_rng();

    let mat_ground = material::Lambertian::new(Color::new(0.8, 0.8, 0.0));
    let mat_center = material::Lambertian::new(Color::new(0.1, 0.2, 0.5));
    let mat_left = material::Dielectric::new(1.5);
    let mat_right = material::Metal::new(Color::new(0.8, 0.6, 0.2), 0.0);

    let objects = vec![
        Sphere::new(Vector::new(0.0, -100.5, -1.0), 100.0, &mat_ground),
        Sphere::new(Vector::new(0.0, 0.0, -1.0), 0.5, &mat_center),
        Sphere::new(Vector::new(-1.0, 0.0, -1.0), 0.5, &mat_left),
        Sphere::new(Vector::new(-1.0, 0.0, -1.0), -0.4, &mat_left),
        Sphere::new(Vector::new(1.0, 0.0, -1.0), 0.5, &mat_right),
    ];

    let world = SurfaceList::new(
        &objects
            .iter()
            .map(|s| s as &dyn Surface)
            .collect::<Vec<&dyn Surface>>(),
    );

    let camera = Camera::new();

    let mut writer = BufWriter::new(io::stdout());

    write!(writer, "P3\n")?;
    write!(writer, "{} {}\n", IMAGE_WIDTH, IMAGE_HEIGHT)?;
    write!(writer, "255\n")?;

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        for i in 0..IMAGE_WIDTH {
            let mut color = Color::new(0.0, 0.0, 0.0);

            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f32 + rng.gen_range(0.0, 1.0)) / (IMAGE_WIDTH as f32 - 1.0);
                let v = (j as f32 + rng.gen_range(0.0, 1.0)) / (IMAGE_HEIGHT as f32 - 1.0);
                color.add_samples(&ray_color(
                    &camera.ray_from((u, v)),
                    &world,
                    MAX_REFLECTION_DEPTH,
                ));
            }

            write!(writer, "{}\n", color)?;
        }
    }
    eprintln!("");

    Ok(())
}
