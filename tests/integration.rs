use lambda_http::{Body, Request};
use serde_json::json;
use wishlist_api::{handle_delete, handle_get, handle_post, handle_put};
use wishlist_api::handlers::Wishlist;

#[tokio::test]
async fn test_full_wishlist_lifecycle() {
    // 1. Verify initial empty state
    let empty_res = handle_get(Request::new(Body::Empty)).await.unwrap();
    assert_eq!(empty_res.status(), 200);
    let empty_wishlists: Vec<Wishlist> = serde_json::from_slice(empty_res.body()).unwrap();
    assert!(empty_wishlists.is_empty());

    // 2. Create a new wishlist
    let create_req = Request::new(Body::from(
        json!({
            "name": "Christmas List",
            "items": ["Socks", "Chocolate"]
        })
        .to_string(),
    ));
    let create_res = handle_post(create_req).await.unwrap();
    assert_eq!(create_res.status(), 201);
    let created: Wishlist = serde_json::from_slice(create_res.body()).unwrap();
    assert_eq!(created.name, "Christmas List");
    assert_eq!(created.items, vec!["Socks", "Chocolate"]);

    // 3. Verify wishlist appears in GET
    let get_res = handle_get(Request::new(Body::Empty)).await.unwrap();
    let wishlists: Vec<Wishlist> = serde_json::from_slice(get_res.body()).unwrap();
    assert_eq!(wishlists.len(), 1);
    assert_eq!(wishlists[0].id, created.id);

    // 4. Update the wishlist
    let update_req = Request::new(Body::from(
        json!({
            "id": created.id,
            "name": "Updated Christmas List",
            "items": ["Socks", "Chocolate", "Book"]
        })
        .to_string(),
    ));
    let update_res = handle_put(update_req).await.unwrap();
    assert_eq!(update_res.status(), 200);
    let updated: Wishlist = serde_json::from_slice(update_res.body()).unwrap();
    assert_eq!(updated.name, "Updated Christmas List");
    assert_eq!(updated.items.len(), 3);

    // 5. Verify update persisted
    let get_updated = handle_get(Request::new(Body::Empty)).await.unwrap();
    let updated_wishlists: Vec<Wishlist> = serde_json::from_slice(get_updated.body()).unwrap();
    assert_eq!(updated_wishlists[0].items.len(), 3);

    // 6. Delete the wishlist
    let delete_req = Request::new(Body::from(json!({"id": created.id}).to_string()));
    let delete_res = handle_delete(delete_req).await.unwrap();
    assert_eq!(delete_res.status(), 204);

    // 7. Verify deletion
    let final_get = handle_get(Request::new(Body::Empty)).await.unwrap();
    let final_wishlists: Vec<Wishlist> = serde_json::from_slice(final_get.body()).unwrap();
    assert!(final_wishlists.is_empty());
}

#[tokio::test]
async fn test_item_operations() {
    // Create wishlist
    let create_req = Request::new(Body::from(
        json!({
            "name": "Test Items",
            "items": ["Initial"]
        })
        .to_string(),
    ));
    let create_res = handle_post(create_req).await.unwrap();
    let wishlist: Wishlist = serde_json::from_slice(create_res.body()).unwrap();

    // Test adding item via PUT
    let add_item_req = Request::new(Body::from(
        json!({
            "id": wishlist.id,
            "name": "Test Items",
            "items": ["Initial", "Added"]
        })
        .to_string(),
    ));
    let add_item_res = handle_put(add_item_req).await.unwrap();
    assert_eq!(add_item_res.status(), 200);
    let updated: Wishlist = serde_json::from_slice(add_item_res.body()).unwrap();
    assert_eq!(updated.items, vec!["Initial", "Added"]);

    // Test removing item via PUT
    let remove_item_req = Request::new(Body::from(
        json!({
            "id": wishlist.id,
            "name": "Test Items",
            "items": ["Added"] // Remove "Initial"
        })
        .to_string(),
    ));
    let remove_item_res = handle_put(remove_item_req).await.unwrap();
    let final_state: Wishlist = serde_json::from_slice(remove_item_res.body()).unwrap();
    assert_eq!(final_state.items, vec!["Added"]);
}

#[tokio::test]
async fn test_error_handling() {
    // Invalid JSON
    let invalid_json = Request::new(Body::from("invalid json"));
    let res = handle_post(invalid_json).await;
    assert!(res.is_err());

    // Nonexistent wishlist operations
    let fake_id = "nonexistent-id";
    let update_req = Request::new(Body::from(
        json!({
            "id": fake_id,
            "name": "Test",
            "items": []
        })
        .to_string(),
    ));
    let update_res = handle_put(update_req).await.unwrap();
    assert_eq!(update_res.status(), 404);

    let delete_req = Request::new(Body::from(json!({"id": fake_id}).to_string()));
    let delete_res = handle_delete(delete_req).await.unwrap();
    assert_eq!(delete_res.status(), 404);
}
