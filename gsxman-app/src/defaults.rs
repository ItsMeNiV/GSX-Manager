extern crate directories;
use directories::BaseDirs;

pub struct Paths{
    pub Appdata: String
}
pub static mut DEFAULT_PATHS: Paths = Paths{
    Appdata: String::new()
};
pub fn init_defaults() {
    unsafe { DEFAULT_PATHS.Appdata = String::from(BaseDirs::new().unwrap().preference_dir().to_str().unwrap()) };
}