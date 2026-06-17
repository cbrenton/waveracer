use crate::render::{SomeHittable, VideoCamera};

pub struct SceneData<T> {
    pub world: Vec<SomeHittable>,
    pub name: String,
    pub camera: VideoCamera<T>,
    // TODO: add accel structure here maybe?
}
