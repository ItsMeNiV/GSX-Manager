use std::{collections::HashMap, fs, io};
use core::{Airport, ConfigFile};

mod util;
mod core;
mod app;

fn main() -> io::Result<()> {
    let airport_data: HashMap<String, Airport> = core::filehandler::get_airport_data();
    let owned_config_files = core::filehandler::get_installed_gsx_profiles(&airport_data);

    for config in owned_config_files {
        println!("{:?}", config);
    }

    Ok(())
}
