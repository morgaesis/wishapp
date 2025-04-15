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
    
}
