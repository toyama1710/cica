use iced::widget::{image::Handle, text_editor};
use image::RgbaImage;
use std::path::PathBuf;

#[derive(Debug, Default)]
pub struct CicaModel {
    pub loading_files_count: usize,
    pub images: Vec<ImgFileStatus>,
    pub active_main_tab: MainTab,
    pub expr_state: text_editor::Content,
}

#[derive(Debug, Clone)]
pub enum ImgFileStatus {
    Loading(PathBuf),
    // String はエラーメッセージ
    NotImage((PathBuf, String)),
    IOerror((PathBuf, String)),
    Image(ImageItem),
}

#[derive(Debug, Clone)]
pub struct ImageItem {
    pub id: String,
    pub path: PathBuf,
    pub filename: String,
    pub pixel: RgbaImage,
    pub thumbnail_handle: Handle,
}

#[derive(Debug, Clone, Default)]
pub enum MainTab {
    #[default]
    ImageView,
    RepresentativeColors,
    ColorHistgrams,
}
