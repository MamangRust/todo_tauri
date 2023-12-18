use crate::config::AppState;
use crate::models::User;
use crate::utils::AppError;
use anyhow::Result;
use tauri::State;
use tracing::info;

#[tauri::command]
pub async fn register_user(state: State<'_, AppState>, name: String,email: String, password: String) -> Result<User, AppError>{
    let user= state.auth_service.register_user(&name, &email, &password).await?;

    info!("Register User: {:#?}", user);

    Ok(user)
}

#[tauri::command]
pub async fn login_user(state: State<'_, AppState>, email: String, password: String) -> Result<String, AppError>{
    let user = state.auth_service.login_user(&email, &password).await?;

    info!("Login {}", user);

    Ok(user)
}