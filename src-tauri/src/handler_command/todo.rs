use tauri::State;
use crate::{service::TodoService, domain::TodoResponse};

#[tauri::command]
pub async fn get_todos(state: State<'_, TodoService>) -> Result<Vec<TodoResponse>, String> {
    state.get_todos().await.map_err(|err| err.to_string())
}

#[tauri::command]
pub async fn get_todo(state: State<'_, TodoService>, id: i32) -> Result<Option<TodoResponse>, String> {
    state.get_todo(id).await.map_err(|err| err.to_string())
}

#[tauri::command]
pub async fn create_todo(state: State<'_, TodoService>, name: String) -> Result<TodoResponse, String> {
    state.create_todo(&name).await.map_err(|err| err.to_string())
}

#[tauri::command]
pub async fn update_todo(
    state: State<'_, TodoService>,
    id: i32,
    name: String,
) -> Result<Option<TodoResponse>, String> {
    state.update_todo(id, &name).await.map_err(|err| err.to_string())
}

#[tauri::command]
pub async fn delete_todo(state: State<'_, TodoService>, id: i32) -> Result<(), String> {
    state.delete_todo(id).await.map_err(|err| err.to_string())
}