use crate::{config::ConnectionPool, abstract_trait::TodoRepositoryTrait};
use async_trait::async_trait;
use chrono::Utc;
use sqlx::Error;
use anyhow::Result;
use crate::models::Todo;


#[derive(Clone)]
pub struct TodoRepository{
    pub db_pool: ConnectionPool,
}

impl TodoRepository{
    pub fn new(db_pool: ConnectionPool) -> Self{
        Self { db_pool }
    }
}


#[async_trait]
impl TodoRepositoryTrait for TodoRepository{
    async fn create(&self, title: &str, completed: bool) -> Result<(), Error> {
        let created_at = Utc::now().to_rfc3339();
        let updated_at = Utc::now().to_rfc3339();
    
        sqlx::query("INSERT INTO todo (title, completed, created_at, updated_at) VALUES (?, ?, ?, ?)")
            .bind(title)
            .bind(completed)
            .bind(created_at)
            .bind(updated_at)
            .execute(&self.db_pool)
            .await?;
        Ok(())
    }

    async fn get_all(&self) -> Result<Vec<Todo>, Error> {
        let todos = sqlx::query_as::<_, Todo>("SELECT * FROM todo")
            .fetch_all(&self.db_pool)
            .await?;
        Ok(todos)
    }

    async fn update_completed(&self, id: i64, completed: bool) -> Result<(), Error> {
        let updated_at = Utc::now().to_rfc3339();

        sqlx::query("UPDATE todo SET completed = ?, updated_at = ? WHERE id = ?")
            .bind(completed)
            .bind(updated_at)
            .bind(id)
            .execute(&self.db_pool)
            .await?;
        Ok(())
    }

    async fn delete(&self, id: i64) -> Result<(), Error> {
        sqlx::query("DELETE FROM todo WHERE id = ?")
            .bind(id)
            .execute(&self.db_pool)
            .await?;
        Ok(())
    }
}


// impl TodoRepository {
//     pub async fn create(&self,title: &str, completed: bool) -> Result<(), Error> {
//         sqlx::query(
//             "INSERT INTO todo (title, completed) VALUES (?, ?)",
//         )
//         .bind(title)
//         .bind(completed)
//         .execute(&self.db_pool)
//         .await?;
//         Ok(())
//     }

//     pub async fn get_all(&self) -> Result<Vec<Todo>, Error> {
//         let todos = sqlx::query_as::<_, Todo>("SELECT id, title, completed FROM todo")
//             .fetch_all(&self.db_pool)
//             .await?;
//         Ok(todos)
//     }

//     pub async fn update_completed(
//         &self,
//         id: i64,
//         completed: bool,
//     ) -> Result<(), Error> {
//         sqlx::query("UPDATE todo SET completed = ? WHERE id = ?")
//             .bind(completed)
//             .bind(id)
//             .execute(&self.db_pool)
//             .await?;
//         Ok(())
//     }

//     pub async fn delete(&self,id: i64) -> Result<(), Error> {
//         sqlx::query("DELETE FROM todo WHERE id = ?")
//             .bind(id)
//             .execute(&self.db_pool)
//             .await?;
//         Ok(())
//     }
// }
