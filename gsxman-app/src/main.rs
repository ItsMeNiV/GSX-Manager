use std::{collections::HashMap, io};
use core::Airport;

use app::AppConfig;

mod util;
mod core;
mod app;

fn main() -> Result<(), eframe::Error> {
    let airport_data: HashMap<String, Airport> = core::filehandler::get_airport_data();
    let owned_config_files = core::filehandler::get_installed_gsx_profiles(&airport_data);

    for config in owned_config_files {
        println!("{:?}", config);
    }

    let app_config: AppConfig = Default::default();
    app::start_app(&app_config)?;

    Ok(())
}
