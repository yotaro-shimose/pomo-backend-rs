use actix_web::{error, Result};
use serde::{de::DeserializeOwned, Serialize};
use sled::Db;
pub trait SledIO {
    const TABLE_NAME: &'static str;
    type SledKey: AsRef<[u8]>;
    type SledValue: Serialize + DeserializeOwned;
    fn get_db(&self) -> &Db;
    fn save(&self, key: &Self::SledKey, value: &Self::SledValue) -> Result<()> {
        let key = key.as_ref();
        let value = serde_json::to_string(value)?;
        let byte_key = value.as_bytes();
        let db = self.get_db();
        db.open_tree(Self::TABLE_NAME)
            .map_err(error::ErrorInternalServerError)?
            .insert(key, byte_key)
            .map_err(error::ErrorInternalServerError)?;
        Ok(())
    }

    fn fetch(&self, key: &Self::SledKey) -> Result<Option<Self::SledValue>> {
        let db = self.get_db();
        let byte_key = key.as_ref();
        let ret = db
            .open_tree(Self::TABLE_NAME)
            .map_err(error::ErrorInternalServerError)?
            .get(byte_key)
            .map_err(error::ErrorInternalServerError)?;
        match ret {
            Some(ivec) => {
                let string =
                    String::from_utf8(ivec.to_vec()).map_err(error::ErrorInternalServerError)?;
                let value = serde_json::from_str::<Self::SledValue>(&string)
                    .map_err(error::ErrorInternalServerError)?;
                Ok(Some(value))
            }
            None => Ok(None),
        }
    }
}
