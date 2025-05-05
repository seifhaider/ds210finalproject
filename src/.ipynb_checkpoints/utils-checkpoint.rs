// src/utils.rs
//! Utility functions: normalization and data transformations.

use crate::data::PlayerProfile;
use ndarray::{Array2, Array1, Axis};

/// Normalizes features to zero mean and unit variance. Returns (matrix, ids).
pub fn normalize_features(profiles: &[PlayerProfile]) -> (Array2<f64>, Vec<String>) {
    let n = profiles.len();
    let m = 3; // number of metrics
    let mut data = Array2::<f64>::zeros((n, m));
    let mut ids = Vec::with_capacity(n);
    for (i, p) in profiles.iter().enumerate() {
        data[[i, 0]] = p.dribbles;
        data[[i, 1]] = p.prog_carries;
        data[[i, 2]] = p.final_third;
        ids.push(p.name.clone());
    }
    // compute mean & std
    let mean: Array1<f64> = data.mean_axis(Axis(0)).unwrap();
    let std: Array1<f64> = data.std_axis(Axis(0), 0.0);
    // normalize
    let mut norm = data.clone();
    for mut row in norm.rows_mut() {
        row -= &mean;
        row /= &std;
    }
    (norm, ids)
}