use cica::color_space::{Hsv, Lab, Srgb, Xyz};
use cica::image_processing::{extract_image_data, into_cie_xyz};
use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(Debug, Clone, ValueEnum)]
enum OutputColorSpace {
    Xyz,
    Hsv,
    Srgb,
    Lab,
    RawRgb,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    path: PathBuf,

    #[arg(short, long, value_enum, default_value_t = OutputColorSpace::Xyz)]
    color_space: OutputColorSpace,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();

    let (icc_chunk, pixels) = extract_image_data(args.path.clone()).await?;

    if matches!(args.color_space, OutputColorSpace::RawRgb) {
        for pixel in pixels.chunks(4) {
            let r = pixel[0] as f32 / 65535.0;
            let g = pixel[1] as f32 / 65535.0;
            let b = pixel[2] as f32 / 65535.0;
            println!("{}, {}, {}", r, g, b);
        }
        return Ok(());
    }

    let xyz_pixels: Vec<Xyz> = into_cie_xyz(pixels, icc_chunk).await?;

    match args.color_space {
        OutputColorSpace::Xyz => {
            for xyz in &xyz_pixels {
                println!("{}, {}, {}", xyz.x, xyz.y, xyz.z);
            }
        }
        OutputColorSpace::Lab => {
            for xyz in &xyz_pixels {
                let lab: Lab = (*xyz).into();
                println!("{}, {}, {}", lab.l, lab.a, lab.b);
            }
        }
        OutputColorSpace::Srgb => {
            for xyz in &xyz_pixels {
                let srgb: Srgb = (*xyz).into();
                println!("{}, {}, {}", srgb.r, srgb.g, srgb.b);
            }
        }
        OutputColorSpace::Hsv => {
            for xyz in &xyz_pixels {
                let hsv: Hsv = (*xyz).into();
                println!("{}, {}, {}", hsv.h, hsv.s, hsv.v);
            }
        }
        OutputColorSpace::RawRgb => unreachable!(),
    }

    Ok(())
}
