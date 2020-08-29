use super::util::*;
use crate::client::raw_provider::RawProvider;
use crate::error::Error::*;
use crate::error::*;
use crate::profile::Access::{AccessKey, AssumeRole};
use crate::profile::{AssumedRole, Credential, Profile, ProfileMap};
use rusoto_core::Region;
use rusoto_sts::{AssumeRoleRequest, Sts, StsClient};

pub async fn send(
    profile_name: &str,
    profile_map: &mut ProfileMap,
    mfa_token: Option<String>,
) -> Result<()> {
    let profile = profile_map.get(profile_name)?;
    let access = &profile
        .access
        .as_ref()
        .ok_or(AssumeRoleSettingNotFound(profile.profile_name.clone()))?;

    match access {
        AccessKey(_) => Err(AssumeRoleSettingNotFound(profile.profile_name.clone())),

        AssumeRole(assume_role) => {
            let source = profile_map.get(&assume_role.source_profile)?.clone();
            let client = client(assume_role, &mfa_token, source)?;
            let response = client.assume_role(request(assume_role, mfa_token)).await?;
            let rusoto_credential = &response.credentials.ok_or(AwsResponseFormatError(
                "no credential in AssumeRole response".to_string(),
            ))?;
            let profile = profile_map.get_mut(profile_name)?;
            profile.credential = Some(Credential::new(rusoto_credential)?);
            Ok(())
        }
    }
}

fn client(
    assume_role: &AssumedRole,
    mfa_token: &Option<String>,
    source: &Profile,
) -> Result<StsClient> {
    Ok(match mfa_token {
        Some(_) => StsClient::new_with(
            http_client()?,
            profile_provider(&assume_role.source_profile)?,
            Region::default(),
        ),
        _ => StsClient::new_with(
            http_client()?,
            RawProvider {
                credential: source.credential.as_ref().unwrap().clone(),
            },
            Region::default(),
        ),
    })
}

fn request(assume_role: &AssumedRole, mfa_token: Option<String>) -> AssumeRoleRequest {
    AssumeRoleRequest {
        duration_seconds: None,
        external_id: None,
        policy: None,
        policy_arns: None,
        role_arn: assume_role.role_arn.to_owned(),
        role_session_name: "session_name".to_string(),
        serial_number: if mfa_token.is_some() {
            Some(assume_role.mfa_serial.to_owned())
        } else {
            None
        },
        tags: None,
        token_code: mfa_token,
        transitive_tag_keys: None,
    }
}
