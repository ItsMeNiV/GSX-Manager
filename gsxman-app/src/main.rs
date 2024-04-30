mod util;
mod core;
mod app;

fn main() {
    let f = util::get_gsx_profile_path();
    println!("{}", f);
}
