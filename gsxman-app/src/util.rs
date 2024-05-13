extern crate directories;
use std::path::{Path, PathBuf};

use directories::BaseDirs;

const GSX_PATH: &str = r"virtuali\GSX\MSFS";
pub fn get_gsx_profile_path() -> PathBuf {
    let path = get_appdata_path() + r"\" + GSX_PATH;
    Path::new(path.as_str()).to_owned()
}

fn get_appdata_path() -> String {
    String::from(BaseDirs::new().unwrap().preference_dir().to_str().unwrap())
}
