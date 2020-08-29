mod client;
mod env;
mod error;
mod file;
mod opts;
mod profile;
use client::{assume_role, create_session};
use env::clear_environment_vars;
use env::list_environment_vars;
use error::Error::*;
use error::Result;
use file::aws_setting::read_aws_config;
use file::credentials::{delete_credentials, store_credentials};
use profile::show::show_current_profile;
use profile::Profile;
use structopt::StructOpt;

#[tokio::main]
async fn main() {
    match execute(opts::Opts::from_args()).await {
        Ok(_) => (),
        Err(err) => eprintln!("{}", err),
    }
}

async fn execute(opts: opts::Opts) -> Result<()> {
    let mut profile_map = read_aws_config()?;

    use opts::{ClearCommand, SubCommand};
    match opts.sub_command {
        SubCommand::Profile => Profile::print_table(&profile_map),
        SubCommand::Export { profile } => {
            profile_map.print_export(&profile).await?;
            store_credentials(&mut profile_map)?;
        }
        SubCommand::In {
            profile: profile_name,
            token,
        } => {
            let profile = profile_map.get_mut(&profile_name)?;
            use crate::profile::ProfileType::*;
            match profile.profile_type() {
                AssumeRole(_) => {
                    assume_role::send(&profile_name, &mut profile_map, Some(token.to_owned()))
                        .await?
                }
                SessionWithMFA => create_session::send(profile, &token).await?,
                _ => return Err(ProfileNotForSignIn(profile_name.to_owned())),
            }
            profile_map.print_export(&profile_name).await?;
            store_credentials(&mut profile_map)?;
        }
        SubCommand::Env => list_environment_vars(),
        SubCommand::Show => show_current_profile(&profile_map),
        SubCommand::Clear { command } => match command {
            ClearCommand::Session => {
                delete_credentials()?;
                println!("credentials file deleted.");
            }
            ClearCommand::Env => clear_environment_vars(),
        },
    }

    Ok(())
}
