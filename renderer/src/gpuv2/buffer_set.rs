
use super::Buffer;
use ash::vk;

pub struct BufferSet {
    descriptor_type: vk::DescriptorType,
    buffer_infos: Vec<vk::DescriptorBufferInfo>,
    descriptor_set_layout: Option<vk::DescriptorSetLayout>,
    descriptor_pool: Option<vk::DescriptorPool>,
    descriptor_sets: Option<Vec<vk::DescriptorSet>>
}

impl BufferSet {
    pub fn new(descriptor_type: vk::DescriptorType) -> Self {
        Self {
            descriptor_type,
            buffer_infos: vec![],
            descriptor_set_layout: None,
            descriptor_pool: None,
            descriptor_sets: None
        }
    }

    pub fn add<M: Copy, const N: usize>(self, buffer: &Buffer<M, N>) -> Self {
        let mut buffer_infos = self.buffer_infos.clone();
        buffer_infos.push(buffer.descriptor_info());

        Self {
            buffer_infos,
            ..self
        }
    }

    pub fn descriptor_sets(&self) -> Vec<vk::DescriptorSet> {
        let sets = self.descriptor_sets.as_ref().unwrap();
        sets.clone()
    }

    pub fn descriptor_set_layout(&self) -> vk::DescriptorSetLayout {
        self.descriptor_set_layout.unwrap()
    }

    pub fn descriptor_type(&self) -> vk::DescriptorType {
        self.descriptor_type
    }

    pub fn allocate(self, dvc: &ash::Device) -> Self {
        let binding = vk::DescriptorSetLayoutBinding {
            binding: 0,
            descriptor_type: vk::DescriptorType::UNIFORM_BUFFER,
            descriptor_count: 1,
            stage_flags: vk::ShaderStageFlags::VERTEX,
            ..Default::default()
        };

        let create_info = vk::DescriptorSetLayoutCreateInfo {
            s_type: vk::StructureType::DESCRIPTOR_SET_LAYOUT_CREATE_INFO,
            binding_count: 1,
            p_bindings: &binding,
            ..Default::default()
        };

        let descriptor_set_layout = unsafe {
            dvc
                .create_descriptor_set_layout(&create_info, None)
                .expect("Error creating descriptor set layout")
        };

        let pool_size = vk::DescriptorPoolSize {
            ty: vk::DescriptorType::UNIFORM_BUFFER,
            descriptor_count: self.buffer_infos.len() as u32
        };
        
        let pool_sizes = [pool_size];
        let descriptor_pool_create_info = vk::DescriptorPoolCreateInfo {
            s_type: vk::StructureType::DESCRIPTOR_POOL_CREATE_INFO,
            pool_size_count: pool_sizes.len() as u32,
            p_pool_sizes: pool_sizes.as_ptr(),
            max_sets: self.buffer_infos.len() as u32,
            ..Default::default()
        };

        let descriptor_pool = unsafe {
            dvc
                .create_descriptor_pool(&descriptor_pool_create_info, None)
                .expect("Error creating descriptor pool")
        };

        let set_layouts = [descriptor_set_layout];
        let set_alloc_info = vk::DescriptorSetAllocateInfo {
            s_type: vk::StructureType::DESCRIPTOR_SET_ALLOCATE_INFO,
            descriptor_pool: descriptor_pool,
            descriptor_set_count: self.buffer_infos.len() as u32,
            p_set_layouts: set_layouts.as_ptr(),
            ..Default::default()
        };

        let descriptor_sets = unsafe {
            dvc
                .allocate_descriptor_sets(&set_alloc_info)
                .expect("Error creating descriptor sets")
        };

        Self {
            descriptor_set_layout: Some(descriptor_set_layout),
            descriptor_pool: Some(descriptor_pool),
            descriptor_sets: Some(descriptor_sets),
            ..self
        }
    }
}

impl super::Cleanup for BufferSet {
    fn cleanup(&self, engine: &crate::Engine) {
        let device = engine.device_id.device();

        if self.descriptor_set_layout.is_some() {
            unsafe {
                device.device
                    .destroy_descriptor_set_layout(self.descriptor_set_layout.unwrap(), None)
            }
        }

        if self.descriptor_pool.is_some() {
            unsafe {
                device.device
                    .destroy_descriptor_pool(self.descriptor_pool.unwrap(), None)
            }
        }
    }
}