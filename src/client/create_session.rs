use super::util::*;
use crate::error::Error::*;
use crate::error::*;
use crate::profile::*;
use rusoto_core::Region;
use rusoto_sts::{GetSessionTokenRequest, Sts, StsClient};

pub async fn send(profile: &mut Profile, token: &str) -> Result<()> {
    let serial_number;
    let access = profile.access.as_ref().ok_or(ProfileParamNotFound(
        profile.profile_name.clone(),
        "ACCESS_KEY".to_string(),
    ))?;
    match access {
        Access::AccessKey(key) => {
            serial_number = key.mfa_device.as_ref();
        }
        Access::AssumeRole(_) => {
            return Err(ProfileTypeError(
                profile.profile_name.clone(),
                "CreateSession".to_string(),
            ))
        }
    }

    let client = StsClient::new_with(
        http_client()?,
        profile_provider(&profile.profile_name)?,
        Region::default(),
    );

    let request = GetSessionTokenRequest {
        duration_seconds: Some(12 * 60 * 60),
        serial_number: serial_number.map(|s| s.to_owned()),
        token_code: Some(token.to_string()),
    };

    let response = client.get_session_token(request).await?;
    let rusoto_credential = &response.credentials.ok_or(AwsResponseFormatError(
        "no credential in GetSessionToken response".to_string(),
    ))?;

    profile.credential = Some(Credential::new(rusoto_credential)?);
    Ok(())
}
