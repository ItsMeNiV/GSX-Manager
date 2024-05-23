use std::{collections::HashMap, fs::File, io::{self, BufRead}, num::NonZeroU8};

use regex::Regex;


type GSXIniFile = HashMap<String, HashMap<String, String>>;

pub fn parse_file(path: &str) -> io::Result<GSXIniFile> {
    let mut ini_file = GSXIniFile::new();

    let lines_iter = get_file_iter(path)?;

    let mut current_section: Option<String> = None;
    for line in lines_iter {
        let line_string = line.as_str().trim();
        if let Some(first_char) = line_string.chars().next() {
            match first_char {
                '[' => {
                    let section_name = handle_section_line(&line, &mut ini_file);
                    current_section = section_name;
                },
                token if token.is_ascii_alphabetic() => handle_key_value_line(&line, &current_section, &mut ini_file),
                _ => {}
            }
        }
    }

    Ok(ini_file)
}

fn get_file_iter(path: &str) -> io::Result<impl Iterator<Item = String>> {
    let file = File::open(path)?;
    Ok(io::BufReader::new(file).lines().flatten())
}

fn handle_section_line(section_string: &String, ini_file: &mut GSXIniFile) -> Option<String> {
    let mut handled_section_name = None;
    let section_name_regex = Regex::new(r"^\[(?<section_name>.+)\]$").unwrap();

    if let Some(caps) = section_name_regex.captures(&section_string) {
        let section_name = String::from(caps["section_name"].trim());
        if !section_name.is_empty() {
            ini_file.insert(section_name.clone(), HashMap::new());
            handled_section_name = Some(section_name);
        }
    }
    handled_section_name
}

fn handle_key_value_line(key_value_string: &String, current_section_string: &Option<String>, ini_file: &mut GSXIniFile) {
    if current_section_string.is_none() || key_value_string.find("=").is_none() { return; }
    let current_section = current_section_string.clone().unwrap();

    let key_values: Vec<&str> = key_value_string.split("=").collect();
    if key_values.len() != 2 { return; }

    let key = key_values[0].trim();
    let value = key_values[1].trim();

    if let Some(section) = ini_file.get_mut(&current_section) {
        section.insert(String::from(key), String::from(value));
    }
}

#[cfg(test)]
mod tests {
    use std::{path::PathBuf};

    use super::*;

    fn get_test_file_path() -> String {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("res/test/lszh-fsdt.ini");
        String::from(d.as_os_str().to_str().unwrap())
    }

    #[test]
    fn get_file_iter_file_exists() {
        let result = get_file_iter(&get_test_file_path());
        assert!(result.is_ok());
    }

    #[test]
    fn get_file_iter_file_doesnt_exist() {
        let result = get_file_iter("notexisting.ini");
        assert!(result.is_err());
    }

    #[test]
    fn handle_section_line_testcases() {
        let section1_string = String::from("[section1]");
        let section2_string = String::from("[    section2]");
        let section3_string = String::from("[      section3       ]");
        let section4_string = String::from("[section4       ]");
        let section5_string = String::from("[  section 5      ]");
        let invalid_section_string = String::from("[      ]");

        let mut ini_file = GSXIniFile::new();

        handle_section_line(&section1_string, &mut ini_file);
        handle_section_line(&section2_string, &mut ini_file);
        handle_section_line(&section3_string, &mut ini_file);
        handle_section_line(&section4_string, &mut ini_file);
        handle_section_line(&section5_string, &mut ini_file);
        handle_section_line(&invalid_section_string, &mut ini_file);

        assert_eq!(ini_file.keys().len(), 5);
        assert!(ini_file.contains_key("section1"));
        assert!(ini_file.contains_key("section2"));
        assert!(ini_file.contains_key("section3"));
        assert!(ini_file.contains_key("section4"));
        assert!(ini_file.contains_key("section 5"));
    }
    
    #[test]
    fn handle_key_value_line_testcases() {
        let current_section_string = String::from("[Testsection]");
        let current_section = String::from("Testsection");
        let key_value1_string = String::from("key1 = value");
        let key_value2_string = String::from("key2= value");
        let key_value3_string = String::from("key3 =value");
        let key_value4_string = String::from("key4=value");
        let key_value4_override_string = String::from("key4 = valuenew");
        let key_value5_string = String::from("key5 = value");
        let key_value6_string = String::from("key6 = [(value1),(value2)]");

        let mut ini_file = GSXIniFile::new();
        handle_section_line(&current_section_string, &mut ini_file);

        let current_section = Some(current_section);

        handle_key_value_line(&key_value1_string, &current_section, &mut ini_file);
        handle_key_value_line(&key_value2_string, &current_section, &mut ini_file);
        handle_key_value_line(&key_value3_string, &current_section, &mut ini_file);
        handle_key_value_line(&key_value4_string, &current_section, &mut ini_file);
        handle_key_value_line(&key_value4_override_string, &current_section, &mut ini_file); // Old value should be overridden
        handle_key_value_line(&key_value5_string, &None, &mut ini_file);  // No active Section, cannot assign
        handle_key_value_line(&key_value6_string, &current_section, &mut ini_file); // Handle arrays as value

        let current_section = current_section.unwrap();
        let section = ini_file.get(&current_section).unwrap();

        // Assert different formatting of key value lines
        assert!(ini_file.contains_key(&current_section));
        assert_eq!(section.get("key1").unwrap().to_owned(), String::from("value"));
        assert_eq!(section.get("key2").unwrap().to_owned(), String::from("value"));
        assert_eq!(section.get("key3").unwrap().to_owned(), String::from("value"));
        assert_eq!(section.get("key4").unwrap().to_owned(), String::from("valuenew"));
        assert_eq!(section.contains_key("key5"), false);
        assert_eq!(section.get("key6").unwrap().to_owned(), String::from("[(value1),(value2)]"));

    }
    
}
