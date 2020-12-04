mod camera;
mod color;
mod ray;
mod scene;
mod surface;
mod vector;

use std::io::{self, BufWriter, Write};
use std::str::FromStr;
use std::sync::Arc;
use std::thread;

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use once_cell::sync::OnceCell;
use rand::distributions::{Distribution, Uniform};
use structopt::StructOpt;

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
    samples_per_pixel: usize,
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

#[derive(StructOpt)]
#[structopt(setting = structopt::clap::AppSettings::DeriveDisplayOrder, rename_all = "kebab-case")]
struct Config {
    #[structopt(short, long, default_value = "400")]
    width: usize,
    #[structopt(short, long, default_value = "225")]
    height: usize,

    #[structopt(short = "p", long, help = "Of the form \"x,y,z\"")]
    camera_pos: Option<Vector>,
    #[structopt(short = "t", long, help = "Where the camera is looking")]
    camera_target: Option<Vector>,
    #[structopt(
        short = "f",
        long,
        default_value = "20.0",
        help = "Vertical field of view, in degrees"
    )]
    camera_fov: f32,
    #[structopt(short = "a", long, default_value = "0.1")]
    camera_aperture: f32,
    #[structopt(long)]
    camera_focus_distance: Option<f32>,

    #[structopt(short, long, default_value = "50")]
    samples: usize,

    #[structopt(short, long)]
    jobs: Option<usize>,

    #[structopt(default_value = "field")]
    scene: SceneName,
}

enum SceneName {
    Field,
}

impl SceneName {
    fn make(&self) -> Scene {
        match self {
            Self::Field => Scene::field(),
        }
    }

    fn camera_default_pos(&self) -> Vector {
        match self {
            Self::Field => Vector::new(13.0, 2.0, 3.0),
        }
    }

    fn camera_default_target(&self) -> Vector {
        match self {
            Self::Field => Vector::new(0.0, 0.0, 0.0),
        }
    }
}

impl FromStr for SceneName {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_lowercase().as_str() {
            "field" => Self::Field,
            _ => Err("Unknown scene.")?,
        })
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::from_args();

    let world: Arc<dyn Surface> = Arc::new(config.scene.make());

    let aspect_ratio = config.width as f32 / config.height as f32;

    let from = config
        .camera_pos
        .unwrap_or(config.scene.camera_default_pos());
    let at = config
        .camera_target
        .unwrap_or(config.scene.camera_default_target());

    let camera = Arc::new(Camera::new(
        from,
        at,
        Vector::new(0.0, 1.0, 0.0),
        config.camera_fov.to_radians(),
        aspect_ratio,
        config.camera_aperture,
        config.camera_focus_distance.unwrap_or((from - at).length()),
    ));

    let mut writer = BufWriter::new(io::stdout());

    write!(writer, "P3\n")?;
    write!(writer, "{} {}\n", config.width, config.height)?;
    write!(writer, "255\n")?;

    let jobs = config.jobs.unwrap_or(num_cpus::get());
    let samples_per_pixel = config.samples / jobs;

    let bars = MultiProgress::new();
    let bar_style = ProgressStyle::default_bar()
        .template("[{bar:60.white/white}] {pos}/{len} {msg}")
        .progress_chars("=> ");

    let handles: Vec<_> = (0..jobs)
        .map(|_| {
            let dim = (config.width, config.height);
            let camera_ref = Arc::clone(&camera);
            let world_ref = Arc::clone(&world);
            let bar = bars.add(ProgressBar::new(
                (config.width * config.height * samples_per_pixel as usize) as u64,
            ));
            bar.set_style(bar_style.clone());
            thread::spawn(move || render(camera_ref, dim, world_ref, samples_per_pixel, bar))
        })
        .collect();

    bars.join()?;

    let mut screen: Vec<Color> = [Default::default()].repeat(config.width * config.height);
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
