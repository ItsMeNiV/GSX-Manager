use std::path::PathBuf;

mod directimporter;
mod zipimporter;
mod rarimporter;

pub fn import_from_path(path: PathBuf) {
    let path_extension = path.extension().unwrap().to_str().unwrap();
    if path_extension == "zip" {
        zipimporter::import(path);
    } else if path_extension == "rar" {
        rarimporter::import(path);
    } else {
        directimporter::import(path);
    }
}