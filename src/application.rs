use tracing::{error, info, warn};
use winit::event_loop;
use winit::window::WindowAttributes;
use winit::{application::ApplicationHandler, event::WindowEvent};

use crate::state::State;

#[derive(Default)]
pub struct App<'a> {
    state: Option<State<'a>>,
}

impl ApplicationHandler for App<'_> {
    fn resumed(&mut self, event_loop: &event_loop::ActiveEventLoop) {
        let window = event_loop
            .create_window(
                WindowAttributes::default()
                    .with_title("Let's Make A Game Engine!")
                    .with_visible(false),
            )
            .unwrap();

        let mut state = State::new(window);
        state.resume();
        state.window().request_redraw();
        state.resize(state.window_size());
        let _ = state.render();
        state.window().set_visible(true);
        self.state = Some(state);
    }

    fn window_event(
        &mut self,
        event_loop: &event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        let state = match self.state.as_mut() {
            Some(state) => state,
            None => return,
        };
        let window = &state.window();
        if window_id != window.id() {
            return;
        }

        // TODO: Create an input manager
        if state.input(&event) {
            return;
        }
        match event {
            WindowEvent::CloseRequested => {
                info!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::Resized(physical_size) => {
                state.resize(physical_size);
            }
            WindowEvent::RedrawRequested => {
                state.update();
                match state.render() {
                    Ok(_) => {}
                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                        state.resize(state.window_size())
                    }
                    Err(wgpu::SurfaceError::OutOfMemory) => {
                        error!("OutOfMemory");
                        event_loop.exit();
                    }
                    Err(wgpu::SurfaceError::Timeout) => {
                        warn!("Surface timeout");
                    }
                };
                state.window().request_redraw();
            }
            _ => {}
        }
    }
}
