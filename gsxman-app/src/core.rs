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
}

impl ProfileFile {
    pub fn new(filename: String, filelocation: PathBuf, airportref: Airport) -> ProfileFile {
        ProfileFile {
            file_name: filename,
            file_location: filelocation,
            airport: airportref,
        }
    }
}

pub mod constants {
    use egui::Vec2;
    pub static WINDOW_SIZE: Vec2 = Vec2 {
        x: 1200.0,
        y: 500.0,
    };
}
