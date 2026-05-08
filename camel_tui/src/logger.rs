use std::panic;
use tracing_appender::rolling;
use tracing_subscriber::{EnvFilter, fmt};

pub fn init() -> tracing_appender::non_blocking::WorkerGuard {
    let file_appender = rolling::never(".", "debug.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(tracing::Level::DEBUG.into()))
        .with_writer(non_blocking)
        .init();

    panic::set_hook(Box::new(|info| {
        let _ = crossterm::terminal::disable_raw_mode();
        let _ = crossterm::execute!(std::io::stdout(), crossterm::terminal::LeaveAlternateScreen);
        tracing::error!("Panic occurred: {}", info);
        eprintln!("Panic occurred: {}", info);
    }));

    guard
}
