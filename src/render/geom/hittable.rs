use dyn_clone::DynClone;
use mockall::{automock, mock};

use crate::{
    math::{Bounds3, DInterval, Ray},
    render::HitRecord,
};

pub type SomeHittable = Box<dyn Hittable>;

pub trait Hittable: DynClone {
    fn hit(&self, ray: &Ray, ray_t: DInterval) -> Option<HitRecord>;
    fn aabb(&self) -> Bounds3;
}

dyn_clone::clone_trait_object!(Hittable);

mock! {
    pub Hittable {}

    impl Hittable for Hittable {
        fn hit(&self, ray: &Ray, ray_t: DInterval) -> Option<HitRecord>;
        fn aabb(&self) -> Bounds3;
    }

    impl Clone for Hittable {
        fn clone(&self) -> Self;
    }
}
