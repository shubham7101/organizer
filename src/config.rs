use serde::Deserialize;
use std::fs;

fn default_true() -> bool {
    true
}

fn default_target() -> Target {
    Target::Files
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
    #[serde(default = "default_target")]
    pub target: Target,

    pub extensions: Option<Vec<String>>,
    pub not_extensions: Option<Vec<String>>,
    pub name: Option<NameFilterConfig>,
    pub regex: Option<String>,
    pub empty: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Target {
    Files,
    Dirs,
}

#[derive(Debug, Deserialize)]
pub struct NameFilterConfig {
    #[serde(default)]
    pub case_sensitive: bool,
    #[serde(default)]
    pub match_any: bool,
    pub starts_with: Option<Vec<String>>,
    pub ends_with: Option<Vec<String>>,
    pub contains: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Action {
    Move(MoveConfig),
    Delete,
}

#[derive(Debug, Deserialize)]
pub struct MoveConfig {
    pub destination: String,
    #[serde(default)]
    pub over_ride: bool,
}

#[derive(Debug, Deserialize)]
pub struct CopyConfig {
    pub destination: String,
}

#[derive(Debug, Deserialize)]
pub struct CompressConfig {
    pub destination: String,
}

impl Config {
    pub fn load(path: &str) -> Result<Self, String> {
        let config_str =
            fs::read_to_string(path).map_err(|e| format!("Failed to load config file: {}", e))?;

        let config: Config =
            serde_yaml::from_str(&config_str).map_err(|e| format!("Invalid YAML format: {}", e))?;

        Ok(config)
    }
}
