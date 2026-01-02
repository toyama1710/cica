use cica::color_space::{Hsv, Srgb, Xyz};
use cica::image_processing::{extract_image_data, into_cie_lab};
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

    #[arg(short, long, value_enum, default_value_t = OutputColorSpace::Lab)]
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

    let lab_pixels = into_cie_lab(pixels, icc_chunk).await?;

    match args.color_space {
        OutputColorSpace::Lab => {
            for lab in lab_pixels {
                println!("{}, {}, {}", lab.l, lab.a, lab.b);
            }
        }
        OutputColorSpace::Xyz => {
            for lab in lab_pixels {
                let xyz: Xyz = lab.into();
                println!("{}, {}, {}", xyz.x, xyz.y, xyz.z);
            }
        }
        OutputColorSpace::Srgb => {
            for lab in lab_pixels {
                let xyz: Xyz = lab.into();
                let srgb: Srgb = xyz.into();
                println!("{}, {}, {}", srgb.r, srgb.g, srgb.b);
            }
        }
        OutputColorSpace::Hsv => {
            for lab in lab_pixels {
                let xyz: Xyz = lab.into();
                let hsv: Hsv = xyz.into();
                println!("{}, {}, {}", hsv.h, hsv.s, hsv.v);
            }
        }
        OutputColorSpace::RawRgb => unreachable!(),
    }

    Ok(())
}
