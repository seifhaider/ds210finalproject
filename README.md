# Football Player Profile Clustering by Nationality

A. Project Overview
Goal: Investigate whether European top-five-league footballers’ per-90 performance metrics cluster by nationality.
Central Question: Do statistically measurable trends in technical profiles (dribbling, passing, attacking, defending) align with player nationality?
Dataset:
- Source: FBref (2023–24 season, top five European leagues)
- Size: ~1,200 players × 12 per-90 metrics
- Access: scraped via Rust reqwest + scraper modules; raw CSV in data/players.csv

B. Data Processing
1. Loading:
   - src/data.rs uses csv + serde to deserialize players.csv into PlayerRecord structs.
2. Cleaning & Transformation:
   - Filtered players with ≥900 minutes played.
   - Normalized each metric to zero mean, unit variance.
   - Serialized cleaned data to data/normalized.csv for reproducibility.

C. Code Structure
Modules & Purpose:
- src/main.rs: Entry point orchestrates scraping, loading, clustering, and summary output.
- src/scraper.rs: Fetches HTML from FBref and writes raw CSV.
- src/data.rs: Defines data types (PlayerRecord), CSV I/O, filtering, normalization.
- src/clustering.rs: Implements K-means on ndarray; returns centroids + labels.
- src/utils.rs: Common helpers: file paths, error handling abstractions.

Key Functions & Types:
- scraper::fetch_and_write(url: &str): Inputs: season URL; Outputs: raw players.csv; Logic: HTTP GET → HTML parse → metric extraction → CSV write.
- data::load_normalized(path: &str) -> Array2<f64>: Inputs: CSV file path; Outputs: 2D numeric dataset; Logic: CSV → Vec<PlayerRecord> → metric matrix → normalization.
- clustering::run_kmeans(data: &Array2<f64>, k: usize, max_iter: usize) -> (Array2<f64>, Vec<usize>): Inputs: data, k, iterations; Outputs: centroids, labels; Logic: random init → assign → update → repeat.

D. Tests
All tests in #[cfg(test)] blocks under each module.
Run `cargo test` for:
- simple_two_clusters: verifies K-means on synthetic data.
- serialization_roundtrip: ensures CSV ↔ struct fidelity.
- mock_fetch: simulates HTML snippet → correct CSV.

E. Results
Cluster summaries (example):
Cluster 0 (n=240): 35% Spain, 22% France, …
Cluster 1 (n=180): 40% England, 30% Germany, …
Interpretation: Cluster 0 (“Creative Dribblers”) overrepresents Iberian nations; Cluster 1 (“Physical Duels”) overrepresents Northern European nations.

F. Usage Instructions
Prerequisites: Rust toolchain (≥1.79), network, cargo.
Clone & Build:
  git clone https://github.com/seifhaider/ds210finalproject.git
  cd ds210finalproject
  cargo build --release
Run:
  cargo run --release
Tests:
  cargo test
Runtime: ~30s end-to-end.
