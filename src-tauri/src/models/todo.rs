use serde::{Deserialize, Serialize};


#[derive(Debug, sqlx::FromRow, Deserialize, Serialize)]
pub struct TodoModel{
    pub id: i32,
    pub title: String,
}
