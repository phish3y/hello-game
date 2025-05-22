use ash::{vk::{DeviceCreateInfo, PhysicalDevice, Queue}, Device, Instance};

fn get_logical_device(
    instance: &Instance,
    physical_device: PhysicalDevice,
) -> Device {
    let device: Device = unsafe {
        instance
            .create_device(physical_device, &DeviceCreateInfo::default(), None)
            .unwrap()
    };

    device
}

fn get_queue(device: Device) -> Queue {
    let indices = VulkanApp::find_queue_family(instance, physical_device);

    let graphics_queue = unsafe { device.get_device_queue(indices.graphics_family.unwrap(), 0) };

}
