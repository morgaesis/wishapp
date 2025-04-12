use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Wishlist {
    pub id: String,
    pub name: String,
    pub items: Vec<String>,
}

impl Wishlist {}
