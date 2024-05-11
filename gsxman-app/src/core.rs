pub mod filehandler;

use std::path::PathBuf;
use geoutils::Location;

#[derive(Debug)]
pub struct Airport {
    icao: String,
    name: String,
    location: Location
}

#[derive(Debug)]
pub struct ConfigFile<'a> {
    file_name: String,
    file_location: PathBuf,
    airport: &'a Airport
}

impl ConfigFile<'_> {
    pub fn new<'a>(filename: String, filelocation: PathBuf, airportref: &'a Airport) -> ConfigFile<'a> {
        ConfigFile {file_name: filename, file_location: filelocation, airport: airportref}
    }
}