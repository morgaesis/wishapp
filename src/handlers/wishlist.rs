use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Wishlist {
    pub id: String,
    pub name: String,
    pub owner: String,
    pub items: Vec<String>,
}

impl Wishlist {
    pub fn new(name: String, owner: String) -> Self {
        Wishlist {
            id: Uuid::new_v4().to_string(),
            name,
            owner,
            items: Vec::new(),
        }
    }
}
