use crate::model::value::{Token, UserConfig, UserId};
use actix_web::{error, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct User {
    pub id: UserId,
    pub token: Token,
    pub user_config: Option<UserConfig>,
}

impl User {
    pub fn new(id: UserId, token: Token, user_config: Option<UserConfig>) -> Self {
        Self {
            id,
            token,
            user_config,
        }
    }

    pub fn try_get_user_config(&self) -> Result<UserConfig> {
        self.user_config.clone().ok_or_else(|| {
            error::ErrorNotFound(format!(
                "This User (Id: {}) Has Not Configured His Or Her Config",
                self.id
            ))
        })
    }

    pub fn update_token(&mut self, token: Token) {
        self.token = token;
    }

    pub fn update_config(&mut self, user_config: UserConfig) {
        self.user_config = Some(user_config);
    }
}
