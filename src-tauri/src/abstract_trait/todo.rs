use async_trait::async_trait;
use crate::models::TodoModel;
use crate::domain::TodoResponse;
use std::sync::Arc;
use sqlx::Error;


pub type DynTodoRepository = Arc<dyn TodoRepositoryTrait + Send + Sync>;
pub type DynTodoService = Arc<dyn TodoServiceTrait + Send + Sync>;


pub trait TodoRepositoryTrait {
    fn get_todos(&self) -> Result<Vec<TodoModel>, Error>;
    fn get_todo(&self, id: i32) -> Result<Option<TodoModel>, Error>;
    fn create_todo(&self, title: &str) -> Result<TodoModel, Error>;
    fn update_todo(
        &self,
        id: i32,
        title: &str
    ) -> Result<Option<TodoModel>, Error>;
    fn delete_todo(&self, id: i32) -> Result<(), Error>;
}

pub trait TodoServiceTrait {
    fn get_todos(&self) -> anyhow::Result<Vec<TodoResponse>>;
    fn get_todo(&self, id: i32) -> anyhow::Result<Option<TodoResponse>>;
    fn create_todo(&self, name: &str) -> anyhow::Result<TodoResponse>;
    fn update_todo(&self, id: i32, name: &str) -> anyhow::Result<Option<TodoResponse>>;
    fn delete_todo(&self, id: i32) -> anyhow::Result<()>;
}