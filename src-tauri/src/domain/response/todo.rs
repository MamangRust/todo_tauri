use serde::{Deserialize, Serialize};
use crate::models::TodoModel;


#[derive(Debug, Deserialize, Serialize)]
pub struct  TodoResponse{
    pub id: i32,
    pub title: String
}

impl From<TodoModel> for TodoResponse {
    fn from(todo: TodoModel) -> Self {
        TodoResponse {
            id: todo.id,
            title: todo.title,
        }
    }
}