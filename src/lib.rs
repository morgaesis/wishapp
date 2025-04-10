use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Wishlist {
    pub id: String,
    pub name: String,
    pub items: Vec<String>,
}

impl Wishlist {
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            items: Vec::new(),
        }
    }

    pub fn add_item(&mut self, item: String) {
        self.items.push(item);
    }

    pub fn remove_item(&mut self, item: &str) -> bool {
        if let Some(pos) = self.items.iter().position(|x| x == item) {
            self.items.remove(pos);
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wishlist_creation() {
        let name = "Test Wishlist".to_string();
        let wishlist = Wishlist::new(name.clone());

        assert!(!wishlist.id.is_empty());
        assert_eq!(wishlist.name, name);
        assert!(wishlist.items.is_empty());
    }

    #[test]
    fn test_add_item() {
        let mut wishlist = Wishlist::new("Test".to_string());
        wishlist.add_item("Item 1".to_string());

        assert_eq!(wishlist.items.len(), 1);
        assert_eq!(wishlist.items[0], "Item 1");
    }

    #[test]
    fn test_remove_item() {
        let mut wishlist = Wishlist::new("Test".to_string());
        wishlist.add_item("Item 1".to_string());

        assert!(wishlist.remove_item("Item 1"));
        assert!(!wishlist.remove_item("Nonexistent"));
        assert!(wishlist.items.is_empty());
    }
}
