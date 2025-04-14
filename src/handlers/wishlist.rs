use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Wishlist {
    pub id: String,
    pub name: String,
    pub items: Vec<String>,
}

impl Wishlist {
    pub fn new(name: String) -> Self {
        Wishlist {
            id: Uuid::new_v4().to_string(),
            name,
            items: Vec::new(),
        }
    }
}
