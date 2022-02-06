use super::domain::{
    model::entity::Credential,
    service::{RequestAndParse, WithCredential},
};
use crate::domain::{
    model::value::{ClientInfo, Code, GmailAddress, Token},
    repository::GoogleRepository,
};
use actix_web::Result;
use async_trait::async_trait;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Deserializer, Serialize};

const REDIRECT_URI: &str = "postmessage";
const GOOGLE_EMAIL_ENDPOINT: &str = "https://people.googleapis.com/v1/people/me";

#[derive(Debug, Clone)]
pub struct ReqwestGoogleRepository {
    client_info: ClientInfo,
}

impl ReqwestGoogleRepository {
    pub fn new() -> Result<Self> {
        let client_info = ClientInfo::load()?;
        Ok(Self { client_info })
    }
}

#[async_trait]
impl GoogleRepository for ReqwestGoogleRepository {
    async fn fetch_token(&self, code: &Code) -> Result<Token> {
        let client = reqwest::Client::new();
        let params = TokenQuery::new(code, &self.client_info);
        let token_response: TokenResponse = client
            .post(&self.client_info.token_uri)
            .form(&params)
            .request_and_parse()
            .await?;
        Ok(token_response.into())
    }
    async fn fetch_gmail_address(&self, token: &Token) -> Result<GmailAddress> {
        let mut credential = Credential::new(token, &self.client_info);
        let query = EmailQuery::new();
        let builder = reqwest::Client::new()
            .get(GOOGLE_EMAIL_ENDPOINT)
            .query(&query)
            .with_credential(&mut credential)
            .await?;
        let response: EmailResponse = builder.request_and_parse().await?;
        let string_address = response.email_addresses.into_iter().next().unwrap().value;
        Ok(GmailAddress::new(&string_address))
    }
}

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

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct EmailQuery {
    person_fields: String,
}

impl EmailQuery {
    fn new() -> Self {
        Self {
            person_fields: "emailAddresses".to_string(),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct EmailResponse {
    email_addresses: Vec<EmailAddresses>,
}

#[derive(Debug, Deserialize)]
struct EmailAddresses {
    value: String,
}
