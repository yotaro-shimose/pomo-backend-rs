use super::super::interface::SledIO;
use crate::domain::model::{entity::User, value::UserId};
use sled::Db;
pub struct UserTable<'a> {
    db: &'a Db,
}

impl<'a> UserTable<'a> {
    pub fn new(db: &'a Db) -> Self {
        Self { db }
    }
}

impl<'a> SledIO for UserTable<'a> {
    const TABLE_NAME: &'static str = "User";
    type SledKey = UserId;
    type SledValue = User;
    fn get_db(&self) -> &Db {
        self.db
    }
}

impl AsRef<[u8]> for UserId {
    fn as_ref(&self) -> &[u8] {
        self.value.as_ref()
    }
}
