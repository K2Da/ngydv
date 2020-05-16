use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "opts")]
pub struct Opts {
    #[structopt(subcommand)]
    pub sub_command: SubCommand,
}

#[derive(StructOpt, Debug)]
pub enum SubCommand {
    /// Lists profiles and current sessions.
    #[structopt(name = "profile")]
    Profile,

    /// Lists current environment variables related AWS CLI.
    #[structopt(name = "env")]
    Env,

    /// Shows current default session.
    #[structopt(name = "show")]
    Show,

    /// Creates session or assume role based on provided profile type.
    #[structopt(name = "in")]
    In {
        /// profile name.
        profile: String,
        /// mfa-device token. 6 digits.
        token: String,
    },

    /// Prints shell script to export environment variables for created session.
    #[structopt(name = "export")]
    Export {
        /// profile name.
        profile: String,
    },

    /// Clears session or environment variables for AWS CLI.
    #[structopt(name = "clear")]
    Clear {
       #[structopt(subcommand)]
       command: ClearCommand,
    }
}

#[derive(StructOpt, Debug)]
pub enum ClearCommand {
    /// Removes file storing session tokens created by this command.
    #[structopt(name = "session")]
    Session,

    /// Clears environment variables related AWS CLI.
    #[structopt(name = "env")]
    Env,
}
