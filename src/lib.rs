use std::collections::HashMap;

use tracing::info;
use winit::event_loop::{self, ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};
use winit::{application::ApplicationHandler, event::WindowEvent};

#[derive(Default)]
struct App {
    windows: HashMap<WindowId, Window>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &event_loop::ActiveEventLoop) {
        let window = event_loop
            .create_window(Window::default_attributes())
            .unwrap();

        self.windows.insert(window.id(), window);
    }

    fn window_event(
        &mut self,
        event_loop: &event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        let _window = match self.windows.get_mut(&window_id) {
            Some(window) => window,
            None => return,
        };

        match event {
            WindowEvent::CloseRequested => {
                info!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {}
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        for window in self.windows.values() {
            window.request_redraw()
        }
    }
}

pub async fn run() {
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "lmage=trace".into()),
        )
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting tracing default failed");

    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::default();
    event_loop.run_app(&mut app).unwrap();
}
