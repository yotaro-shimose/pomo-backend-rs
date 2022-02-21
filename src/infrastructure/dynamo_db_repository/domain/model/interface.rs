use actix_web::{error, Result};
use async_trait::async_trait;
use aws_sdk_dynamodb::model::{AttributeAction, AttributeValue, AttributeValueUpdate};
use aws_sdk_dynamodb::Client;
use serde::de::DeserializeOwned;
use serde::Serialize;

const KEY: &str = "key";
const VALUE: &str = "value";

#[async_trait]
pub trait DynamoDBTable {
    const TABLE_NAME: &'static str;
    type Key: ToString + Sync + Send;
    type Value: Serialize + DeserializeOwned + Sync + Send;

    fn get_client(&self) -> &Client;

    async fn create(&self, key: &Self::Key, value: &Self::Value) -> Result<()> {
        let client = self.get_client();
        let dynamo_key = key.to_string();
        let dynamo_value = serde_json::to_string(value)?;
        client
            .put_item()
            .table_name(Self::TABLE_NAME)
            .item(KEY, AttributeValue::S(dynamo_key))
            .item(VALUE, AttributeValue::S(dynamo_value))
            .send()
            .await
            .map_err(error::ErrorInternalServerError)?;
        Ok(())
    }

    async fn read(&self, key: &Self::Key) -> Result<Option<Self::Value>> {
        let client = self.get_client();
        let dynamo_key = key.to_string();
        let res = client
            .get_item()
            .table_name(Self::TABLE_NAME)
            .key(KEY, AttributeValue::S(dynamo_key))
            .send()
            .await
            .map_err(error::ErrorInternalServerError)?;

        match res.item {
            None => Ok(None),
            Some(map) => {
                let raw_string = map
                    .get(VALUE)
                    .ok_or_else(|| error::ErrorInternalServerError("No such key in this table"))?
                    .as_s()
                    .map_err(|_| error::ErrorInternalServerError("Could not parse"))?;
                Ok(Some(serde_json::from_str::<Self::Value>(raw_string)?))
            }
        }
    }

    async fn update(&self, key: &Self::Key, value: &Self::Value) -> Result<()> {
        let dynamo_value = serde_json::to_string(value)?;
        let client = self.get_client();
        let dynamo_key = key.to_string();
        let attr_val_up = AttributeValueUpdate::builder()
            .action(AttributeAction::Put)
            .value(AttributeValue::S(dynamo_value))
            .build();

        client
            .update_item()
            .table_name(Self::TABLE_NAME)
            .key(KEY, AttributeValue::S(dynamo_key))
            .attribute_updates(VALUE, attr_val_up)
            .send()
            .await
            .map_err(error::ErrorInternalServerError)?;
        Ok(())
    }

    async fn delete(&self, key: &Self::Key) -> Result<()> {
        let client = self.get_client();
        let dynamo_key = key.to_string();
        client
            .delete_item()
            .table_name(Self::TABLE_NAME)
            .key(KEY, AttributeValue::S(dynamo_key))
            .send()
            .await
            .map_err(error::ErrorInternalServerError)?;
        Ok(())
    }
}
