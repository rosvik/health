use serde::{Deserialize, Serialize};
use std::{fs::File, io::Read};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub endpoints: Vec<Endpoint>,
    pub interval: u64,
    pub database: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Endpoint {
    pub name: String,
    pub url: String,
}

impl Config {
    pub fn load_from_file(path: &str) -> Config {
        let mut buffer = String::new();
        let mut file = File::open(path).unwrap();
        file.read_to_string(&mut buffer).unwrap();
        toml::from_str(&buffer).unwrap()
    }
}

pub fn endpoint_config(config: &Config, name: &str) -> Option<Endpoint> {
    config.endpoints.iter().find(|e| e.name == name).cloned()
}
