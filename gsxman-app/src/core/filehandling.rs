use std::{fs, io};
use std::path::Path;

pub fn get_files(path: &Path) -> io::Result<()> {
    for elem in fs::read_dir(path)? {
        let elem = elem?;
        println!("{:?}", elem.path());
    }

    Ok(())
}