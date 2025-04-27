use rfd::AsyncFileDialog;
use rfd::FileHandle;

pub async fn pick_files() -> Vec<FileHandle> {
    AsyncFileDialog::new().pick_files().await.unwrap_or(vec![])
}
