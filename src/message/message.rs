use crate::model::*;
use iced::widget::text_editor;
use rfd::FileHandle;

#[derive(Debug, Clone)]
pub enum Message {
    AddImageClicked,
    TabSelected(MainTab),
    FilesPicked(Vec<FileHandle>),
    FileOpened((ImgFileStatus, usize)), // ファイル名とidxの組
    ExprUpdated(text_editor::Action),
    EvalExprRequested,
}
