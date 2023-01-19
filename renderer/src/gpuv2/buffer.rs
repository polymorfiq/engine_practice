use super::DeviceProperties;
use ash::vk;
use std::marker::PhantomData;

pub struct Buffer<T, const N: usize> {
    memory_properties: vk::PhysicalDeviceMemoryProperties,
    sharing_mode: vk::SharingMode,
    memory_flags: vk::MemoryPropertyFlags,
    usage: vk::BufferUsageFlags,
    pub binding: u32,
    stride: u32,
    input_rate: vk::VertexInputRate,
    attributes: Vec<vk::VertexInputAttributeDescription>,
    pub buffer: Option<vk::Buffer>,
    device_memory: Option<vk::DeviceMemory>,
    memory_req: Option<vk::MemoryRequirements>,
    phantom: PhantomData<T>
}

impl<T: Copy, const N: usize> Buffer<T, N> {
    pub fn new(device_props: &DeviceProperties) -> Self {
        let memory_properties = device_props.physical_memory;

        Self {
            memory_properties,
            sharing_mode: vk::SharingMode::EXCLUSIVE,
            memory_flags: vk::MemoryPropertyFlags::DEVICE_LOCAL,
            usage: vk::BufferUsageFlags::UNIFORM_BUFFER,
            binding: 0,
            stride: std::mem::size_of::<T>() as u32,
            input_rate: vk::VertexInputRate::VERTEX,
            attributes: vec![],
            buffer: None,
            device_memory: None,
            memory_req: None,
            phantom: PhantomData
        }
    }

    pub fn binding(self, binding: u32) -> Self {
        Self {
            binding,
            ..self
        }
    }

    pub fn usage(self, usage: vk::BufferUsageFlags) -> Self {
        Self {
            usage,
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

    pub fn input_rate(self, rate: vk::VertexInputRate) -> Self {
        Self {
            input_rate: rate,
            ..self
        }
    }

    pub fn attribute(self, location: u32, offset: u32, format: vk::Format) -> Self {
        let desc = vk::VertexInputAttributeDescription {
            location,
            binding: self.binding,
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

    pub fn info(&self) -> vk::BufferCreateInfo {
        vk::BufferCreateInfo {
            size: (N as u64) * std::mem::size_of::<T>() as u64,
            usage: self.usage,
            sharing_mode: self.sharing_mode,
            ..Default::default()
        }
    }

    pub fn description(&self) -> vk::VertexInputBindingDescription {
        vk::VertexInputBindingDescription {
            binding: self.binding,
            stride: self.stride,
            input_rate: self.input_rate,
        }
    }

    pub fn descriptor_info(&self) -> vk::DescriptorBufferInfo {
        vk::DescriptorBufferInfo {
            buffer: self.buffer.unwrap(),
            offset: 0,
            range: std::mem::size_of::<T>() as u64
        }
    }

    pub fn attributes(&self) -> &[vk::VertexInputAttributeDescription] {
        self.attributes.as_slice()
    }

    pub fn allocate(self, dvc: &ash::Device) -> Self {
        let buffer_info = self.info();

        if self.buffer.is_some() {
            return self;
        }

        unsafe {
            let buffer = dvc
                .create_buffer(&buffer_info, None)
                .expect("Error creating buffer");

            let memory_req = dvc
                .get_buffer_memory_requirements(buffer);

            let memory_index = find_memorytype_index(
                &memory_req,
                &self.memory_properties,
                self.memory_flags
            )
                .expect("Unable to find suitable memorytype for the buffer.");

            let allocate_info = vk::MemoryAllocateInfo {
                allocation_size: memory_req.size,
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
                ..self
            }
        }
    }

    pub fn copy(&self, dvc: &ash::Device, data: &[T]) {
        let memory = self.device_memory.expect("copy() called before allocate()");
        let memory_req = self.memory_req.unwrap();

        unsafe {
            let buffer_ptr = dvc
                .map_memory(memory, 0, memory_req.size, vk::MemoryMapFlags::empty())
                .expect("Error mapping buffer memory");

            let mut buffer_align = ash::util::Align::new(
                buffer_ptr,
                std::mem::align_of::<T>() as u64,
                memory_req.size,
            );

            buffer_align.copy_from_slice(data);
            dvc.unmap_memory(memory)
        }
    }

    pub fn load(self, dvc: &ash::Device, data: &[T]) -> Self {
        let resp = self.allocate(dvc);
        resp.copy(dvc, data);
        resp
    }
}

impl<T, const N: usize> super::Cleanup for Buffer<T, N> {
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