mod lerp;

pub use lerp::*;

use glam::DVec3;

use dyn_clone::DynClone;
use mockall::mock;

pub trait TransformFunc: DynClone {
    // t is a value in range 0.0..1.0
    fn at(&self, t: f64) -> Option<DVec3>;
}

dyn_clone::clone_trait_object!(TransformFunc);

mock! {
    pub TransformFunc {}

    impl TransformFunc for TransformFunc {
        fn at(&self, t: f64) -> Option<DVec3>;
    }

    impl Clone for TransformFunc {
        fn clone(&self) -> Self;
    }
}
