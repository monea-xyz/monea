use crate::authentication::{check_auth_session, login};
use crate::utils::user_input;
use anyhow::Result;
use clap::Parser;

#[derive(Parser, Debug)]
pub struct BillingArgs {}

pub fn billing(_args: BillingArgs) -> Result<()> {
    match check_auth_session() {
        Ok(true) => {
            // User is authenticated, proceed with billing logic
            println!("Accessing billing information...");
        }
        Ok(false) => {
            println!("You need to be logged in to access billing information.");
            if user_input::yn_confirm("Would you like to log in now?")? {
                login()?;
            } else {
                println!("Billing access cancelled.");
                return Ok(());
            }
        }
        Err(e) => {
            eprintln!("Error checking authentication: {}", e);
            return Err(e.into());
        }
    }

    Ok(())
}
