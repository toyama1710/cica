use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::Parser;
use csv::ReaderBuilder;
use serde::Deserialize;

use cica::kmeans::kmeans_pp;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    path: PathBuf,
    /// Number of clusters
    k: usize,
    /// Random seed for reproducibility
    seed: u64,
}

#[derive(Debug, Deserialize)]
struct Point {
    x: f32,
    y: f32,
    z: f32,
    #[serde(default = "default_w")]
    w: f32,
}

fn default_w() -> f32 {
    1.0
}

fn main() -> Result<()> {
    let args = Args::parse();

    let points = read_csv(&args.path)?;

    if points.is_empty() {
        anyhow::bail!("No points found in CSV file");
    }

    if args.k > points.len() {
        anyhow::bail!(
            "k ({}) cannot exceed the number of points ({})",
            args.k,
            points.len()
        );
    }

    let result = kmeans_pp(&points, args.k, args.seed);

    println!("# k-means++ clustering result");
    println!(
        "# k = {}, seed = {}, total points = {}",
        args.k,
        args.seed,
        points.len()
    );
    println!();

    for (i, centroid) in result.centroids.iter().enumerate() {
        println!(
            "Cluster {}: center = [{:.6}, {:.6}, {:.6}, {:.6}], size = {}",
            i,
            centroid[0],
            centroid[1],
            centroid[2],
            centroid[3],
            result.clusters[i].len()
        );
    }

    println!();
    println!("# Points per cluster:");
    for (i, cluster) in result.clusters.iter().enumerate() {
        println!("## Cluster {}", i);
        for point in cluster {
            println!(
                "  {:.6}, {:.6}, {:.6}, {:.6}",
                point[0], point[1], point[2], point[3]
            );
        }
    }

    Ok(())
}

fn read_csv(path: &PathBuf) -> Result<Vec<[f32; 4]>> {
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .comment(Some(b'#'))
        .trim(csv::Trim::All)
        .from_path(path)
        .with_context(|| format!("Failed to open file: {}", path.display()))?;

    let mut points = Vec::new();

    for result in reader.deserialize() {
        let point: Point = result.with_context(|| "Failed to parse CSV record")?;
        points.push([point.x, point.y, point.z, point.w]);
    }

    Ok(points)
}
