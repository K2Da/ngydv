use crate::error::*;

pub fn profile_provider(profile: &str) -> Result<rusoto_credential::ProfileProvider> {
    let mut provider = rusoto_credential::ProfileProvider::new()?;
    provider.set_profile(profile);
    Ok(provider)
}

pub fn http_client() -> Result<rusoto_core::HttpClient> {
    Ok(rusoto_core::HttpClient::new()?)
}
