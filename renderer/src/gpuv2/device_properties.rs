use ash::vk;

#[derive(Copy, Clone, Debug)]
pub struct DeviceProperties {
    pub physical_memory: vk::PhysicalDeviceMemoryProperties,
    pub physical_props: vk::PhysicalDeviceProperties
}