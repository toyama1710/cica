use crate::model::*;
use rfd::FileHandle;

#[derive(Debug, Clone)]
pub enum Message {
    AddImageClicked,
    TabSelected(MainTab),
    FilesPicked(Vec<FileHandle>),
    FileOpened((ImgFileStatus, usize)), // ファイル名とidxの組
}
