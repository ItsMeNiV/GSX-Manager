use std::{
    collections::HashMap,
    fs::{self, File},
    io::{self, Read, Write},
    path::{Path, PathBuf},
    time::SystemTime,
};

use geoutils::Location;
use json::JsonValue;
use regex::Regex;
use tracing::{debug, error, warn};
use uuid::Uuid;
use walkers::Position;

use gsx_ini_parser;

use crate::util;

use super::{Airport, GsxProfile, GsxSection, ProfileFile};

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

pub fn get_user_data() -> JsonValue {
    let mut file = match File::options()
        .read(true)
        .write(true)
        .create(true)
        .open("gsxman_userdata.json")
    {
        Ok(f) => f,
        Err(_) => panic!("Could not create Userdata File!"),
    };
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    if data.is_empty() {
        data += "{}";
    }
    json::parse(data.as_str()).unwrap()
}

pub fn write_user_data(user_data: &JsonValue) {
    let mut file = match File::options()
        .read(true)
        .write(true)
        .create(true)
        .open("gsxman_userdata.json")
    {
        Ok(f) => f,
        Err(_) => panic!("Could not create Userdata File!"),
    };

    if let Err(_) = file.write(&*user_data.dump().as_bytes()) {
        panic!("Could not write to Userdata File!");
    }
}

pub fn get_installed_gsx_profiles(
    airport_data: &HashMap<String, Airport>,
) -> HashMap<Uuid, ProfileFile> {
    let mut installed_config_files: HashMap<Uuid, ProfileFile> = HashMap::new();
    let gsx_path = util::get_gsx_profile_path();

    if let Ok(gsx_dir) = fs::read_dir(gsx_path) {
        let entries = gsx_dir
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, io::Error>>()
            .unwrap();

        let gsx_regex = Regex::new(r"^\w{4}-.*\.(ini|py)").unwrap();
        let profile_file_regex = Regex::new(r"^(?<icao_code>\w{4})-.*\.ini").unwrap();

        for path_entry in &entries {
            let file_name = String::from(path_entry.file_name().unwrap().to_str().unwrap());

            let mut last_modified = SystemTime::UNIX_EPOCH;
            if let Ok(metadata) = path_entry.metadata() {
                if let Ok(modified) = metadata.modified() {
                    last_modified = modified;
                }
            }

            if !gsx_regex.is_match(&file_name) {
                warn!("File {} is not a GSX Profile file", &file_name);
                continue;
            }

            if let Some(caps) = profile_file_regex.captures(&file_name) {
                debug!("File {} is a .ini profile", &file_name);

                let icao_code = &caps["icao_code"].to_uppercase();

                let python_file = get_associated_python_file(path_entry);

                let Some(airport) = airport_data.get(&String::from(icao_code)) else {
                    warn!("Airport with icao {} not found!", icao_code);
                    continue;
                };

                let mut creator = String::from("");
                let parse_result =
                    gsx_ini_parser::parse_file(path_entry.as_os_str().to_str().unwrap());
                if let Err(error) = parse_result {
                    error!("{}", error);
                } else {
                    let data_map = parse_result.unwrap();

                    if let Some(general_section) = data_map.get("general") {
                        if let Some(creator_string) = general_section.get("creator") {
                            creator = creator_string.to_owned();
                        }
                    }
                }

                let mut config = ProfileFile::new(
                    file_name,
                    path_entry.clone(),
                    airport.to_owned().clone(),
                    python_file,
                    last_modified.into(),
                    creator,
                    String::from(""), //TODO
                );

                for (_, profile_file) in installed_config_files.iter_mut() {
                    if profile_file.airport.icao == config.airport.icao {
                        warn!("Duplicate Profile for Airport {}", config.airport.icao);
                        profile_file.has_duplicate_error = true;
                        config.has_duplicate_error = true;
                    }
                }

                installed_config_files.insert(config.id, config);
            };
        }
    }

    installed_config_files
}

pub fn load_profile_data(file: &mut ProfileFile) {
    let parse_result = gsx_ini_parser::parse_file(file.file_location.as_os_str().to_str().unwrap());
    if let Err(error) = parse_result {
        error!("{}", error);
        return;
    }
    let data_map = parse_result.unwrap();
    let mut profile_data = GsxProfile::new();

    if let Some(_general_section) = data_map.get("general") {
        //TODO: Handle Deice areas
    }

    for (section_name, values) in data_map.iter() {
        if !values.contains_key("pushback_pos") || section_name.eq_ignore_ascii_case("general") {
            // For now we only handle sections that have a pushback_pos
            continue;
        }

        let name = section_name.clone();

        let position = position_string_to_position(&values["pushback_pos"]);
        if position.is_none() {
            warn!("Section {} has no Position", &name);
            continue;
        }
        let position = position.unwrap();

        let mut pushback_label_left = None;
        let mut pushback_position_left = None;
        let mut pushback_label_right = None;
        let mut pushback_position_right = None;
        if let Some(pushback_labels) = values.get("pushbacklabels") {
            let pushback_labels: Vec<&str> = pushback_labels.split('|').collect();
            if !pushback_labels.is_empty() {
                if let Some(string_value) = values.get("pushbackleftpos") {
                    pushback_label_left = Some(pushback_labels[0].to_string());
                    pushback_position_left = position_string_to_position(string_value);
                }

                if pushback_labels.len() > 1 {
                    if let Some(string_value) = values.get("pushbackrightpos") {
                        pushback_label_right = Some(pushback_labels[1].to_string());
                        pushback_position_right = position_string_to_position(string_value);
                    }
                }
            }
        }

        let section = GsxSection {
            id: Uuid::new_v4(),
            name,
            position,
            pushback_label_left,
            pushback_position_left,
            pushback_label_right,
            pushback_position_right,
        };

        profile_data.sections.push(section);
    }

    file.profile_data = Some(profile_data);
}

#[inline]
fn position_string_to_position(string_value: &str) -> Option<Position> {
    let mut pos = Position::from_lat_lon(0.0, 0.0);
    let coord_strings: Vec<&str> = string_value.split(' ').collect();
    let lat = coord_strings[0].parse::<f64>();
    let lon = coord_strings[1].parse::<f64>();
    if lat.is_ok() && lon.is_ok() {
        pos = Position::from_lat_lon(lat.unwrap(), lon.unwrap());
    }
    Some(pos)
}

pub fn import_config_file_dialog() {
    if let Some(path) = rfd::FileDialog::new()
        .add_filter("GSX-Profile", &["ini"])
        .set_directory("/")
        .pick_file()
    {
        let to_path = util::get_gsx_profile_path().join(path.file_name().unwrap());
        match fs::copy(&path, to_path) {
            Ok(_) => {
                if let Some(python_file) = get_associated_python_file(&path) {
                    let to_path =
                        util::get_gsx_profile_path().join(python_file.file_name().unwrap());
                    match fs::copy(&path, to_path) {
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

pub fn delete_config_file(airport_path_to_delete: &PathBuf) -> bool {
    let filename = airport_path_to_delete
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    let message_dialog = rfd::MessageDialog::new()
        .set_buttons(rfd::MessageButtons::OkCancelCustom(
            "Yes".to_string(),
            "Cancel".to_string(),
        ))
        .set_description(format!(
            "Are you sure you want to delete profile {}",
            filename
        ))
        .set_title("Delete Profile")
        .set_level(rfd::MessageLevel::Warning)
        .show();
    match message_dialog {
        rfd::MessageDialogResult::Ok => match fs::remove_file(airport_path_to_delete) {
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
                            return true;
                        }
                        Err(error) => {
                            error!("{}", error);
                            return false;
                        }
                    }
                }
                true
            }
            Err(error) => {
                error!("{}", error);
                false
            }
        },
        _ => false,
    }
}

fn get_associated_python_file(gsx_profile_path: &Path) -> Option<PathBuf> {
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
