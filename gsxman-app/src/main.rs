use app::AppConfig;
use tracing::{error, Level};
use tracing_subscriber::FmtSubscriber;

mod app;
mod core;
mod util;

fn main() -> Result<(), eframe::Error> {
    let subscriber = FmtSubscriber::builder().with_max_level(Level::WARN).finish();
    tracing::subscriber::set_global_default(subscriber).expect("Failed setting global Logging Subscriber");

    let app_config: AppConfig = Default::default();
    if let Err(error) = app::start_app(app_config) {
        error!("{}", error.to_string());
    }

    Ok(())
}
