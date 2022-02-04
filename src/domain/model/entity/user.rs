use crate::domain::model::value::{Token, UserId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: UserId,
    pub token: Token,
}

impl User {
    pub fn new(id: UserId, token: Token) -> Self {
        Self { id, token }
    }
}
