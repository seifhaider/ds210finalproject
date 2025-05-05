use ndarray::{Array2, Axis};
use ndarray_rand::RandomExt;
use rand::seq::SliceRandom;
use rand::thread_rng;

/// Run K-means clustering on `data` into `k` clusters, up to `max_iter` iterations.
pub fn run_kmeans(data: &Array2<f64>, k: usize, max_iter: usize)
    -> (Array2<f64>, Vec<usize>)
{
    let n_samples = data.len_of(Axis(0));
    let mut centroids = data
        .select(Axis(0), &((0..n_samples).collect::<Vec<_>>()))
        .random_subset(k, &mut thread_rng())
        .to_owned();

    let mut labels = vec![0; n_samples];
    for _ in 0..max_iter {
        // 1) assign labels
        for i in 0..n_samples {
            let row = data.index_axis(Axis(0), i);
            labels[i] = centroids
                .rows()
                .into_iter()
                .enumerate()
                .min_by(|(_, a), (_, b)| {
                    let da = (&row - &a).mapv(f64::powi).scalar_sum();
                    let db = (&row - &b).mapv(f64::powi).scalar_sum();
                    da.partial_cmp(&db).unwrap()
                })
                .unwrap().0;
        }
        // 2) update centroids
        let mut new_centroids = Array2::zeros(centroids.dim());
        let mut counts = vec![0usize; k];
        for (i, &lab) in labels.iter().enumerate() {
            new_centroids
                .row_mut(lab)
                .scaled_add(1.0, &data.index_axis(Axis(0), i));
            counts[lab] += 1;
        }
        for j in 0..k {
            if counts[j] > 0 {
                new_centroids
                    .row_mut(j)
                    .mapv_inplace(|x| x / counts[j] as f64);
            }
        }
        centroids = new_centroids;
    }
    (centroids, labels)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::array;
    #[test]
    fn simple_two_clusters() {
        let data = array![[0.0], [0.1], [0.9], [1.0]];
        let (centroids, labels) = run_kmeans(&data, 2, 10);
        // expect two clusters roughly at 0.05 and 0.95
        assert_eq!(labels[..2], [0, 0]);
        assert_eq!(labels[2..], [1, 1]);
        assert!(centroids.shape() == &[2, 1]);
    }
}