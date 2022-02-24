use domain::model::entity::Calendar;
use domain::model::value::{CalendarId, ClientInfo, Token};
use crate::reqwest_google_repository::domain::{
        model::entity::Credential,
        service::{RequestAndParse, WithCredential},
    };
use actix_web::Result;
use serde::{Deserialize, Serialize};

const GOOGLE_CALENDAR_ENDPOINT: &str =
    "https://www.googleapis.com/calendar/v3/users/me/calendarList";

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct CalendarQuery {
    max_results: u8,
    show_hidden: bool,
}

impl CalendarQuery {
    fn new() -> Self {
        Self {
            max_results: 100,
            show_hidden: true,
        }
    }
}

#[derive(Debug, Deserialize)]
struct CalendarResponse {
    items: Vec<RawCalendar>,
}

#[derive(Debug, Deserialize)]
struct RawCalendar {
    id: String,
    summary: String,
}

impl From<RawCalendar> for Calendar {
    fn from(raw: RawCalendar) -> Self {
        Self {
            id: CalendarId::new(raw.id),
            name: raw.summary,
        }
    }
}

pub async fn fetch_calendar_usecase(
    token: &Token,
    client_info: &ClientInfo,
) -> Result<Vec<Calendar>> {
    let mut credential = Credential::new(token, client_info);
    let query = CalendarQuery::new();
    let builder = reqwest::Client::new()
        .get(GOOGLE_CALENDAR_ENDPOINT)
        .query(&query)
        .with_credential(&mut credential)
        .await?;
    let response: CalendarResponse = builder.request_and_parse().await?;
    let task_lists = response
        .items
        .into_iter()
        .map(|raw| raw.into())
        .collect::<Vec<Calendar>>();
    Ok(task_lists)
}
