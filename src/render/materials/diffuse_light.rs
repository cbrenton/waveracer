use std::{fmt, sync::Arc};

use glam::DVec3;

use crate::{
    math::{Color, Ray},
    render::{HitRecord, Material, ScatterData, SolidColor, Texture},
};

#[derive(Clone)]
pub struct DiffuseLight {
    pub tex: Arc<dyn Texture>,
}

impl fmt::Debug for DiffuseLight {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DiffuseLight").finish()
    }
}

impl DiffuseLight {
    pub fn new(tex: Arc<dyn Texture>) -> Self {
        Self { tex }
    }

    pub fn from_color(albedo: Color) -> Self {
        Self {
            tex: Arc::new(SolidColor::new(albedo)),
        }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, _ray_in: &Ray, _rec: &HitRecord) -> Option<ScatterData> {
        None
    }

    fn emitted(&self, u: f64, v: f64, p: DVec3) -> Color {
        self.tex.value(u, v, p)
    }
}
