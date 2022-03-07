use actix_web::{error, Result};
use async_trait::async_trait;
use domain::model::{entity::User, value::UserId};
use domain::repository::DBRepository;
use sled::Db;

use super::domain::model::interface::SledIO;
use super::domain::model::value::UserTable;

const DB_PATH: &str = "sleddb";

#[derive(Debug, Clone)]
pub struct SledDBRepository {
    db: Db,
}

impl SledDBRepository {
    pub fn new() -> Result<Self> {
        let db = sled::open(DB_PATH).map_err(error::ErrorInternalServerError)?;
        Ok(Self { db })
    }
}

#[async_trait]
impl DBRepository for SledDBRepository {
    async fn fetch_user(&self, id: &UserId) -> Result<Option<User>> {
        let user_table = UserTable::new(&self.db);
        user_table.fetch(id)
    }
    async fn save_user(&self, user: &User) -> Result<()> {
        let user_table = UserTable::new(&self.db);
        let id = &user.id;
        user_table.save(id, user)
    }
    async fn delete_user(&self, id: &UserId) -> Result<()> {
        let user_table = UserTable::new(&self.db);
        user_table.delete(id)
    }
}
