use crate::{
    math::{DInterval, Ray},
    render::HitRecord,
};

// TODO: add triangle, hittablelist, bvhnode, etc
pub enum Hittable {
    Dummy,
}

impl Hittable {
    pub fn hit(&self, ray: &Ray, ray_t: DInterval) -> Option<HitRecord> {
        match self {
            Hittable::Dummy => None,
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
