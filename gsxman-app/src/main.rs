use app::AppConfig;
use tracing::{error, Level};
use tracing_subscriber::FmtSubscriber;

mod app;
mod core;
mod util;

#[cfg(debug_assertions)]
fn set_logger() {
    // DEV BUILD
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed setting global Logging Subscriber");
}
#[cfg(not(debug_assertions))]
fn set_logger() {
    // RELEASE BUILD
    let file_appender = tracing_appender::rolling::never("./", "gsxman.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    let subscriber = FmtSubscriber::builder()
        .with_writer(non_blocking)
        .with_max_level(Level::ERROR)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed setting global Logging Subscriber");
}

fn main() -> Result<(), eframe::Error> {
    set_logger();

    let app_config: AppConfig = Default::default();
    if let Err(error) = app::start_app(app_config) {
        error!("{}", error.to_string());
    }

    Ok(())
}
