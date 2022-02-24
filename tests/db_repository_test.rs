use actix_web::Result;
use backend::{
    domain::{
        model::{
            entity::User,
            value::{CalendarId, TaskListId, Token, UserConfig, UserId},
        },
        repository::DBRepository,
    },
    infrastructure::{
        dynamo_db_repository::DynamoDBRepository, sled_db_repository::SledDBRepository,
    },
};
use chrono::Utc;
use rstest::*;
use std::future::Future;

#[fixture]
async fn sleddb_repository() -> SledDBRepository {
    SledDBRepository::new().unwrap()
}

#[fixture]
async fn dynamodb_repository() -> DynamoDBRepository {
    DynamoDBRepository::new().await
}

fn get_user(id_noise: &str, value_noise: &str) -> User {
    let user_id = UserId::new(format!("user_id{}", id_noise));
    let token = Token::new(
        format!("access_token{}", value_noise),
        format!("refresh_token{}", value_noise),
        Utc::now(),
    );
    let config = UserConfig::new(
        TaskListId::new(format!("task_list_id{}", value_noise)),
        CalendarId::new(format!("calendar_id{}", value_noise)),
    );
    User::new(user_id, token, Some(config))
}

#[rstest]
#[case(sleddb_repository())]
#[case(dynamodb_repository())]
#[actix_rt::test]
pub async fn crud<D: DBRepository>(#[case] db_repository: impl Future<Output = D>) -> Result<()> {
    let origin_user = get_user("crud", "1");
    let user_id = &origin_user.id;
    let db_repository = db_repository.await;
    db_repository.save_user(&origin_user).await?;
    let retrieved_user = db_repository.retrieve_user(user_id).await?;
    assert_eq!(origin_user, retrieved_user);
    let modified_user = get_user("crud", "2");
    db_repository.save_user(&modified_user).await?;
    let retrieved_modified_user = db_repository.retrieve_user(user_id).await?;
    assert_eq!(modified_user, retrieved_modified_user);
    db_repository.delete_user(user_id).await?;
    let null_user = db_repository.fetch_user(user_id).await?;
    assert!(null_user.is_none());
    Ok(())
}

#[rstest]
#[case(sleddb_repository())]
#[case(dynamodb_repository())]
#[actix_rt::test]
pub async fn error_when_delete_non_existing_user<D: DBRepository>(
    #[case] db_repository: impl Future<Output = D>,
) -> Result<()> {
    let none_user = get_user("none", "");
    let user_id = &none_user.id;
    let db_repository = db_repository.await;
    let retrieved_user = db_repository.fetch_user(user_id).await?;
    assert!(retrieved_user.is_none());
    let ret = db_repository.delete_user(user_id).await;
    assert!(ret.is_err());
    Ok(())
}
