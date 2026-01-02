//! k-means++ clustering algorithm for 4D vectors.

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

/// Result of k-means++ clustering.
#[derive(Debug, Clone)]
pub struct KMeansResult {
    /// `clusters[i]` contains all points assigned to i-th cluster
    pub clusters: Vec<Vec<[f32; 4]>>,
    /// `centroids[i]` is center of i-th cluster
    pub centroids: Vec<[f32; 4]>,
}

/// Runs the k-means++ algorithm on a set of 4D points.
///
/// # Arguments
/// * `points` - The input points to cluster.
/// * `k` - The number of clusters.
/// * `seed` - Random seed for reproducibility.
///
/// # Returns
/// A `KMeansResult` containing the clustered points and their centroids.
pub fn kmeans_pp(points: &[[f32; 4]], k: usize, seed: u64) -> KMeansResult {
    assert!(k > 0, "k must be greater than 0");
    assert!(!points.is_empty(), "points must not be empty");
    assert!(k <= points.len(), "k must not exceed the number of points");

    let mut rng = StdRng::seed_from_u64(seed);

    // Initialize centroids using k-means++ initialization
    let mut centroids = init_centroids_pp(points, k, &mut rng);

    // Run Lloyd's algorithm until convergence
    loop {
        let assignments = assign_points(points, &centroids);
        let new_centroids = compute_centroids(points, &assignments, k);

        if centroids_converged(&centroids, &new_centroids) {
            centroids = new_centroids;
            break;
        }
        centroids = new_centroids;
    }

    // Build final clusters
    let assignments = assign_points(points, &centroids);
    let mut clusters: Vec<Vec<[f32; 4]>> = vec![Vec::new(); k];
    for (point, &cluster_idx) in points.iter().zip(assignments.iter()) {
        clusters[cluster_idx].push(*point);
    }

    KMeansResult {
        clusters,
        centroids,
    }
}

/// k-means++ initialization: select initial centroids with probability proportional to D(x)^2.
fn init_centroids_pp(points: &[[f32; 4]], k: usize, rng: &mut StdRng) -> Vec<[f32; 4]> {
    let mut centroids = Vec::with_capacity(k);

    // Choose the first centroid uniformly at random
    let first_idx = rng.random_range(0..points.len());
    centroids.push(points[first_idx]);

    // Choose remaining centroids
    for _ in 1..k {
        // Compute D(x)^2 for each point (squared distance to nearest centroid)
        let distances_sq: Vec<f32> = points
            .iter()
            .map(|p| {
                centroids
                    .iter()
                    .map(|c| distance_sq(p, c))
                    .fold(f32::INFINITY, f32::min)
            })
            .collect();

        // Compute cumulative distribution
        let total: f32 = distances_sq.iter().sum();
        let threshold = rng.random_range(0.0..=total);

        let mut cumulative = 0.0;
        let mut chosen_idx = 0;
        for (i, &d_sq) in distances_sq.iter().enumerate() {
            cumulative += d_sq;
            if cumulative >= threshold {
                chosen_idx = i;
                break;
            }
        }

        centroids.push(points[chosen_idx]);
    }

    centroids
}

/// Assigns each point to the nearest centroid.
fn assign_points(points: &[[f32; 4]], centroids: &[[f32; 4]]) -> Vec<usize> {
    points
        .iter()
        .map(|p| {
            centroids
                .iter()
                .enumerate()
                .map(|(i, c)| (i, distance_sq(p, c)))
                .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
                .map(|(i, _)| i)
                .unwrap()
        })
        .collect()
}

/// Computes new centroids as the mean of assigned points.
fn compute_centroids(points: &[[f32; 4]], assignments: &[usize], k: usize) -> Vec<[f32; 4]> {
    let mut sums = vec![[0.0f32; 4]; k];
    let mut counts = vec![0usize; k];

    for (point, &cluster_idx) in points.iter().zip(assignments.iter()) {
        for d in 0..4 {
            sums[cluster_idx][d] += point[d];
        }
        counts[cluster_idx] += 1;
    }

    sums.iter()
        .zip(counts.iter())
        .map(|(sum, &count)| {
            if count == 0 {
                *sum
            } else {
                let mut centroid = [0.0; 4];
                for d in 0..4 {
                    centroid[d] = sum[d] / count as f32;
                }
                centroid
            }
        })
        .collect()
}

/// Checks if centroids have converged (no significant change).
fn centroids_converged(old: &[[f32; 4]], new: &[[f32; 4]]) -> bool {
    const EPSILON: f32 = 1e-6;
    old.iter()
        .zip(new.iter())
        .all(|(o, n)| distance_sq(o, n) < EPSILON)
}

/// Computes squared Euclidean distance between two 4D points.
#[inline]
fn distance_sq(a: &[f32; 4], b: &[f32; 4]) -> f32 {
    (0..4).map(|d| (a[d] - b[d]).powi(2)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kmeans_pp_basic() {
        let points = vec![
            [0.0, 0.0, 0.0, 1.0],
            [0.1, 0.1, 0.1, 1.0],
            [10.0, 10.0, 10.0, 1.0],
            [10.1, 10.1, 10.1, 1.0],
        ];

        let result = kmeans_pp(&points, 2, 42);

        assert_eq!(result.clusters.len(), 2);
        assert_eq!(result.centroids.len(), 2);

        // Each cluster should have 2 points
        let total_points: usize = result.clusters.iter().map(|c| c.len()).sum();
        assert_eq!(total_points, 4);
    }

    #[test]
    fn test_kmeans_pp_single_cluster() {
        let points = vec![
            [1.0, 2.0, 3.0, 4.0],
            [1.1, 2.1, 3.1, 4.1],
            [0.9, 1.9, 2.9, 3.9],
        ];

        let result = kmeans_pp(&points, 1, 42);

        assert_eq!(result.clusters.len(), 1);
        assert_eq!(result.clusters[0].len(), 3);
    }
}
