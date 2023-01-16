macro_rules! find_queue {
    ($instance:expr, $surface_loader:expr, $surface:expr, $queue:expr) => {
        $instance
        .enumerate_physical_devices()
        .expect("Physical device error")
        .iter()
        .find_map(|pdevice| {
            $instance
                .get_physical_device_queue_family_properties(*pdevice)
                .iter()
                .enumerate()
                .find_map(|(index, info)| {
                    let supports_queue = info.queue_flags.contains($queue);
                    
                    let supports_surface = $surface_loader.get_physical_device_surface_support(
                        *pdevice,
                        index as u32,
                        $surface,
                    ).unwrap();

                    if supports_queue && supports_surface {
                        Some((*pdevice, index))
                    } else {
                        None
                    }
                })
        })
    }
}