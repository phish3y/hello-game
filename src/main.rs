use ash::vk::{InstanceCreateInfo, PhysicalDevice};
use ash::{Entry, Instance};
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event_loop::EventLoop;
use winit::window::{Window, WindowAttributes};

mod v;

const WINDOW_TITLE: &'static str = "Game";
const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

#[derive(Default)]
struct Game {
    window: Option<Window>,
    instance: Option<Instance>,
    device: Option<PhysicalDevice>,
}

impl ApplicationHandler for Game {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if self.window.is_none() || self.instance.is_none() || self.device.is_none() {
            let attrs: WindowAttributes = WindowAttributes::default()
                .with_title(WINDOW_TITLE)
                .with_inner_size(LogicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT))
                .with_resizable(true)
                .with_visible(true);

            let window: Window = event_loop.create_window(attrs).unwrap();

            let entry: Entry = unsafe { Entry::load().unwrap() };
            let instance: Instance = unsafe {
                entry
                    .create_instance(&InstanceCreateInfo::default(), None)
                    .unwrap()
            };

            let device: PhysicalDevice = v::physical::get_best_device(&instance).unwrap();
            tracing::info!(device = ?unsafe { std::ffi::CStr::from_ptr(instance.get_physical_device_properties(device).device_name.as_ptr()) });

            self.window = Some(window);
            self.instance = Some(instance);
            self.device = Some(device);
        }
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            winit::event::WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            winit::event::WindowEvent::RedrawRequested => {}
            _ => {}
        }
    }

    fn new_events(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        cause: winit::event::StartCause,
    ) {
    }
}

impl Drop for Game {
    fn drop(&mut self) {
        self.instance.take().map(|instance| {
            unsafe { instance.destroy_instance(None) };
        });
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().json().init();

    Ok(EventLoop::new()?.run_app(&mut Game::default())?)
}
