use anyhow::Result;
use jsonwebtoken::{encode, EncodingKey, Header};
use reqwest;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::authentication::{get_session_path, prompt_input, prompt_password, Claims};

pub fn login_with_email_password() -> Result<()> {
    let email = prompt_input("Enter your email: ")?;
    let password = prompt_password("Enter your password (this will be logged for debugging): ")?;

    println!("Logging in with email: {}", email);
    println!("Password: {}", password);

    // TODO send login request to api
    // let client = reqwest::blocking::Client::new();
    // let response = client
    //     .post("https://api.monea.xyz/auth/login")
    //     .json(&serde_json::json!({
    //         "email": email,
    //         "password": password
    //     }))
    //     .send()?;

    // if !response.status().is_success() {
    //     return Err(anyhow::anyhow!("login failed: {}", response.status()));
    // }

    // let jwt: String = response.json()?;
    // println!("login successful");

    // For now, we'll just create a dummy session
    let claims = Claims {
        exp: SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as usize + 900, // 15 minutes
    };

    let key = b"secret"; // TODO use a proper secret key
    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(key))?;

    let session_path = get_session_path();
    fs::create_dir_all(session_path.parent().unwrap())?;
    fs::write(session_path, token)?;

    Ok(())
}
