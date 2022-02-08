use crate::{
    domain::model::value::{CalendarId, ClientInfo, Event, Token},
    infrastructure::reqwest_google_repository::domain::{
        model::entity::Credential,
        service::{RequestAndParse, WithCredential},
    },
};
use actix_web::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct PushEventBody {
    summary: String,
    start: Time,
    end: Time,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Time {
    date_time: DateTime<Utc>,
}
impl Time {
    fn new(date_time: DateTime<Utc>) -> Self {
        Self { date_time }
    }
}

impl From<Event> for PushEventBody {
    fn from(event: Event) -> Self {
        let title = event.task.name;
        let start = Time::new(event.start);
        let end = Time::new(event.end);
        Self {
            summary: title,
            start,
            end,
        }
    }
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct PushEventResponse {
    id: String,
    start: Time,
    end: Time,
}

pub async fn push_event_usecase(
    event: Event,
    token: &Token,
    calendar_id: &CalendarId,
    client_info: &ClientInfo,
) -> Result<()> {
    let mut credential = Credential::new(token, client_info);
    let endpoint = format!(
        "https://www.googleapis.com/calendar/v3/calendars/{}/events",
        calendar_id.value
    );
    let body = PushEventBody::from(event);
    let builder = reqwest::Client::new()
        .post(endpoint)
        .json(&body)
        .with_credential(&mut credential)
        .await?;
    builder.request_and_parse::<PushEventResponse>().await?;
    Ok(())
}
