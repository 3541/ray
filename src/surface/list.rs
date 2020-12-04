use super::{Hit, Surface};
use crate::Ray;

pub struct SurfaceList<'m>(Vec<Box<dyn Surface + 'm>>);

impl<'m> SurfaceList<'m> {
    pub fn new(surfaces: Vec<Box<dyn Surface + 'm>>) -> Self {
        SurfaceList(surfaces)
    }
}

impl Surface for SurfaceList<'_> {
    fn hit(&self, ray: &Ray, t_range: (f32, f32)) -> Option<Hit> {
        let mut min_dist = t_range.1;
        self.0
            .iter()
            .filter_map(|s| {
                let hit = s.hit(ray, (t_range.0, min_dist))?;
                min_dist = hit.t;
                Some(hit)
            })
            .last()
    }
}
