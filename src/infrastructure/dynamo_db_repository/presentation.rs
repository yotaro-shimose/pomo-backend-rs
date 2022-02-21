use super::domain::model::interface::DynamoDBTable;
use super::domain::model::value::UserTable;
use crate::domain::model::{entity::User, value::UserId};
use crate::domain::repository::DBRepository;
use actix_web::Result;
use async_trait::async_trait;
use aws_sdk_dynamodb as dynamodb;

#[derive(Clone)]
pub struct DynamoDBRepository {
    client: dynamodb::Client,
}

impl DynamoDBRepository {
    pub async fn new() -> Self {
        let config = aws_config::load_from_env().await;
        let client = dynamodb::Client::new(&config);
        Self { client }
    }
}

#[async_trait]
impl DBRepository for DynamoDBRepository {
    async fn fetch_user(&self, id: &UserId) -> Result<Option<User>> {
        let table = UserTable::new(&self.client);
        table.read(id).await
    }
    async fn save_user(&self, user: &User) -> Result<()> {
        let table = UserTable::new(&self.client);
        let user_id = &user.id;
        table.create(user_id, user).await
    }
}
