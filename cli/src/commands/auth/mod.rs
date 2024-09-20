mod account;
mod login;
mod logout;

use clap::{Args, Subcommand};

#[derive(Args, Debug)]
pub struct AuthArgs {
    #[clap(subcommand)]
    pub command: AuthSubcommands,
}

#[derive(Subcommand, Debug)]
pub enum AuthSubcommands {
    Login(login::LoginArgs),
    Account(account::AccountArgs),
    Logout(logout::LogoutArgs),
}

pub fn auth(args: AuthArgs) -> Result<(), Box<dyn std::error::Error>> {
    match args.command {
        AuthSubcommands::Login(login_args) => login::login(login_args).map_err(Into::into),
        AuthSubcommands::Account(account_args) => {
            account::account(account_args).map_err(Into::into)
        }
        AuthSubcommands::Logout(logout_args) => logout::logout(logout_args).map_err(Into::into),
    }
}
