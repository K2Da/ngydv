use prettytable::*;
use serde::Deserialize;

/// https://docs.aws.amazon.com/cli/latest/userguide/cli-configure-envvars.html
#[derive(Deserialize, Debug)]
pub struct EnvVariables {
    /// Specifies an AWS access key associated with an IAM user or role.
    /// If defined, this environment variable overrides the value for the profile setting aws_access_key_id. You can't specify the access key ID by using a command line option.
    pub aws_access_key_id: Option<String>,

    /// Specifies the path to a certificate bundle to use for HTTPS certificate validation.
    /// If defined, this environment variable overrides the value for the profile setting ca_bundle. You can override this environment variable by using the --ca-bundle command line parameter.
    pub aws_ca_bundle: Option<String>,

    /// Specifies the location of the file that the AWS CLI uses to store configuration profiles. The default path is ~/.aws/config).
    /// You can't specify this value in a named profile setting or by using a command line parameter.
    pub aws_config_file: Option<String>,

    /// Specifies the output format to use.
    /// If defined, this environment variable overrides the value for the profile setting output. You can override this environment variable by using the --output command line parameter.
    pub aws_default_output: Option<String>,

    /// Specifies the AWS Region to send the request to.
    /// If defined, this environment variable overrides the value for the profile setting region. You can override this environment variable by using the --region command line parameter.
    pub aws_default_region: Option<String>,

    /// Specifies the pager program used for output. By default, AWS CLI version 2 returns all output through your operating system’s default pager program.
    /// To disable all use of an external paging program, set the variable to an empty string.
    /// If defined, this environment variable overrides the value for the profile setting cli_pager.
    pub aws_pager: Option<String>,

    /// Specifies the name of the CLI profile with the credentials and options to use. This can be the name of a profile stored in a credentials or config file, or the value default to use the default profile.
    /// If defined, this environment variable overrides the behavior of using the profile named [default] in the configuration file. You can override this environment variable by using the --profile command line parameter.
    pub aws_profile: Option<String>,

    /// Specifies a name to associate with the role session. This value appears in CloudTrail logs for commands performed by the user of this profile.
    /// If defined, this environment variable overrides the value for the profile setting role_session_name. You can't specify a role session name as a command line parameter.
    pub aws_role_session_name: Option<String>,

    /// Specifies the secret key associated with the access key. This is essentially the "password" for the access key.
    /// If defined, this environment variable overrides the value for the profile setting aws_secret_access_key. You can't specify the access key ID as a command line option.
    pub aws_secret_access_key: Option<String>,

    /// Specifies the session token value that is required if you are using temporary security credentials that you retrieved directly from AWS STS operations. For more information, see the Output section of the assume-role command in the AWS CLI Command Reference.
    /// If defined, this environment variable overrides the value for the profile setting aws_session_token. You can't specify the session token as a command line option.
    pub aws_session_token: Option<String>,

    /// Specifies the location of the file that the AWS CLI uses to store access keys. The default path is ~/.aws/credentials).
    /// You can't specify this value in a named profile setting or by using a command line parameter.
    pub aws_shared_credentials_file: Option<String>,
}

pub fn env_vars() -> EnvVariables {
    let env;
    match envy::from_env::<EnvVariables>() {
        Ok(config) => {
            env = config;
        }
        Err(error) => panic!("{:#?}", error),
    }
    env
}

fn create_env_list() -> Vec<(Option<String>, Option<String>, String, String)> {
    let env= env_vars();
    vec![
        (
            env.aws_access_key_id.clone(),
            None,
            "aws_access_key_id".to_string(),
            "AWS access key associated with an IAM user or role".to_string(),
        ),
        (
            env.aws_ca_bundle.clone(),
            None,
            "aws_ca_bundle".to_string(),
            "The path to a certificate bundle to use for HTTPS certificate validation".to_string(),
        ),
        (
            env.aws_config_file.clone(),
            Some("~/.aws/config".to_string()),
            "aws_config_file".to_string(),
            "The location of the file that the AWS CLI uses to store configuration profiles".to_string(),
        ),
        (
            env.aws_default_output.clone(),
            Some("json".to_string()),
            "aws_default_output".to_string(),
            "The output format to use".to_string(),
        ),
        (
            env.aws_default_region.clone(),
            None,
            "aws_default_region".to_string(),
            "The AWS Region to send the request to".to_string(),
        ),
        (
            env.aws_pager.clone(),
            None,
            "aws_pager".to_string(),
            "The pager program used for output".to_string(),
        ),
        (
            env.aws_profile.clone(),
            Some("default".to_string()),
            "aws_profile".to_string(),
            "The name of the CLI profile with the credentials and options to use".to_string(),
        ),
        (
            env.aws_role_session_name.clone(),
            None,
            "aws_role_session_name".to_string(),
            "A name to associate with the role session".to_string(),
        ),
        (
            env.aws_secret_access_key.clone(),
            None,
            "aws_secret_access_key".to_string(),
            "The secret key associated with the access key".to_string(),
        ),
        (
            env.aws_session_token.clone(),
            None,
            "aws_session_token".to_string(),
            "The session token value that is required if you are using temporary security credentials".to_string(),
        ),
        (
            env.aws_shared_credentials_file.clone(),
            Some("~/.aws/credentials".to_string()),
            "aws_shared_credentials_file".to_string(),
            "The location of the file that the AWS CLI uses to store access keys".to_string(),
        ),
    ]
}

pub fn list_environment_vars() {
    let rows = create_env_list();
    let mut table = Table::new();
    table.set_titles(row!["name", "desc", "default", "value"]);
    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
    for row in rows {
        let (value, default, name, desc) = row;
        table.add_row(row![
            name.to_uppercase(),
            desc.to_owned(),
            match default {
                None => "".to_owned(),
                Some(v) => v.to_owned(),
            },
            match value {
                None => "".to_owned(),
                Some(v) => v.to_owned(),
            }
        ]);
    }
    table.printstd();
}

pub fn clear_environment_vars() {
    let rows = create_env_list();
    for row in rows {
        let (_, _, name, _) = row;
        println!("unset {}", name.to_uppercase());
    }
    println!("echo clear all aws cli related environment variables.");
}
