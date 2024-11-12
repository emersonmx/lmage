mod application;
//mod state;
mod renderer;

use application::{App, AppEvent};
use winit::event_loop::{ControlFlow, EventLoop};

pub fn run() {
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "lmage=trace".into()),
        )
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting tracing default failed");

    let event_loop = EventLoop::<AppEvent>::with_user_event().build().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::new(&event_loop);
    event_loop.run_app(&mut app).unwrap();
}
