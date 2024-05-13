use crate::util;
use geoutils::Location;
use regex::Regex;
use tracing::{error, warn};
use std::{collections::HashMap, fs, io, path::PathBuf};

use super::{Airport, ConfigFile};

pub fn get_airport_data() -> HashMap<String, Airport> {
    let mut return_map = HashMap::new();
    let bytes = include_bytes!("airport_data.csv");
    let airports_text = String::from_utf8_lossy(bytes);
    let mut reader = csv::Reader::from_reader(airports_text.as_bytes());
    reader.records().for_each(|record| {
        let record = record.unwrap();
        let airport_icao = record[0].to_string().to_uppercase();
        let airport_name = record[1].to_string();
        let airport_location = Location::new(
            record[2].to_string().parse::<f64>().unwrap(),
            record[3].to_string().parse::<f64>().unwrap(),
        );
        let airport = Airport {
            icao: airport_icao.clone(),
            name: airport_name,
            location: airport_location,
        };
        return_map.insert(airport_icao, airport);
    });
    return_map
}

pub fn get_installed_gsx_profiles(airport_data: &HashMap<String, Airport>) -> Vec<ConfigFile> {
    let mut owned_config_files: Vec<ConfigFile> = Vec::new();
    let gsx_path = util::get_gsx_profile_path();

    let entries = fs::read_dir(gsx_path)
        .unwrap()
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()
        .unwrap();

    let re = Regex::new(r"^(?<icao_code>\w{4})-.*\.ini").unwrap();

    for path_entry in entries {
        let filename = path_entry.file_name().unwrap().to_str().unwrap();

        let Some(caps) = re.captures(filename) else {
            warn!("Invalid filename of config-file {}", filename);
            continue;
        };
        let icao_code = &caps["icao_code"].to_uppercase();

        let Some(airport) = airport_data.get(&String::from(icao_code)) else {
            warn!("Airport with icao {} not found!", icao_code);
            continue;
        };
        let config = ConfigFile::new(
            String::from(path_entry.file_name().unwrap().to_str().unwrap()),
            path_entry,
            airport.to_owned().clone(),
        );
        owned_config_files.push(config);
    }

    owned_config_files
}

pub fn import_config_file_dialog() {
    if let Some(path) = rfd::FileDialog::new()
    .add_filter("GSX-Profile", &["ini"])
    .set_directory("/")
    .pick_file() {
        let to_path = util::get_gsx_profile_path().join( path.file_name().unwrap());
        match fs::copy(path, to_path) {
            Ok(_) => {},
            Err(error) => {error!("{:?}", error)}
        }
    }
}

pub fn delete_config_file(airport_path_to_delete: &PathBuf) {
    match fs::remove_file(airport_path_to_delete) {
        Ok(_) => {},
        Err(error) => {error!("{}", error)}
    }
}