// src/data.rs
//! Defines the PlayerProfile type and CSV (de)serialization functionality.

use serde::{Deserialize, Serialize};
use csv::{Writer, Reader};
use std::error::Error;

/// Represents a player's per-90 metrics.
#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerProfile {
    /// Unique identifier: name
    pub name: String,
    /// Dribbles completed per 90
    pub dribbles: f64,
    /// Progressive carries per 90
    pub prog_carries: f64,
    /// Touches in final third per 90
    pub final_third: f64,
}

impl PlayerProfile {
    /// Constructs a new PlayerProfile
    pub fn new(name: &str, dribbles: f64, prog_carries: f64, final_third: f64) -> Self {
        Self { name: name.to_string(), dribbles, prog_carries, final_third }
    }
}

/// Saves profiles to CSV file at `path`.
pub fn save_to_csv(profiles: &[PlayerProfile], path: &str) -> Result<(), Box<dyn Error>> {
    let mut wtr = Writer::from_path(path)?;
    for p in profiles {
        wtr.serialize(p)?;
    }
    wtr.flush()?;
    Ok(())
}

/// Loads profiles from CSV file at `path`.
pub fn load_from_csv(path: &str) -> Result<Vec<PlayerProfile>, Box<dyn Error>> {
    let mut rdr = Reader::from_path(path)?;
    let mut profiles = Vec::new();
    for result in rdr.deserialize() {
        let record: PlayerProfile = result?;
        profiles.push(record);
    }
    Ok(profiles)
}