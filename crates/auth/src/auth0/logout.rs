use super::client;

pub fn logout(client: client::Auth0Client, token_hint: String) -> Result<(), std::io::Error> {
    let endpoint = format! {"https://{}/oidc/logout?
    id_token_hint={}", client.domain, token_hint};

    let req_client = reqwest::blocking::Client::new();
    req_client.get(&endpoint).send().map_err(|err| {
        std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Error sending request: {}", err),
        )
    })?;

    Ok(())
}
