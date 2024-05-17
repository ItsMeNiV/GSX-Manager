pub mod filehandler;

use geoutils::Location;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Airport {
    pub icao: String,
    pub name: String,
    pub location: Location,
}

#[derive(Debug, Clone)]
pub struct ProfileFile {
    pub file_name: String,
    pub file_location: PathBuf,
    pub airport: Airport,
    pub py_file_location: Option<PathBuf>,
}

impl ProfileFile {
    pub fn new(
        file_name: String,
        file_location: PathBuf,
        airport: Airport,
        py_file_location: Option<PathBuf>,
    ) -> ProfileFile {
        ProfileFile {
            file_name,
            file_location,
            airport,
            py_file_location,
        }
    }
}

pub mod constants {
    use egui::Vec2;
    pub static WINDOW_SIZE: Vec2 = Vec2 {
        x: 1600.0,
        y: 900.0,
    };
}
