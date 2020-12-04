use crate::{Ray, Vector};

pub struct Camera {
    origin: Vector,
    lower_left: Vector,
    horizontal: Vector,
    vertical: Vector,
    basis: (Vector, Vector, Vector),
    lens_radius: f32,
}

impl Camera {
    pub fn new(
        look_from: Vector,
        look_at: Vector,
        up: Vector,
        fov: f32,
        aspect_ratio: f32,
        aperture: f32,
        focus_dist: f32,
    ) -> Self {
        let viewport_height = 2.0 * (fov / 2.0).tan();
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from - look_at).unit();
        let u = up.cross(&w).unit();
        let v = w.cross(&u);

        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        Self {
            origin: look_from,
            horizontal,
            vertical,
            lower_left: look_from - horizontal / 2.0 - vertical / 2.0 - focus_dist * w,
            basis: (u, v, w),
            lens_radius: aperture / 2.0,
        }
    }

    pub fn ray_from(&self, pos: (f32, f32)) -> Ray {
        let lens_pos = self.lens_radius * Vector::random_in_unit_disk();
        let offset = self.basis.0 * lens_pos[0] + self.basis.1 * lens_pos[1];
        Ray::new(
            self.origin + offset,
            self.lower_left + pos.0 * self.horizontal + pos.1 * self.vertical
                - self.origin
                - offset,
        )
    }
}
