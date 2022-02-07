use super::super::domain::{
    model::entity::Credential,
    service::{RequestAndParse, WithCredential},
};

use crate::domain::model::value::{ClientInfo, GmailAddress, Token};
use actix_web::Result;
use serde::{Deserialize, Serialize};

const GOOGLE_EMAIL_ENDPOINT: &str = "https://people.googleapis.com/v1/people/me";

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

pub async fn fetch_gmail_address_usecase(
    token: &Token,
    client_info: &ClientInfo,
) -> Result<GmailAddress> {
    let mut credential = Credential::new(token, client_info);
    let query = EmailQuery::new();
    let builder = reqwest::Client::new()
        .get(GOOGLE_EMAIL_ENDPOINT)
        .query(&query)
        .with_credential(&mut credential)
        .await?;
    let response: EmailResponse = builder.request_and_parse().await?;
    let string_address = response.email_addresses.into_iter().next().unwrap().value;
    Ok(GmailAddress::new(string_address))
}
