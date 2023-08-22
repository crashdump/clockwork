use std::fs;
use std::error::Error;
use serde::Deserialize;
use toml::from_str;

#[derive(Deserialize)]
pub(crate) struct Config {
    pub task: Task,
}

#[derive(Deserialize)]
pub(crate) struct Task {
    pub name: String,
    pub timeout: u64,
    pub command: Command, 
}

#[derive(Deserialize)]
pub(crate) struct Command {
    pub name: String,
    pub args: String,
}

pub(crate) fn load_config(config_path: String) -> Result<Config, Box<dyn Error>> {
    let config_string = fs::read_to_string(config_path)?;
    let config: Config = toml::from_str(config_string.as_str())?;
    Ok(config)
}