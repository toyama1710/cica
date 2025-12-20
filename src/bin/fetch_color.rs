use clap::Parser;
use image::{ColorType, ImageBuffer, ImageDecoder, ImageReader, Rgba};
use moxcms::{ColorProfile, Layout, TransformOptions};
use std::fs::File;
use std::io::{BufReader, Seek};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    path: PathBuf,
}

fn extract_icc_chunk(file: &File) -> Result<Option<Vec<u8>>, anyhow::Error> {
    let reader = ImageReader::new(BufReader::new(file)).with_guessed_format()?;
    let mut decoder = reader.into_decoder()?;
    let icc_chunk = decoder.icc_profile()?;
    Ok(icc_chunk)
}

// always return rgba16 even if the source image has 8-bit color
fn extract_pixels(file: &File) -> Result<ImageBuffer<Rgba<u16>, Vec<u16>>, anyhow::Error> {
    let reader = ImageReader::new(BufReader::new(file)).with_guessed_format()?;
    let img = reader.decode()?;
    let pixels = img.to_rgba16();
    Ok(pixels)
}

fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();

    let mut file = File::open(args.path)?;
    let icc_chunk = extract_icc_chunk(&file)?;
    file.rewind()?; // fail extract_pixels if not rewind
    let pixels = extract_pixels(&file)?.into_raw();

    let src_profile = if let Some(icc_chunk) = icc_chunk {
        ColorProfile::new_from_slice(&icc_chunk)?
    } else {
        ColorProfile::new_srgb()
    };

    let transform = src_profile.create_transform_16bit(
        Layout::Rgba,
        &ColorProfile::new_lab(),
        Layout::Rgb,
        TransformOptions::default(),
    )?;

    let mut lab_pixels = vec![0; 3 * pixels.len() / 4];
    transform.transform(&pixels, &mut lab_pixels)?;
    for lab in lab_pixels.chunks(3) {
        println!("L: {}, a: {}, b: {}", lab[0], lab[1], lab[2]);
    }

    Ok(())
}
