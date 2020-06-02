use crate::profile::Credential;
use async_trait::async_trait;
use rusoto_credential::ProvideAwsCredentials;
use rusoto_credential::{AwsCredentials, CredentialsError};

pub struct RawProvider {
    credential: Credential,
}

#[async_trait]
impl ProvideAwsCredentials for RawProvider {
    async fn credentials(&self) -> std::result::Result<AwsCredentials, CredentialsError> {
        Ok(AwsCredentials::new(
            self.credential.access_key_id.to_owned(),
            self.credential.secret_access_key.to_owned(),
            Some(self.credential.session_token.to_owned()),
            Some(self.credential.expiration.to_owned()),
        ))
    }
}
