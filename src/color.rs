use core::fmt::{self, Display};
use core::ops::Deref;

use derive_more::{Add, Mul};

use crate::vector::Vector;

#[derive(Mul, Add, Copy, Clone)]
pub struct Color {
    value: Vector,
    samples: usize,
}

impl Color {
    pub const fn new(r: f32, g: f32, b: f32) -> Self {
        Self {
            value: Vector::new(r, g, b),
            samples: 1,
        }
    }

    fn scaled(&self) -> Vector {
        self.value / self.samples as f32
    }

    fn bytes(&self) -> (u8, u8, u8) {
        let scaled = self.scaled();
        (
            (scaled[0].sqrt().clamp(0.0, 0.999) * 256.0) as u8,
            (scaled[1].sqrt().clamp(0.0, 0.999) * 256.0) as u8,
            (scaled[2].sqrt().clamp(0.0, 0.999) * 256.0) as u8,
        )
    }

    pub fn add_samples(&mut self, other: &Color) {
        self.value += &other.value;
        self.samples += other.samples;
    }
}

impl Default for Color {
    fn default() -> Self {
        Self {
            value: Vector::new(0.0, 0.0, 0.0),
            samples: 0,
        }
    }
}

impl From<Vector> for Color {
    fn from(v: Vector) -> Self {
        Color {
            value: v,
            samples: 1,
        }
    }
}

impl Deref for Color {
    type Target = Vector;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (r, g, b) = self.bytes();
        write!(f, "{} {} {}", r, g, b)
    }
}
