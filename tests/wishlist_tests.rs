use wishlist_api::handlers::Wishlist;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wishlist_creation() {
        let w = Wishlist {
            id: "test-id".to_string(),
            name: "Test List".to_string(),
            items: Vec::new(),
        };
        assert_eq!(w.name, "Test List");
        assert!(w.items.is_empty());
    }

    #[test]
    fn test_item_addition() {
        let mut w = Wishlist {
            id: "test-id".to_string(),
            name: "Test".to_string(),
            items: Vec::new(),
        };
        w.items.push("Item 1".to_string());
        assert_eq!(w.items.len(), 1);
    }
}
