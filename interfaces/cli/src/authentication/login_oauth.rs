use anyhow::Result;
use jsonwebtoken::{encode, EncodingKey, Header};
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};
use webbrowser;

use crate::authentication::{get_session_path, Claims};

pub fn login_with_oauth(provider: &str) -> Result<()> {
    println!("Redirecting to {} login...", provider);
    webbrowser::open("http://monea.xyz")?;

    // TODO: Implement actual OAuth flow
    println!("Please complete the login process in your browser.");
    println!("Press Enter when you've completed the login.");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    // For now, we'll just create a dummy session
    let claims = Claims {
        exp: SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as usize + 900, // 15 minutes
    };

    let key = b"secret"; // TODO use a proper secret key
    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(key))?;

    let session_path = get_session_path();
    fs::create_dir_all(session_path.parent().unwrap())?;
    fs::write(session_path, token)?;

    println!("Login successful!");
    Ok(())
}
