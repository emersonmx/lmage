use std::sync::Arc;

use winit::{dpi::PhysicalSize, window::Window};

#[derive(Debug)]
struct WindowContext {
    window: Arc<Window>,
    size: PhysicalSize<u32>,
}

#[derive(Debug)]
struct RenderContext<'a> {
    instance: wgpu::Instance,
    surface: wgpu::Surface<'a>,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
}

pub struct State<'a> {
    window_context: WindowContext,
    render_context: RenderContext<'a>,
}

impl<'a> State<'a> {
    pub fn new(window: Window) -> State<'a> {
        let window = Arc::new(window);
        let size = window.inner_size();

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
            window_context: WindowContext { window, size },
            render_context: RenderContext {
                instance,
                surface,
                adapter,
                device,
                queue,
            },
        }
    }

    pub fn window(&self) -> &Window {
        &self.window_context.window
    }

    pub fn resume(&mut self) {}

    pub fn render(&self) {}
}
