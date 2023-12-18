use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub user_id: i32,
    pub email: String,
}

impl Claims {
    pub fn new(user_id: i32, email: String) -> Self {
        Claims { user_id, email }
    }
}
