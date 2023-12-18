// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod models;
mod utils;
mod dto;

mod repository;
mod service;
mod handler_command;
mod config;
mod abstract_trait;

use tauri::Manager;
use tracing::info;

use crate::config::{ConnectionManager, AppState};
use crate::handler_command::{get_todos, add_todo, update_todo, remove_todo, login_user, register_user};
use crate::utils::tracing;

fn main(){
    tracing();


    tauri::Builder::default()
   
        .setup(|app|{

            tauri::async_runtime::block_on(async{
                let connection_manager = ConnectionManager::new_pool("hello.db", false)
                .await
                .expect("Failed to create connection manager");

                

                let app_state = AppState::new(connection_manager.db_pool);

                app.manage(app_state);
            });
          
          
            info!("Running up database");

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![add_todo, get_todos, update_todo, remove_todo, login_user, register_user])
        .run(tauri::generate_context!())
        .expect("errror while running tauri application");
}