use std::sync::Arc;

use winit::window::Window;

pub struct State<'a> {
    window: Arc<Window>,
    instance: wgpu::Instance,
    surface: wgpu::Surface<'a>,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
}

impl<'a> State<'a> {
    pub fn new(window: Window) -> State<'a> {
        let window = Arc::new(window);
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        });
        let surface = instance.create_surface(window.clone()).unwrap();
        let adapter =
            futures_executor::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            }))
            .unwrap();

        let (device, queue) = futures_executor::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                label: None,
                memory_hints: Default::default(),
            },
            None,
        ))
        .unwrap();

        Self {
            window,
            instance,
            surface,
            adapter,
            device,
            queue,
        }
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn resume(&mut self) {}

    pub fn render(&self) {}
}
