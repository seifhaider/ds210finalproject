// src/scraper.rs
//! Fetches and parses player data from FBref for the 2023–24 season.

use reqwest::blocking::Client;
use scraper::{Html, Selector};
use serde::Serialize;
use crate::data::PlayerProfile;
use std::error::Error;

/// Retrieves and parses HTML, returning a vector of PlayerProfile structs.
pub fn scrape_player_data() -> Result<Vec<PlayerProfile>, Box<dyn Error>> {
    let url = "https://fbref.com/en/comps/Big5/2023-2024/players/";
    let client = Client::builder().build()?;
    let resp = client.get(url).send()?.text()?;
    let document = Html::parse_document(&resp);

    let row_selector = Selector::parse("table.stats_table tbody tr").unwrap();
    let cell_selector = Selector::parse("td").unwrap();

    let mut profiles = Vec::new();
    for row in document.select(&row_selector) {
        let mut cells = row.select(&cell_selector);
        // Example: skip rank column
        let _rank = cells.next();
        // Next: player name
        let name_cell = cells.next().unwrap();
        let name = name_cell.text().collect::<Vec<_>>().join("");
        // Skipping to metrics columns: dribbles, progressive carries, final third touches
        let dribbles: f64 = cells.nth(2).unwrap().text().collect::<String>().parse().unwrap_or(0.0);
        let prog_carries: f64 = cells.nth(1).unwrap().text().collect::<String>().parse().unwrap_or(0.0);
        let final_third: f64 = cells.nth(1).unwrap().text().collect::<String>().parse().unwrap_or(0.0);

        profiles.push(PlayerProfile::new(&name, dribbles, prog_carries, final_third));
        if profiles.len() >= 1000 { break; } // limit
    }
    Ok(profiles)
}
