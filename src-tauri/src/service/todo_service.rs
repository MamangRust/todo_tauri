use crate::abstract_trait::{DynTodoRepository, TodoServiceTrait};
use async_trait::async_trait;
use sqlx::Error;
use crate::models::Todo;
use anyhow::Result;



pub struct TodoService{
    repository: DynTodoRepository,
}


impl TodoService{
    pub fn new(repository: DynTodoRepository) -> Self{
        Self { repository }
    }
}

#[async_trait]
impl TodoServiceTrait for TodoService{
    async fn create_todo(&self, title: &str, completed: bool) ->Result<(), Error>{
        self.repository.create(title, completed).await
    }

    async fn get_all_todos(&self) -> Result<Vec<Todo>, Error>{
        self.repository.get_all().await
    }


    async fn update_completed(&self, id: i64, completed: bool) -> Result<(), Error>{
        self.repository.update_completed(id, completed).await
    }
    
    async fn delete_todo(&self,id: i64) -> Result<(), Error>{
        self.repository.delete(id).await
    } 
}
