mod opts;
mod error;
use chuoku::prelude::*;
use chuoku::client::{assume_role, create_session};
use crate::error::Result;
use crate::error::Error::*;
use structopt::StructOpt;
use chuoku::profile::show::show_current_profile;

#[tokio::main]
async fn main() {
    match execute(opts::Opts::from_args()).await {
        Ok(_) => (),
        Err(err) => eprintln!("{}", err),
    }
}

async fn execute(opts: opts::Opts) -> Result<()> {
    let mut profile_map = read_aws_config()?;

    use opts::{SubCommand, ClearCommand};
    match opts.sub_command {
        SubCommand::Profile => Profile::print_table(&profile_map),
        SubCommand::Export { profile } => profile_map.print_export(&profile)?,
        SubCommand::In { profile: profile_name, token} => {
            let profile = profile_map.get_mut(&profile_name)?;
            use chuoku::profile::ProfileType::*;
            match profile.profile_type() {
                AssumeRole(_) => assume_role::send(profile, &token).await?,
                SessionWithMFA => create_session::send(profile, &token).await?,
                _ => return Err(ProfileNotForSignIn(profile_name.to_owned())),
            }
            profile_map.print_export(&profile_name)?;
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
