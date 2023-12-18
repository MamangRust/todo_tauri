use async_trait::async_trait;
use anyhow::Result;
use std::sync::Arc;
use sqlx::Error;

use crate::models::Todo;

pub type DynTodoRepository = Arc<dyn TodoRepositoryTrait + Send + Sync>;
pub type DynTodoService = Arc<dyn TodoServiceTrait + Send + Sync>;


#[async_trait]
pub trait TodoRepositoryTrait{
    async fn create(&self,title: &str, complated: bool) -> Result<(), Error>;
    async fn get_all(&self) -> Result<Vec<Todo>, Error>;
    async fn update_completed(&self,id: i64, complated: bool) -> Result<(), Error>;
    async fn delete(&self,id: i64) -> Result<(), Error>;
}

#[async_trait]
pub trait TodoServiceTrait{
    async fn create_todo(&self,title: &str, completed: bool) -> Result<(), Error>;
    async fn get_all_todos(&self) -> Result<Vec<Todo>, Error>;
    async fn update_completed(&self,id: i64, completed: bool) -> Result<(), Error>;
    async fn delete_todo(&self,id: i64) -> Result<(), Error>;
}