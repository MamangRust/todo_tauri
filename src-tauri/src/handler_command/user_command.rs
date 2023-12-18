use crate::config::AppState;
use crate::models::User;
use crate::utils::AppError;
use anyhow::Result;
use tauri::State;

#[tauri::command]
pub async fn create_user(state: State<'_, AppState>, name: String,email: String, password: String) -> Result<User, AppError>{
    let user= state.auth_service.register_user(&name, &email, &password).await?;

    Ok(user)
}

#[tauri::command]
pub async fn find_by_email_exists(state: State<'_, AppState>, email: String) -> Result<bool, AppError> {
    state.user_service.find_by_email_exists(&email).await
}

#[tauri::command]
pub async fn find_user_by_email(state: State<'_, AppState>, email: String) -> Result<Option<User>, AppError> {
    state.user_service.find_user_by_email(&email).await
}

#[tauri::command]
pub async fn find_by_id(state: State<'_, AppState>, id: i32) -> Result<Option<User>, AppError> {
    state.user_service.find_by_id(id).await
}

#[tauri::command]
pub async fn update_user(state: State<'_, AppState>, email: String, name: String, password: String) -> Result<Option<User>, AppError> {
    state.user_service.update_user(&email, &name, &password).await
}

#[tauri::command]
pub async fn delete_user(state: State<'_, AppState>, email: String) -> Result<bool, AppError> {
    state.user_service.delete_user(&email).await
}