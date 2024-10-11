use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct UserCredentials {
    pub access_token: String,
    pub refresh_token: Option<String>, // TODO: is required for device?
}
