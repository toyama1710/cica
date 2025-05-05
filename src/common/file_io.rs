use crate::model::ImageItem;
use crate::model::ImgFileStatus;
use crate::view::thumbnail::img_thumbnail_handle;
use iced::widget::image::Handle;
use image::GenericImageView;
use image::ImageError;
use rfd::AsyncFileDialog;
use rfd::FileHandle;
use std::path::PathBuf;

pub async fn pick_files() -> Vec<FileHandle> {
    AsyncFileDialog::new().pick_files().await.unwrap_or(vec![])
}

pub async fn open_file(path: PathBuf, idx: usize) -> (ImgFileStatus, usize) {
    match image::open(&path) {
        Ok(img) => (
            ImgFileStatus::Image(ImageItem {
                id: format!("{}", idx).to_string(),
                filename: path.file_name().unwrap().to_string_lossy().into_owned(),
                pixel: img.to_rgba8(),
                path,
                thumbnail_handle: img_thumbnail_handle(&img),
            }),
            idx,
        ),
        Err(e) => match e {
            ImageError::IoError(e) => (ImgFileStatus::IOerror((path, e.to_string())), idx),
            ImageError::Limits(e) => (ImgFileStatus::IOerror((path, e.to_string())), idx),
            _ => (ImgFileStatus::NotImage((path, e.to_string())), idx),
        },
    }
}
