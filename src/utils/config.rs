use serde::Deserialize;
use anyhow::Result;
use std::{fs::read_to_string, path::Path};
use serde_yaml::from_str;

#[derive(Deserialize, Clone)]
pub struct MasterConfig {
    pub site: SiteConfig,
    pub server: ServerConfig,
    pub database: DatabaseConfig
}

#[derive(Deserialize, Clone)]
pub struct SiteConfig {
    pub title: String,
    pub description: String,
    pub guidelines: Vec<String>,
    pub allow_request: bool,
    pub secret_key: String,
    pub admin_password: String
}

#[derive(Deserialize, Clone)]
pub struct ServerConfig {
    pub bind: String,
    pub port: u16,
    pub theme: String
}

#[derive(Deserialize, Clone)]
pub struct DatabaseConfig {
    pub dbname: String
}


pub fn read_config(path: &Path) -> Result<MasterConfig> {
    Ok(from_str(&read_to_string(path)?)?)
}