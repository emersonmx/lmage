use std::sync::Arc;

use tracing::{error, info, warn};
use winit::dpi::PhysicalSize;
use winit::event_loop;
use winit::window::{Window, WindowAttributes};
use winit::{application::ApplicationHandler, event::WindowEvent};

use crate::renderer::Renderer;

#[derive(Debug)]
pub enum AppEvent {
    ContextReadyEvent { renderer: Renderer<'static> },
}

pub struct App {
    window: Option<Arc<Window>>,
    last_window_size: (u32, u32),
    renderer: Option<Renderer<'static>>,
    event_loop_proxy: event_loop::EventLoopProxy<AppEvent>,
}

impl App {
    pub fn new(event_loop: &event_loop::EventLoop<AppEvent>) -> Self {
        App {
            window: None,
            last_window_size: (0, 0),
            renderer: None,
            event_loop_proxy: event_loop.create_proxy(),
        }
    }
}

impl ApplicationHandler<AppEvent> for App {
    fn resumed(&mut self, event_loop: &event_loop::ActiveEventLoop) {
        if let Ok(window) = event_loop.create_window(
            WindowAttributes::default()
                .with_title("Let's Make A Game Engine!")
                .with_visible(false),
        ) {
            let first_window = self.window.is_none();
            let window = Arc::new(window);
            let (width, height) = window.inner_size().into();
            self.window = Some(window.clone());
            self.last_window_size = (width, height);

            if !first_window {
                return;
            }

            #[cfg(target_arch = "wasm32")]
            {
                todo!("Make WASM setup!");
            }

            #[cfg(not(target_arch = "wasm32"))]
            {
                let renderer = futures_executor::block_on(async move {
                    let mut renderer = Renderer::new(window.clone(), width, height).await;
                    window.request_redraw();
                    renderer.resize(width, height);
                    let _ = renderer.render();
                    window.set_visible(true);
                    renderer
                });
                self.event_loop_proxy
                    .send_event(AppEvent::ContextReadyEvent { renderer })
                    .expect("Failed to send context ready event");
            }
        }
    }

    fn user_event(&mut self, _event_loop: &event_loop::ActiveEventLoop, event: AppEvent) {
        match event {
            AppEvent::ContextReadyEvent { renderer } => {
                info!("Received context ready event");
                self.renderer = Some(renderer);
            }
        }
    }

    fn window_event(
        &mut self,
        event_loop: &event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        let renderer = match self.renderer.as_mut() {
            Some(renderer) => renderer,
            None => return,
        };
        let window = match self.window.as_ref() {
            Some(window) => window,
            None => return,
        };
        if window_id != window.id() {
            return;
        }

        match event {
            WindowEvent::CloseRequested => {
                info!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::Resized(PhysicalSize { width, height }) => {
                renderer.resize(width, height);
            }
            WindowEvent::RedrawRequested => {
                match renderer.render() {
                    Ok(_) => {}
                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                        let (width, height) = self.last_window_size;
                        renderer.resize(width, height);
                    }
                    Err(wgpu::SurfaceError::OutOfMemory) => {
                        error!("OutOfMemory");
                        event_loop.exit();
                    }
                    Err(wgpu::SurfaceError::Timeout) => {
                        warn!("Surface timeout");
                    }
                }
                window.request_redraw();
            }
            _ => {}
        }
    }
}
