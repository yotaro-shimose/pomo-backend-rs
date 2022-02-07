use super::IdHeader;
use crate::domain::model::entity::TaskList;
use crate::domain::{
    model::value::{AppState, UserId},
    repository::{DBRepository, GoogleRepository},
};
use crate::usecase::fetch_task_list_usecase;
use actix_web::{web, HttpResponse, Result};
use serde::Serialize;

#[derive(Debug, Serialize)]
struct FrontEndTaskList {
    id: String,
    summary: String,
}

impl From<TaskList> for FrontEndTaskList {
    fn from(task_list: TaskList) -> Self {
        Self {
            id: task_list.id,
            summary: task_list.name,
        }
    }
}

pub async fn fetch_task_list<G, U>(
    id_header: IdHeader,
    state: web::Data<AppState<G, U>>,
) -> Result<HttpResponse>
where
    G: GoogleRepository,
    U: DBRepository,
{
    let id = UserId::new(id_header.id);
    let google_repository = &state.google_repository;
    let db_repository = &state.db_repository;
    let response = fetch_task_list_usecase(&id, google_repository, db_repository)
        .await?
        .into_iter()
        .map(|val| val.into())
        .collect::<Vec<FrontEndTaskList>>();
    Ok(HttpResponse::Ok().json(response))
}
