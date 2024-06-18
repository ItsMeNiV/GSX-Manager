use std::path::PathBuf;

mod directimporter;
mod zipimporter;

pub fn import_from_path(path: PathBuf) {
    if path.extension().unwrap() == "zip" {
        zipimporter::import(path);
    } else {
        directimporter::import(path);
    }
}