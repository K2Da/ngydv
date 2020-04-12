use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("{0}")]
    GetSessionTokenError(#[from] chuoku::error::Error),

    #[error("Profile {0:?} is not for crate session nor assume role")]
    ProfileNotForSignIn(String),
}
