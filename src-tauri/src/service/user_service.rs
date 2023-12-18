use async_trait::async_trait;
use anyhow::Result;

use crate::{abstract_trait::{DynUserRepository, UserServiceTrait}, config::Hashing, models::User, utils::AppError};



pub struct UserService{
    repository: DynUserRepository,
    hashing: Hashing
}

impl UserService{
    pub fn new(repository: DynUserRepository, hashing: Hashing) -> Self{
        Self { repository, hashing }
    }
}

#[async_trait]
impl UserServiceTrait for UserService{
    async fn create_user(
        &self,
        name: &str, 
        email: &str,
        password: &str,
    ) -> Result<User, AppError>{
        let exists = self.repository.find_by_email_exists(email).await;

        if let Err(err) = exists{
            return Err(err.into());
        }

        let hashed_password = self.hashing.hash_password(password).await.map_err(AppError::HashingError)?;

        let create_user = self.repository.create_user(name, email, &hashed_password).await?;
        
        Ok(create_user)
    }

    async fn find_by_email_exists(&self, email: &str) -> Result<bool, AppError> {
        self.repository.find_by_email_exists(email).await.map_err(AppError::SqlxError)
    }

    async fn find_user_by_email(&self, email: &str) -> Result<Option<User>, AppError> {
        match self.repository.find_by_email(email).await {
            Ok(user) => Ok(user),
            Err(_) => Err(AppError::UserNotFound), 
        }
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<User>, AppError> {
        self.repository.find_by_id(id).await.map_err(AppError::SqlxError)
    }

    async fn update_user(
        &self,
        email: &str,
        name: &str,
        password: &str,
    ) -> Result<Option<User>, AppError> {
        self.repository.update_user(email, name, password).await.map_err(AppError::SqlxError)
    }

    async fn delete_user(&self, email: &str) -> Result<bool, AppError> {
        self.repository.delete_user(email).await.map_err(AppError::SqlxError)
    }
}