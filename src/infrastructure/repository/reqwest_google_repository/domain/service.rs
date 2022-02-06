use actix_web::{error, Result};
use async_trait::async_trait;
use reqwest::RequestBuilder;
use serde::de::DeserializeOwned;

use super::model::entity::Credential;
#[async_trait]
pub trait RequestAndParse {
    async fn request_and_parse<T: DeserializeOwned>(self) -> Result<T>;
}

#[async_trait]
impl RequestAndParse for RequestBuilder {
    async fn request_and_parse<T: DeserializeOwned>(self) -> Result<T> {
        let res = self.send().await.map_err(error::ErrorInternalServerError)?;
        let raw = res.text().await.map_err(error::ErrorInternalServerError)?;
        let parsed = serde_json::from_str::<T>(&raw).map_err(error::ErrorUnauthorized)?;
        Ok(parsed)
    }
}

#[async_trait]
pub trait WithCredential<T> {
    async fn with_credential(self, credential: &mut Credential) -> Result<T>;
}

#[async_trait]
impl WithCredential<RequestBuilder> for RequestBuilder {
    async fn with_credential(self, credential: &mut Credential) -> Result<Self> {
        credential.update().await?;
        Ok(self.bearer_auth(&credential.token.access_token))
    }
}
