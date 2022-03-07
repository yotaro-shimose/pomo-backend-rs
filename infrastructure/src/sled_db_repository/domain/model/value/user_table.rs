use super::super::interface::SledIO;
use domain::model::{entity::User, value::UserId};
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
