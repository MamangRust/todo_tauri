use sqlx::FromRow;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, FromRow,Serialize, Deserialize)]
pub struct Todo {
    id: Option<i64>,
    title: String,
    completed: bool,
    created_at: Option<DateTime<Utc>>, 
    updated_at: Option<DateTime<Utc>>, 
}