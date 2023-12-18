use crate::config::AppState;
use crate::models::Todo;
use crate::utils::AppError;
use anyhow::Result;
use tauri::State;
use tracing::info;

#[tauri::command]
pub async fn add_todo(
    state: State<'_, AppState>,
    title: String,
    completed: bool,
) -> Result<(), AppError> {

    info!("Add Todo: {}, {}", title, completed);

    state.todo_service.create_todo(&title, completed).await?;

    Ok(())
}

#[tauri::command]
pub async fn get_todos(state: tauri::State<'_, AppState>) -> Result<Vec<Todo>, AppError> {
    let todos = state.todo_service.get_all_todos().await?;

    info!("Todo List: {:#?}", todos);


    Ok(todos)
}


#[tauri::command]
pub async fn update_todo(
    state: tauri::State<'_, AppState>,
    id: i64,
    completed: bool,
) -> Result<(), AppError> {
    info!("id: {}, completed: {}", id, completed);

   
    state.todo_service.update_completed(id, completed).await?;
    Ok(())
}


#[tauri::command]
pub async fn remove_todo(state: tauri::State<'_, AppState>, id: i64) -> Result<(), AppError> {

    info!("id: {}", id);
       
    state.todo_service.delete_todo(id).await?;
    Ok(())
}
