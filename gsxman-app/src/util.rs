extern crate directories;
use std::path::Path;

use directories::BaseDirs;

const GSX_PATH: &str = r"virtuali\GSX\MSFS";
pub fn get_gsx_profile_path() -> String {
    let path = get_appdata_path() + r"\" + GSX_PATH;
    String::from(Path::new(path.as_str()).to_str().unwrap())
}

fn get_appdata_path() -> String {
    String::from(BaseDirs::new().unwrap().preference_dir().to_str().unwrap())
}