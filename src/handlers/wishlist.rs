use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
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

    pub fn add_item(&mut self, item: String) {
        self.items.push(item);
    }

    pub fn remove_item(&mut self, item: &str) -> bool {
        if let Some(pos) = self.items.iter().position(|i| i == item) {
            self.items.remove(pos);
            true
        } else {
            false
        }
    }
}
