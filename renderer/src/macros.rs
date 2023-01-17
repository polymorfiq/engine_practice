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