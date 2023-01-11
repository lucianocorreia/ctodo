use std::fs;
use std::io::Error as IoError;

use serde::{Deserialize, Serialize};

// Config struct
#[derive(Debug)]
pub struct Config {
    pub title: String,
    pub db_url: String,
}

impl Config {
    // creats a new instance of Config based on the configuration file
    pub fn new() -> Self {
        match Self::read_config_file() {
            Some(content) => {
                let config_toml: ConfigToml = toml::from_str(&content).unwrap_or_else(|_| {
                    panic!("failed to create ConfigToml object out of config file.");
                });

                // app
                let title: String = match config_toml.app {
                    Some(app) => app.title.unwrap_or_else(|| "".to_owned()),
                    None => "".to_owned(),
                };

                // database
                let db_url: String = match config_toml.database {
                    Some(database) => database.db_url.unwrap_or_else(|| "".to_owned()),
                    None => "".to_owned(),
                };

                Self { title, db_url }
            }
            None => panic!("config file not found or empty."),
        }
    }

    fn read_config_file() -> Option<String> {
        let config_files: [&str; 2] = ["./ctodo.toml", "~/.config/ctodo.toml"];

        let mut content: String = "".to_owned();

        for file_path in config_files {
            let result: Result<String, IoError> = fs::read_to_string(file_path);

            if result.is_ok() {
                content = result.unwrap();
                break;
            }
        }

        (!content.is_empty()).then(|| content)
    }
}
#[derive(Serialize, Deserialize, Debug)]
struct ConfigToml {
    app: Option<ConfigTomlApp>,
    database: Option<ConfigTomlDatabase>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ConfigTomlApp {
    title: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ConfigTomlDatabase {
    db_url: Option<String>,
}
