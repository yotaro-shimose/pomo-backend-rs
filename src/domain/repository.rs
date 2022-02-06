use super::model::{
    entity::User,
    value::{GmailAddress, UserId},
};
use crate::domain::model::value::{Code, Token};
use actix_web::Result;
use async_trait::async_trait;

#[async_trait]
pub trait GoogleRepository: Send + Sync + Clone {
    async fn fetch_token(&self, code: &Code) -> Result<Token>;
    async fn fetch_gmail_address(&self, token: &Token) -> Result<GmailAddress>;
}

pub trait DBRepository: Send + Sync + Clone {
    fn fetch_user(&self, id: &UserId) -> Result<Option<User>>;
    fn save_user(&self, user: &User) -> Result<()>;
}
