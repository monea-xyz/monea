mod account;

use clap::{Args, Subcommand};
use monea_auth;

#[derive(Args, Debug)]
pub struct AuthArgs {
    #[clap(subcommand)]
    pub command: AuthSubcommands,
}

#[derive(Subcommand, Debug)]
pub enum AuthSubcommands {
    Login,
    Account(account::AccountArgs),
    Logout,
}

pub fn auth(args: AuthArgs) -> Result<(), Box<dyn std::error::Error>> {
    match args.command {
        AuthSubcommands::Login => monea_auth::commands::login().map_err(Into::into),
        AuthSubcommands::Account(account_args) => {
            account::account(account_args).map_err(Into::into)
        }
        AuthSubcommands::Logout => monea_auth::commands::logout().map_err(Into::into),
    }
}
