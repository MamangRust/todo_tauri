use std::sync::Arc;

use super::{ConnectionPool, Hashing};
use crate::repository::{TodoRepository, UserRepository};
use crate::service::{AuthService, TodoService};

use crate::abstract_trait::{
    DynAuthService, DynTodoRepository, DynTodoService, DynUserRepository,
};

#[derive(Clone)]
pub struct AppState {
    pub todo_service: DynTodoService,
    pub auth_service: DynAuthService,
}

impl AppState {
    pub fn new(pool: ConnectionPool) -> Self {
        let todo_repository = Arc::new(TodoRepository::new(pool.clone())) as DynTodoRepository;
        let todo_service = Arc::new(TodoService::new(todo_repository)) as DynTodoService;

        let hash = Hashing::new();

        let user_repository = Arc::new(UserRepository::new(pool.clone())) as DynUserRepository;
        // let user_service =
        //     Arc::new(UserService::new(user_repository.clone(), hash.clone())) as DynUserService;

        let auth_service =
            Arc::new(AuthService::new(user_repository.clone(), hash.clone())) as DynAuthService;

        Self {
            todo_service,

            auth_service,
        }
    }
}
