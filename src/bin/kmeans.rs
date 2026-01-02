use std::io::{self, BufReader, Read};
use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::Parser;
use csv::ReaderBuilder;
use serde::Deserialize;

use cica::kmeans::kmeans_pp;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short)]
    /// Number of clusters
    k: usize,
    #[arg(short, long)]
    /// Random seed for reproducibility
    seed: u64,
    #[arg(short, long)]
    /// Input CSV file (reads from stdin if not specified)
    path: Option<PathBuf>,
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

    let points = read_csv(args.path.as_ref())?;

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

fn read_csv(path: Option<&PathBuf>) -> Result<Vec<[f32; 4]>> {
    // Read all input data first
    let data = match path {
        Some(p) => std::fs::read_to_string(p)
            .with_context(|| format!("Failed to open file: {}", p.display()))?,
        None => {
            let mut buf = String::new();
            BufReader::new(io::stdin())
                .read_to_string(&mut buf)
                .context("Failed to read from stdin")?;
            buf
        }
    };

    // Detect if first line is a header (contains non-numeric values)
    let has_headers = detect_header(&data);

    let mut reader = ReaderBuilder::new()
        .has_headers(has_headers)
        .comment(Some(b'#'))
        .trim(csv::Trim::All)
        .from_reader(data.as_bytes());

    let mut points = Vec::new();

    for result in reader.deserialize() {
        let point: Point = result.with_context(|| "Failed to parse CSV record")?;
        points.push([point.x, point.y, point.z, point.w]);
    }

    Ok(points)
}

/// Detect if the first non-comment line is a header row
fn detect_header(data: &str) -> bool {
    for line in data.lines() {
        let trimmed = line.trim();
        // Skip empty lines and comments
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        // Check if first field is numeric
        if let Some(first_field) = trimmed.split(',').next() {
            return first_field.trim().parse::<f32>().is_err();
        }
        break;
    }
    false
}
