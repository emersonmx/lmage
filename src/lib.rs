#[cfg(target_arch = "wasm32")]
use winit::platform::web::EventLoopExtWebSys;

use application::{App, AppEvent};
use winit::event_loop::{ControlFlow, EventLoop};

mod application;
//mod state;
mod renderer;

#[cfg(target_arch = "wasm32")]
mod wasm {
    use wasm_bindgen::prelude::*;
    #[wasm_bindgen(start)]
    pub fn run() {
        crate::run();
    }
}

pub fn run() {
    #[cfg(target_arch = "wasm32")]
    {
        use tracing_subscriber::fmt::format::Pretty;
        use tracing_subscriber::prelude::*;
        use tracing_web::{performance_layer, MakeWebConsoleWriter};

        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        let fmt_layer = tracing_subscriber::fmt::layer()
            .with_ansi(false) // Only partially supported across browsers
            .without_time() // std::time is not available in browsers
            .with_writer(MakeWebConsoleWriter::new());
        let perf_layer = performance_layer().with_details_from_fields(Pretty::default());
        tracing_subscriber::registry()
            .with(fmt_layer)
            .with(perf_layer)
            .init();
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        use tracing_subscriber::layer::SubscriberExt;

        let subscriber = tracing_subscriber::FmtSubscriber::builder()
            .with_env_filter(
                tracing_subscriber::EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| "lmage=trace".into()),
            )
            .finish();
        tracing::subscriber::set_global_default(subscriber)
            .expect("setting tracing default failed");
    }

    let event_loop = EventLoop::<AppEvent>::with_user_event().build().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    #[cfg(target_arch = "wasm32")]
    {
        let app = App::new(&event_loop);
        event_loop.spawn_app(app);
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let mut app = App::new(&event_loop);
        event_loop.run_app(&mut app).unwrap();
    }
}
