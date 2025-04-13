use lambda_http::{Body, Request};
use serde_json::json;
use wishlist_api::handlers::Wishlist;
use wishlist_api::{handle_delete, handle_get, handle_post, handle_put};

#[tokio::test]
async fn test_full_wishlist_lifecycle() {
    // Generate unique test ID and skip cleanup
    let test_id = format!(
        "test-{}-{}",
        chrono::Utc::now().timestamp_nanos_opt().unwrap(),
        rand::random::<u32>()
    );
    println!("[DEBUG] Using unique test ID: {}", test_id);

    // Verify no existing wishlists with this ID
    let check_req = Request::new(Body::Empty);
    let check_res = handle_get(check_req).await.unwrap();
    let existing: Vec<Wishlist> = serde_json::from_slice(check_res.body()).unwrap();
    assert!(
        !existing.iter().any(|w| w.id == test_id),
        "Test ID collision detected for ID: {}",
        test_id
    );

    // Create test wishlist with our unique ID
    let test_wishlist = json!({
        "id": test_id,
        "name": "Test Wishlist",
        "items": ["Initial"]
    });
    let create_req = Request::new(Body::from(test_wishlist.to_string()));
    let create_res = handle_post(create_req).await.unwrap();
    assert_eq!(create_res.status(), 201, "Failed to create test wishlist");

    // Create additional test data using our unique ID
    let test_wishlist = json!({
        "id": test_id,
        "name": "Test Wishlist",
        "items": ["Initial"]
    });
    let create_req = Request::new(Body::from(test_wishlist.to_string()));
    let _ = handle_post(create_req).await.unwrap();

    // Actively clean up any existing test wishlists
    let check_req = Request::new(Body::Empty);
    let check_res = handle_get(check_req).await.unwrap();
    let wishlists: Vec<Wishlist> = serde_json::from_slice(check_res.body()).unwrap();
    for wishlist in wishlists {
        if wishlist.id.starts_with("test-") {
            let delete_req = Request::new(Body::from(json!({"id": wishlist.id}).to_string()));
            let _ = handle_delete(delete_req).await.unwrap();
        }
    }

    // 1. Verify initial empty state
    let empty_res = handle_get(Request::new(Body::Empty)).await.unwrap();
    assert_eq!(empty_res.status(), 200);
    let empty_wishlists: Vec<Wishlist> = serde_json::from_slice(empty_res.body()).unwrap();
    assert_eq!(
        empty_wishlists.len(),
        0,
        "Should start with empty wishlists"
    );

    println!("[DEBUG] Verified empty state, proceeding with test setup");

    // 2. Create a new wishlist with single initial item
    let create_req = Request::new(Body::from(
        json!({
            "id": "test-id-1",
            "name": "Christmas List",
            "items": ["Socks"] // Starting with just one item
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
    println!("Current wishlists: {:?}", updated_wishlists); // Debug output
    assert_eq!(
        updated_wishlists.len(),
        1,
        "Should have exactly one wishlist"
    );
    assert_eq!(
        updated_wishlists[0].items,
        vec!["Socks", "Chocolate", "Book"],
        "Items should match expected set"
    );

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
            "id": "test-id-2",
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
