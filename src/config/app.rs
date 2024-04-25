use directories::ProjectDirs;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppConfigError {
    #[error("failed to calculate app directories")]
    GetDirFail,
}

#[derive(Debug)]
pub struct AppConfig {
    pub dirs: ProjectDirs,
}

impl AppConfig {
    pub fn new() -> Result<Self, AppConfigError> {
        let dirs =
            ProjectDirs::from("lyn", "saplyn", "relief").ok_or(AppConfigError::GetDirFail)?;

        Ok(Self { dirs })
    }
}
