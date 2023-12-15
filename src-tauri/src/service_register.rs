use std::sync::Arc;

use crate::{
    abstract_trait::{DynTodoService, DynTodoRepository},
    config::ConnectionPool,
    repository::TodoRepository,
    service::TodoService
};


#[derive(Clone)]
pub struct ServiceRegister{
    pub todo_service: DynTodoService,
}

// impl ServiceRegister{
//     pub fn new(pool: ConnectionPool) -> Self{
//         let todo_repository = TodoRepository::new(pool);

       
//     }
// }