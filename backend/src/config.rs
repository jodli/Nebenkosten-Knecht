use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use toml;

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct UnitConfig {
    pub id: String,
    pub name: String,
    pub wohnflaeche_real: f64,
    pub wohnflaeche_heizung: f64,
    pub personen: u32,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct CounterConfig {
    pub id: String,
    pub name: String,
    pub unit: String,
    pub type_: String,
    // Optional fields can be handled with Option<>
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct CostTypeConfig {
    pub id: String,
    pub name: String,
    pub umlage_logik_ref: String,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct HeizungParams {
    pub oel_verbrauch_pro_stunde_stufe1: f64,
    pub oel_verbrauch_pro_stunde_stufe2: f64,
}

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub units: Vec<UnitConfig>,
    pub counters: Vec<CounterConfig>,
    pub cost_types: Vec<CostTypeConfig>,
    pub heizung_config: HeizungParams,
}

pub fn load_app_config(file_path: &str) -> Result<AppConfig> {
    let contents = fs::read_to_string(file_path)
        .with_context(|| format!("Failed to read config file: {}", file_path))?;

    let config: AppConfig =
        toml::from_str(&contents).with_context(|| "Failed to parse config file")?;

    Ok(config)
}
