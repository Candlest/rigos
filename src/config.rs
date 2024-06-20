use serde::Deserialize;
use std::fs;
use toml;

#[derive(Deserialize)]
pub struct Config {
    pub site_title: String,
    pub site_link: String,
    pub site_description: String,
    pub theme: String,
    pub pages: Vec<String>,
}

pub fn read_config(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let config_str = fs::read_to_string(path)?;
    let config = toml::from_str::<Config>(&config_str)?;
    Ok(config)
}
