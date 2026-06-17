use crate::{
    math::{Bounds3, DInterval, Ray},
    render::{HitRecord, Hittable},
};

#[derive(Clone)]
pub struct Empty {}

impl Hittable for Empty {
    fn hit(&self, _ray: &Ray, _ray_t: DInterval) -> Option<HitRecord> {
        None
    }

    fn aabb(&self) -> Bounds3 {
        Bounds3::EMPTY
    }
}

impl Default for Empty {
    fn default() -> Self {
        Self {}
    }
}
