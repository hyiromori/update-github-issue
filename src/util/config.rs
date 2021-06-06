use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Error, Read};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub workspace_id: String,
    pub workspace_name: String,
}

pub fn get_config_file_path() -> String {
    let home_dir = dirs::home_dir().unwrap();
    String::from(home_dir.join(".zenhub_issue_config.json").to_str().unwrap())
}

pub fn read_config() -> Option<Config> {
    let file = File::open(get_config_file_path());
    match file {
        Ok(val) => {
            let mut buf_reader = BufReader::new(val);
            let mut contents = String::new();
            buf_reader.read_to_string(&mut contents);
            let config = serde_json::from_str(contents.as_str());
            match config {
                Ok(val) => Some(val),
                Err(_) => None,
            }
        }
        Err(_) => None,
    }
}

pub fn write_config(config: &Config) -> Result<(), std::io::Error> {
    let mut file = File::create(get_config_file_path()).unwrap();
    let data = serde_json::to_string(config).unwrap();
    file.write_all(data.as_str().as_bytes())?;
    Ok(())
}
