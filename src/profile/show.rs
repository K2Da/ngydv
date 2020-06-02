use crate::env;
use crate::profile::collection::ProfileMap;

pub fn show_current_profile(profile_map: &ProfileMap) {
    let env_vars = env::env_vars();

    println!("1. checking AWS_ACCESS_KEY_ID and AWS_ACCESS_KEY_ID.");
    if let (Some(access_key_id), Some(secret_access_key)) = (env_vars.aws_access_key_id, env_vars.aws_secret_access_key) {
        if let Some(profile) = profile_map.profile_by_key(&access_key_id, &secret_access_key) {
            println!("\nenvironment variables are set. current profile is {}.", profile.profile_name);
            println!("{}", profile.credential_str());
            return
        }
    } else {
        println!("  environment variables are not set.");
    }

    println!("2. checking AWS_PROFILE.");
    if let Some(profile_name) = env_vars.aws_profile {
        println!("\nAWS_PROFILE is set. current profile is {}.", profile_name);
        return
    } else {
        println!("  environment variable is not set.");
    }

    println!("3. checking profile named 'default'.");
    if let Ok(_) = profile_map.get("default") {
        println!("\nuse profile \"default\" as default.");
        return
    } else {
        println!("\n no profile with name \"default\".");
    }

    // none
    println!("\nno default profile.");
}