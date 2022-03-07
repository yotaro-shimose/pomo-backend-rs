use chrono::{DateTime, TimeZone, Utc};
use chrono_tz::Asia::Tokyo;
use domain::model::{
    entity::{Calendar, Task, TaskList},
    value::{CalendarId, Code, Event, TaskListId, UserConfig, UserId},
};
use serde::{Deserialize, Deserializer, Serialize};
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginRequest {
    pub authorization_code: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    id: String,
}

impl LoginResponse {
    pub fn new(user_id: UserId) -> Self {
        Self { id: user_id.value }
    }
}

impl From<LoginRequest> for Code {
    fn from(req: LoginRequest) -> Code {
        Code::new(req.authorization_code)
    }
}

#[derive(Debug, Serialize)]
pub struct FrontEndCalendar {
    id: String,
    summary: String,
}

impl From<Calendar> for FrontEndCalendar {
    fn from(calendar: Calendar) -> Self {
        Self {
            id: calendar.id.value,
            summary: calendar.name,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct FrontEndTaskList {
    id: String,
    summary: String,
}

impl From<TaskList> for FrontEndTaskList {
    fn from(task_list: TaskList) -> Self {
        Self {
            id: task_list.id.value,
            summary: task_list.name,
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FrontEndUserConfig {
    task_list_id: Option<String>,
    calendar_id: Option<String>,
}

impl From<FrontEndUserConfig> for Option<UserConfig> {
    fn from(raw: FrontEndUserConfig) -> Self {
        if let (Some(task_list_id), Some(calendar_id)) = (raw.task_list_id, raw.calendar_id) {
            let task_list_id = TaskListId::new(task_list_id);
            let calendar_id = CalendarId::new(calendar_id);
            let user_config = UserConfig::new(task_list_id, calendar_id);
            Some(user_config)
        } else {
            None
        }
    }
}

impl From<Option<UserConfig>> for FrontEndUserConfig {
    fn from(user_config: Option<UserConfig>) -> Self {
        match user_config {
            Some(user_config) => Self {
                task_list_id: Some(user_config.task_list_id.value),
                calendar_id: Some(user_config.calendar_id.value),
            },
            None => Self {
                task_list_id: None,
                calendar_id: None,
            },
        }
    }
}

#[derive(Debug, Serialize)]
pub struct SuccessResponse {}

impl SuccessResponse {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FrontEndEvent {
    task: Task,
    #[serde(deserialize_with = "format_frontend_datetime")]
    start_time: DateTime<Utc>,
    #[serde(deserialize_with = "format_frontend_datetime")]
    end_time: DateTime<Utc>,
}

fn format_frontend_datetime<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let string_time: String = Deserialize::deserialize(deserializer)?;
    let local = Tokyo
        .datetime_from_str(&string_time, "%Y-%m-%d %H:%M:%S")
        .map_err(serde::de::Error::custom)?;
    Ok(local.with_timezone(&Utc))
}

impl From<FrontEndEvent> for Event {
    fn from(raw: FrontEndEvent) -> Self {
        Self {
            task: raw.task,
            start: raw.start_time,
            end: raw.end_time,
        }
    }
}
