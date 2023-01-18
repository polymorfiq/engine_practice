use ash::vk;
use std::convert::Into;
use crate::gpu::ids::DeviceID;

#[derive(Clone)]
pub struct Builder {
    pub scissors: Vec<vk::Rect2D>,
    pub viewports: Vec<vk::Viewport>,
    pub state: vk::PipelineViewportStateCreateInfo
}

impl Builder {
    pub fn new(device_id: &DeviceID) -> Self {
        let surface_resolution = device_id.device().surface_resolution();
        let scissors = [surface_resolution.into()];

        let viewports = [vk::Viewport {
            x: 0.0,
            y: 0.0,
            width: surface_resolution.width as f32,
            height: surface_resolution.height as f32,
            min_depth: 0.0,
            max_depth: 1.0,
        }];

        Self {
            scissors: scissors.to_vec(),
            viewports: viewports.to_vec(),
            state: get_state(&scissors, &viewports)
        }
    }
}

fn get_state(scissors: &[vk::Rect2D], viewports: &[vk::Viewport]) -> vk::PipelineViewportStateCreateInfo {
    vk::PipelineViewportStateCreateInfo::builder()
        .scissors(scissors)
        .viewports(viewports)
        .build()
}