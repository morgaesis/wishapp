use wishlist_api::handlers::Wishlist;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wishlist_creation() {
        let w = Wishlist::new("Test List".to_string());
        assert_eq!(w.name, "Test List");
        assert!(w.items.is_empty());
    }

    #[test]
    fn test_item_addition() {
        let mut w = Wishlist::new("Test".to_string());
        w.add_item("Item 1".to_string());
        assert_eq!(w.items.len(), 1);
    }
}
