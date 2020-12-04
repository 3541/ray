use crate::{Ray, Vector};

pub struct Camera {
    origin: Vector,
    lower_left: Vector,
    horizontal: Vector,
    vertical: Vector,
}

impl Camera {
    pub fn new(
        look_from: Vector,
        look_at: Vector,
        up: Vector,
        fov: f32,
        aspect_ratio: f32,
    ) -> Self {
        let viewport_height = 2.0 * (fov / 2.0).tan();
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from - look_at).unit();
        let u = up.cross(&w).unit();
        let v = w.cross(&u);

        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        Self {
            origin: look_from,
            horizontal,
            vertical,
            lower_left: look_from - horizontal / 2.0 - vertical / 2.0 - w,
        }
    }

    pub fn ray_from(&self, pos: (f32, f32)) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left + pos.0 * self.horizontal + pos.1 * self.vertical - self.origin,
        )
    }
}
