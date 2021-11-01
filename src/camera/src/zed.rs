use opencv::prelude::*;
use opencv::core::Mat;

pub enum VideoQuality {
    HD2K_15fps,
    HD1080_15fps,
    HD1080_30fps,
    HD720_15fps,
    HD720_30fps,
    HD720_60fps,
    VGA_15fps,
    VGA_30fps,
    VGA_60fps,
    VGA_100fps
}

pub struct VideoFrame {
    pub image: Mat,
    pub depth_map: Mat,
    pub point_cloud: Mat,
}