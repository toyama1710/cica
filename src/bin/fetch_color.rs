use clap::Parser;
use image::{ImageBuffer, ImageDecoder, ImageReader, Rgba};
use lcms2::{CIExyY, Intent, PixelFormat, Profile, ThreadContext, Transform};
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

struct CIELabPix {
    l: f32,
    a: f32,
    b: f32,
}

fn into_cie_lab(
    pixels: &ImageBuffer<Rgba<u16>, Vec<u16>>,
    src_profile: &Profile<ThreadContext>,
    context: &ThreadContext,
) -> Result<Vec<CIELabPix>, anyhow::Error> {
    let dest_profile = Profile::new_lab4_context(context, CIExyY::d50())?;

    let t: Transform<[u16; 4], [f32; 3], ThreadContext> = Transform::new_context(
        context,
        src_profile,
        PixelFormat::RGBA_16,
        &dest_profile,
        PixelFormat::Lab_FLT,
        Intent::Perceptual,
    )?;

    let rgba_pixels = pixels
        .chunks(4)
        .map(|chunk| [chunk[0], chunk[1], chunk[2], chunk[3]])
        .collect::<Vec<[u16; 4]>>();

    let mut lab_pixels: Vec<[f32; 3]> = vec![[0.0; 3]; rgba_pixels.len()];

    t.transform_pixels(&rgba_pixels, lab_pixels.as_mut_slice());

    Ok(lab_pixels
        .iter()
        .map(|lab| CIELabPix {
            l: lab[0] as f32,
            a: lab[1] as f32,
            b: lab[2] as f32,
        })
        .collect())
}

fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();

    let context = ThreadContext::new();

    let mut file = File::open(args.path)?;
    let icc_chunk = extract_icc_chunk(&file)?;
    file.rewind()?; // fail extract_pixels if not rewind
    let pixels = extract_pixels(&file)?;

    let src_profile = if let Some(icc_chunk) = icc_chunk {
        Profile::new_icc_context(&context, &icc_chunk)?
    } else {
        Profile::new_srgb_context(&context)
    };

    let lab_pixels = into_cie_lab(&pixels, &src_profile, &context)?;
    for lab in lab_pixels {
        println!("L: {}, a: {}, b: {}", lab.l, lab.a, lab.b);
    }

    Ok(())
}
