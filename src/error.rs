use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("AssumeRole request error {0:?}")]
    AssumeRoleError(#[from] rusoto_core::RusotoError<rusoto_sts::AssumeRoleError>),

    #[error("GetSessionToken request error {0:?}")]
    GetSessionTokenError(#[from] rusoto_core::RusotoError<rusoto_sts::GetSessionTokenError>),

    #[error("Profile {0:?} is not for crate session nor assume role")]
    ProfileNotForSignIn(String),

    #[error("Profile {0:?} is not signed in.")]
    ProfileNotSignedIn(String),

    #[error("Request get session token error {0:?}")]
    RusotoTlsError(#[from] rusoto_core::request::TlsError),

    #[error("rusoto credentials error {0:?}")]
    RusotoCredentialsError(#[from] rusoto_credential::CredentialsError),

    #[error("Profile {0} is not found.")]
    ProfileNotFound(String),

    #[error("Profile {0} is not for {1}.")]
    ProfileTypeError(String, String),

    #[error("Profile {0} does not have value of {1}.")]
    ProfileParamNotFound(String, String),

    #[error("profile {0} is not for assume role.")]
    AssumeRoleSettingNotFound(String),

    #[error("Config file error. {0:?}")]
    ConfigFileError(String),

    #[error("Credential file error. {0:?}")]
    CredentialFileError(String),

    #[error("Ngydv config file error. {0:?}")]
    NgydvConfigError(String),

    #[error("AWS response format error. {0:?}")]
    AwsResponseFormatError(String),

    #[error("User home not found.")]
    UserHomeNotFoundError,

    #[error("Unable to write credential file at {0:?}.")]
    UnableToWriteCredentialFileError(String),

    #[error("Unable to remove credentail file at {0:?}.")]
    UnableToRemoveCredentialFileError(String),

    #[error("Session expired at {0:?}.")]
    SessionExpiredError(String),
}
