macro_rules! spv_to_shader {
    ($device_id:expr, $path:expr) => {
        {
            let mut cursor = std::io::Cursor::new(&include_bytes!($path)[..]);
            let vertex_code = ash::util::read_spv(&mut cursor).expect("Failed to read spv file");
            let shader_info = vk::ShaderModuleCreateInfo::builder().code(&vertex_code);

            unsafe {
                $device_id.device().device
                    .create_shader_module(&shader_info, None)
                    .expect("Vertex shader module error")
            }
        }
    }
}


macro_rules! glsl_to_shader {
    ($device_id:expr, $path:expr, $kind:expr) => {
        {
            let source = include_str!($path);
            let compiler = shaderc::Compiler::new().unwrap();
            let mut options = shaderc::CompileOptions::new().unwrap();
            options.add_macro_definition("EP", Some("main"));
            let binary_result = compiler.compile_into_spirv(
                source,
                $kind,
                $path,
                "main",
                Some(&options)
            ).expect("Failed to compile to SPIR-V");

            let shader_info = vk::ShaderModuleCreateInfo::builder().code(&binary_result.as_binary());

            unsafe {
                $device_id.device().device
                    .create_shader_module(&shader_info, None)
                    .expect("Vertex shader module error")
            }
        }
    }
}
