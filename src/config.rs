use serde::Deserialize;
use std::error::Error;
use std::fs;

#[derive(Deserialize)]
pub(crate) struct Config {
    pub task: Task,
    pub auth: Auth,
}

#[derive(Deserialize)]
pub(crate) struct Auth {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub(crate) struct Task {
    pub name: String,
    pub timeout: u64,
    pub command: Command,
}

#[derive(Deserialize)]
pub(crate) struct Command {
    pub args: String,
}

pub(crate) fn load_config(config_path: String) -> Result<Config, Box<dyn Error>> {
    let config_string = fs::read_to_string(config_path)?;
    let config: Config = toml::from_str(config_string.as_str())?;
    Ok(config)
}
