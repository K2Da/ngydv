mod collection;
mod export;
pub mod show;
pub use self::collection::ProfileMap;
use crate::error::Error::*;
use crate::error::*;
use chrono::{DateTime, Duration, FixedOffset, Local, Utc};
use prettytable::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, PartialEq)]
pub struct AccessKey {
    pub access_key_id: String,
    pub secret_access_key: String,
    pub mfa_device: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Credential {
    pub access_key_id: String,
    pub expiration: DateTime<Utc>,
    pub secret_access_key: String,
    pub session_token: String,
}

pub enum ProfileType {
    AssumeRole(String),
    SessionWithMFA,
    Keys,
    None,
}

impl Credential {
    pub fn new(cred: &rusoto_sts::Credentials) -> Result<Self> {
        let expiration = DateTime::<FixedOffset>::parse_from_rfc3339(&cred.expiration)
            .map(|dt| dt.with_timezone(&Utc))
            .map_err(|e| AwsResponseFormatError(format!("{:?}", e)))?;
        Ok(Self {
            access_key_id: cred.access_key_id.to_owned(),
            expiration,
            secret_access_key: cred.secret_access_key.to_owned(),
            session_token: cred.session_token.to_owned(),
        })
    }

    pub fn life(&self) -> Duration {
        self.expiration.signed_duration_since(Utc::now())
    }

    pub fn local_expired_at_str(&self) -> String {
        format!("{}", self.expiration.with_timezone(&Local).to_string())
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct AssumedRole {
    pub role_arn: String,
    pub mfa_serial: String,
    pub source_profile: String,
}

#[derive(Debug, PartialEq)]
pub enum Access {
    AccessKey(AccessKey),
    AssumeRole(AssumedRole),
}

#[derive(Debug, Default, PartialEq)]
pub struct Profile {
    pub order: usize,
    pub profile_name: String,
    pub region: Option<String>,
    pub access: Option<Access>,
    pub credential: Option<Credential>,
}

impl Profile {
    pub fn print_table(profile_map: &ProfileMap) {
        let mut profiles = profile_map.profiles();
        profiles.sort_by(|a, b| a.order.cmp(&b.order));

        let mut table = Table::new();
        table.set_titles(row!["id", "profile", "region", "type", "credential"]);
        table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
        for profile in profiles.iter() {
            table.add_row(row![
                profile.order + 1,
                profile.profile_name,
                profile.region_str(),
                profile.profile_type_str(),
                profile.credential_str(),
            ]);
        }
        table.printstd();
    }

    pub fn export(&self) -> Result<String> {
        match &self.credential {
            Some(cred) => {
                if cred.life().gt(&chrono::Duration::seconds(0)) {
                    Ok(export::rc(
                        vec![
                            ("AWS_ACCESS_KEY_ID", &cred.access_key_id),
                            ("AWS_SECRET_ACCESS_KEY", &cred.secret_access_key),
                            ("AWS_SESSION_TOKEN", &cred.session_token),
                        ],
                        vec!["AWS_PROFILE"],
                        vec![&format!(
                            "set access_key_id, secret_access_key, session_token to env for profile '{}'",
                            self.profile_name
                        )],
                    ))
                } else {
                    Err(SessionExpiredError(cred.local_expired_at_str()))
                }
            }
            None => match self.profile_type() {
                ProfileType::Keys | ProfileType::None => Ok(export::rc(
                    vec![("AWS_PROFILE", &self.profile_name)],
                    vec![
                        "AWS_ACCESS_KEY_ID",
                        "AWS_SECRET_ACCESS_KEY",
                        "AWS_SESSION_TOKEN",
                    ],
                    vec![&format!(
                        "set AWS_PROFILE for profile '{}'",
                        self.profile_name
                    )],
                )),
                ProfileType::AssumeRole(_) | ProfileType::SessionWithMFA => {
                    Err(ProfileNotSignedIn(format!("{}", self.profile_name)))
                }
            },
        }
    }

    fn profile_type_str(&self) -> String {
        match &self.profile_type() {
            ProfileType::AssumeRole(assumed_role) => format!("Assume role from {}", assumed_role),
            ProfileType::SessionWithMFA => "Access key with mfa device".to_string(),
            ProfileType::Keys => "Access key".to_string(),
            ProfileType::None => "".to_string(),
        }
    }

    pub fn profile_type(&self) -> ProfileType {
        match &self.access {
            Some(Access::AssumeRole(assumed_role)) => {
                ProfileType::AssumeRole(assumed_role.source_profile.to_string())
            }
            Some(Access::AccessKey(access_key)) => match access_key.mfa_device {
                Some(_) => ProfileType::SessionWithMFA,
                None => ProfileType::Keys,
            },
            None => ProfileType::None,
        }
    }
    fn credential_str(&self) -> String {
        match &self.credential {
            Some(cred) => {
                let life = cred.life();
                if life.gt(&chrono::Duration::seconds(0)) {
                    format!(
                        "{}{}{}",
                        duration_to_string(life.num_hours(), "hour", "hours"),
                        duration_to_string(life.num_minutes() % 60, "minute", "minutes"),
                        duration_to_string(life.num_seconds() % 60, "second", "seconds")
                    )
                } else {
                    format!("expired at {}", cred.local_expired_at_str())
                }
            }
            None => "-".to_string(),
        }
    }

    fn region_str(&self) -> String {
        self.region
            .as_ref()
            .unwrap_or(&"none".to_string())
            .to_string()
    }
}

fn duration_to_string(num: i64, singular: &str, plural: &str) -> String {
    if num <= 0 {
        "".to_string()
    } else if num == 1 {
        format!(" 1 {}", singular)
    } else {
        format!(" {} {}", num, plural)
    }
}
