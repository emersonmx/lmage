mod state;

use state::State;
use tracing::info;
use winit::event_loop::{self, ControlFlow, EventLoop};
use winit::window::WindowAttributes;
use winit::{application::ApplicationHandler, event::WindowEvent};

#[derive(Default)]
struct App<'a> {
    state: Option<State<'a>>,
}

impl ApplicationHandler for App<'_> {
    fn resumed(&mut self, event_loop: &event_loop::ActiveEventLoop) {
        let window = event_loop
            .create_window(WindowAttributes::default().with_title("Let's Make A Game Engine!"))
            .unwrap();

        let mut state = State::new(window);
        state.resume();
        self.state = Some(state);
    }

    fn window_event(
        &mut self,
        event_loop: &event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        let window = match self.state.as_ref() {
            Some(state) => &state.window(),
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
            WindowEvent::RedrawRequested => {
                if let Some(state) = self.state.as_ref() {
                    state.render();
                    state.window().request_redraw();
                }
            }
            _ => {}
        }
    }
}

pub fn run() {
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
