use super::{Airport, ProfileFile};
use crate::util;
use geoutils::Location;
use regex::Regex;
use std::{collections::HashMap, fs, io, path::PathBuf};
use tracing::{debug, error, warn};

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

pub fn get_installed_gsx_profiles(airport_data: &HashMap<String, Airport>) -> Vec<ProfileFile> {
    let mut owned_config_files: Vec<ProfileFile> = Vec::new();
    let gsx_path = util::get_gsx_profile_path();

    let entries = fs::read_dir(gsx_path)
        .unwrap()
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()
        .unwrap();

    let gsx_regex = Regex::new(r"^\w{4}-.*\.(ini|py)").unwrap();
    let profile_file_regex = Regex::new(r"^(?<icao_code>\w{4})-.*\.ini").unwrap();

    for path_entry in &entries {
        let file_name = String::from(path_entry.file_name().unwrap().to_str().unwrap());

        if !gsx_regex.is_match(&file_name) {
            warn!("File {} is not a GSX Profile file", &file_name);
            continue;
        }

        if let Some(caps) = profile_file_regex.captures(&file_name) {
            debug!("File {} is a .ini profile", &file_name);

            let icao_code = &caps["icao_code"].to_uppercase();

            let python_file = get_associated_python_file(&path_entry);

            let Some(airport) = airport_data.get(&String::from(icao_code)) else {
                warn!("Airport with icao {} not found!", icao_code);
                continue;
            };
            let config = ProfileFile::new(
                file_name,
                path_entry.clone(),
                airport.to_owned().clone(),
                python_file,
            );
            owned_config_files.push(config);
        };
    }

    owned_config_files
}

pub fn import_config_file_dialog() {
    if let Some(path) = rfd::FileDialog::new()
        .add_filter("GSX-Profile", &["ini"])
        .set_directory("/")
        .pick_file()
    {
        let to_path = util::get_gsx_profile_path().join(path.file_name().unwrap());
        match fs::copy(&path, &to_path) {
            Ok(_) => {
                if let Some(python_file) = get_associated_python_file(&path) {
                    let to_path =
                        util::get_gsx_profile_path().join(python_file.file_name().unwrap());
                    match fs::copy(&path, &to_path) {
                        Ok(_) => {}
                        Err(error) => {
                            error!("{:?}", error);
                        }
                    }
                }
            }
            Err(error) => {
                error!("{:?}", error)
            }
        }
    }
}

pub fn delete_config_file(airport_path_to_delete: &PathBuf) {
    match fs::remove_file(airport_path_to_delete) {
        Ok(_) => {
            debug!(
                "Deleted profile {}",
                airport_path_to_delete
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
            );
            if let Some(python_file) = get_associated_python_file(airport_path_to_delete) {
                match fs::remove_file(&python_file) {
                    Ok(_) => {
                        debug!(
                            "Deleted associated python file {}",
                            python_file.file_name().unwrap().to_str().unwrap()
                        );
                    }
                    Err(error) => {
                        error!("{}", error)
                    }
                }
            }
        }
        Err(error) => {
            error!("{}", error)
        }
    }
}

fn get_associated_python_file(gsx_profile_path: &PathBuf) -> Option<PathBuf> {
    let gsx_profile_file_name = gsx_profile_path.file_name().unwrap().to_str().unwrap();
    let gsx_profile_file_dir = gsx_profile_path.parent().unwrap();
    let possible_python_file_name = format!(
        "{}.py",
        &gsx_profile_file_name[..gsx_profile_file_name.len() - 4]
    );
    let python_file_regex: Regex =
        Regex::new(format!(r"^{}$", possible_python_file_name).as_str()).unwrap();
    let mut python_file: Option<PathBuf> = None;

    let entries = fs::read_dir(gsx_profile_file_dir)
        .unwrap()
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()
        .unwrap();

    for path_entry in entries {
        let possible_python_file = path_entry.file_name().unwrap().to_str().unwrap();
        if python_file_regex.is_match(possible_python_file) {
            python_file = Some(path_entry);
            break;
        }
    }
    match &python_file {
        Some(found) => {
            debug!(
                "Found Python file {} for Profile {}",
                found.file_name().unwrap().to_str().unwrap(),
                gsx_profile_file_name
            );
        }
        None => {
            debug!("No Python file for Profile {} found", gsx_profile_file_name);
        }
    }

    python_file
}
