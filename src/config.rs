use std::{env, error::Error, fs};

use serde::{Deserialize, Serialize};


const CONFIG_PATH: &str = "$HOME/.config/mdt/mdt.yaml";

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub notes_directory: String,
}

pub fn read_config() -> Result<Config, Box<dyn Error>> {
    let conf_file_contents: String =  match fs::read_to_string(CONFIG_PATH) {
        Ok(content ) => content,
        // Add debug logs
        Err(_e) => return get_default_config()
    };
    let configs: Config = match serde_yaml::from_str(&conf_file_contents) {
        Ok(c) => c,
        // Add debug logs
        Err(_e) => return get_default_config()
    };
    return Ok(configs);
}

fn get_default_config() -> Result<Config, Box<dyn Error>> {
    let home_dir: String = env::var("HOME")?;
    let notes_dir = String::from(format!("{}/notes/", home_dir));

    return Ok(Config {
        notes_directory: notes_dir
    });
}
