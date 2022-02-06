use super::super::domain::service::RequestAndParse;

use crate::domain::model::value::{ClientInfo, Code, Token};
use actix_web::Result;
use chrono::{DateTime, Duration, Utc};
use serde::{de::Deserializer, Deserialize, Serialize};

const REDIRECT_URI: &str = "postmessage";

#[derive(Debug, Serialize)]
struct TokenQuery {
    client_id: String,
    client_secret: String,
    redirect_uri: String,
    code: String,
    grant_type: String,
}

impl TokenQuery {
    fn new(code: &Code, client_info: &ClientInfo) -> Self {
        Self {
            client_id: client_info.client_id.clone(),
            client_secret: client_info.client_secret.clone(),
            redirect_uri: REDIRECT_URI.to_string(),
            code: code.value.to_string(),
            grant_type: "authorization_code".to_string(),
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]
struct TokenResponse {
    pub access_token: String,
    pub refresh_token: String,
    #[serde(deserialize_with = "from_remainder")]
    pub expires_in: DateTime<Utc>,
}

impl From<TokenResponse> for Token {
    fn from(info: TokenResponse) -> Self {
        Self {
            access_token: info.access_token,
            refresh_token: info.refresh_token,
            expiry_date: info.expires_in,
        }
    }
}

fn from_remainder<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let seconds: i64 = Deserialize::deserialize(deserializer)?;
    let now = Utc::now();
    Ok(now + Duration::seconds(seconds))
}

pub async fn fetch_token_usecase(code: &Code, client_info: &ClientInfo) -> Result<Token> {
    let client = reqwest::Client::new();
    let params = TokenQuery::new(code, client_info);
    let token_response: TokenResponse = client
        .post(&client_info.token_uri)
        .form(&params)
        .request_and_parse()
        .await?;
    Ok(token_response.into())
}
