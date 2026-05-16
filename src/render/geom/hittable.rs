use crate::{
    math::{Bounds3, DInterval, Ray},
    render::{HitRecord, Triangle},
};

// TODO: add triangle, hittablelist, bvhnode, etc
pub enum Hittable {
    Dummy,
    Triangle(Triangle),
}

impl Hittable {
    pub fn hit(&self, ray: &Ray, ray_t: DInterval) -> Option<HitRecord> {
        match self {
            Hittable::Dummy => None,
            Hittable::Triangle(tri) => tri.hit(ray, ray_t),
        }
    }

    pub fn aabb(&self) -> Bounds3 {
        match self {
            Hittable::Dummy => Bounds3::EMPTY,
            Hittable::Triangle(tri) => tri.aabb(),
        }
    }
}

#[cfg(test)]
mod tests {
    use glam::DVec3;

    use super::*;

    #[test]
    fn test_hit_returns_none() {
        let foo = Hittable::Dummy;
        let ray = Ray::new(DVec3::ZERO, DVec3::new(0.0, 0.0, -1.0));
        let ray_t = DInterval::UNIVERSE;
        assert!(foo.hit(&ray, ray_t).is_none());
    }
}
