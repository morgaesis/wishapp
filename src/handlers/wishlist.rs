use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Wishlist {
    pub id: String,
    pub name: String,
    pub owner: String,
    pub items: Vec<String>,
}

impl Wishlist {}
