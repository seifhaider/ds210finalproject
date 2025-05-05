// src/main.rs
//! Orchestrates the entire clustering pipeline: scraping, processing, clustering, and output.

mod scraper;
mod data;
mod clustering;
mod utils;

use env_logger;
use log::info;
use data::PlayerProfile;
use scraper::scrape_player_data;
use data::{save_to_csv, load_from_csv};
use clustering::run_kmeans;
use utils::normalize_features;

fn main() {
    env_logger::init();
    // Step 1: Scrape data from FBref
    info!("Scraping player data...");
    let profiles: Vec<PlayerProfile> = match scrape_player_data() {
        Ok(v) => v,
        Err(e) => {
            log::error!("Scrape failed: {}. Loading from local CSV.", e);
            load_from_csv("data/players.csv").expect("Failed to load CSV")
        }
    };

    // Step 2: Save raw data
    save_to_csv(&profiles, "data/players_raw.csv").expect("Failed to save raw data");

    // Step 3: Normalize & prepare matrix
    let (matrix, ids) = normalize_features(&profiles);

    // Step 4: Run K-Means clustering
    let n_clusters = 5;
    info!("Running K-Means with k = {}...", n_clusters);
    let assignments = run_kmeans(&matrix, n_clusters).expect("Clustering failed");

    // Step 5: Output results
    for (i, &cluster) in assignments.iter().enumerate() {
        println!("Player {} -> Cluster {}", ids[i], cluster);
    }
}
