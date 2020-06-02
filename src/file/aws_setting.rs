use crate::error::Error::*;
use crate::error::*;
use crate::file;
use crate::file::credentials::restore_credentials;
use crate::profile::*;
use ini::ini::{Ini, Properties};
use std::collections::HashMap;

pub type PropertyMap = HashMap<String, (usize, HashMap<String, String>)>;

pub fn read_aws_config() -> Result<ProfileMap> {
    let mut props = HashMap::new();

    let conf = Ini::load_from_file(file::aws_config_file()?)
        .map_err(|e| ConfigFileError(format!("{:?}", e)))?;
    read_config(&conf, &mut props)?;

    let cred = Ini::load_from_file(file::aws_credential_file()?)
        .map_err(|e| CredentialFileError(format!("{:?}", e)))?;
    read_credentials(&cred, &mut props)?;

    if let Ok(ngydv) = Ini::load_from_file(file::ngydv_config_file()?)  {
        read_ngydv(&ngydv, &mut props)?;
    }

    let mut profiles = ProfileMap::new();
    for (name, prop) in props.iter() {
        profiles.insert(name, create_profile(name, prop));
    }

    restore_credentials(&mut profiles)?;

    Ok(profiles)
}

fn create_profile(name: &str, prop: &(usize, HashMap<String, String>)) -> Profile {
    let (order, prop) = prop;
    let mut profile = Profile {
        profile_name: name.to_owned(),
        order: *order,
        ..Profile::default()
    };

    if let Some(region) = prop.get("region") {
        profile.region = Some(region.to_string());
    }

    if let (Some(access_key_id), Some(secret_access_key)) = (
        prop.get("aws_access_key_id"),
        prop.get("aws_secret_access_key"),
    ) {
        profile.access = Some(Access::AccessKey(AccessKey {
            access_key_id: access_key_id.to_string(),
            secret_access_key: secret_access_key.to_string(),
            mfa_device: prop.get("mfa_device").map(|s| s.to_string()),
        }));
    }

    if let (Some(role_arn), Some(mfa_serial), Some(source_profile)) = (
        prop.get("role_arn"),
        prop.get("mfa_serial"),
        prop.get("source_profile"),
    ) {
        profile.access = Some(Access::AssumeRole(AssumedRole {
            role_arn: role_arn.to_string(),
            mfa_serial: mfa_serial.to_string(),
            source_profile: source_profile.to_string(),
        }))
    }

    profile
}

fn read_config(conf: &Ini, props: &mut PropertyMap) -> Result<()> {
    for (section_key, properties) in conf {
        let section = section_key.ok_or(ConfigFileError("section name not found".to_string()))?;
        let profile_name = profile_name(section).ok_or(ConfigFileError(format!(
            "section header line {} is empty.",
            section
        )))?;

        props.insert(profile_name, (props.len(), properties_to_vec(properties)));
    }
    Ok(())
}

fn read_credentials(cred: &Ini, props: &mut PropertyMap) -> Result<()> {
    for (section_key, prop) in cred {
        let section =
            section_key.ok_or(CredentialFileError("section name not found".to_string()))?;
        let profile_name = profile_name(section).ok_or(CredentialFileError(format!(
            "section header line {} is empty.",
            section
        )))?;

        merge_props(&profile_name, prop, props);
    }
    Ok(())
}

fn read_ngydv(cred: &Ini, props: &mut PropertyMap) -> Result<()> {
    for (section_key, prop) in cred {
        let section = section_key.ok_or(NgydvConfigError("section name not found".to_string()))?;
        let profile_name = profile_name(section).ok_or(NgydvConfigError(format!(
            "section header line {} is empty.",
            section
        )))?;
        merge_props(&profile_name, prop, props);
    }
    Ok(())
}

fn merge_props(profile_name: &str, prop: &Properties, props: &mut PropertyMap) {
    let prop_map = properties_to_vec(prop);
    match props.get_mut(profile_name) {
        Some((_, map)) => map.extend(prop_map),
        None => {
            props.insert(profile_name.to_owned(), (props.len(), prop_map));
        }
    }
}

fn properties_to_vec(properties: &Properties) -> HashMap<String, String> {
    properties
        .iter()
        .map(|(k, v)| (k.to_owned(), v.to_owned()))
        .collect()
}

fn profile_name(section_key: &str) -> Option<String> {
    let profile_name = section_key.replacen("profile ", "", 1).trim().to_owned();

    if profile_name.is_empty() || profile_name == "profile" {
        None
    } else {
        Some(profile_name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const KEY_ID: &str = "XXXXXXXXXXXXXXXXXXXX";
    const SECRET_KEY: &str = "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx";

    fn profile_text() -> String {
        r#"
[profile default]
output = yaml
region = ap-northeast-1
"#
        .to_string()
    }

    fn credential_text() -> String {
        format!(
            r#"
[default]
aws_access_key_id = {}
aws_secret_access_key = {}
"#,
            KEY_ID, SECRET_KEY
        )
        .to_string()
    }

    fn add_prop(expected: &mut PropertyMap, profile_name: &str, kvs: Vec<(&str, &str)>) {
        let mut values = HashMap::new();
        for (k, v) in kvs {
            values.insert(k.to_string(), v.to_string());
        }
        expected.insert(profile_name.to_string(), (0, values));
    }

    mod create_profile {
        use super::*;

        #[test]
        fn create_profile_with_credential_and_config() {
            let mut props = HashMap::new();

            let ini = Ini::load_from_str(&profile_text()).unwrap();
            read_config(&ini, &mut props).unwrap();
            let ini = Ini::load_from_str(&credential_text()).unwrap();
            read_credentials(&ini, &mut props).unwrap();

            let expected = Profile {
                order: 0,
                profile_name: "default".to_string(),
                region: Some("ap-northeast-1".to_string()),
                access: Some(Access::AccessKey(AccessKey {
                    access_key_id: KEY_ID.to_string(),
                    secret_access_key: SECRET_KEY.to_string(),
                    mfa_device: None,
                })),
                credential: None,
            };
            assert_eq!(
                create_profile("default", &props.get("default").unwrap()),
                expected
            );
        }
    }

    mod read_credentials {
        use super::*;

        #[test]
        fn read_credentials_only() {
            let mut props = HashMap::new();

            let ini = Ini::load_from_str(&credential_text()).unwrap();
            assert!(read_credentials(&ini, &mut props).is_ok());
            let mut expected = HashMap::new();
            add_prop(
                &mut expected,
                &"default",
                vec![
                    ("aws_access_key_id", KEY_ID),
                    ("aws_secret_access_key", SECRET_KEY),
                ],
            );
            assert_eq!(props, expected);
        }

        #[test]
        fn read_credential_with_config() {
            let mut props = HashMap::new();

            let ini = Ini::load_from_str(&profile_text()).unwrap();
            assert!(read_config(&ini, &mut props).is_ok());

            let ini = Ini::load_from_str(&credential_text()).unwrap();
            assert!(read_credentials(&ini, &mut props).is_ok());

            let mut expected = HashMap::new();
            add_prop(
                &mut expected,
                &"default",
                vec![
                    ("aws_access_key_id", KEY_ID),
                    ("aws_secret_access_key", SECRET_KEY),
                    ("output", "yaml"),
                    ("region", "ap-northeast-1"),
                ],
            );
            assert_eq!(props, expected);
        }

        fn read_credential_error(str: &str) {
            let mut props = HashMap::new();
            let ini = Ini::load_from_str(str).unwrap();
            assert!(read_credentials(&ini, &mut props).is_err());
        }

        #[test]
        fn read_credential_empty_profile_header() {
            read_credential_error(
                r#"
[]
aws_access_key_id = AAAAAAAAA
aws_secret_access_key = aaaaaaaaaaaaaaaaaa
"#,
            );
        }
    }

    mod read_config {
        use super::*;

        #[test]
        fn read_config_ok() {
            let mut props = HashMap::new();
            let ini = Ini::load_from_str(&profile_text()).unwrap();
            assert!(read_config(&ini, &mut props).is_ok());
            let mut expected = HashMap::new();
            add_prop(
                &mut expected,
                "default",
                vec![("output", "yaml"), ("region", "ap-northeast-1")],
            );
            assert_eq!(props, expected);
        }

        fn read_config_error(str: &str) {
            let mut props = HashMap::new();
            let ini = Ini::load_from_str(str).unwrap();
            assert!(read_config(&ini, &mut props).is_err());
        }

        #[test]
        fn read_config_profile_name() {
            read_config_error(
                r#"
    [profile ]
    output = yaml
    region = ap-northeast-1
"#,
            );
        }

        #[test]
        fn read_config_empty_profile_header() {
            read_config_error(
                r#"
    [ ]
    output = yaml
    region = ap-northeast-1
"#,
            );
        }
    }
}
