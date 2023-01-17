
extern crate ash;
use ash::vk;
use std::ffi::CString;
use std::ffi::CStr;

mod window;
use window::Window;

mod gpu;
use gpu::System;

mod engines;
use engines::basic::{Engine, Vertex};

#[macro_use]
mod macros;

fn main() {
    let app_name_str = CString::new("my_renderer_app").expect("Unable to wrap app_name in CString");
    let engine_name_str = CString::new("my_renderer_engine").expect("Unable to wrap engine_name in CString");

    let entry = unsafe { ash::Entry::load().expect("Error loading ash::Entry") };
    let app_info = vk::ApplicationInfo::builder()
        .application_name(&app_name_str)
        .application_version(0)
        .engine_name(&engine_name_str)
        .engine_version(0)
        .api_version(vk::make_api_version(0, 1, 0, 0));
        
    let mut system = System::new(entry, app_info);
    let window = Window::new("My Window", 800, 600);

    let instance_id = system.instance(window);
    let surface_id = system.surface(&instance_id);
    let device_id = system.device(&surface_id).expect("Device not found");

    let engine = Engine::new(&system, &device_id);

    let vertices = [
        Vertex {
            pos: [-1.0, 1.0, 0.0, 1.0],
            color: [0.0, 1.0, 0.0, 1.0],
        },
        Vertex {
            pos: [1.0, 1.0, 0.0, 1.0],
            color: [0.0, 0.0, 1.0, 1.0],
        },
        Vertex {
            pos: [0.0, -1.0, 0.0, 1.0],
            color: [1.0, 0.0, 0.0, 1.0],
        },
    ];
    let index_buffer_data = [0u32, 1, 2];

    let vertex_input = engine.vertex_input(&vertices, &index_buffer_data);
    let vertex_input_binding_descriptions = [vk::VertexInputBindingDescription {
        binding: 0,
        stride: std::mem::size_of::<Vertex>() as u32,
        input_rate: vk::VertexInputRate::VERTEX,
    }];

    let vertex_input_attribute_descriptions = [
        vk::VertexInputAttributeDescription {
            location: 0,
            binding: 0,
            format: vk::Format::R32G32B32A32_SFLOAT,
            offset: offset_of!(Vertex, pos) as u32,
        },
        vk::VertexInputAttributeDescription {
            location: 1,
            binding: 0,
            format: vk::Format::R32G32B32A32_SFLOAT,
            offset: offset_of!(Vertex, color) as u32,
        },
    ];

    let vertex_input_state_info = vk::PipelineVertexInputStateCreateInfo::builder()
        .vertex_attribute_descriptions(&vertex_input_attribute_descriptions)
        .vertex_binding_descriptions(&vertex_input_binding_descriptions);

    let vertex_input_assembly_state_info = vk::PipelineInputAssemblyStateCreateInfo {
        topology: vk::PrimitiveTopology::TRIANGLE_LIST,
        ..Default::default()
    };
    
    let vert_shader = spv_to_shader!(device_id, "./shaders/vert.spv");
    let frag_shader = spv_to_shader!(device_id, "./shaders/frag.spv");

    let shader_entry_name = unsafe { CStr::from_bytes_with_nul_unchecked(b"main\0") };
    let shader_stage_create_infos = [
        vk::PipelineShaderStageCreateInfo {
            module: vert_shader,
            p_name: shader_entry_name.as_ptr(),
            stage: vk::ShaderStageFlags::VERTEX,
            ..Default::default()
        },
        vk::PipelineShaderStageCreateInfo {
            s_type: vk::StructureType::PIPELINE_SHADER_STAGE_CREATE_INFO,
            module: frag_shader,
            p_name: shader_entry_name.as_ptr(),
            stage: vk::ShaderStageFlags::FRAGMENT,
            ..Default::default()
        },
    ];

    let surface_resolution = device_id.device().surface_resolution();
    let viewports = [vk::Viewport {
        x: 0.0,
        y: 0.0,
        width: surface_resolution.width as f32,
        height: surface_resolution.height as f32,
        min_depth: 0.0,
        max_depth: 1.0,
    }];
    let scissors = [surface_resolution.into()];
    let viewport_state_info = vk::PipelineViewportStateCreateInfo::builder()
        .scissors(&scissors)
        .viewports(&viewports);

    let rasterization_info = vk::PipelineRasterizationStateCreateInfo {
        front_face: vk::FrontFace::COUNTER_CLOCKWISE,
        line_width: 1.0,
        polygon_mode: vk::PolygonMode::FILL,
        ..Default::default()
    };
    let multisample_state_info = vk::PipelineMultisampleStateCreateInfo {
        rasterization_samples: vk::SampleCountFlags::TYPE_1,
        ..Default::default()
    };
    let noop_stencil_state = vk::StencilOpState {
        fail_op: vk::StencilOp::KEEP,
        pass_op: vk::StencilOp::KEEP,
        depth_fail_op: vk::StencilOp::KEEP,
        compare_op: vk::CompareOp::ALWAYS,
        ..Default::default()
    };
    let depth_state_info = vk::PipelineDepthStencilStateCreateInfo {
        depth_test_enable: 1,
        depth_write_enable: 1,
        depth_compare_op: vk::CompareOp::LESS_OR_EQUAL,
        front: noop_stencil_state,
        back: noop_stencil_state,
        max_depth_bounds: 1.0,
        ..Default::default()
    };
    let color_blend_attachment_states = [vk::PipelineColorBlendAttachmentState {
        blend_enable: 0,
        src_color_blend_factor: vk::BlendFactor::SRC_COLOR,
        dst_color_blend_factor: vk::BlendFactor::ONE_MINUS_DST_COLOR,
        color_blend_op: vk::BlendOp::ADD,
        src_alpha_blend_factor: vk::BlendFactor::ZERO,
        dst_alpha_blend_factor: vk::BlendFactor::ZERO,
        alpha_blend_op: vk::BlendOp::ADD,
        color_write_mask: vk::ColorComponentFlags::RGBA,
    }];
    let color_blend_state = vk::PipelineColorBlendStateCreateInfo::builder()
        .logic_op(vk::LogicOp::CLEAR)
        .attachments(&color_blend_attachment_states);

    let dynamic_state = [vk::DynamicState::VIEWPORT, vk::DynamicState::SCISSOR];
    let dynamic_state_info =
        vk::PipelineDynamicStateCreateInfo::builder().dynamic_states(&dynamic_state);

    let graphic_pipeline_info = vk::GraphicsPipelineCreateInfo::builder()
        .stages(&shader_stage_create_infos)
        .vertex_input_state(&vertex_input_state_info)
        .input_assembly_state(&vertex_input_assembly_state_info)
        .viewport_state(&viewport_state_info)
        .rasterization_state(&rasterization_info)
        .multisample_state(&multisample_state_info)
        .depth_stencil_state(&depth_state_info)
        .color_blend_state(&color_blend_state)
        .dynamic_state(&dynamic_state_info)
        .layout(engine.pipeline_layout)
        .render_pass(engine.render_pass);

    let device = device_id.device();
    let graphics_pipelines = unsafe {
        device.device
            .create_graphics_pipelines(
                vk::PipelineCache::null(),
                &[graphic_pipeline_info.build()],
                None,
            )
            .expect("Unable to create graphics pipeline")
    };

    let graphic_pipeline = graphics_pipelines[0];

    let clear_values = [
        vk::ClearValue {
            color: vk::ClearColorValue {
                float32: [0.0, 0.0, 0.0, 0.0],
            },
        },
        vk::ClearValue {
            depth_stencil: vk::ClearDepthStencilValue {
                depth: 1.0,
                stencil: 0,
            },
        },
    ];

    let draw_command_buffer = engine.create_command_buffers(1)[0];
    let present_complete_semaphore = engine.create_semaphore();
    let rendering_complete_semaphore = engine.create_semaphore();
    let render_fence = engine.create_fence();

    engine.setup();
    instance_id.instance().window.handle_events(|| {
        unsafe {
            device.device
                .wait_for_fences(&[render_fence], true, std::u64::MAX)
                .expect("Wait for fence failed.");

            device.device
                .reset_fences(&[render_fence])
                .expect("Reset fences failed.");
        }

        let present_idx = engine.present_idx(present_complete_semaphore);

        let device = device_id.device();
        let surface_resolution = device.surface_resolution();
        let render_pass_begin_info = vk::RenderPassBeginInfo::builder()
            .render_pass(engine.render_pass)
            .framebuffer(engine.framebuffers[present_idx as usize])
            .render_area(surface_resolution.into())
            .clear_values(&clear_values);

        engine.record_command_buffer(draw_command_buffer, |dvc, command_buffer| {
            unsafe {
                dvc.cmd_begin_render_pass(
                    command_buffer,
                    &render_pass_begin_info,
                    vk::SubpassContents::INLINE,
                );

                dvc.cmd_bind_pipeline(
                    command_buffer,
                    vk::PipelineBindPoint::GRAPHICS,
                    graphic_pipeline,
                );
                dvc.cmd_set_viewport(command_buffer, 0, &viewports);
                dvc.cmd_set_scissor(command_buffer, 0, &scissors);
                dvc.cmd_bind_vertex_buffers(
                    command_buffer,
                    0,
                    &[vertex_input.buffer.buffer],
                    &[0],
                );
                dvc.cmd_bind_index_buffer(
                    command_buffer,
                    vertex_input.index_buffer.buffer,
                    0,
                    vk::IndexType::UINT32,
                );
                dvc.cmd_draw_indexed(
                    command_buffer,
                    index_buffer_data.len() as u32,
                    1,
                    0,
                    0,
                    1,
                );
                // Or draw without the index buffer
                // device.cmd_draw(draw_command_buffer, 3, 1, 0, 0);
                dvc.cmd_end_render_pass(command_buffer);
            }
        });

        let command_buffers = vec![draw_command_buffer];
        let wait_sempahores = [present_complete_semaphore];
        let signal_semaphores = [rendering_complete_semaphore];
        let submit_info = vk::SubmitInfo::builder()
            .wait_semaphores(&wait_sempahores)
            .wait_dst_stage_mask(&[vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT])
            .command_buffers(&command_buffers)
            .signal_semaphores(&signal_semaphores);

        unsafe {
            device.device
                .queue_submit(device.queue, &[submit_info.build()], render_fence)
                .expect("queue submit failed.");
        }

        let wait_semaphors = [rendering_complete_semaphore];
        let swapchains = [engine.swapchain];
        let image_indices = [present_idx];
        let present_info = vk::PresentInfoKHR::builder()
            .wait_semaphores(&wait_semaphors) // &base.rendering_complete_semaphore)
            .swapchains(&swapchains)
            .image_indices(&image_indices);

        unsafe {
            device.swapchain_loader
                .queue_present(device.queue, &present_info)
                .expect("Error queueing present info");
        }

    });

    engine.wait_idle();
    
    for pipeline in graphics_pipelines {
        engine.cleanup_pipeline(&pipeline);
    }
    engine.cleanup_shader(&vert_shader);
    engine.cleanup_shader(&frag_shader);

    engine.cleanup_sempahore(&present_complete_semaphore);
    engine.cleanup_sempahore(&rendering_complete_semaphore);
    engine.cleanup_fence(&render_fence);
    engine.cleanup_buffer(&vertex_input.index_buffer);
    engine.cleanup_buffer(&vertex_input.buffer);
    engine.cleanup();

    println!("Cleaned up!!");
}