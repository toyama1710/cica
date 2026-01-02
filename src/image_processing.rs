use crate::color_space::Xyz;
use image::{ImageBuffer, ImageDecoder, ImageReader, Rgba};
use lcms2::{Intent, PixelFormat, Profile, ThreadContext, Transform};
use std::io::BufReader;
use std::path::PathBuf;
use tokio::task::spawn_blocking;

/// extract icc chunk from image file as a byte sequence
pub async fn extract_icc_chunk(path: PathBuf) -> Result<Option<Vec<u8>>, anyhow::Error> {
    spawn_blocking(move || {
        let file = std::fs::File::open(&path)?;
        let reader = ImageReader::new(BufReader::new(file)).with_guessed_format()?;
        let mut decoder = reader.into_decoder()?;
        let icc_chunk = decoder.icc_profile()?;
        Ok(icc_chunk)
    })
    .await?
}

/// extract pixels from image file as 16bit RGBA pixels
pub async fn extract_pixels(
    path: PathBuf,
) -> Result<ImageBuffer<Rgba<u16>, Vec<u16>>, anyhow::Error> {
    spawn_blocking(move || {
        let file = std::fs::File::open(&path)?;
        let reader = ImageReader::new(BufReader::new(file)).with_guessed_format()?;
        let img = reader.decode()?;
        let pixels = img.to_rgba16();
        Ok(pixels)
    })
    .await?
}

/// extract icc chunk and pixels from image file at once
///
/// efficient when same file is opened twice
pub async fn extract_image_data(
    path: PathBuf,
) -> Result<(Option<Vec<u8>>, ImageBuffer<Rgba<u16>, Vec<u16>>), anyhow::Error> {
    spawn_blocking(move || {
        use std::io::Seek;

        let mut file = std::fs::File::open(&path)?;

        let icc_chunk = {
            let reader = ImageReader::new(BufReader::new(&file)).with_guessed_format()?;
            let mut decoder = reader.into_decoder()?;
            decoder.icc_profile()?
        };

        file.rewind()?;
        let reader = ImageReader::new(BufReader::new(&file)).with_guessed_format()?;
        let img = reader.decode()?;
        let pixels = img.to_rgba16();

        Ok((icc_chunk, pixels))
    })
    .await?
}

/// convert pixels to CIE XYZ color space (D50 illuminant)
pub async fn into_cie_xyz(
    pixels: ImageBuffer<Rgba<u16>, Vec<u16>>,
    icc_profile: Option<Vec<u8>>,
) -> Result<Vec<Xyz>, anyhow::Error> {
    spawn_blocking(move || {
        let context = ThreadContext::new();

        let src_profile = if let Some(ref icc_chunk) = icc_profile {
            Profile::new_icc_context(&context, icc_chunk)?
        } else {
            Profile::new_srgb_context(&context)
        };

        let dest_profile = Profile::new_xyz_context(&context);

        let t: Transform<[u16; 4], [f32; 3], ThreadContext> = Transform::new_context(
            &context,
            &src_profile,
            PixelFormat::RGBA_16,
            &dest_profile,
            PixelFormat::XYZ_FLT,
            Intent::Perceptual,
        )?;

        let rgba_pixels = pixels
            .chunks(4)
            .map(|chunk| [chunk[0], chunk[1], chunk[2], chunk[3]])
            .collect::<Vec<[u16; 4]>>();

        let mut xyz_pixels: Vec<[f32; 3]> = vec![[0.0; 3]; rgba_pixels.len()];

        t.transform_pixels(&rgba_pixels, xyz_pixels.as_mut_slice());

        Ok(xyz_pixels
            .iter()
            .map(|xyz| Xyz::new(xyz[0], xyz[1], xyz[2]))
            .collect())
    })
    .await?
}
