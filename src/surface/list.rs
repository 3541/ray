use super::{Hit, Surface};
use crate::Ray;

pub struct SurfaceList<'a>(Vec<&'a dyn Surface>);

impl<'a> SurfaceList<'a> {
    pub fn new(surfaces: &[&'a dyn Surface]) -> SurfaceList<'a> {
        SurfaceList(surfaces.into())
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
