use opencv::prelude::*;
use opencv::core::{Rect, Point3f};
use camera::zed::VideoFrame;

pub struct DetectedObject {
    pub bounding_box: Rect,
    pub id: u8,
    pub name: &'static str,
    pub distance: f64,
    pub location: Point3f,
    pub timestamp: u64,
}

pub type DetectedObjects = Vec<DetectedObject>;

pub trait PipelineModule {
    const NAME: &'static str;

    // TODO: Create errors and return result
    fn detect(self, frame: &VideoFrame, objs: &mut DetectedObjects);
}

pub type Pipeline = Vec<Box<PipelineModule>>;

pub struct PipelineManager {
    pub pipeline: Pipeline,
}

impl PipelineManager {
    pub fn run(self, frame: &VideoFrame) -> DetectedObjects {
        let mut detectedObjects: DetectedObjects = vec![];

        for module in self.pipeline.iter() {
            module.detect(frame, &mut detectedObjects);
        }

        detectedObjects
    }
}