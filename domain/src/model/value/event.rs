use super::super::entity::Task;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Event {
    pub task: Task,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}
impl Event {
    pub fn new(task: Task, start: DateTime<Utc>, end: DateTime<Utc>) -> Self {
        Self { task, start, end }
    }
}
