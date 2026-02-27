use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub tables: HashMap<String, TableConfig>,
}

#[derive(Debug, Deserialize)]
pub struct TableConfig {
    pub columns: Vec<ColumnConfig>,
}

#[derive(Debug, Deserialize)]
pub struct ColumnConfig {
    pub name: String,
    
    #[serde(flatten)]
    pub strategy: StrategyConfig,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "strategy", rename_all = "snake_case")]
pub enum StrategyConfig {
    Hmac,
    FakerName,
    FakerEmail,
    Nullify,
    Fixed { 
        value: String 
    },
    DpLaplace { 
        epsilon: f64, 
        sensitivity: f64 
    },
}

impl Config {
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }
}