use ash::{
    vk,
    extensions::khr::Swapchain
};
use super::ids::{InstanceID, SurfaceID};

pub struct Device {
    pub surface_id: SurfaceID,
    pub instance_id: InstanceID,
    pdevice: vk::PhysicalDevice,
    pub device: ash::Device,
    pub swapchain_loader: Swapchain,
    pub queue: vk::Queue,
    pub command_pool: vk::CommandPool
}

impl Device {
    pub(super) fn new(surface_id: SurfaceID, pdevice: vk::PhysicalDevice, device: ash::Device, queue_family_idx: u32) -> Self {
        let surface_id_clone = surface_id.clone();
        let surface = surface_id_clone.surface();
        let instance = surface.instance_id.instance();
        let swapchain_loader = Swapchain::new(&instance.instance, &device);

        let queue = unsafe {
            device.get_device_queue(queue_family_idx, 0)
        };

        let pool_create_info = vk::CommandPoolCreateInfo::builder()
            .flags(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER)
            .queue_family_index(queue_family_idx);
            
        let command_pool = unsafe {
            device
                .create_command_pool(&pool_create_info, None)
                .expect("Error creating command pool")
        };

        Self {
            surface_id,
            instance_id: surface.instance_id.clone(),
            pdevice,
            device,
            swapchain_loader,
            queue,
            command_pool
        }
    }

    pub fn surface_format(&self) -> vk::SurfaceFormatKHR {
        let surface = self.surface_id.surface();
        let instance = self.instance_id.instance();

        unsafe {
            instance.surface_loader
                .get_physical_device_surface_formats(self.pdevice, surface.surface)
                .unwrap()[0]
        }
    }

    pub fn surface_capabilities(&self) -> vk::SurfaceCapabilitiesKHR {
        let surface = self.surface_id.surface();
        let instance = self.instance_id.instance();

        unsafe {
            instance.surface_loader
                .get_physical_device_surface_capabilities(self.pdevice, surface.surface)
                .unwrap()
        }
    }

    pub fn desired_image_count(&self) -> u32 {
        let caps = self.surface_capabilities();
        let mut desired_image_count = caps.min_image_count + 1;
        if caps.max_image_count > 0
            && desired_image_count > caps.max_image_count
        {
            desired_image_count = caps.max_image_count;
        }

        desired_image_count
    }

    pub fn surface_resolution(&self) -> vk::Extent2D {
        let caps = self.surface_capabilities();
        let surface = self.surface_id.surface();
        let instance = self.instance_id.instance();
        match caps.current_extent.width {
            std::u32::MAX => vk::Extent2D {
                width: instance.window.width,
                height: instance.window.height,
            },
            _ => caps.current_extent,
        }
    }

    pub fn pre_transform(&self) -> vk::SurfaceTransformFlagsKHR {
        let caps = self.surface_capabilities();
        let transforms = caps.supported_transforms;

        if transforms.contains(vk::SurfaceTransformFlagsKHR::IDENTITY) {
            vk::SurfaceTransformFlagsKHR::IDENTITY
        } else {
            caps.current_transform
        }
    }

    pub fn present_modes(&self) -> Vec<vk::PresentModeKHR> {
        let surface = self.surface_id.surface();
        let instance = self.instance_id.instance();

        unsafe {
            instance.surface_loader
                .get_physical_device_surface_present_modes(self.pdevice, surface.surface)
                .unwrap()
        }
    }

    pub fn present_mode(&self) -> vk::PresentModeKHR {
        self.present_modes()
            .iter()
            .cloned()
            .find(|&mode| mode == vk::PresentModeKHR::MAILBOX)
            .unwrap_or(vk::PresentModeKHR::FIFO)
    }

    pub fn swapchain(&self) -> vk::SwapchainKHR {
        let surface = self.surface_id.surface();
        let surface_format = self.surface_format();
        let surface_resolution = self.surface_resolution();
        let pre_transform = self.pre_transform();
        let present_mode = self.present_mode();

        let swapchain_create_info = vk::SwapchainCreateInfoKHR::builder()
            .surface(surface.surface)
            .min_image_count(self.desired_image_count())
            .image_color_space(surface_format.color_space)
            .image_format(surface_format.format)
            .image_extent(surface_resolution)
            .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
            .image_sharing_mode(vk::SharingMode::EXCLUSIVE)
            .pre_transform(pre_transform)
            .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
            .present_mode(present_mode)
            .clipped(true)
            .image_array_layers(1);

        unsafe {
            self.swapchain_loader
                .create_swapchain(&swapchain_create_info, None)
                .expect("Error creating swapchain")
        }
    }

    pub fn present_images(&self, swapchain: vk::SwapchainKHR) -> Vec<vk::Image> {
        unsafe {
            self.swapchain_loader
                .get_swapchain_images(swapchain)
                .expect("Error getting swapchain images")
        }
    }

    pub fn present_image_views(&self, swapchain: vk::SwapchainKHR) -> Vec<vk::ImageView> {
        let surface_format = self.surface_format();

        self.present_images(swapchain)
            .iter()
            .map(|&image| {
                let create_view_info = vk::ImageViewCreateInfo::builder()
                    .view_type(vk::ImageViewType::TYPE_2D)
                    .format(surface_format.format)
                    .components(vk::ComponentMapping {
                        r: vk::ComponentSwizzle::R,
                        g: vk::ComponentSwizzle::G,
                        b: vk::ComponentSwizzle::B,
                        a: vk::ComponentSwizzle::A,
                    })
                    .subresource_range(vk::ImageSubresourceRange {
                        aspect_mask: vk::ImageAspectFlags::COLOR,
                        base_mip_level: 0,
                        level_count: 1,
                        base_array_layer: 0,
                        layer_count: 1,
                    })
                    .image(image);

                unsafe {
                    self.device
                        .create_image_view(&create_view_info, None)
                        .expect("Error creating image view")
                }
            })
            .collect()
    }

    pub fn memory_properties(&self) -> vk::PhysicalDeviceMemoryProperties {
        let instance = self.instance_id.instance();

        unsafe {
            instance.instance
                .get_physical_device_memory_properties(self.pdevice)
        }
    }


    pub fn cleanup(&self) {
        unsafe {
            self.device.destroy_command_pool(self.command_pool, None);
            self.device.destroy_device(None);
        }
    }
}