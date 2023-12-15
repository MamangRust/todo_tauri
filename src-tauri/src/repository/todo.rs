
use crate::config::ConnectionPool;
use crate::models::TodoModel;
use sqlx::Error;

#[derive(Clone)]
pub struct TodoRepository{
    pub db_pool: ConnectionPool,
}

impl TodoRepository{
    pub fn new(db_pool: ConnectionPool) -> Self{
        Self { db_pool }
    }
}



impl TodoRepository {
    pub async fn get_todos(&self) -> Result<Vec<TodoModel>, Error> {
        let todos = sqlx::query_as::<_, TodoModel>("SELECT * FROM todo")
            .fetch_all(&self.db_pool)
            .await?;

        Ok(todos)
    }

    pub async fn get_todo(&self, id: i32) -> Result<Option<TodoModel>,Error> {
        let todo = sqlx::query_as::<_, TodoModel>("SELECT * FROM todo WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.db_pool)
            .await?;

        Ok(todo)
    }

    pub async fn create_todo(&self, title: &str) -> Result<TodoModel, Error> {
     

        let todo = sqlx::query_as::<_, TodoModel>(
            "INSERT INTO todo (id, title, created_at, updated_at) VALUES ($1, $2, $3, $4) RETURNING *",
        )
        
        .bind(title)
        .fetch_one(&self.db_pool)
        .await?;

        Ok(todo)
    }

    pub async fn update_todo(&self, id: i32, name: &str) -> Result<Option<TodoModel>, Error> {
        

        let todo = sqlx::query_as::<_, TodoModel>(
            "UPDATE todo SET title = $1 WHERE id = $3 RETURNING *",
        )
        .bind(name)
        .bind(id)
        .fetch_optional(&self.db_pool)
        .await?;

        Ok(todo)
    }

    pub async fn delete_todo(&self, id: i32) -> Result<(), Error> {
        sqlx::query!(
            "DELETE FROM todo WHERE id = $1",
            id,
        )
        .execute(&self.db_pool)
        .await?;

        Ok(())
    }
}
