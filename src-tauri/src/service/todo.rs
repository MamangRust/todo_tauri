
use anyhow::Result;

use crate::{
    repository::TodoRepository,
    domain::TodoResponse
};

#[derive(Clone)]
pub struct TodoService{
    repository: TodoRepository
}

impl TodoService{
    pub fn new(repository: TodoRepository) -> Self{
        Self { repository }
    }
}




impl TodoService {
    pub async fn get_todos(&self) -> Result<Vec<TodoResponse>> {
        let todos_data = self.repository.get_todos().await?;
        let todo_responses: Vec<TodoResponse> = todos_data.into_iter().map(|todo| todo.into()).collect();
        Ok(todo_responses)
    }

    pub async fn get_todo(&self, id: i32) -> Result<Option<TodoResponse>> {
        let todo_data = self.repository.get_todo(id).await?;
        match todo_data {
            Some(todo) => Ok(Some(todo.into())),
            None => Ok(None),
        }
    }

    pub async fn create_todo(&self, name: &str) -> Result<TodoResponse> {
        let new_todo_data = self.repository.create_todo(name).await?;
        Ok(new_todo_data.into())
    }

    pub async fn update_todo(&self, id: i32, name: &str) -> Result<Option<TodoResponse>> {
        let updated_todo_data = self.repository.update_todo(id, name).await?;
        match updated_todo_data {
            Some(updated_todo) => Ok(Some(updated_todo.into())),
            None => Ok(None),
        }
    }

    pub async fn delete_todo(&self, id: i32) -> Result<()> {
        self.repository.delete_todo(id).await?;
        Ok(())
    }
}
