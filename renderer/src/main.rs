
extern crate ash;
extern crate proc_macro;
extern crate shaderc;
extern crate winit;

use ash::vk;
use std::{cell::RefCell, ffi::CString};
use models::{Model, Vertex};
use world::ModelMatrix;

#[macro_use]
mod window;
use window::Window;

mod gpu;
use gpu::System;

mod gpuv2;
use gpuv2::{push_constants, shaders, viewport, Buffer, BufferSet};

mod engines;
use engines::basic::Engine;

use winit::event::VirtualKeyCode;

const MOVE_SPEED: f32 = 0.1;
const ROT_SPEED: f32 = 0.3;
const Z_SPEED: f32 = 0.05;

pub struct ModelGroup<'a> {
    models: Vec<Model<'a>>,
    model_matrices: Vec<ModelMatrix>,
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
    transforms: Vec<Transformation>,
    materials: Vec<Material>
}

impl<'a> ModelGroup<'a> {
    pub fn new() -> Self {
        Self {
            models: vec![],
            model_matrices: vec![],
            vertices: vec![],
            indices: vec![],
            transforms: vec![],
            materials: vec![]
        }
    }

    pub fn load(self, model: Model<'a>) -> Self {
        let mut models = self.models.clone();
        models.push(model);

        let mut model_matrices = self.model_matrices.clone();
        model_matrices.push(Default::default());

        let mut vertices = vec![];
        for model in &models {
            vertices.extend_from_slice(model.vertices);
        }

        let mut indices = vec![];
        for model in &models {
            for idx in model.indices {
                indices.push(*idx as u32);
            }
        }

        let mut transforms = vec![];
        let mut materials = vec![];
        for m in model_matrices.clone() {
            transforms.push(Transformation{matrix: m.matrix().col_major()});
            materials.push(Material{id: 1})
        }

        Self {
            models,
            model_matrices,
            vertices,
            indices,
            transforms,
            materials
        }
    }

    pub fn set_model_matrix(&mut self, index: usize, matrix: ModelMatrix) -> &Self {
        self.model_matrices[index] = matrix;
        self.transforms[index] = Transformation{matrix: matrix.matrix().col_major()};

        self
    }

    pub fn set_material(&mut self, index: usize, material: Material) -> &Self {
        self.materials[index] = material;
        self
    }

    pub fn vertices(&self) -> &Vec<Vertex> {
        &self.vertices
    }

    pub fn indices(&self) -> &Vec<u32> {
        &self.indices
    }

    pub fn materials(&self) -> &Vec<Material> {
        &self.materials
    }

    pub fn transforms(&self) -> &Vec<Transformation> {
        &self.transforms
    }

    pub fn model_matrices(&self) -> &Vec<ModelMatrix> {
        &self.model_matrices
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Transformation {
    pub matrix: [[f32; 4]; 4],
}

#[derive(Copy, Clone, Debug)]
pub struct Material {
    pub id: u32,
}

#[derive(Copy, Clone, Debug)]
pub struct Animation {
    pub start_transform: [[f32; 4]; 4],
    pub end_transform: [[f32; 4]; 4],
    pub start_time: u32,
    pub end_time: u32,
    pub copied: bool
}

#[macro_use]
mod macros;

fn main() {
    //
    // Open Window
    //
    let window = Window::new("My Window", 800, 600);

    //
    // App Info
    //
    let entry = unsafe { ash::Entry::load().expect("Error loading ash::Entry") };
    let app_name_str = CString::new("my_renderer_app").unwrap();
    let engine_name_str = CString::new("my_renderer_engine").unwrap();
    let start_time = std::time::SystemTime::now();
    let app_info = vk::ApplicationInfo::builder()
        .application_name(&app_name_str)
        .application_version(0)
        .engine_name(&engine_name_str)
        .engine_version(0)
        .api_version(vk::make_api_version(0, 1, 0, 0));
    
    //
    // Initialize Engine
    //
    let mut system = System::new(entry, app_info);
    let instance_id = system.instance(window);
    let surface_id = system.surface(&instance_id);
    let device_id = system.device(&surface_id).expect("Device not found");
    let engine = Engine::new(&system, &device_id);
    
    let device_props = gpuv2::DeviceProperties {
        physical_memory: device_id.device().memory_properties(),
        physical_props: device_id.device().physical_device_properties()
    };

    //
    // Setup Shader Inputs
    //
    let models = ModelGroup::new()
        .load(models::d2::Triangle::new().into())
        .load(models::d2::Rectangle::new().into())
        .load(models::d2::Rectangle::new().into());

    let models_ref = RefCell::new(models);
    models_ref.borrow_mut().set_material(1, Material{id: 2});
    models_ref.borrow_mut().set_material(2, Material{id: 3});

    models_ref.borrow_mut().set_model_matrix(0, ModelMatrix {
        scale: (1.0, 1.0, 1.0), rotation: (0.0, 0.0, 0.0), translation: (0.8, 0.8, -2.0)
    });

    models_ref.borrow_mut().set_model_matrix(1, ModelMatrix {
        scale: (1.0, 1.0, 1.0), rotation: (0.0, 0.0, 0.0), translation: (-1.0, -1.5, -2.3)
    });

    models_ref.borrow_mut().set_model_matrix(2, ModelMatrix {
        scale: (1.0, 1.0, 1.0), rotation: (0.0, 0.0, 0.0), translation: (0.0, 0.0, -2.5)
    });

    let transformations_need_copied = RefCell::new(false);
    let transformation_buffer: Buffer<Transformation> = Buffer::new(&device_props)
        .stages(vk::ShaderStageFlags::VERTEX)
        .binding(0)
        .usage(vk::BufferUsageFlags::UNIFORM_BUFFER)
        .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER_DYNAMIC)
        .sharing_mode(vk::SharingMode::EXCLUSIVE)
        .memory_flags(vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT)
        .load(&device_id.device().device, &models_ref.borrow().transforms());

    let material_buffer: Buffer<Material> = Buffer::new(&device_props)
        .stages(vk::ShaderStageFlags::FRAGMENT)
        .binding(1)
        .usage(vk::BufferUsageFlags::UNIFORM_BUFFER)
        .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER_DYNAMIC)
        .sharing_mode(vk::SharingMode::EXCLUSIVE)
        .memory_flags(vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT)
        .load(&device_id.device().device, &models_ref.borrow().materials());

    let vertex_input: Buffer<Vertex> = Buffer::new(&device_props)
        .usage(vk::BufferUsageFlags::VERTEX_BUFFER)
        .sharing_mode(vk::SharingMode::EXCLUSIVE)
        .memory_flags(vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT)
        .vertex_input_attribute(0, 0, offset_of!(Vertex, pos) as u32, vk::Format::R32G32B32_SFLOAT)
        .load(&device_id.device().device, models_ref.borrow().vertices().as_slice());

    let index_buffer: Buffer<u32> = Buffer::new(&device_props)
        .usage(vk::BufferUsageFlags::INDEX_BUFFER)
        .sharing_mode(vk::SharingMode::EXCLUSIVE)
        .memory_flags(vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT)
        .load(&device_id.device().device, models_ref.borrow().indices().as_slice());

    let vertex_descs = [
        vk::VertexInputBindingDescription {
            binding: 0,
            stride: std::mem::size_of::<Vertex>() as u32,
            input_rate: vk::VertexInputRate::VERTEX,
        }
    ];

    let vertex_input_state_info = vk::PipelineVertexInputStateCreateInfo::builder()
        .vertex_attribute_descriptions(vertex_input.attributes())
        .vertex_binding_descriptions(&vertex_descs);

    let per_pass_buffers = BufferSet::new()
        .add(&transformation_buffer)
        .add(&material_buffer)
        .allocate(&device_id.device().device);

    //
    // Initialize Shaders
    //
    let shader_stages = shaders::Stages::new(&[
        shaders::Stage::new(
            vk::ShaderStageFlags::VERTEX, 
            glsl_to_shader!(device_id, "./shaders/triangle.vert", shaderc::ShaderKind::Vertex)
        ),
        
        shaders::Stage::new(
            vk::ShaderStageFlags::FRAGMENT, 
            glsl_to_shader!(device_id, "./shaders/triangle.frag", shaderc::ShaderKind::Fragment)
        )
    ]);

    //
    // Setup Push Constants
    //
    let push_constant_ranges = push_constants::Ranges::new()
        .add::<u32>(0, vk::ShaderStageFlags::FRAGMENT | vk::ShaderStageFlags::VERTEX);

    let layout_info = vk::PipelineLayoutCreateInfo::builder()
        .set_layouts(&[
            per_pass_buffers.descriptor_set_layout(),
        ])
        .push_constant_ranges(push_constant_ranges.build())
        .build();

    //
    // Pre-Fill Uniform Buffers
    //
    let mut write_desc_sets = vec![];
    write_desc_sets.extend(per_pass_buffers.write_descriptor_sets());
   
    unsafe {
        device_id.device().device.update_descriptor_sets(
            write_desc_sets.as_slice(),
            &[]
        )
    }

    //
    // Initialize Graphics Pipeline
    //
    let dynamic_state_info = get_dynamic_state_info(&[
        vk::DynamicState::VIEWPORT,
        vk::DynamicState::SCISSOR
    ]);

    let pipeline_layout = engine.create_pipeline_layout(layout_info);
    let viewport = viewport::Builder::new(&device_id);
    let rasterization_info = get_rasterization_info();
    let multisample_state_info = get_multisample_state_info();
    let depth_state_info = get_depth_state_info();
    let attachment_states = get_color_blend_attachment_states();
    let color_blend_state = get_color_blend_state(&attachment_states);
    let vertex_input_assembly_state_info = get_vertex_input_assembly_state_info();
    let graphic_pipeline_info = vk::GraphicsPipelineCreateInfo::builder()
        .stages(shader_stages.build())
        .viewport_state(&viewport.state)
        .vertex_input_state(&vertex_input_state_info)
        .input_assembly_state(&vertex_input_assembly_state_info)
        .rasterization_state(&rasterization_info)
        .multisample_state(&multisample_state_info)
        .depth_stencil_state(&depth_state_info)
        .color_blend_state(&color_blend_state)
        .dynamic_state(&dynamic_state_info)
        .layout(pipeline_layout)
        .render_pass(engine.render_pass);

    let graphics_pipeline = unsafe {
        device_id.device().device
            .create_graphics_pipelines(
                vk::PipelineCache::null(),
                &[graphic_pipeline_info.build()],
                None,
            )
            .expect("Unable to create graphics pipeline")
    }[0];

    //
    // Initialize Command Buffers and Gates
    //
    let clear_values = get_clear_values();
    let draw_command_buffer = engine.create_command_buffers(1)[0];
    let present_complete_semaphore = engine.create_semaphore();
    let rendering_complete_semaphore = engine.create_semaphore();
    let render_fence = engine.create_fence();

    //
    // Prepare to draw
    //
    engine.setup();

    //
    // Render Loop
    //
    let mut shift_pressed = false;
    let mut transform_selected = 0;
    let handle_event = |_: &ash::Device, event: winit::event::Event<()>, _curr_time: std::time::SystemTime| {
        match event {
            key_pressed!(VirtualKeyCode::LShift) => shift_pressed = true,
            key_released!(VirtualKeyCode::LShift) => shift_pressed = false,
            key_pressed!(VirtualKeyCode::Key1) => transform_selected = 0,
            key_pressed!(VirtualKeyCode::Key2) => transform_selected = 1,

            key_pressed!(VirtualKeyCode::W) | key_pressed!(VirtualKeyCode::A) | key_pressed!(VirtualKeyCode::S) | key_pressed!(VirtualKeyCode::D) => {
                let mut translation_change = (0.0, 0.0, 0.0);
                let scale_change = (0.0, 0.0, 0.0);
                let mut rot_change = (0.0, 0.0, 0.0);

                match event {
                    key_pressed!(VirtualKeyCode::W) if shift_pressed => {
                        translation_change = (0.0, 0.0, Z_SPEED);
                    }
                    
                    key_pressed!(VirtualKeyCode::A) if shift_pressed => {
                        rot_change = (0.0, -ROT_SPEED, 0.0);
                    }

                    key_pressed!(VirtualKeyCode::S) if shift_pressed => {
                        translation_change = (0.0, 0.0, -Z_SPEED);
                    }

                    key_pressed!(VirtualKeyCode::D) if shift_pressed => {
                        rot_change = (0.0, ROT_SPEED, 0.0);
                    }

                    key_pressed!(VirtualKeyCode::W) if !shift_pressed => {
                        translation_change = (0.0, -MOVE_SPEED, 0.0);
                    }
                    
                    key_pressed!(VirtualKeyCode::A) if !shift_pressed => {
                        translation_change = (-MOVE_SPEED, 0.0, 0.0);
                    }

                    key_pressed!(VirtualKeyCode::S) if !shift_pressed => {
                        translation_change = (0.0, MOVE_SPEED, 0.0);
                    }

                    key_pressed!(VirtualKeyCode::D) if !shift_pressed => {
                        translation_change = (MOVE_SPEED, 0.0, 0.0);
                    },

                    _ => {}
                };

                let start = models_ref.borrow().model_matrices()[transform_selected];
                let end = ModelMatrix {
                    translation: (
                        start.translation.0 + translation_change.0,
                        start.translation.1 + translation_change.1,
                        start.translation.2 + translation_change.2,
                    ),
                    scale: (
                        start.scale.0 + scale_change.0,
                        start.scale.1 + scale_change.1,
                        start.scale.2 + scale_change.2,
                    ),
                    rotation: (
                        start.rotation.0 + rot_change.0,
                        start.rotation.1 + rot_change.1,
                        start.rotation.2 + rot_change.2,
                    ),
                };

                models_ref.borrow_mut().set_model_matrix(transform_selected, end);
                *transformations_need_copied.borrow_mut() = true;
            },

            _ => ()
        }
    };

    let render_loop = |device: &ash::Device, curr_time: std::time::SystemTime| {
        unsafe {
            device
                .wait_for_fences(&[render_fence], true, std::u64::MAX)
                .expect("Wait for fence failed.");

            device
                .reset_fences(&[render_fence])
                .expect("Reset fences failed.");
        }

        let elapsed = curr_time.duration_since(start_time).unwrap().as_millis() as u32;
        let all_push_constants = [elapsed];
        let (_, push_constant_bytes, _) = unsafe {
            all_push_constants.as_slice().align_to::<u8>()
        };

        let present_idx = engine.present_idx(present_complete_semaphore);
        let device = device_id.device();
        let surface_resolution = device.surface_resolution();
        let render_pass_begin_info = vk::RenderPassBeginInfo::builder()
            .render_pass(engine.render_pass)
            .framebuffer(engine.framebuffers[present_idx as usize])
            .render_area(surface_resolution.into())
            .clear_values(&clear_values);

        let mut needs_copied = transformations_need_copied.borrow_mut();
        if *needs_copied {
            *needs_copied = false;
            transformation_buffer.copy(&device.device, models_ref.borrow().transforms().as_slice());
        }

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
                    graphics_pipeline,
                );
                dvc.cmd_set_viewport(command_buffer, 0, &viewport.viewports);
                dvc.cmd_set_scissor(command_buffer, 0, &viewport.scissors);
                dvc.cmd_bind_vertex_buffers(
                    command_buffer,
                    0,
                    &[vertex_input.buffer.unwrap()],
                    &[0],
                );

                dvc.cmd_bind_index_buffer(
                    command_buffer,
                    index_buffer.buffer.unwrap(),
                    0,
                    vk::IndexType::UINT32,
                );

                dvc.cmd_push_constants(
                    command_buffer,
                    pipeline_layout,
                    vk::ShaderStageFlags::FRAGMENT | vk::ShaderStageFlags::VERTEX,
                    0,
                    push_constant_bytes,
                );

                let models = models_ref.borrow();

                let desc_set = per_pass_buffers.descriptor_set();
                let mut curr_idx: usize = 0;
                let mut curr_vtx: usize = 0;
                for i in 0..models.models.len() {
                    let indices = models.models[i].indices;
                    let vertices = models.models[i].vertices;

                    dvc.cmd_bind_descriptor_sets(
                        command_buffer, 
                        vk::PipelineBindPoint::GRAPHICS,
                        pipeline_layout,
                        0,
                        &[desc_set],
                        &[
                            per_pass_buffers.dynamic_offset(0, i),
                            per_pass_buffers.dynamic_offset(1, i)
                        ]
                    );

                    dvc.cmd_draw_indexed(
                        command_buffer,
                        indices.len() as u32,
                        1,
                        curr_idx as u32,
                        curr_vtx as i32,
                        1,
                    );

                    curr_idx += indices.len();
                    curr_vtx += vertices.len();
                }
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

    };

    engine.render_loop(render_loop, handle_event);

    engine.wait_idle();
    
    engine.cleanup_pipeline_layout(&pipeline_layout);
    engine.cleanup_pipeline(&graphics_pipeline);
    engine.cleanup(&shader_stages);

    engine.cleanup_sempahore(&present_complete_semaphore);
    engine.cleanup_sempahore(&rendering_complete_semaphore);
    engine.cleanup_fence(&render_fence);
    engine.cleanup(&per_pass_buffers);
    engine.cleanup(&index_buffer);
    engine.cleanup(&vertex_input);
    engine.cleanup(&transformation_buffer);
    engine.cleanup(&material_buffer);
    engine.cleanup(&engine);

    println!("Cleaned up!!");
}

fn get_vertex_input_assembly_state_info() -> vk::PipelineInputAssemblyStateCreateInfo {
    vk::PipelineInputAssemblyStateCreateInfo {
        topology: vk::PrimitiveTopology::TRIANGLE_LIST,
        ..Default::default()
    }
}

fn get_rasterization_info() -> vk::PipelineRasterizationStateCreateInfo {
    vk::PipelineRasterizationStateCreateInfo {
        front_face: vk::FrontFace::COUNTER_CLOCKWISE,
        line_width: 1.0,
        polygon_mode: vk::PolygonMode::FILL,
        ..Default::default()
    }
}

fn get_multisample_state_info() -> vk::PipelineMultisampleStateCreateInfo {
    vk::PipelineMultisampleStateCreateInfo {
        rasterization_samples: vk::SampleCountFlags::TYPE_1,
        ..Default::default()
    }
}

fn get_depth_state_info() -> vk::PipelineDepthStencilStateCreateInfo {
    let noop_stencil_state = vk::StencilOpState {
        fail_op: vk::StencilOp::KEEP,
        pass_op: vk::StencilOp::KEEP,
        depth_fail_op: vk::StencilOp::KEEP,
        compare_op: vk::CompareOp::ALWAYS,
        ..Default::default()
    };

    vk::PipelineDepthStencilStateCreateInfo {
        depth_test_enable: 1,
        depth_write_enable: 1,
        depth_compare_op: vk::CompareOp::LESS_OR_EQUAL,
        front: noop_stencil_state,
        back: noop_stencil_state,
        max_depth_bounds: 1.0,
        ..Default::default()
    }
}

fn get_color_blend_attachment_states() -> [vk::PipelineColorBlendAttachmentState; 1] {
    [vk::PipelineColorBlendAttachmentState {
        blend_enable: 0,
        src_color_blend_factor: vk::BlendFactor::SRC_COLOR,
        dst_color_blend_factor: vk::BlendFactor::ONE_MINUS_DST_COLOR,
        color_blend_op: vk::BlendOp::ADD,
        src_alpha_blend_factor: vk::BlendFactor::ZERO,
        dst_alpha_blend_factor: vk::BlendFactor::ZERO,
        alpha_blend_op: vk::BlendOp::ADD,
        color_write_mask: vk::ColorComponentFlags::RGBA,
    }]
}

fn get_color_blend_state<'a>(attachment_states: &'a [vk::PipelineColorBlendAttachmentState]) -> vk::PipelineColorBlendStateCreateInfoBuilder<'a> {
    vk::PipelineColorBlendStateCreateInfo::builder()
        .logic_op(vk::LogicOp::CLEAR)
        .attachments(attachment_states)
}

fn get_dynamic_state_info(states: &[vk::DynamicState]) -> vk::PipelineDynamicStateCreateInfo {
    vk::PipelineDynamicStateCreateInfo::builder()
        .dynamic_states(states)
        .build()
}

fn get_clear_values() -> [vk::ClearValue; 2] {
    [
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
    ]
}