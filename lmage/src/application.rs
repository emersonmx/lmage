use std::sync::Arc;

use tracing::{error, info, warn};
use winit::dpi::PhysicalSize;
use winit::event_loop;
use winit::window::{Window, WindowAttributes};
use winit::{application::ApplicationHandler, event::WindowEvent};

use crate::renderer::Renderer;

#[derive(Debug)]
pub enum AppEvent {
    Ready { renderer: Renderer<'static> },
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
        let attributes = WindowAttributes::default().with_title("Let's Make A Game Engine!");
        #[cfg(not(target_arch = "wasm32"))]
        let attributes = attributes.with_visible(false);

        if let Ok(window) = event_loop.create_window(attributes) {
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
                use winit::platform::web::WindowExtWebSys;
                web_sys::window()
                    .and_then(|win| win.document())
                    .and_then(|doc| {
                        let dst = doc.get_element_by_id("game-window")?;
                        let canvas = web_sys::Element::from(window.canvas()?);
                        dst.append_child(&canvas).ok()?;
                        Some(())
                    })
                    .expect("Couldn't append canvas to document body.");

                let _ = window.request_inner_size(PhysicalSize::new(640, 480));
                let (width, height) = window.inner_size().into();
                self.last_window_size = (width, height);

                let event_loop_proxy = self.event_loop_proxy.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let renderer = Renderer::new(window.clone(), width, height).await;

                    event_loop_proxy
                        .send_event(AppEvent::Ready { renderer })
                        .expect("Failed to send ready event");
                });
            }
            #[cfg(not(target_arch = "wasm32"))]
            {
                let renderer = futures_executor::block_on(async move {
                    let mut renderer = Renderer::new(window.clone(), width, height).await;
                    window.request_redraw();
                    renderer.resize(width, height);
                    let _ = renderer.present();
                    window.set_visible(true);
                    renderer
                });
                self.event_loop_proxy
                    .send_event(AppEvent::Ready { renderer })
                    .expect("Failed to send ready event");
            }
        }
    }

    fn user_event(&mut self, _event_loop: &event_loop::ActiveEventLoop, event: AppEvent) {
        match event {
            AppEvent::Ready { renderer } => {
                info!("Received ready event");
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
                self.last_window_size = (width, height);
            }
            WindowEvent::RedrawRequested => {
                match renderer.present() {
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
