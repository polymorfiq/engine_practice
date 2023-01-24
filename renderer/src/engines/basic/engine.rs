use crate::gpu::System;
use crate::gpu::ids::{DeviceID, InstanceID, SurfaceID};
use crate::gpuv2::Cleanup;
use ash::vk;
use std::cell::RefCell;

pub struct Engine<'a> {
    pub system: &'a System<'a>,
    pub device_id: DeviceID,
    pub surface_id: SurfaceID,
    pub instance_id: InstanceID,
    pub swapchain: vk::SwapchainKHR,
    present_image_views: Vec<vk::ImageView>,
    depth_image: vk::Image,
    depth_image_view: vk::ImageView,
    depth_image_memory: vk::DeviceMemory,
    pub render_pass: vk::RenderPass,
    pub framebuffers: Vec<vk::Framebuffer>,
    pub setup_fence: vk::Fence
}

impl<'a> Engine<'a> {
    pub fn new<'b>(system: &'b System<'b>, device_id: &DeviceID) -> Engine<'b> {
        let device = device_id.device();
        let surface_id = device.surface_id.clone();
        let instance_id = device.instance_id.clone();

        let swapchain = device.swapchain();

        let present_image_views = get_present_image_views(device_id, swapchain);

        let (
            depth_image,
            depth_image_memory,
            depth_image_view
        ) = get_depth_image(device_id);

        let render_pass = get_render_pass(device_id);
        let framebuffers = get_framebuffers(
            device_id,
            depth_image_view,
            &present_image_views,
            render_pass
        );

        let fence_create_info = vk::FenceCreateInfo::builder().flags(vk::FenceCreateFlags::SIGNALED);
        let setup_fence = unsafe {
            device.device
                .create_fence(&fence_create_info, None)
                .expect("Create fence failed.")
        };

        Engine {
            system,
            device_id: device_id.clone(),
            surface_id: surface_id,
            instance_id: instance_id,
            swapchain,
            present_image_views,
            depth_image,
            depth_image_memory,
            depth_image_view,
            render_pass,
            framebuffers,
            setup_fence
        }
    }

    pub fn render_loop<R: FnMut(&ash::Device, std::time::SystemTime), E: FnMut(&ash::Device, winit::event::Event<()>, std::time::SystemTime)>(&self, mut render: R, mut handle_event: E) {
        let world_time = RefCell::new(std::time::SystemTime::now());

        self.instance_id.instance().window.handle_events(||{
            let mut curr_time = world_time.borrow_mut();
            render(&self.device_id.device().device, *curr_time);
            *curr_time = std::time::SystemTime::now();
        }, |e| {
            let mut curr_time = world_time.borrow_mut();
            *curr_time = std::time::SystemTime::now();
            handle_event(&self.device_id.device().device, e, *curr_time);
        })
    }

    pub fn create_pipeline_layout(&self, info: vk::PipelineLayoutCreateInfo) -> vk::PipelineLayout {
        let device = self.device_id.device();

        unsafe {
            device.device
                .create_pipeline_layout(&info, None)
                .expect("Error creating pipeline layout")
        }
    }

    pub fn present_idx(&self, present_complete_semaphore: vk::Semaphore) -> u32 {
        let device = self.device_id.device();
        
        let (present_index, _) = unsafe {
            device.swapchain_loader
                .acquire_next_image(
                    self.swapchain,
                    std::u64::MAX,
                    present_complete_semaphore,
                    vk::Fence::null(),
                )
                .expect("Error getting present index")
        };

        present_index
    }

    pub fn create_semaphore(&self) -> vk::Semaphore {
        let semaphore_create_info = vk::SemaphoreCreateInfo::default();
    
        let device = self.device_id.device();
        unsafe {
            device.device
                .create_semaphore(&semaphore_create_info, None)
                .expect("Unable to create semaphore")
        }
    }

    pub fn create_fence(&self) -> vk::Fence {
        let fence_create_info =
        vk::FenceCreateInfo::builder().flags(vk::FenceCreateFlags::SIGNALED);

        let device = self.device_id.device();
        unsafe {
            device.device
                .create_fence(&fence_create_info, None)
                .expect("Create fence failed.")
        }
    }

    pub fn create_command_buffers(&self, count: u32) -> Vec<vk::CommandBuffer> {
        let device = self.device_id.device();

        let command_buffer_allocate_info = vk::CommandBufferAllocateInfo::builder()
            .command_buffer_count(count)
            .command_pool(device.command_pool)
            .level(vk::CommandBufferLevel::PRIMARY);

        unsafe {
            device.device
                .allocate_command_buffers(&command_buffer_allocate_info)
                .expect("Error allocating command buffers")
        }
    }

    pub fn record_command_buffer<F: FnOnce(&ash::Device, vk::CommandBuffer)>(
        &self,
        command_buffer: vk::CommandBuffer,
        f: F,
    ) -> vk::CommandBuffer {
        let device = self.device_id.device();
        unsafe {
            device.device
                .reset_command_buffer(
                    command_buffer,
                    vk::CommandBufferResetFlags::RELEASE_RESOURCES,
                )
                .expect("Reset command buffer failed.");
    
            }
            
            let command_buffer_begin_info = vk::CommandBufferBeginInfo::builder()
                .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);
        
            unsafe {
                device.device
                    .begin_command_buffer(command_buffer, &command_buffer_begin_info)
                    .expect("Begin command buffer");

                f(&device.device, command_buffer);
                
                device.device
                    .end_command_buffer(command_buffer)
                    .expect("End command buffer");
            }
    
            command_buffer
    }

    pub fn setup(&self) {
        let device = self.device_id.device();

        unsafe {
            device.device
                .wait_for_fences(&[self.setup_fence], true, std::u64::MAX)
                .expect("Wait for fence failed.");

            device.device
                .reset_fences(&[self.setup_fence])
                .expect("Reset fences failed.");
        }

        let layout_transition_barriers = vk::ImageMemoryBarrier::builder()
            .image(self.depth_image)
            .dst_access_mask(
                vk::AccessFlags::DEPTH_STENCIL_ATTACHMENT_READ
                    | vk::AccessFlags::DEPTH_STENCIL_ATTACHMENT_WRITE,
            )
            .new_layout(vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL)
            .old_layout(vk::ImageLayout::UNDEFINED)
            .subresource_range(
                vk::ImageSubresourceRange::builder()
                    .aspect_mask(vk::ImageAspectFlags::DEPTH)
                    .layer_count(1)
                    .level_count(1)
                    .build(),
            );

        let setup_command_buffer = self.create_command_buffers(1)[0];
        self.record_command_buffer(setup_command_buffer, |dvc, command_buffer| {
            unsafe {
                dvc.cmd_pipeline_barrier(
                    command_buffer,
                    vk::PipelineStageFlags::BOTTOM_OF_PIPE,
                    vk::PipelineStageFlags::LATE_FRAGMENT_TESTS,
                    vk::DependencyFlags::empty(),
                    &[],
                    &[],
                    &[layout_transition_barriers.build()],
                )
            }
        });

        let command_buffers = vec![setup_command_buffer];
        let submit_info = vk::SubmitInfo::builder()
            .wait_semaphores(&[])
            .wait_dst_stage_mask(&[])
            .command_buffers(&command_buffers)
            .signal_semaphores(&[]);

        let device = self.device_id.device();
        unsafe {
            device.device
                .queue_submit(device.queue, &[submit_info.build()], self.setup_fence)
                .expect("queue submit failed.");
        }
    }

    pub fn wait_idle(&self) {
        let device = self.device_id.device();

        unsafe {
            device.device
                .device_wait_idle()
                .expect("Error waiting for device to be idle");
        }
    }

    pub fn cleanup_pipeline(&self, pipeline: &vk::Pipeline) {
        let device = self.device_id.device();

        unsafe {
            device.device
                .destroy_pipeline(*pipeline, None);
        }
    }

    pub fn cleanup_sempahore(&self, semaphore: &vk::Semaphore) {
        let device = self.device_id.device();

        unsafe {
            device.device
                .destroy_semaphore(*semaphore, None);
        }
    }

    pub fn cleanup_fence(&self, fence: &vk::Fence) {
        let device = self.device_id.device();

        unsafe {
            device.device
                .destroy_fence(*fence, None);
        }
    }

    pub fn cleanup_pipeline_layout(&self, layout: &vk::PipelineLayout) {
        let device = self.device_id.device();

        unsafe {
            device.device
                .destroy_pipeline_layout(*layout, None)
        }
    }

    pub fn cleanup(&self, obj: &dyn Cleanup) {
        obj.cleanup(self);
    }
}

impl Cleanup for Engine<'_> {
    fn cleanup(&self, _engine: &Engine) {
        let device = self.device_id.device();

        self.cleanup_fence(&self.setup_fence);

        for framebuffer in &self.framebuffers {
            unsafe {
                device.device.destroy_framebuffer(*framebuffer, None);
            }
        }

        unsafe {
            device.device.destroy_render_pass(self.render_pass, None);
        }

        unsafe {
            device.device.free_memory(self.depth_image_memory, None);
            device.device.destroy_image_view(self.depth_image_view, None);
            device.device.destroy_image(self.depth_image, None);
        }

        for &image_view in self.present_image_views.iter() {
            unsafe {
                device.device.destroy_image_view(image_view, None);
            }
        }
        
        unsafe {
            device.swapchain_loader.destroy_swapchain(self.swapchain, None);
        }

        self.system.cleanup();
    }
}

fn get_present_image_views(device_id: &DeviceID, swapchain: vk::SwapchainKHR) -> Vec<vk::ImageView> {
    let device = device_id.device();
    let surface_format = device.surface_format();

    device
        .present_images(swapchain)
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
                device.device
                    .create_image_view(&create_view_info, None)
                    .expect("Error creating Image View")
            }
        })
        .collect()
}

fn get_depth_image(device_id: &DeviceID) -> (vk::Image, vk::DeviceMemory, vk::ImageView) {
    let device = device_id.device();
    let surface_resolution = device.surface_resolution();

    let depth_image_create_info = vk::ImageCreateInfo::builder()
        .image_type(vk::ImageType::TYPE_2D)
        .format(vk::Format::D16_UNORM)
        .extent(surface_resolution.into())
        .mip_levels(1)
        .array_layers(1)
        .samples(vk::SampleCountFlags::TYPE_1)
        .tiling(vk::ImageTiling::OPTIMAL)
        .usage(vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT)
        .sharing_mode(vk::SharingMode::EXCLUSIVE);

    let image = unsafe {
        device.device
        .create_image(&depth_image_create_info, None)
        .expect("Error creating depth image")
    };

    let image_memory = get_image_memory(device_id, image);

    let depth_image_view_info = vk::ImageViewCreateInfo::builder()
        .subresource_range(
            vk::ImageSubresourceRange::builder()
                .aspect_mask(vk::ImageAspectFlags::DEPTH)
                .level_count(1)
                .layer_count(1)
                .build(),
        )
        .image(image)
        .format(depth_image_create_info.format)
        .view_type(vk::ImageViewType::TYPE_2D);

    let depth_image_view = unsafe {
        device.device
            .create_image_view(&depth_image_view_info, None)
            .expect("Error creating depth image view")
    };

    (image, image_memory, depth_image_view)
}

fn get_image_memory(device_id: &DeviceID, image: vk::Image) -> vk::DeviceMemory {
    let device = device_id.device();
    let image_memory_req = unsafe {
        device.device.get_image_memory_requirements(image)
    };

    let device_memory_properties = device.memory_properties();
    let image_memory_index = find_memorytype_index(
        &image_memory_req,
        &device_memory_properties,
        vk::MemoryPropertyFlags::DEVICE_LOCAL,
    )
    .expect("Unable to find suitable memory index for image.");

    let image_allocate_info = vk::MemoryAllocateInfo::builder()
        .allocation_size(image_memory_req.size)
        .memory_type_index(image_memory_index);

    let image_memory = unsafe {
        device.device
            .allocate_memory(&image_allocate_info, None)
            .unwrap()
    };

    unsafe {
        device.device
            .bind_image_memory(image, image_memory, 0)
            .expect("Unable to bind depth image memory")
    }

    image_memory
}

fn get_render_pass(device_id: &DeviceID) -> vk::RenderPass {
    let device = device_id.device();
    let surface_format = device.surface_format();

    let renderpass_attachments = [
        vk::AttachmentDescription {
            format: surface_format.format,
            samples: vk::SampleCountFlags::TYPE_1,
            load_op: vk::AttachmentLoadOp::CLEAR,
            store_op: vk::AttachmentStoreOp::STORE,
            final_layout: vk::ImageLayout::PRESENT_SRC_KHR,
            ..Default::default()
        },
        vk::AttachmentDescription {
            format: vk::Format::D16_UNORM,
            samples: vk::SampleCountFlags::TYPE_1,
            load_op: vk::AttachmentLoadOp::CLEAR,
            initial_layout: vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
            final_layout: vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
            ..Default::default()
        },
    ];

    let color_attachment_refs = [vk::AttachmentReference {
        attachment: 0,
        layout: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
    }];

    let depth_attachment_ref = vk::AttachmentReference {
        attachment: 1,
        layout: vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
    };

    let dependencies = [vk::SubpassDependency {
        src_subpass: vk::SUBPASS_EXTERNAL,
        src_stage_mask: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
        dst_access_mask: vk::AccessFlags::COLOR_ATTACHMENT_READ
            | vk::AccessFlags::COLOR_ATTACHMENT_WRITE,
        dst_stage_mask: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
        ..Default::default()
    }];

    let subpass = vk::SubpassDescription::builder()
        .color_attachments(&color_attachment_refs)
        .depth_stencil_attachment(&depth_attachment_ref)
        .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS);

    let renderpass_create_info = vk::RenderPassCreateInfo::builder()
        .attachments(&renderpass_attachments)
        .subpasses(std::slice::from_ref(&subpass))
        .dependencies(&dependencies);

    unsafe {
        device.device
            .create_render_pass(&renderpass_create_info, None)
            .expect("Error creating RenderPass")
    }
}

fn get_framebuffers(device_id: &DeviceID, depth_image_view: vk::ImageView, present_image_views: &Vec<vk::ImageView>, render_pass: vk::RenderPass) -> Vec<vk::Framebuffer> {
    let device = device_id.device();
    let surface_resolution = device.surface_resolution();

    present_image_views
        .iter()
        .map(|&present_image_view| {
            let framebuffer_attachments = [present_image_view, depth_image_view];
            let frame_buffer_create_info = vk::FramebufferCreateInfo::builder()
                .render_pass(render_pass)
                .attachments(&framebuffer_attachments)
                .width(surface_resolution.width)
                .height(surface_resolution.height)
                .layers(1);

            unsafe {
                device.device
                    .create_framebuffer(&frame_buffer_create_info, None)
                    .expect("Error creating framebuffer")
            }
        })
        .collect()
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