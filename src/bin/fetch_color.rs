use cica::image_processing::{extract_image_data, into_cie_lab};
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    path: PathBuf,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();

    let (icc_chunk, pixels) = extract_image_data(args.path).await?;
    let lab_pixels = into_cie_lab(pixels, icc_chunk).await?;

    for lab in lab_pixels {
        println!("L: {}, a: {}, b: {}", lab.l, lab.a, lab.b);
    }

    Ok(())
}
