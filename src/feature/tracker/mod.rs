use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

mod flatfile;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct StartTime(DateTime<Utc>);
impl StartTime {
    pub fn now() -> Self {
        StartTime(Utc::now())
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct EndTime(DateTime<Utc>);
impl EndTime {
    pub fn now() -> Self {
        EndTime(Utc::now())
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct TimeRecord {
start: StartTime,
end: EndTime,
}