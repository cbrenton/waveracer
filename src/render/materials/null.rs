use std::sync::Arc;

use crate::{
    math::Ray,
    render::{HitRecord, Material, ScatterData},
};

#[derive(Clone, Default)]
pub struct NullMaterial {}

impl Material for NullMaterial {
    fn scatter(&self, _ray_in: &Ray, _rec: &HitRecord) -> Option<ScatterData> {
        None
    }
}

pub fn null_material_ptr() -> Arc<NullMaterial> {
    Arc::new(NullMaterial::default())
}
