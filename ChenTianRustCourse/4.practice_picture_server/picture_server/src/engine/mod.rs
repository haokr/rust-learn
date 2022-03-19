use crate::pb::Spec;
use image::ImageOutputFormat;

mod photon;
pub use photon::Photon;

// Engine trait：未来可以添加更多的 engine，主流程只需要替换 engine
pub trait Engine {
    fn apply(&mut self, specs: &[Spec]);
    fn generate(self, format: ImageOutputFormat) -> Vec<u8>;
}

// SpecTransform: 未来如果添加更多的Spec，只需要实现它即可
pub trait SpecTransform<T> {
    fn transform(&mut self, op: T);
}