use std::path::PathBuf;

use chrono::{DateTime, Utc};
use geoutils::Location;
use uuid::Uuid;
use walkers::{extras::Place, Position};

pub mod filehandler;

#[derive(Debug, Clone)]
pub struct Airport {
    pub icao: String,
    pub name: String,
    pub location: Location,
}

#[derive(Debug, Clone)]
pub struct GsxSection {
    pub id: Uuid,
    pub name: String,
    pub position: Position,
    pub pushback_label_left: Option<String>,
    pub pushback_position_left: Option<Position>,
    pub pushback_label_right: Option<String>,
    pub pushback_position_right: Option<Position>,
}

#[derive(Debug, Clone)]
pub struct GsxProfile {
    pub creator: String,
    pub deice_labels: Vec<String>,
    pub deice_areas: Vec<Position>,
    pub sections: Vec<GsxSection>,
}

#[derive(Debug, Clone)]
pub struct ProfileFile {
    pub id: Uuid,
    pub file_name: String,
    pub file_location: PathBuf,
    pub airport: Airport,
    pub py_file_location: Option<PathBuf>,
    pub profile_data: Option<GsxProfile>,
    pub has_duplicate_error: bool,
    pub last_modified: DateTime<Utc>,
}

impl GsxProfile {
    pub fn new() -> Self {
        Self {
            creator: String::from(""),
            deice_labels: vec![],
            deice_areas: vec![],
            sections: vec![],
        }
    }
}

impl ProfileFile {
    pub fn new(
        file_name: String,
        file_location: PathBuf,
        airport: Airport,
        py_file_location: Option<PathBuf>,
        last_modified: DateTime<Utc>
    ) -> ProfileFile {
        ProfileFile {
            id: Uuid::new_v4(),
            file_name,
            file_location,
            airport,
            py_file_location,
            profile_data: None,
            has_duplicate_error: false,
            last_modified,
        }
    }
}

pub struct GsxPlace(pub Place);

impl GsxPlace {
    pub fn to_place(&self) -> Place {
        Place {
            position: self.0.position.clone(),
            label: self.0.label.clone(),
            symbol: self.0.symbol.clone(),
            style: self.0.style.clone(),
        }
    }
}

impl Clone for GsxPlace {
    fn clone(&self) -> Self {
        Self {
            0: Place {
                position: self.0.position.clone(),
                label: self.0.label.clone(),
                symbol: self.0.symbol.clone(),
                style: self.0.style.clone(),
            },
        }
    }
}

pub mod constants {
    use egui::Vec2;

    pub static WINDOW_SIZE: Vec2 = Vec2 {
        x: 1700.0,
        y: 900.0,
    };
}
