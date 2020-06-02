pub mod aws_setting;
pub mod credentials;
use crate::error::Error::*;
use crate::error::*;
use std::path::PathBuf;

fn aws_config_dir() -> Result<PathBuf> {
    let home_dir = dirs::home_dir().ok_or(UserHomeNotFoundError)?;
    Ok(home_dir.as_path().join(".aws/"))
}

pub fn aws_config_file() -> Result<PathBuf> {
    Ok(aws_config_dir()?.join("config"))
}

pub fn aws_credential_file() -> Result<PathBuf> {
    Ok(aws_config_dir()?.join("credentials"))
}

pub fn ngydv_config_file() -> Result<PathBuf> {
    Ok(aws_config_dir()?.join("ngydv"))
}

pub fn credentials_path() -> Result<PathBuf> {
    Ok(aws_config_dir()?.join("ngydv_credentials.yaml"))
}
