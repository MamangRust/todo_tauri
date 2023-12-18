use crate::{abstract_trait::{DynUserRepository, AuthServiceTrait}, config::Hashing, models::User, utils::AppError, dto::Claims};
use async_trait::async_trait;
use anyhow::Result;
use jsonwebtoken::{encode, Header, EncodingKey};


pub struct AuthService{
    repository: DynUserRepository,
    hashing: Hashing
}




impl AuthService{
    pub fn new(repository: DynUserRepository, hashing: Hashing) -> Self{
        Self { repository, hashing }
    }
}

#[async_trait]
impl AuthServiceTrait for AuthService{
    async fn register_user(&self, name: &str,email: &str, password: &str) -> Result<User, AppError>{
        let exists = self.repository.find_by_email_exists(email).await;

        if let Err(err) = exists{
            return Err(err.into());
        }

        let hashed_password = self.hashing.hash_password(password).await.map_err(AppError::HashingError)?;

        let create_user = self.repository.create_user(name, email, &hashed_password).await?;
        
        Ok(create_user)
    }

    async fn login_user(&self, email: &str, password: &str) -> Result<String, AppError> {
        let user = self.repository.find_by_email(email).await?;

        if let Some(user) = user {
            // Compare the password
            self.hashing
                .compare_password(&user.password, password)
                .await
                .map_err(|_| AppError::InvalidCredentials)?;
    
           
           
            let claims = Claims::new(user.id, user.email);
    
            let token = encode(
                &Header::default(),
                &claims,
                &EncodingKey::from_secret("YOUR_SECRET_KEY".as_ref()), // Convert to &[u8]
            ).unwrap();
    
            Ok(token.to_owned())
        } else {
            Err(AppError::UserNotFound)
        }
    }
}