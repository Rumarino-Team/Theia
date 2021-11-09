#[cfg(test)]
pub mod tests {
    use camera::zed::VideoFrame;
    use crate::pipeline::{DetectedObject, DetectedObjects, Pipeline, PipelineManager, PipelineModule};

    struct TestModule {
        pub objects: u8,
    }

    impl PipelineModule for TestModule {
        const NAME: &'static str = "TestModule";

        fn detect(self, frame: &VideoFrame, objs: &mut DetectedObjects) {
            for i in 0..self.objects {
                objs.push(DetectedObject{
                    bounding_box: Default::default(),
                    id: i,
                    name: "Some Name",
                    distance: 0.0,
                    location: Default::default(),
                    timestamp: 0
                });
            }
        }
    }

    #[test]
    fn test_persistence() {
        let frame = VideoFrame{
            image: Default::default(),
            depth_map: Default::default(),
            point_cloud: Default::default()
        };

        // Initialize pipeline
        let objects = 5;
        let module = TestModule{ objects };
        let mut pipeline: Pipeline = vec![];
        pipeline.push(Box::new(module));

        let pipelineManager = PipelineManager { pipeline };

        assert_eq!(pipelineManager.run(&frame).len(), objects);

    }
}