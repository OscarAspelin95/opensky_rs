use serde::Deserialize;

#[derive(Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
}
