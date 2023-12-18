use anyhow::Result;
use async_trait::async_trait;
use sqlx::Error;
use crate::{models::User, utils::AppError};

use std::sync::Arc;

pub type DynUserRepository = Arc<dyn UserRepositoryTrait + Send + Sync>;

// pub type DynUserService = Arc<dyn UserServiceTrait + Send + Sync>;
pub type DynAuthService = Arc<dyn AuthServiceTrait + Send + Sync>;


#[async_trait]
pub trait UserRepositoryTrait{
    async fn find_by_email_exists(&self, email: &str) -> Result<bool, Error>;
    async fn create_user(
        &self,
        name: &str,
        email: &str,
        password: &str,
    ) -> Result<User, Error>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, Error>;
    async fn find_by_id(&self, id: i32) -> Result<Option<User>, Error>;
    async fn update_user(
        &self,
        email: &str,
        name: &str, 
        password: &str,
    ) -> Result<Option<User>, Error>;
    async fn delete_user(&self, email: &str) -> Result<bool, Error>;
}

#[async_trait]
pub trait UserServiceTrait {
    async fn create_user(
        &self,
        name: &str, 
        email: &str,
        password: &str,
    ) -> Result<User, AppError>;
    async fn find_by_email_exists(&self, email: &str) -> Result<bool, AppError>;
    async fn find_user_by_email(&self, email: &str) -> Result<Option<User>, AppError>;
    async fn find_by_id(&self, id: i32) -> Result<Option<User>, AppError>;
    async fn update_user(
        &self,
        email: &str,
        name: &str,
        password: &str,
    ) -> Result<Option<User>, AppError>;
    async fn delete_user(&self, email: &str) -> Result<bool, AppError>;
}

#[async_trait]
pub trait AuthServiceTrait{
    async fn register_user(&self, name: &str,email: &str, password: &str) -> Result<User, AppError>;
    async fn login_user(&self, email: &str, password: &str) -> Result<String, AppError>;
}