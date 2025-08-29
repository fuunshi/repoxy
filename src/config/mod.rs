use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct RouteConfig {
    pub path: String,
    pub backends: Vec<String>,

    pub strategy: Option<String>,
    pub timeout: Option<u64>,
    pub retries: Option<u32>,
    pub method : Option<Vec<String>>,
    pub add_headers: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub routes: Vec<RouteConfig>,
}

pub fn load_config(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let config_file = fs::read_to_string(path)?;
    let config: Config = toml::de::from_str(&config_file)?;
    Ok(config)
}