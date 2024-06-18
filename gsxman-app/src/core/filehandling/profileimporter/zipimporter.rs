use std::{
    fs::{self, File},
    io::{self, BufReader},
    path::PathBuf,
};

use tracing::{debug, error};
use zip::{read::ZipFile, ZipArchive};

use crate::{core::filehandling::get_associated_python_file, util};

pub fn import(path: PathBuf) {
    let zip_file_name = path
        .components()
        .next_back()
        .unwrap()
        .as_os_str()
        .to_str()
        .unwrap();
    let zip_file_name = &zip_file_name[..zip_file_name.len() - 4];
    let extraction_path = path.parent().unwrap().to_path_buf().join(zip_file_name);
    match fs::File::open(path) {
        Ok(file) => {
            let reader: BufReader<File> = BufReader::new(file);
            if let Some((mut archive, ini_files, py_files)) = read_files_from_zip_archive(reader) {
                let ini_path_to_import =
                    prepare_ini_import(&mut archive, &ini_files, &extraction_path);
                prepare_py_import(
                    &mut archive,
                    &py_files,
                    &extraction_path,
                    &ini_path_to_import,
                );
                import_ini(ini_path_to_import);
            } else {
                error!("Error opening Zip Archive");
            }
        }
        Err(err) => error!("{:?}", err),
    }
}

fn read_files_from_zip_archive(
    reader: BufReader<File>,
) -> Option<(ZipArchive<BufReader<File>>, Vec<String>, Vec<String>)> {
    if let Ok(mut archive) = ZipArchive::new(reader) {
        let mut ini_files: Vec<String> = vec![];
        let mut py_files: Vec<String> = vec![];
        for i in 0..archive.len() {
            if let Ok(file) = archive.by_index(i) {
                let file_name = file.name();
                if file_name.ends_with("ini") {
                    ini_files.push(file.name().to_owned());
                } else if file_name.ends_with("py") {
                    py_files.push(file.name().to_owned());
                }
            } else {
                error!("Error reading from Zip Archive");
                return None;
            }
        }
        return Some((archive, ini_files, py_files));
    } else {
        return None;
    }
}

fn prepare_ini_import(
    archive: &mut ZipArchive<BufReader<File>>,
    ini_files: &Vec<String>,
    extraction_path: &PathBuf,
) -> PathBuf {
    let mut ini_path_to_import = PathBuf::new();
    match ini_files.len() {
        0 => error!("No ini-Files found in Zip Archive"),
        1 => {
            let file_name = ini_files[0].as_str();
            ini_path_to_import =
                extract_file_from_zip(archive.by_name(file_name).unwrap(), &extraction_path);
        }
        _ => {
            for file in ini_files {
                extract_file_from_zip(archive.by_name(file).unwrap(), &extraction_path);
            }

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

    ini_path_to_import
}

fn prepare_py_import(
    archive: &mut ZipArchive<BufReader<File>>,
    py_files: &Vec<String>,
    extraction_path: &PathBuf,
    ini_import_path: &PathBuf,
) {
    let mut py_to_import_path = PathBuf::new();
    match py_files.len() {
        0 => (),
        1 => {
            py_to_import_path = extract_file_from_zip(
                archive.by_name(&py_files[0].as_str()).unwrap(),
                &extraction_path,
            );
        }
        _ => {
            for file in py_files {
                extract_file_from_zip(archive.by_name(file).unwrap(), &extraction_path);
            }

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
        let new_py_file_name = ini_import_path
            .as_os_str()
            .to_str()
            .unwrap()
            .replace("ini", "py");
        let _ = fs::rename(py_to_import_path, new_py_file_name);
    }
}

fn extract_file_from_zip(mut file: ZipFile, target_path: &PathBuf) -> PathBuf {
    if let Some(file_name) = file.enclosed_name() {
        let outpath = target_path.join(file_name.components().next_back().unwrap());
        debug!("{:?}", outpath);

        if let Some(p) = outpath.parent() {
            if !p.exists() {
                fs::create_dir_all(p).unwrap();
            }
        }

        let mut outfile = fs::File::create(&outpath).unwrap();
        io::copy(&mut file, &mut outfile).unwrap();

        return outpath;
    };

    PathBuf::new()
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
