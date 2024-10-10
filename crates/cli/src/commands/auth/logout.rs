use crate::authentication::session;
use anyhow::Result;
use clap::Args;

#[derive(Args, Debug)]
pub struct LogoutArgs {}

pub fn logout(_args: LogoutArgs) -> Result<()> {
    println!("Logging out...");
    session::logout()?;
    println!("Logout successful!");
    Ok(())
}
