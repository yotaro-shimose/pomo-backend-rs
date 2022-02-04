use crate::domain::model::value::{Code, Token};
use actix_web::Result;
use async_trait::async_trait;

use super::model::{
    entity::user::User,
    value::{GmailAddress, UserId},
};

#[async_trait]
pub trait GoogleRepository: Send + Sync + Clone {
    async fn fetch_token(&self, code: &Code) -> Result<Token>;
    async fn fetch_gmail_address(&self, token: &Token) -> Result<GmailAddress>;
}

#[async_trait]
pub trait UserRepository: Send + Sync + Clone {
    fn fetch(&self, id: &UserId) -> Result<User>;
    fn save(&self, user: &User) -> Result<()>;
}
