use sqlx::FromRow;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};


#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_at: Option<DateTime<Utc>>, 
    pub updated_at: Option<DateTime<Utc>>, 
}