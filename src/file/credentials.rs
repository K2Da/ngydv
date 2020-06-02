use crate::error::Error::*;
use crate::error::*;
use crate::file::*;
use crate::profile;
use crate::profile::*;
use std::collections::HashMap;
use std::fs::File;

pub fn store_credentials(profile_map: &ProfileMap) -> Result<()> {
    let mut save_data: HashMap<String, profile::Credential> = HashMap::new();
    for profile in profile_map.profiles() {
        match &profile.credential {
            None => (),
            Some(credential) => {
                save_data.insert(profile.profile_name.to_owned(), credential.clone());
            }
        }
    }
    let path = credentials_path()?;
    let path_str = path.as_path().display().to_string();
    let writer = File::create(path).or(Err(UnableToWriteCredentialFileError(path_str.clone())))?;
    serde_yaml::to_writer(writer, &save_data)
        .or(Err(UnableToWriteCredentialFileError(path_str.clone())))?;
    Ok(())
}

pub fn restore_credentials(profile_map: &mut ProfileMap) -> Result<()> {
    if let Ok(file) = File::open(credentials_path()?) {
        match serde_yaml::from_reader::<File, HashMap<String, profile::Credential>>(file) {
            Err(_) => (),
            Ok(save_data) => {
                for (profile_name, profile) in profile_map.iter_mut() {
                    match save_data.get(profile_name) {
                        None => (),
                        Some(cred) => profile.credential = Some(cred.clone()),
                    }
                }
            }
        }
    }
    Ok(())
}

pub fn delete_credentials() -> Result<()> {
    if credentials_path()?.exists() {
        std::fs::remove_file(credentials_path()?).or(Err(UnableToRemoveCredentialFileError(
            credentials_path()?.as_path().display().to_string(),
        )))?;
    }
    Ok(())
}
