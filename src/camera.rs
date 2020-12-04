use crate::{Ray, Vector};

pub struct Camera {
    origin: Vector,
    lower_left: Vector,
    horizontal: Vector,
    vertical: Vector,
}

impl Camera {
    pub fn new() -> Self {
        const ASPECT_RATIO: f32 = 16.0 / 9.0;
        const VIEWPORT_HEIGHT: f32 = 2.0;
        const VIEWPORT_WIDTH: f32 = VIEWPORT_HEIGHT * ASPECT_RATIO;
        const FOCAL_LENGTH: f32 = 1.0;

        let origin = Vector::new(0.0, 0.0, 0.0);
        let horizontal = Vector::new(VIEWPORT_WIDTH, 0.0, 0.0);
        let vertical = Vector::new(0.0, VIEWPORT_HEIGHT, 0.0);
        Self {
            origin,
            horizontal,
            vertical,
            lower_left: origin
                - horizontal / 2.0
                - vertical / 2.0
                - Vector::new(0.0, 0.0, FOCAL_LENGTH),
        }
    }

    pub fn ray_from(&self, pos: (f32, f32)) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left + pos.0 * self.horizontal + pos.1 * self.vertical - self.origin,
        )
    }
}
