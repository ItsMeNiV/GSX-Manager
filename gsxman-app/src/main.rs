mod util;
mod core;
mod app;

fn main() -> Result<(), eframe::Error> {
    let config = app::AppConfig {
        msfs_windowsstore: false,
        paths: Default::default()
   };

   app::start_app(&config)
}
