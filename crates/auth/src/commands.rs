use crate::{
    auth0,
    credentials_store::{client::CredentialsStoreClient, model::UserCredentials},
};

pub fn login() -> Result<(), std::io::Error> {
    // TODO: check if already logged in
    let client = auth0::client::Auth0Client::new();
    let response = match auth0::login::login(client) {
        Ok(response) => response,
        Err(e) => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Error logging in: {}", e),
            ));
        }
    };

    let user_credentials = UserCredentials {
        access_token: response.access_token.ok_or(std::io::Error::new(
            std::io::ErrorKind::Other,
            "No access token found",
        ))?,
        refresh_token: response.refresh_token,
    };

    let credentials_client = CredentialsStoreClient::new()?;
    match credentials_client.set_credentials(user_credentials) {
        Ok(_) => {}
        Err(e) => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Error setting credentials: {}", e),
            ));
        }
    }

    Ok(())
}

pub fn logout() -> Result<(), std::io::Error> {
    // TODO: check if already logged out
    let credentials_client = CredentialsStoreClient::new()?;
    credentials_client.set_credentials(UserCredentials::default())?;

    println!("\n\nLogged out!\n\n");

    Ok(())
}

pub fn whoami() -> Result<(), std::io::Error> {
    unimplemented!()
}
