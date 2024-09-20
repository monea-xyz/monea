use anyhow::Result;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

use super::{login_with_email_password, login_with_oauth, prompt_login_method};

const SESSION_FILE: &str = ".monea/session.txt";

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize,
}

pub fn get_session_path() -> PathBuf {
    let home = dirs::home_dir().expect("Unable to get home directory");
    home.join(SESSION_FILE)
}

pub fn check_auth_session() -> Result<bool> {
    let session_path = get_session_path();
    if !session_path.exists() {
        return Ok(false);
    }

    let token = fs::read_to_string(session_path)?;
    let validation = Validation::default();
    let key = b"secret"; // TODO use a proper secret key

    match decode::<Claims>(&token, &DecodingKey::from_secret(key), &validation) {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}

pub fn login() -> Result<()> {
    let login_methods = vec!["Email/Password", "Google", "GitHub"];
    let selected_method = prompt_login_method(&login_methods)?;

    match selected_method {
        "Email/Password" => login_with_email_password(),
        "Google" => login_with_oauth("Google"),
        "GitHub" => login_with_oauth("GitHub"),
        _ => Err(anyhow::anyhow!("Invalid login method")),
    }
}

pub fn logout() -> Result<()> {
    let session_path = get_session_path();
    if session_path.exists() {
        fs::remove_file(session_path)?;
    }
    Ok(())
}
