mod defaults;
mod core;
mod app;

fn main() {
    defaults::init_defaults();
    let f = unsafe { &defaults::DEFAULT_PATHS.Appdata };
    println!("{}", f);
}
