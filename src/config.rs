use std::fs;
use serde::{Deserialize};

fn default_true() -> bool {
    true
}

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default = "default_true")]
    pub dry_run: bool,
    pub log_file: String,
    pub rules: Vec<Rule>,
}

#[derive(Debug, Deserialize)]
pub struct Rule {
    pub name: String,
    #[serde(default = "default_true")]
    pub enabled: bool,
    pub locations: Vec<String>,
    #[serde(default)]
    pub recursive: bool,
    pub max_depth: Option<usize>,
    pub filters: Filters,
    pub action: Action,
}

#[derive(Debug, Deserialize)]
pub struct Filters {
    pub extensions: Option<Vec<String>>,
    pub not_extensions: Option<Vec<String>>,
    pub name: Option<NameFilter>,
    pub regex: Option<String>,
    pub empty: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct NameFilter {
    pub starts_with: Option<Vec<String>>,
    pub ends_with: Option<Vec<String>>,
    pub contains: Option<Vec<String>>,
    #[serde(default)]
    pub case_sensitive: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Action {
    Move(String),
    Delete,
    Copy(String),
    Compress(bool),
}

impl Config {
    pub fn load(path: &str) -> Result<Self, String> {
        let config_str = fs::read_to_string(path).map_err(|e| format!("Failed to load config file: {}", e))?;

        let config: Config = serde_yaml::from_str(&config_str).map_err(|e| format!("Invalid YAML format: {}", e))?;

        Ok(config)
    }
}
