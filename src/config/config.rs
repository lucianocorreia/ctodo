use std::fs;
use std::io::{Error as IoError, ErrorKind};
use std::path::Path;

use serde::{Deserialize, Serialize};

// Config struct
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub title: String,
    pub db_url: String,
}

impl Config {
    pub fn new() -> Result<Self, IoError> {
        let paths = ["./ctodo.toml", "~/.config/ctodo.toml"];

        let path = paths
            .iter()
            .find(|path| Path::new(path).exists())
            .ok_or(IoError::new(ErrorKind::NotFound, "config file not found"))?;
        let content = fs::read_to_string(path)?;
        let config_toml: ConfigToml = toml::from_str(&content)?;

        Ok(Self {
            title: config_toml.app.title,
            db_url: config_toml.database.db_url,
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct ConfigToml {
    app: ConfigTomlApp,
    database: ConfigTomlDatabase,
}

#[derive(Serialize, Deserialize, Debug)]
struct ConfigTomlDatabase {
    db_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ConfigTomlApp {
    title: String,
}

