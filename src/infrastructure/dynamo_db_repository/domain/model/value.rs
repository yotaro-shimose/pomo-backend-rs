use aws_sdk_dynamodb::Client;

use crate::domain::model::{entity::User, value::UserId};

use super::interface::DynamoDBTable;

pub struct UserTable<'a> {
    client: &'a Client,
}

impl<'a> UserTable<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }
}

impl<'a> DynamoDBTable for UserTable<'a> {
    const TABLE_NAME: &'static str = "User";
    type Key = UserId;
    type Value = User;
    fn get_client(&self) -> &Client {
        self.client
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::model::value::Token;
    use actix_web::Result;
    use chrono::Utc;

    #[actix_rt::test]
    async fn test_user_table_crud() -> Result<()> {
        let conf = aws_config::load_from_env().await;
        let client = Client::new(&conf);
        let user_table = UserTable::new(&client);
        let user_id = UserId::new("user_id".to_string());
        let user = User::new(
            user_id.clone(),
            Token::new(
                "access_token".to_string(),
                "refresh_token".to_string(),
                Utc::now(),
            ),
            None,
        );
        let user2 = User::new(
            user_id.clone(),
            Token::new(
                "access_token".to_string(),
                "refresh_token".to_string(),
                Utc::now(),
            ),
            None,
        );
        user_table.create(&user_id, &user).await?;
        let retrieved_user = user_table.read(&user_id).await?;
        assert_eq!(Some(user), retrieved_user);
        user_table.update(&user_id, &user2).await?;
        let retrieved_user2 = user_table.read(&user_id).await?;
        assert_eq!(Some(user2), retrieved_user2);
        user_table.delete(&user_id).await?;
        let none_user = user_table.read(&user_id).await?;
        assert_eq!(None, none_user);
        Ok(())
    }
}
