use image::{ImageBuffer, ImageDecoder, ImageReader, Rgba};
use lcms2::{CIExyY, Intent, PixelFormat, Profile, ThreadContext, Transform};
use std::io::BufReader;
use std::path::PathBuf;
use tokio::task::spawn_blocking;

/// CIE Lab 色空間のピクセルデータ
#[derive(Debug, Clone)]
pub struct CIELabPix {
    pub l: f32,
    pub a: f32,
    pub b: f32,
}

/// 画像ファイルから ICC プロファイルを抽出する
///
/// # Arguments
/// * `path` - 画像ファイルのパス
///
/// # Returns
/// ICC プロファイルのバイト列（存在しない場合は None）
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

/// 画像ファイルからピクセルデータを抽出する（RGBA16形式）
///
/// # Arguments
/// * `path` - 画像ファイルのパス
///
/// # Returns
/// RGBA16 形式のピクセルデータ
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

/// ICC プロファイルとピクセルデータを一度に抽出する
///
/// 同じファイルを2回開く必要がある場合に効率的
///
/// # Arguments
/// * `path` - 画像ファイルのパス
///
/// # Returns
/// (ICC プロファイル（存在しない場合は None）, RGBA16 形式のピクセルデータ)
pub async fn extract_image_data(
    path: PathBuf,
) -> Result<(Option<Vec<u8>>, ImageBuffer<Rgba<u16>, Vec<u16>>), anyhow::Error> {
    spawn_blocking(move || {
        use std::io::Seek;

        let mut file = std::fs::File::open(&path)?;

        // ICC プロファイルを抽出（スコープで借用を終了させる）
        let icc_chunk = {
            let reader = ImageReader::new(BufReader::new(&file)).with_guessed_format()?;
            let mut decoder = reader.into_decoder()?;
            decoder.icc_profile()?
        };

        // ファイルを巻き戻してピクセルを抽出
        file.rewind()?;
        let reader = ImageReader::new(BufReader::new(&file)).with_guessed_format()?;
        let img = reader.decode()?;
        let pixels = img.to_rgba16();

        Ok((icc_chunk, pixels))
    })
    .await?
}

/// ピクセルデータを CIE Lab 色空間に変換する
///
/// # Arguments
/// * `pixels` - RGBA16 形式のピクセルデータ
/// * `icc_profile` - ICC プロファイルのバイト列（None の場合は sRGB を使用）
///
/// # Returns
/// CIE Lab 色空間のピクセルデータ
pub async fn into_cie_lab(
    pixels: ImageBuffer<Rgba<u16>, Vec<u16>>,
    icc_profile: Option<Vec<u8>>,
) -> Result<Vec<CIELabPix>, anyhow::Error> {
    spawn_blocking(move || {
        let context = ThreadContext::new();

        let src_profile = if let Some(ref icc_chunk) = icc_profile {
            Profile::new_icc_context(&context, icc_chunk)?
        } else {
            Profile::new_srgb_context(&context)
        };

        let dest_profile = Profile::new_lab4_context(&context, CIExyY::d50())?;

        let t: Transform<[u16; 4], [f32; 3], ThreadContext> = Transform::new_context(
            &context,
            &src_profile,
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
                l: lab[0],
                a: lab[1],
                b: lab[2],
            })
            .collect())
    })
    .await?
}

