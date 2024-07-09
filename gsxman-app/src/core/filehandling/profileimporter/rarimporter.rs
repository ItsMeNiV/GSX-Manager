use std::{fs, path::PathBuf};

use tracing::error;
use unrar::Archive;

use crate::{core::filehandling::get_associated_python_file, util};

pub fn import(path: PathBuf) {
    let rar_file_name = path
        .components()
        .next_back()
        .unwrap()
        .as_os_str()
        .to_str()
        .unwrap();
    let rar_file_name = &rar_file_name[..rar_file_name.len() - 4];
    let extraction_path = path.parent().unwrap().to_path_buf().join(rar_file_name);

    let ini_path_to_import = prepare_import(path.as_os_str().to_str().unwrap(), &extraction_path);
    import_ini(ini_path_to_import);
}

fn prepare_import(rar_file_name: &str, extraction_path: &PathBuf) -> PathBuf {
    let mut ini_files: Vec<String> = vec![];
    let mut py_files: Vec<String> = vec![];

    match Archive::new(rar_file_name).open_for_processing() {
        Err(_) => {
            error!("Error opening RAR-Archive {}", rar_file_name);
            return PathBuf::new();
        },
        Ok(mut archive) => {
            while let Ok(Some(header)) = archive.read_header() {
                archive = if header.entry().is_file() {
                    let extract_to = extraction_path
                        .clone()
                        .join(header.entry().filename.clone());
                    let arch = header.extract_to(&extract_to).unwrap();
                    let path_extension = extract_to.extension().unwrap().to_str().unwrap();
                    if path_extension == "ini" {
                        ini_files.push(String::from(extract_to.as_os_str().to_str().unwrap()));
                    } else if path_extension == "py" {
                        py_files.push(String::from(extract_to.as_os_str().to_str().unwrap()));
                    }
                    arch
                } else {
                    header.skip().unwrap()
                };
            }
        }
    };

    let mut ini_path_to_import = PathBuf::new();
    match ini_files.len() {
        0 => error!("No ini-Files found in Zip Archive"),
        1 => {
            ini_path_to_import = PathBuf::from(&ini_files[0]);
        }
        _ => {
            if let Some(path) = rfd::FileDialog::new()
                .add_filter("GSX-Profile", &["ini"])
                .set_directory(&extraction_path)
                .set_title("Multiple Ini-Files found. Choose which one to import")
                .pick_file()
            {
                ini_path_to_import = path;
            }
        }
    }

    let mut py_to_import_path = PathBuf::new();
    match py_files.len() {
        0 => (),
        1 => {
            py_to_import_path = PathBuf::from(&py_files[0]);
        }
        _ => {
            if let Some(path) = rfd::FileDialog::new()
                .add_filter("GSX-Profile", &["py"])
                .set_directory(&extraction_path)
                .set_title("Multiple possible Py-Files found. Choose which one to import or Cancel if none")
                .pick_file()
            {
                py_to_import_path = path;
            }
        }
    };

    if !py_to_import_path.to_str().unwrap().is_empty() {
        // Py-File might need to be renamed
        let new_py_file_name = ini_path_to_import
            .as_os_str()
            .to_str()
            .unwrap()
            .replace("ini", "py");
        let _ = fs::rename(py_to_import_path, new_py_file_name);
    }

    ini_path_to_import
}

fn import_ini(path: PathBuf) {
    if path.to_str().unwrap().is_empty() {
        return;
    }

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
