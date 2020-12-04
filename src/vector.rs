use core::ops::{Add, AddAssign, Div, Index, Mul, Neg, Sub};

use once_cell::sync::OnceCell;
use rand::distributions::{Distribution, Uniform};

#[derive(Copy, Clone)]
pub struct Vector([f32; 3]);

impl Vector {
    pub const fn new(e1: f32, e2: f32, e3: f32) -> Self {
        Self([e1, e2, e3])
    }

    pub fn random<D: Distribution<f32>>(dist: &D) -> Self {
        let mut rng = rand::thread_rng();
        Self::new(
            dist.sample(&mut rng),
            dist.sample(&mut rng),
            dist.sample(&mut rng),
        )
    }

    pub fn random_in_unit_sphere() -> Self {
        static DIST: OnceCell<Uniform<f32>> = OnceCell::new();
        let dist = DIST.get_or_init(|| Uniform::new(-1.0, 1.0));
        loop {
            let v = Self::random(dist);
            if v.length_squared() < 1.0 {
                return v;
            }
        }
    }

    pub fn random_unit_vector() -> Self {
        Self::random_in_unit_sphere().unit()
    }

    pub fn dot(&self, rhs: &Vector) -> f32 {
        self[0] * rhs[0] + self[1] * rhs[1] + self[2] * rhs[2]
    }

    pub fn cross(&self, rhs: &Vector) -> Vector {
        Vector::new(
            self[1] * rhs[2] - self[2] * rhs[1],
            self[2] * rhs[0] - self[0] * rhs[2],
            self[0] * rhs[1] - self[1] * rhs[0],
        )
    }

    pub fn length_squared(&self) -> f32 {
        self.dot(self)
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn unit(&self) -> Vector {
        self / self.length()
    }

    pub fn near_zero(&self) -> bool {
        self.0.iter().all(|e| e.abs() < 1e-8)
    }

    pub fn reflect(&self, normal: &Vector) -> Vector {
        *self - 2.0 * self.dot(normal) * normal
    }

    pub fn refract(&self, normal: &Vector, index_ratio: f32) -> Vector {
        let incident = self.unit();
        let refracted_perpendicular =
            index_ratio * (incident + normal.dot(&-incident).min(1.0) * normal);
        let refracted_parallel = (1.0 - refracted_perpendicular.length_squared())
            .abs()
            .sqrt()
            .neg()
            * normal;
        refracted_parallel + refracted_perpendicular
    }
}

impl Index<usize> for Vector {
    type Output = f32;

    fn index(&self, i: usize) -> &Self::Output {
        &self.0[i]
    }
}

impl Mul<f32> for &Vector {
    type Output = Vector;

    fn mul(self, rhs: f32) -> Self::Output {
        let mut ret = self.clone();
        ret.0.iter_mut().for_each(|v| *v *= rhs);
        ret
    }
}

impl Mul<&Vector> for f32 {
    type Output = Vector;

    fn mul(self, rhs: &Vector) -> Self::Output {
        rhs * self
    }
}

impl Mul<f32> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f32) -> Self::Output {
        rhs * &self
    }
}

impl Mul<Vector> for f32 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        rhs * self
    }
}

impl Mul<Vector> for Vector {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        Vector::new(self[0] * rhs[0], self[1] * rhs[1], self[2] * rhs[2])
    }
}

impl Div<f32> for &Vector {
    type Output = Vector;

    fn div(self, rhs: f32) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl Div<f32> for Vector {
    type Output = Vector;

    fn div(self, rhs: f32) -> Self::Output {
        &self / rhs
    }
}

impl Add for &Vector {
    type Output = Vector;

    fn add(self, rhs: Self) -> Self::Output {
        Vector::new(self[0] + rhs[0], self[1] + rhs[1], self[2] + rhs[2])
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, rhs: Self) -> Self::Output {
        &self + &rhs
    }
}

impl Add<f32> for Vector {
    type Output = Vector;

    fn add(self, rhs: f32) -> Self::Output {
        self + Vector::new(rhs, rhs, rhs)
    }
}

impl AddAssign<&Vector> for Vector {
    fn add_assign(&mut self, rhs: &Self) {
        *self = &*self + &rhs;
    }
}

impl Neg for &Vector {
    type Output = Vector;

    fn neg(self) -> Self::Output {
        Vector::new(-self[0], -self[1], -self[2])
    }
}

impl Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Self::Output {
        -&self
    }
}

impl Sub for &Vector {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        self + &-rhs
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        &self - &rhs
    }
}
