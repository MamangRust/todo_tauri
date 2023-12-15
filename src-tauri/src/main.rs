// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// mod abstract_trait;
mod config;
mod domain;
mod handler_command;
mod models;
mod repository;
mod service;
// mod service_register;
use std::path::PathBuf;

use config::{Config, ConnectionManager};
use handler_command::{get_todos, get_todo, create_todo, update_todo, delete_todo};
use repository::TodoRepository;
use service::TodoService;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};


// #[tauri::command]
// fn greet(name: &str) -> String {
//     format!("Hello, {}! You've been greeted from Rust!", name)
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = PathBuf::from("");
    dotenv::from_path(&path).ok();

    dotenv::dotenv().ok();
    let config = Config::init();

    
    init_tracing();
    
    
    let pool = ConnectionManager::new_pool(&config.database_url, config.run_migrations).await?;
    
    let todo_repository = TodoRepository::new(pool);
    let todo_service = TodoService::new(todo_repository);


    tauri::Builder::default()
        .manage(todo_service)
        .invoke_handler(tauri::generate_handler![get_todos, get_todo, create_todo, update_todo, delete_todo])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}

fn init_tracing() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("tower_http=debug,todo-list=debug").unwrap_or_else(|_| {
                if cfg!(test) {
                    "tower_http=error"
                } else {
                    "todo-list=debug,tower_http=debug"
                }
                .into()
            }),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();
}
