use opencv::prelude::*;
use opencv::core::{Rect, Point3f};
use camera::zed::VideoFrame;

pub struct DetectedObject {
    pub bounding_box: Rect,
    pub id: u8,
    pub name: str,
    pub distance: f64,
    pub location: Point3f,
}

pub type DetectedObjects = Vec<DetectedObject>;

pub trait PipelineModule {
    const NAME: str;

    // TODO: Create errors and return result
    fn detect(frame: &VideoFrame, objs: &DetectedObjects);
}

pub type Pipeline = Vec<PipelineModule>;

pub struct PipelineManager {
    pub pipeline: Pipeline,
    pub record: bool,
    pub output_path: str,
    pub fps: u16,
}