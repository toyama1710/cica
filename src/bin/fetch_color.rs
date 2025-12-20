use clap::Parser;
use image::{ColorType, ImageReader};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    path: PathBuf,
    #[arg(long, value_name = "COLORTYPE")]
    color_type: Option<String>,
}

struct Config {
    path: PathBuf,
    color_type: ColorType,
}

fn parse_color_type(s: &str) -> Result<ColorType, anyhow::Error> {
    // ATTENTION: case sensitive
    match s {
        "l8" => Ok(ColorType::L8),
        "la8" => Ok(ColorType::La8),
        "rgb8" => Ok(ColorType::Rgb8),
        "rgba8" => Ok(ColorType::Rgba8),
        "l16" => Ok(ColorType::L16),
        "la16" => Ok(ColorType::La16),
        "rgb16" => Ok(ColorType::Rgb16),
        "rgba16" => Ok(ColorType::Rgba16),
        "rgb32f" => Ok(ColorType::Rgb32F),
        "rgba32f" => Ok(ColorType::Rgba32F),
        _ => return Err(anyhow::anyhow!("Invalid color type: {}", s)),
    }
}

impl Config {
    fn new(args: Args) -> Result<Self, anyhow::Error> {
        let color_type = parse_color_type(&args.color_type.unwrap_or("rgba8".to_string()))?;
        Ok(Self {
            path: args.path,
            color_type,
        })
    }
}

fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();
    let config = Config::new(args)?;
    let img = ImageReader::open(config.path)?.decode()?;

    // TODO: use config.color_type in future
    let pixels = img.to_rgba8();

    for pixel in pixels.iter() {
        println!("{:?}", pixel);
    }

    Ok(())
}
