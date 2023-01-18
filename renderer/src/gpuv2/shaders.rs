use ash::vk;
use std::ffi::CStr;

#[derive(Copy, Clone)]
pub struct Stage<'a> {
    entry: &'a CStr,
    stage: vk::ShaderStageFlags,
    module: vk::ShaderModule,
}

impl<'a> Stage<'a> {
    pub fn new(stage: vk::ShaderStageFlags, module: vk::ShaderModule) -> Self {
        let entry = unsafe { CStr::from_bytes_with_nul_unchecked(b"main\0") };

        Self {
            entry,
            stage,
            module
        }
    }
}

impl super::Cleanup for Stage<'_> {
    fn cleanup(&self, engine: &crate::Engine) {
        let device = engine.device_id.device();
        cleanup_shader_module(&device.device, self.module);
    }
}

#[derive(Clone)]
pub struct Stages {
    stages: Vec<vk::PipelineShaderStageCreateInfo>
}

impl Stages {
    pub fn new(stages: &[Stage]) -> Self {
        let mut new_stages = Self{stages: vec![]};
        for stage in stages {
            new_stages.add(stage);
        }

        new_stages
    }

    pub fn add(&mut self, stage: &Stage) {
        let info = vk::PipelineShaderStageCreateInfo {
            module: stage.module,
            p_name: stage.entry.as_ptr(),
            stage: stage.stage,
            ..Default::default()
        };

        self.stages.push(info);
    }

    pub fn build(&self) -> &[vk::PipelineShaderStageCreateInfo] {
        self.stages.as_slice()
    }
}

impl super::Cleanup for Stages {
    fn cleanup(&self, engine: &crate::Engine) {
        let device = engine.device_id.device();

        for stage in &self.stages {
            cleanup_shader_module(&device.device, stage.module);
        }
    }
}

fn cleanup_shader_module(device: &ash::Device, module: vk::ShaderModule) {
    unsafe {
        device.destroy_shader_module(module, None);
    }
}