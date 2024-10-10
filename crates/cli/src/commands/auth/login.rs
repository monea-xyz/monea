use crate::authentication::session;
use anyhow::Result;
use clap::Args;

#[derive(Args, Debug)]
pub struct LoginArgs {}

pub fn login(_args: LoginArgs) -> Result<()> {
    println!("Logging in...");
    session::login()?;
    println!("Login successful!");
    Ok(())
}
