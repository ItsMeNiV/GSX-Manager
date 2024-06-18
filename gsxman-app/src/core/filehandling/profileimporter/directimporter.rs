use std::{fs, path::PathBuf};
use tracing::error;

use crate::{core::filehandling::get_associated_python_file, util};

pub fn import(path: PathBuf) {
    let to_path = util::get_gsx_profile_path().join(path.file_name().unwrap());
    match fs::copy(&path, to_path) {
        Ok(_) => {
            if let Some(python_file) = get_associated_python_file(&path) {
                let to_path = util::get_gsx_profile_path().join(python_file.file_name().unwrap());
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