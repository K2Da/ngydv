use super::util::*;
use crate::error::Error::*;
use crate::error::*;
use crate::profile::Access::{AccessKey, AssumeRole};
use crate::profile::{AssumedRole, Credential, Profile};
use rusoto_core::Region;
use rusoto_sts::{AssumeRoleRequest, Sts, StsClient};

pub async fn send(profile: &mut Profile, mfa_token: &str) -> Result<()> {
    let access = &profile
        .access
        .as_ref()
        .ok_or(AssumeRoleSettingNotFound(profile.profile_name.clone()))?;

    match access {
        AccessKey(_) => Err(AssumeRoleSettingNotFound(profile.profile_name.clone())),

        AssumeRole(assume_role) => {
            let client = StsClient::new_with(
                http_client()?,
                profile_provider(&assume_role.source_profile)?,
                Region::default(),
            );
            let response = client.assume_role(request(assume_role, &mfa_token)).await?;
            let rusoto_credential = &response.credentials.ok_or(AwsResponseFormatError(
                "no credential in AssumeRole response".to_string(),
            ))?;
            profile.credential = Some(Credential::new(rusoto_credential)?);
            Ok(())
        }
    }
}

fn request(assume_role: &AssumedRole, mfa_token: &str) -> AssumeRoleRequest {
    AssumeRoleRequest {
        duration_seconds: None,
        external_id: None,
        policy: None,
        policy_arns: None,
        role_arn: assume_role.role_arn.to_owned(),
        role_session_name: "session_name".to_string(),
        serial_number: Some(assume_role.mfa_serial.to_owned()),
        tags: None,
        token_code: Some(mfa_token.to_string()),
        transitive_tag_keys: None,
    }
}
