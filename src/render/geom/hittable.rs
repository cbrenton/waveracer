use dyn_clone::DynClone;

use crate::{
    math::{Bounds3, DInterval, Ray},
    render::HitRecord,
};

pub trait Hittable: DynClone {
    fn hit(&self, ray: &Ray, ray_t: DInterval) -> Option<HitRecord>;
    fn aabb(&self) -> Bounds3;
}

dyn_clone::clone_trait_object!(Hittable);

pub type SomeHittable = Box<dyn Hittable>;
