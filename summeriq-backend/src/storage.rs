use std::path::{Path, PathBuf};
use std::fs;
use uuid::Uuid;
use zip::ZipArchive;
use std::io::Cursor;
use tracing::{info, error};

pub struct Storage {
    base_dir: PathBuf,
}

impl Storage {
    pub fn new(base_dir: impl AsRef<Path>) -> Self {
        Self {
            base_dir: base_dir.as_ref().to_path_buf(),
        }
    }

    pub fn get_file_path(&self, filename: &str) -> PathBuf {
        self.base_dir.join(filename)
    }

    pub async fn save_file(&self, content: &[u8], filename: &str) -> std::io::Result<()> {
        let file_path = self.get_file_path(filename);
        info!("Saving file to: {:?}", file_path);
        tokio::fs::write(file_path, content).await
    }

    pub async fn read_file(&self, filename: &str) -> std::io::Result<Vec<u8>> {
        let file_path = self.get_file_path(filename);
        info!("Reading file from: {:?}", file_path);
        tokio::fs::read(file_path).await
    }

    pub async fn extract_zip(&self, content: &[u8], base_filename: &str) -> std::io::Result<Vec<String>> {
        let cursor = Cursor::new(content);
        let mut archive = ZipArchive::new(cursor)?;
        let mut extracted_files = Vec::new();

        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            if file.is_dir() {
                continue;
            }

            let outpath = self.base_dir.join(format!("{}_{}", base_filename, file.name()));
            if let Some(parent) = outpath.parent() {
                fs::create_dir_all(parent)?;
            }

            let mut outfile = fs::File::create(&outpath)?;
            std::io::copy(&mut file, &mut outfile)?;

            extracted_files.push(outpath.to_string_lossy().into_owned());
        }

        Ok(extracted_files)
    }
} 