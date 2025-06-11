use std::path::{Path, PathBuf};
use std::fs;
use uuid::Uuid;
use zip::ZipArchive;
use std::io::Cursor;
use tracing::{info, error};

pub struct StorageService {
    base_dir: PathBuf,
}

impl StorageService {
    pub fn new(base_dir: impl AsRef<Path>) -> Self {
        Self {
            base_dir: base_dir.as_ref().to_path_buf(),
        }
    }

    fn get_file_path(&self, filename: &str) -> PathBuf {
        self.base_dir.join(filename)
    }

    pub async fn save_file(&self, content: &[u8], filename: &str) -> Result<(), std::io::Error> {
        let file_path = self.get_file_path(filename);
        fs::write(file_path, content)?;
        info!("File saved successfully: {}", filename);
        Ok(())
    }

    pub async fn read_file(&self, filename: &str) -> Result<Vec<u8>, std::io::Error> {
        let file_path = self.get_file_path(filename);
        let content = fs::read(file_path)?;
        info!("File read successfully: {}", filename);
        Ok(content)
    }

    pub async fn extract_zip(&self, content: &[u8], extract_dir: &str) -> Result<(), std::io::Error> {
        let cursor = Cursor::new(content);
        let mut archive = ZipArchive::new(cursor)?;
        let extract_path = self.base_dir.join(extract_dir);
        fs::create_dir_all(&extract_path)?;

        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            let outpath = extract_path.join(file.name());
            
            if file.name().ends_with('/') {
                fs::create_dir_all(&outpath)?;
            } else {
                if let Some(parent) = outpath.parent() {
                    fs::create_dir_all(parent)?;
                }
                let mut outfile = fs::File::create(&outpath)?;
                std::io::copy(&mut file, &mut outfile)?;
            }
        }
        
        info!("ZIP file extracted successfully to: {}", extract_dir);
        Ok(())
    }
} 