mod todo_trait;
mod user_trait;

pub use self::todo_trait::{TodoRepositoryTrait,  TodoServiceTrait, DynTodoRepository, DynTodoService};
pub use self::user_trait::{UserRepositoryTrait, DynUserRepository, AuthServiceTrait, DynAuthService};
