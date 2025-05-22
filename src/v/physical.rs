use ash::vk::{PhysicalDevice, PhysicalDeviceType, QueueFlags};

pub fn get_best_device(instance: &ash::Instance) -> Option<PhysicalDevice> {
    let physical_devices: Vec<PhysicalDevice> =
        unsafe { instance.enumerate_physical_devices().unwrap() };

    let mut suitable: Vec<PhysicalDevice> = Vec::new();
    for &physical_device in physical_devices.iter() {
        if is_device_suitable(instance, physical_device) {
            suitable.push(physical_device)
        }
    }

    for &device in &suitable {
        let properties = unsafe { instance.get_physical_device_properties(device) };
        if properties.device_type == PhysicalDeviceType::DISCRETE_GPU {
            return Some(device);
        }
    }

    suitable.into_iter().next()
}

fn is_device_suitable(instance: &ash::Instance, physical_device: PhysicalDevice) -> bool {
    let queue_families =
        unsafe { instance.get_physical_device_queue_family_properties(physical_device) };

    for queue_family in queue_families.iter() {
        if queue_family.queue_count > 0 && queue_family.queue_flags.contains(QueueFlags::GRAPHICS) {
            return true;
        }
    }

    return false;
}
