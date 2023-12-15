use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct  TodoRequest{
    pub id: i32,
    pub name: String
}