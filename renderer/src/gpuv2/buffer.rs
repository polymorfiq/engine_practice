use super::DeviceProperties;
use ash::vk;
use std::marker::PhantomData;

pub struct Buffer<T> {
    pub stages: vk::ShaderStageFlags,
    device_props: DeviceProperties,
    sharing_mode: vk::SharingMode,
    memory_flags: vk::MemoryPropertyFlags,
    usage: vk::BufferUsageFlags,
    pub descriptor_type: vk::DescriptorType,
    pub binding: Option<u32>,
    attributes: Vec<vk::VertexInputAttributeDescription>,
    pub buffer: Option<vk::Buffer>,
    device_memory: Option<vk::DeviceMemory>,
    pub memory_req: Option<vk::MemoryRequirements>,
    pub entries: usize,
    phantom: PhantomData<T>
}

impl<T: Copy> Buffer<T> {
    pub fn new(device_props: &DeviceProperties) -> Self {
        Self {
            stages: vk::ShaderStageFlags::empty(),
            device_props: *device_props,
            sharing_mode: vk::SharingMode::EXCLUSIVE,
            memory_flags: vk::MemoryPropertyFlags::DEVICE_LOCAL,
            usage: vk::BufferUsageFlags::UNIFORM_BUFFER,
            descriptor_type: vk::DescriptorType::UNIFORM_BUFFER,
            binding: None,
            attributes: vec![],
            buffer: None,
            device_memory: None,
            memory_req: None,
            entries: 0,
            phantom: PhantomData
        }
    }

    pub fn stages(self, stages: vk::ShaderStageFlags) -> Self {
        Self {
            stages,
            ..self
        }
    }

    pub fn binding(self, binding: u32) -> Self {
        Self {
            binding: Some(binding),
            ..self
        }
    }

    pub fn usage(self, usage: vk::BufferUsageFlags) -> Self {
        Self {
            usage,
            ..self
        }
    }

    pub fn descriptor_type(self, descriptor_type: vk::DescriptorType) -> Self {
        Self {
            descriptor_type,
            ..self
        }
    }

    pub fn sharing_mode(self, mode: vk::SharingMode) -> Self {
        Self {
            sharing_mode: mode,
            ..self
        }
    }

    pub fn memory_flags(self, flags: vk::MemoryPropertyFlags) -> Self {
        Self {
            memory_flags: flags,
            ..self
        }
    }

    pub fn vertex_input_attribute(self, location: u32, binding: u32, offset: u32, format: vk::Format) -> Self {
        let desc = vk::VertexInputAttributeDescription {
            location,
            binding,
            format,
            offset,
        };

        let mut new_attrs = self.attributes.clone();
        new_attrs.push(desc);

        Self {
            attributes: new_attrs,
            ..self
        }
    }

    pub fn info(&self, data: &[T]) -> vk::BufferCreateInfo {
        let size = self.entry_size() * data.len() as u64;

        vk::BufferCreateInfo {
            size,
            usage: self.usage,
            sharing_mode: self.sharing_mode,
            ..Default::default()
        }
    }

    pub fn entry_size(&self) -> u64 {
        let type_size = std::mem::size_of::<T>() as u64;
        let min_dynamic_size = self.device_props.physical_props.limits.min_uniform_buffer_offset_alignment;
        let entry_size = type_size.max(min_dynamic_size);

        entry_size
    }

    pub fn attributes(&self) -> &[vk::VertexInputAttributeDescription] {
        self.attributes.as_slice()
    }

    pub fn allocate(self, dvc: &ash::Device, data: &[T]) -> Self {
        let buffer_info = self.info(data);

        if self.buffer.is_some() {
            return self;
        }

        unsafe {
            let buffer = dvc
                .create_buffer(&buffer_info, None)
                .expect("Error creating buffer");

            let memory_req = dvc
                .get_buffer_memory_requirements(buffer);

            let memory_size = memory_req.size;
            let memory_index = find_memorytype_index(
                &memory_req,
                &self.device_props.physical_memory,
                self.memory_flags
            )
                .expect("Unable to find suitable memorytype for the buffer.");

            let allocate_info = vk::MemoryAllocateInfo {
                allocation_size: memory_size,
                memory_type_index: memory_index,
                ..Default::default()
            };

            let memory =  dvc
                .allocate_memory(&allocate_info, None)
                .expect("Error allocating buffer");


            dvc
                .bind_buffer_memory(buffer, memory, 0)
                .expect("Error binding input buffer memory");

            Self {
                buffer: Some(buffer),
                device_memory: Some(memory),
                memory_req: Some(memory_req),
                entries: data.len(),
                ..self
            }
        }
    }

    pub fn align_buffer(&self) -> u64 {
        use std::mem::align_of;
        use std::cmp::max;
        let mut align = align_of::<T>() as u64;
        if self.usage.intersects(vk::BufferUsageFlags::UNIFORM_BUFFER) {
            align = max(align, self.device_props.physical_props.limits.min_uniform_buffer_offset_alignment);
        }
        if self.usage.intersects(vk::BufferUsageFlags::STORAGE_BUFFER) {
            align = max(align, self.device_props.physical_props.limits.min_storage_buffer_offset_alignment);
        }
        if self.usage.intersects(vk::BufferUsageFlags::UNIFORM_TEXEL_BUFFER) {
            align = max(align, self.device_props.physical_props.limits.min_texel_buffer_offset_alignment);
        }
        align
    }

    pub fn copy(&self, dvc: &ash::Device, data: &[T]) {
        let memory = self.device_memory.expect("copy() called before allocate()");
        let alignment = self.align_buffer();

        unsafe {
            let buffer_ptr = dvc
                .map_memory(memory, 0, self.memory_req.unwrap().size, vk::MemoryMapFlags::empty())
                .expect("Error mapping buffer memory");

            let mut buffer_align = ash::util::Align::new(
                buffer_ptr,
                alignment,
                self.memory_req.unwrap().size,
            );

            buffer_align.copy_from_slice(data);
            dvc.unmap_memory(memory)
        }
    }

    pub fn load(self, dvc: &ash::Device, data: &[T]) -> Self {
        let resp = self.allocate(dvc, data);
        resp.copy(dvc, data);
        resp
    }
}

impl<T> super::Cleanup for Buffer<T> {
    fn cleanup(&self, engine: &crate::Engine) {
        let device = engine.device_id.device();

        if self.buffer.is_some() {
            unsafe {
                device.device.free_memory(self.device_memory.unwrap(), None);
                device.device.destroy_buffer(self.buffer.unwrap(), None);
            }
        }
    }
}

fn find_memorytype_index(
    memory_req: &vk::MemoryRequirements,
    memory_prop: &vk::PhysicalDeviceMemoryProperties,
    flags: vk::MemoryPropertyFlags,
) -> Option<u32> {
    memory_prop.memory_types[..memory_prop.memory_type_count as _]
        .iter()
        .enumerate()
        .find(|(index, memory_type)| {
            (1 << index) & memory_req.memory_type_bits != 0
                && memory_type.property_flags & flags == flags
        })
        .map(|(index, _memory_type)| index as _)
}