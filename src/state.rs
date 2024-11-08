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
    config: wgpu::SurfaceConfiguration,
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

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        Self {
            window_context: WindowContext { window, size },
            render_context: RenderContext {
                instance,
                surface,
                adapter,
                device,
                queue,
                config,
            },
        }
    }

    pub fn window(&self) -> &Window {
        &self.window_context.window
    }

    pub fn resume(&mut self) {}

    pub fn render(&self) {}
}
