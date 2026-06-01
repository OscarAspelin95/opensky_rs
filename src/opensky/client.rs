use crate::opensky::{api::states::StateResponse, consts::OPENSKY_ROOT_URL, token::TokenResponse};
use std::path::Path;

use crate::opensky::consts::OPENSKY_AUTH_URL;
use crate::opensky::credentials::Credentials;
use anyhow::Result;

#[derive(Debug)]
pub struct Client {
    credentials: Credentials,
    url: &'static str,
    client: reqwest::Client,
}

/// General
impl Client {
    pub fn try_from_json(credentials_json: &Path) -> Result<Self> {
        let credentials = Credentials::try_from(credentials_json)?;
        let client = reqwest::Client::new();

        Ok(Self {
            credentials,
            url: OPENSKY_ROOT_URL,
            client: client,
        })
    }

    pub fn url(&self) -> &'static str {
        self.url
    }

    pub async fn fetch_access_token(&self) -> Result<String> {
        let params = [
            ("grant_type", "client_credentials"),
            ("client_id", &self.credentials.client_id),
            ("client_secret", &self.credentials.client_secret),
        ];

        let response = self
            .client
            .post(format!(
                "{}/auth/realms/opensky-network/protocol/openid-connect/token",
                OPENSKY_AUTH_URL
            ))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .form(&params)
            .send()
            .await?
            .json::<TokenResponse>()
            .await?;

        Ok(response.access_token)
    }
}

impl Client {
    pub async fn get_state(&self, lamin: f64, lamax: f64) -> Result<StateResponse> {
        let token = self.fetch_access_token().await?;

        let url = format!("{}/states/all?lamin={}&lamax={}", self.url(), lamin, lamax);

        let response = self.client.get(url).bearer_auth(token).send().await?;

        println!("{:?}", &response);

        let bytes = response.bytes().await?;

        let state_response: StateResponse = serde_json::from_slice(&bytes[..])?;

        Ok(state_response)
    }
}
