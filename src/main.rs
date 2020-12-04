mod camera;
mod color;
mod ray;
mod scene;
mod surface;
mod vector;

use std::io::{self, BufWriter, Write};
use std::sync::Arc;
use std::thread;

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use once_cell::sync::OnceCell;
use rand::distributions::{Distribution, Uniform};

use camera::Camera;
use color::Color;
use ray::Ray;
use scene::Scene;
use surface::{Scatter, Surface};
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

const MAX_REFLECTION_DEPTH: usize = 50;

fn render(
    camera: Arc<Camera>,
    dimensions: (usize, usize),
    world: Arc<dyn Surface>,
    samples_per_pixel: u32,
    progress: ProgressBar,
) -> Vec<Color> {
    let mut ret = Vec::with_capacity(dimensions.0 * dimensions.1);
    let mut rng = rand::thread_rng();
    let dist = Uniform::new(0.0, 1.0);

    for j in (0..dimensions.1).rev() {
        for i in 0..dimensions.0 {
            let mut color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let u = (i as f32 + dist.sample(&mut rng)) / (dimensions.0 as f32 - 1.0);
                let v = (j as f32 + dist.sample(&mut rng)) / (dimensions.1 as f32 - 1.0);
                color.add_samples(&ray_color(
                    &camera.ray_from((u, v)),
                    world.as_ref(),
                    MAX_REFLECTION_DEPTH,
                ));
            }

            ret.push(color);
            progress.inc(samples_per_pixel as u64);
        }
    }

    progress.finish_with_message("Done.");
    ret
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    const ASPECT_RATIO: f32 = 3.0 / 2.0;
    const IMAGE_WIDTH: usize = 400;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as usize;

    const SAMPLES_PER_PIXEL: u32 = 50;

    let world: Arc<dyn Surface> = Arc::new(Scene::random());

    let from = Vector::new(13.0, 2.0, 3.0);
    let at = Vector::new(0.0, 0.0, 0.0);

    let camera = Arc::new(Camera::new(
        from,
        at,
        Vector::new(0.0, 1.0, 0.0),
        std::f32::consts::FRAC_PI_8,
        ASPECT_RATIO,
        0.1,
        10.0,
    ));

    let mut writer = BufWriter::new(io::stdout());

    write!(writer, "P3\n")?;
    write!(writer, "{} {}\n", IMAGE_WIDTH, IMAGE_HEIGHT)?;
    write!(writer, "255\n")?;

    let cpus = num_cpus::get();
    let samples_per_pixel = SAMPLES_PER_PIXEL / cpus as u32;

    let bars = MultiProgress::new();
    let bar_style = ProgressStyle::default_bar()
        .template("[{bar:60.white/white}] {pos}/{len} {msg}")
        .progress_chars("=> ");

    let handles: Vec<_> = (0..cpus)
        .map(|_| {
            let camera_ref = Arc::clone(&camera);
            let world_ref = Arc::clone(&world);
            let bar = bars.add(ProgressBar::new(
                (IMAGE_HEIGHT * IMAGE_WIDTH * samples_per_pixel as usize) as u64,
            ));
            bar.set_style(bar_style.clone());
            thread::spawn(move || {
                render(
                    camera_ref,
                    (IMAGE_WIDTH, IMAGE_HEIGHT),
                    world_ref,
                    samples_per_pixel,
                    bar,
                )
            })
        })
        .collect();

    bars.join()?;

    let mut screen: Vec<Color> = [Default::default()].repeat(IMAGE_WIDTH * IMAGE_HEIGHT);
    handles.into_iter().map(|h| h.join()).for_each(|s| {
        s.unwrap()
            .iter()
            .enumerate()
            .for_each(|(i, c)| screen[i].add_samples(c))
    });

    for color in screen {
        write!(writer, "{}\n", color)?;
    }

    Ok(())
}
