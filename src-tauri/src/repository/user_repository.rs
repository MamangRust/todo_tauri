use crate::{config::ConnectionPool, abstract_trait::UserRepositoryTrait};
use async_trait::async_trait;
use chrono::Utc;
use sqlx::Error;
use anyhow::Result;
use crate::models::User;

#[derive(Clone)]
pub struct UserRepository{
    pub db_pool: ConnectionPool,
}

impl UserRepository{
    pub fn new(db_pool: ConnectionPool) -> Self{
        Self { db_pool }
    }
}

#[async_trait]
impl UserRepositoryTrait for UserRepository{
    async fn find_by_email_exists(&self, email: &str) -> Result<bool, Error> {
        let exists: bool = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM user WHERE email = $1)")
            .bind(email)
            .fetch_one(&self.db_pool)
            .await?;
        Ok(exists)
    }

    async fn create_user(
        &self,
        name: &str,
        email: &str,
        password: &str,
    ) -> Result<User, Error> {
        let created_at = Utc::now().to_rfc3339();
        let updated_at = Utc::now().to_rfc3339();
        
        let query_result = sqlx::query_as::<_, User>(
            "INSERT INTO user (name, email, password, created_at, updated_at) VALUES ($1, $2, $3, $4, $5) RETURNING *",
        )
        .bind(name)
        .bind(email)
        .bind(password)
        .bind(created_at)
        .bind(updated_at) 
        .fetch_one(&self.db_pool)
        .await?;
        Ok(query_result)
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, Error> {
        let query_result = sqlx::query_as::<_, User>("SELECT * FROM user WHERE email = $1")
            .bind(email)
            .fetch_optional(&self.db_pool)
            .await?;
        Ok(query_result)
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<User>, Error> {
        let query_result = sqlx::query_as::<_, User>("SELECT * FROM user WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.db_pool)
            .await?;
        Ok(query_result)
    }

    async fn update_user(
        &self,
        email: &str,
        name: &str,
        password: &str,
    ) -> Result<Option<User>, Error> {
        let updated_at = Utc::now().to_rfc3339();

        let query_result = sqlx::query_as::<_, User>(
            "UPDATE user SET name = ?, password = ? updated_at = ? WHERE email = ? RETURNING *",
        )
        .bind(name)
        .bind(password)
        .bind(updated_at)
        .bind(email)
        .fetch_optional(&self.db_pool)
        .await?;
        Ok(query_result)
    }

    async fn delete_user(&self, email: &str) -> Result<bool, Error> {
        let result = sqlx::query!("DELETE FROM user WHERE email = $1", email)
            .execute(&self.db_pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }
}