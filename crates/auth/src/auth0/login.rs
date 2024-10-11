use std::{thread::sleep, time::Duration};

use serde::{Deserialize, Serialize};
use spinners::{Spinner, Spinners};

use super::client::Auth0Client;

#[derive(Serialize, Debug)]
struct TokenPollRequestPayload {
    client_id: String,
    device_code: String,
    grant_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenPollResponse {
    pub access_token: Option<String>,
    pub token_type: Option<String>,
    pub refresh_token: Option<String>,
    pub expires_in: Option<usize>,
    pub scope: Option<String>,
}

fn activation_polling_loop(
    client: Auth0Client,
    poll_interval: Duration,
    poll_timeout: Duration,
    device_code: String,
) -> Result<TokenPollResponse, std::io::Error> {
    let endpoint = format! {"https://{}/oauth/token", client.domain};
    let payload = TokenPollRequestPayload {
        client_id: client.client_id.to_string(),
        device_code: device_code,
        grant_type: "urn:ietf:params:oauth:grant-type:device_code".to_string(),
    };

    let timeout_time = std::time::Instant::now() + poll_timeout;
    let mut next_time = std::time::Instant::now() + poll_interval;

    let mut loading_spinner = Spinner::new(Spinners::Dots9, "Waiting for browser...".into());
    loop {
        if std::time::Instant::now() > timeout_time {
            loading_spinner.stop();
            break Err(std::io::Error::new(
                std::io::ErrorKind::TimedOut,
                "Login timeout reached",
            ));
        }

        let req_client = reqwest::blocking::Client::new();
        let response_result = req_client
            .post(&endpoint)
            .json(&payload)
            .send()
            .map_err(|err| {
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Error sending request: {}", err),
                )
            })
            .and_then(|response| {
                response.json::<TokenPollResponse>().map_err(|err| {
                    std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("Error parsing response: {}", err),
                    )
                })
            });

        match response_result {
            Ok(response) => {
                if response.access_token.is_some() {
                    loading_spinner.stop();
                    return Ok(response);
                }
            }
            Err(err) => {
                loading_spinner.stop();
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Error polling for token: {}", err),
                ));
            }
        };

        sleep(next_time - std::time::Instant::now());
        next_time += poll_interval;
    }
}

#[derive(Serialize, Debug)]
struct LoginRequestPayload {
    client_id: String,
    scope: String,
}

#[derive(Deserialize, Debug)]
struct LoginResponsePayload {
    device_code: String,
    user_code: String,
    verification_uri: String,
    verification_uri_complete: String,
    expires_in: u32,
    interval: u32,
}

pub fn login(client: Auth0Client) -> Result<TokenPollResponse, std::io::Error> {
    let payload = LoginRequestPayload {
        client_id: client.client_id.to_string(),
        scope: "openid profile".to_string(),
    };

    let endpoint = format! {"https://{}/oauth/device/code", client.domain};

    let req_client = reqwest::blocking::Client::new();
    let response = req_client
        .post(&endpoint)
        .json(&payload)
        .send()
        .map_err(|err| {
            std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Error sending request: {}", err),
            )
        })?
        // TODO: check status code
        .json::<LoginResponsePayload>()
        .map_err(|err| {
            std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Error parsing response: {}", err),
            )
        })?;

    println!("response: {:?}", response);

    match open::that(response.verification_uri_complete.clone()) {
        Ok(_) => {
            println!("\n\nLogin URL opened in your default browser.\n\n");
        }
        Err(err) => {
            println!("\n\nError opening browser: {}", err);
            println!(
                "Please open the following URL in your browser: {}\n\n",
                response.verification_uri_complete
            );
        }
    };

    println!("\n\nConfirmation code: {}\n\n", response.user_code);

    let poll_interval = Duration::from_millis(response.interval.into());
    let poll_timeout = Duration::from_secs(response.expires_in.into());
    let device_code = response.device_code;

    match activation_polling_loop(client, poll_interval, poll_timeout, device_code) {
        Ok(token_response) => {
            println!("Token response: {:?}", token_response);
            println!("\n\nYou're logged in!");
            return Ok(token_response);
        }
        Err(err) => {
            return Err(err);
        }
    };
}
