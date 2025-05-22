use ash::{Device, vk::{DeviceCreateInfo, PhysicalDevice, Queue}, Instance};

fn create_logical_device(
    instance: &Instance,
    physical_device: PhysicalDevice,
) -> (Device, Queue) {
    // TODO let indices = VulkanApp::find_queue_family(instance, physical_device);

    let device: ash::Device = unsafe {
        instance
            .create_device(physical_device, &DeviceCreateInfo::default(), None)
            .expect("Failed to create logical Device!")
    };

    let graphics_queue = unsafe { device.get_device_queue(indices.graphics_family.unwrap(), 0) };

    (device, graphics_queue)
}