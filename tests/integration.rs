use lambda_http::{Body, Request};
use serde_json::json;
use wishlist_api::handlers::Wishlist;
use wishlist_api::handlers::{handle_delete, handle_get, handle_post, handle_put};

#[tokio::test]
async fn test_health_check() {
    let mut event = Request::new(Body::Empty);
    *event.uri_mut() = "/health".parse().unwrap();

    println!("Request URI: {}", event.uri());

    let response = handle_get(event).await.expect("expected Ok(_) value");

    assert_eq!(response.status(), 200);
    match response.body() {
        Body::Text(text) => assert_eq!(text, "OK"),
        _ => panic!("Expected text response body, got {:?}", response.body()),
    }
}
// Cleanup function moved to end of file for final cleanup
#[tokio::test]
async fn test_full_wishlist_lifecycle() {
    // Test uses unique IDs so no initial cleanup needed
    let mut check_req = Request::new(Body::Empty);

    // Retry verification up to 3 times with delay
    let mut retries = 0;
    let max_retries = 3;
    let mut empty_wishlists: Vec<Wishlist> = Vec::new();
    
    while retries < max_retries {
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        let mut empty_req = Request::new(Body::Empty);
        *empty_req.uri_mut() = "/wishlists".parse().unwrap();
        let empty_res = handle_get(empty_req).await.unwrap();
        println!("Empty verification attempt {}: {:?}", retries + 1, empty_res);
        empty_wishlists = serde_json::from_slice(empty_res.body()).unwrap();
        
        if empty_wishlists.is_empty() {
            break;
        }
        retries += 1;
    }

    // Final verification after retries
    println!("Final wishlist state: {:?}", empty_wishlists);
    assert_eq!(empty_wishlists.len(), 0, "Should start with empty wishlists");

    // Generate unique test ID
    let test_id = format!(
        "test-{}-{}",
        chrono::Utc::now().timestamp_nanos_opt().unwrap(),
        rand::random::<u32>()
    );

    // Create test wishlist with our unique ID
    let test_wishlist = json!({
        "id": test_id,
        "owner": "Test Owner",
        "items": ["Initial"]
    });
    let create_req = Request::new(Body::from(test_wishlist.to_string()));
    let create_res = handle_post(create_req).await.unwrap();
    assert_eq!(create_res.status(), 201, "Failed to create test wishlist");

    // Actively clean up any existing test wishlists
    let mut check_req = Request::new(Body::Empty);
    *check_req.uri_mut() = "/wishlists".parse().unwrap();
    let check_res = handle_get(check_req).await.unwrap();
    let wishlists = if check_res.status() == 200 {
        serde_json::from_slice::<Vec<Wishlist>>(check_res.body()).unwrap_or_default()
    } else {
        Vec::new()
    };
    for wishlist in wishlists {
        if wishlist.id.starts_with("test-") || wishlist.id == "test-id-2" {
            let mut delete_req = Request::new(Body::Empty); *delete_req.uri_mut() = format!("/wishlists/{}/", wishlist.id).parse().unwrap();
            let _ = handle_delete(delete_req).await.unwrap();
        }
    }

    // 1. Verify initial empty state
    let mut empty_req = Request::new(Body::Empty);
    *empty_req.uri_mut() = "/wishlists".parse().unwrap();
    let empty_res = handle_get(empty_req).await.unwrap();
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
            "owner": "Christmas Owner",
            "items": ["Socks"] // Starting with just one item
        })
        .to_string(),
    ));
    let create_res = handle_post(create_req).await.unwrap();
    assert_eq!(create_res.status(), 201);
    let created: Wishlist = serde_json::from_slice(create_res.body()).unwrap();
    assert_eq!(created.owner, "Christmas Owner");
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
            "owner": "Updated Christmas Owner",
            "items": ["Socks", "Chocolate", "Book"]
        })
        .to_string(),
    ));
    let update_res = handle_put(update_req).await.unwrap();
    assert_eq!(update_res.status(), 200);
    let updated: Wishlist = serde_json::from_slice(update_res.body()).unwrap();
    assert_eq!(updated.owner, "Updated Christmas Owner");
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
    // Test uses unique IDs so no initial cleanup needed
    // Create wishlist
    let create_req = Request::new(Body::from(
        json!({
            "id": format!("test-{}-{}", chrono::Utc::now().timestamp_nanos_opt().unwrap(), rand::random::<u32>()),
            "owner": "Test Owner",
            "items": ["Initial"]
        })
        .to_string(),
    ));
    let create_res = handle_post(create_req).await.unwrap();
    println!("[DEBUG] Create response: {:?}", create_res.body());
    let wishlist: Wishlist = if create_res.status() == 201 {
        serde_json::from_slice(create_res.body()).expect("Failed to parse wishlist")
    } else {
        panic!("Failed to create wishlist: {}", create_res.status());
    };

    // Test adding item via PUT
    let add_item_req = Request::new(Body::from(
        json!({
            "id": wishlist.id,
            "owner": "Test Owner",
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
            "owner": "Test Owner",
            "items": ["Added"] // Remove "Initial"
        })
        .to_string(),
    ));
    let remove_item_res = handle_put(remove_item_req).await.unwrap();
    assert_eq!(remove_item_res.status(), 200);
    let final_state: Wishlist = serde_json::from_slice(remove_item_res.body()).unwrap();
    assert_eq!(final_state.items.len(), 1);
    assert!(final_state.items.contains(&"Added".to_string()));
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
            "owner": "Test Owner",
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
