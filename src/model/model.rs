use std::path::PathBuf;

#[derive(Debug, Clone, Default)]
pub struct CicaModel {
    pub images: Vec<ImageStub>,
    pub selected_image_idx: Option<usize>,
    pub active_main_tab: MainTab,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ImageStub {
    pub id: usize,
    pub path: PathBuf,
    pub filename: String,
}

#[derive(Debug, Clone, Default)]
pub enum MainTab {
    #[default]
    ImageView,
    RepresentativeColors,
    ColorHistgrams,
}
