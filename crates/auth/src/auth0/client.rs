use crate::config;
#[derive(Clone)]
pub struct Auth0Client {
    pub domain: String,
    pub client_id: String,
}

impl Auth0Client {
    pub fn new() -> Self {
        Auth0Client {
            domain: config::AUTH0_DOMAIN.to_string(),
            client_id: config::AUTH0_CLIENT_ID.to_string(),
        }
    }
}
