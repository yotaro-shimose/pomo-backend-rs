use super::super::service::RequestAndParse;
use crate::domain::model::value::{ClientInfo, Token};
use actix_web::Result;
use chrono::{DateTime, Duration, Utc};
use reqwest;
use serde::Deserialize;
use serde::{Deserializer, Serialize};

#[derive(Serialize)]
struct RefreshForm {
    client_id: String,
    client_secret: String,
    refresh_token: String,
    grant_type: String,
}

impl RefreshForm {
    fn new(token_info: &Token, client_info: &ClientInfo) -> Self {
        Self {
            client_id: client_info.client_id.clone(),
            client_secret: client_info.client_secret.clone(),
            refresh_token: token_info.refresh_token.clone(),
            grant_type: "refresh_token".to_string(),
        }
    }
}

pub struct Credential {
    pub token: Token,
    pub client_info: ClientInfo,
}

impl Credential {
    pub fn new(token: &Token, client_info: &ClientInfo) -> Self {
        Self {
            token: token.clone(),
            client_info: client_info.clone(),
        }
    }

    pub async fn update(&mut self) -> Result<()> {
        if !self.token.is_valid() {
            let refresh_builder = reqwest::Client::new().post(&self.client_info.token_uri);
            let form = RefreshForm::new(&self.token, &self.client_info);
            let refresh_response = refresh_builder
                .form(&form)
                .request_and_parse::<RefreshResponse>()
                .await?;

            self.token = Token::new(
                &refresh_response.access_token,
                &self.token.refresh_token,
                refresh_response.expiry_date,
            )
        }
        Ok(())
    }
}

#[derive(Deserialize)]
struct RefreshResponse {
    access_token: String,
    #[serde(deserialize_with = "from_remainder")]
    expiry_date: DateTime<Utc>,
}

fn from_remainder<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let seconds: i64 = Deserialize::deserialize(deserializer)?;
    let now = Utc::now();
    Ok(now + Duration::seconds(seconds))
}
