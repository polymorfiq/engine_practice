
use super::Buffer;
use ash::vk;
use std::collections::HashMap;

#[derive(Copy, Clone, Debug)]
struct BufferData {
    descriptor_type: vk::DescriptorType,
    descriptor_info: vk::DescriptorBufferInfo,
    stages: vk::ShaderStageFlags,
    entry_size: u64,
    binding: u32
}

pub struct BufferSet {
    buffer_datas: Vec<BufferData>,
    descriptor_set_layout: Option<vk::DescriptorSetLayout>,
    descriptor_pool: Option<vk::DescriptorPool>,
    descriptor_set: Option<vk::DescriptorSet>
}

impl BufferSet {
    pub fn new() -> Self {
        Self {
            buffer_datas: vec![],
            descriptor_set_layout: None,
            descriptor_pool: None,
            descriptor_set: None
        }
    }

    pub fn add<M: Copy>(self, buffer: &Buffer<M>) -> Self {
        let mut buffer_datas = self.buffer_datas.clone();

        let desc_info = if buffer.descriptor_type == vk::DescriptorType::UNIFORM_BUFFER_DYNAMIC {
            vk::DescriptorBufferInfo {
                buffer: buffer.buffer.unwrap(),
                offset: 0,
                range: buffer.entry_size()
            }
        } else {
            vk::DescriptorBufferInfo {
                buffer: buffer.buffer.unwrap(),
                offset: 0,
                range: vk::WHOLE_SIZE
            }
        };

        buffer_datas.push(BufferData{
            descriptor_type: buffer.descriptor_type,
            descriptor_info: desc_info,
            stages: buffer.stages,
            entry_size: buffer.entry_size(),
            binding: buffer.binding.expect("Cannot add buffer without binding to a BufferSet")
        });

        Self {
            buffer_datas,
            ..self
        }
    }

    pub fn dynamic_offset(&self, i: usize, j: usize) -> u32 {
        self.buffer_datas[i].entry_size as u32 * j as u32
    }

    pub fn descriptor_set(&self) -> vk::DescriptorSet {
        self.descriptor_set
            .expect("descriptor_set called with unallocated BufferSet")
    }

    pub fn descriptor_set_layout(&self) -> vk::DescriptorSetLayout {
        self.descriptor_set_layout.unwrap()
    }

    pub fn write_descriptor_sets(&self) -> Vec<vk::WriteDescriptorSet> {
        let mut write_desc_sets = vec![];
        for buffer_data in self.buffer_datas.iter() {
            let desc_set = vk::WriteDescriptorSet {
                s_type: vk::StructureType::WRITE_DESCRIPTOR_SET,
                dst_set: self.descriptor_set(),
                dst_binding: buffer_data.binding,
                dst_array_element: 0,
                descriptor_type: buffer_data.descriptor_type,
                descriptor_count: 1,
                p_buffer_info: &buffer_data.descriptor_info,
                ..Default::default()
            };

            write_desc_sets.push(desc_set);
        }
        
        write_desc_sets
    }

    pub fn allocate(self, dvc: &ash::Device) -> Self {
        let mut bindings = vec![];
        let mut desc_type_counts = HashMap::new();
        for buffer_data in self.buffer_datas.iter() {
            let desc_type = buffer_data.descriptor_type;
            let binding = vk::DescriptorSetLayoutBinding {
                binding: buffer_data.binding,
                descriptor_type: desc_type,
                descriptor_count: 1,
                stage_flags: buffer_data.stages,
                ..Default::default()
            };

            if !desc_type_counts.contains_key(&desc_type) {
                desc_type_counts.insert(desc_type, 1u32);
            } else {
                desc_type_counts.insert(desc_type, desc_type_counts.get(&desc_type).unwrap() + 1u32);
            }

            bindings.push(binding);
        }

        let create_info = vk::DescriptorSetLayoutCreateInfo {
            s_type: vk::StructureType::DESCRIPTOR_SET_LAYOUT_CREATE_INFO,
            binding_count: bindings.len() as u32,
            p_bindings: bindings.as_slice().as_ptr(),
            ..Default::default()
        };

        let descriptor_set_layout = unsafe {
            dvc
                .create_descriptor_set_layout(&create_info, None)
                .expect("Error creating descriptor set layout")
        };

        let mut pool_sizes = vec![];
        for (k, v) in desc_type_counts.iter() {
            pool_sizes.push(vk::DescriptorPoolSize {
                ty: *k,
                descriptor_count: *v
            })
        }

        let descriptor_pool_create_info = vk::DescriptorPoolCreateInfo {
            s_type: vk::StructureType::DESCRIPTOR_POOL_CREATE_INFO,
            pool_size_count: pool_sizes.len() as u32,
            p_pool_sizes: pool_sizes.as_slice().as_ptr(),
            max_sets: 1,
            ..Default::default()
        };

        let descriptor_pool = unsafe {
            dvc
                .create_descriptor_pool(&descriptor_pool_create_info, None)
                .expect("Error creating descriptor pool")
        };

        let set_alloc_info = vk::DescriptorSetAllocateInfo {
            s_type: vk::StructureType::DESCRIPTOR_SET_ALLOCATE_INFO,
            descriptor_pool: descriptor_pool,
            descriptor_set_count: 1,
            p_set_layouts: &descriptor_set_layout,
            ..Default::default()
        };

        let descriptor_set = unsafe {
            dvc
                .allocate_descriptor_sets(&set_alloc_info)
                .expect("Error creating descriptor sets")
        };

        Self {
            descriptor_set_layout: Some(descriptor_set_layout),
            descriptor_pool: Some(descriptor_pool),
            descriptor_set: Some(descriptor_set[0]),
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