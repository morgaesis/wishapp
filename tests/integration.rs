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
        Body::Text(text) => assert_eq!(text, "{\"status\":\"OK\"}"),
        _ => panic!("Expected text response body, got {:?}", response.body()),
    }
}
// Cleanup function moved to end of file for final cleanup
#[tokio::test]
async fn test_full_wishlist_lifecycle() {
    // Clear and initialize storage
    {
        let mut wishlists = wishlist_api::handlers::WISHLISTS.lock().unwrap();
        wishlists.clear();
    }
    // Test uses unique IDs so no initial cleanup needed
    let _check_req = Request::new(Body::Empty);

    // Retry verification up to 3 times with delay
    let mut retries = 0;
    let max_retries = 3;
    let mut empty_wishlists: Vec<Wishlist> = Vec::new();

    while retries < max_retries {
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        let mut empty_req = Request::new(Body::Empty);
        *empty_req.uri_mut() = "/wishlists".parse().unwrap();
        let empty_res = handle_get(empty_req).await.unwrap();
        println!(
            "Empty verification attempt {}: {:?}",
            retries + 1,
            empty_res
        );
        empty_wishlists = serde_json::from_slice(empty_res.body()).unwrap();

        if empty_wishlists.is_empty() {
            break;
        }
        retries += 1;
    }

    // Clear any existing test wishlists before starting
    for wishlist in &empty_wishlists {
        if wishlist.id.starts_with("test-") {
            let delete_req = Request::new(Body::from(json!({"id": wishlist.id}).to_string()));
            let _ = handle_delete(delete_req).await;
        }
    }

    // Generate unique test ID
    let test_id = format!(
        "test-{}-{}",
        chrono::Utc::now().timestamp_nanos_opt().unwrap(),
        rand::random::<u32>()
    );

    // Create test wishlist with our unique ID
    let test_wishlist = json!({
        "id": test_id,
        "name": "Test Wishlist",
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
            let mut delete_req = Request::new(Body::Empty);
            *delete_req.uri_mut() = format!("/wishlists/{}/", wishlist.id).parse().unwrap();
            let _ = handle_delete(delete_req).await.unwrap();
        }
    }

    // 1. Thorough cleanup of any existing test wishlists
    let mut cleanup_req = Request::new(Body::Empty);
    *cleanup_req.uri_mut() = "/wishlists".parse().unwrap();
    let cleanup_res = handle_get(cleanup_req).await.unwrap();
    assert_eq!(cleanup_res.status(), 200);
    let existing_wishlists: Vec<Wishlist> = serde_json::from_slice(cleanup_res.body()).unwrap();

    println!(
        "[DEBUG] Found {} existing wishlists to clean up",
        existing_wishlists.len()
    );

    for wishlist in existing_wishlists {
        if wishlist.id.starts_with("test-") {
            println!("[DEBUG] Deleting wishlist with ID: {}", wishlist.id);
            for _ in 0..3 {
                // Retry up to 3 times
                let mut delete_req =
                    Request::new(Body::from(json!({"id": wishlist.id}).to_string()));
                *delete_req.uri_mut() = format!("/wishlists/{}", wishlist.id).parse().unwrap();
                match handle_delete(delete_req).await {
                    Ok(res) if res.status() == 200 => break,
                    _ => tokio::time::sleep(std::time::Duration::from_millis(100)).await,
                }
            }
        }
    }

    println!("[DEBUG] Verified empty state, proceeding with test setup");

    // 2. Create a new wishlist with single initial item
    let create_req = Request::new(Body::from(
        json!({
            "id": "test-id-1",
            "name": "Christmas Wishlist",
            "owner": "Christmas Owner",
            "items": ["Socks", "Chocolate"] // Starting with two items
        })
        .to_string(),
    ));
    let create_res = handle_post(create_req).await.unwrap();
    assert_eq!(create_res.status(), 201);
    let created: Wishlist = serde_json::from_slice(create_res.body()).unwrap();
    assert_eq!(created.owner, "Christmas Owner");
    assert_eq!(created.items, vec!["Socks", "Chocolate"]);

    // 3. Verify wishlist appears in GET
    let mut get_req = Request::new(Body::Empty);
    *get_req.uri_mut() = "/wishlists".parse().unwrap();
    let get_res = handle_get(get_req).await.unwrap();
    println!("[DEBUG] GET response body: {:?}", get_res.body());
    let body = get_res.body();
    let wishlists: Vec<Wishlist> = serde_json::from_slice(body)
        .unwrap_or_else(|_| panic!("Failed to parse body: {:?}", String::from_utf8_lossy(body)));
    assert_eq!(wishlists.len(), 1);
    assert_eq!(wishlists[0].id, created.id);

    // 4. Update the wishlist
    let update_req = Request::new(Body::from(
        json!({
            "id": created.id,
            "name": "Updated Christmas Wishlist",
            "owner": "Updated Christmas Owner",
            "items": ["Socks", "Chocolate", "Book"]
        })
        .to_string(),
    ));
    println!(
        "[DEBUG] Update request payload: {}",
        String::from_utf8_lossy(update_req.body().as_ref())
    );
    let update_res = handle_put(update_req).await.unwrap();
    assert_eq!(update_res.status(), 200);
    println!("Update response body: {:?}", update_res.body().as_ref());

    // Debug storage state immediately after update
    {
        let wishlists = wishlist_api::handlers::WISHLISTS.lock().unwrap();
        println!("Immediate post-update storage state: {:?}", *wishlists);
    }

    let updated: Wishlist = if update_res.body().is_empty() {
        panic!("Received empty response body when expecting updated wishlist");
    } else {
        serde_json::from_slice(update_res.body())
            .unwrap_or_else(|_| panic!("Failed to parse response body: {:?}", update_res.body()))
    };
    assert_eq!(updated.owner, "Updated Christmas Owner");
    assert_eq!(updated.items.len(), 3);

    // 5. Verify update persisted
    let uri = format!("/wishlists/{}", created.id);
    println!("GET request URI: {}", uri);
    let mut get_req = Request::new(Body::Empty);
    *get_req.uri_mut() = uri.parse().unwrap();
    let get_updated = handle_get(get_req).await.unwrap();
    println!("GET response status: {}", get_updated.status());
    println!("GET response body: {:?}", get_updated.body().as_ref());

    if get_updated.status() == 404 {
        panic!("Wishlist not found after update - persistence failed");
    }

    let body_bytes = get_updated.body();
    let body_str = std::str::from_utf8(body_bytes).unwrap();
    println!("GET response body as string: {}", body_str);
    let updated_wishlist: Wishlist = serde_json::from_slice(body_bytes)
        .unwrap_or_else(|_| panic!("Failed to parse wishlist from: {}", body_str));
    println!("Updated wishlist: {:?}", updated_wishlist);

    // Debug: Print current storage state
    {
        let wishlists = wishlist_api::handlers::WISHLISTS.lock().unwrap();
        println!("Current storage state: {:?}", *wishlists);
    }
    assert_eq!(updated_wishlist.owner, "Updated Christmas Owner");
    assert_eq!(updated_wishlist.items.len(), 3);
    assert_eq!(
        updated_wishlist.id, created.id,
        "Should have exactly one wishlist"
    );
    assert_eq!(
        updated_wishlist.items,
        vec!["Socks", "Chocolate", "Book"],
        "Items should match expected set"
    );

    // 6. Delete the wishlist
    let delete_req = Request::new(Body::from(json!({"id": created.id}).to_string()));
    let delete_res = handle_delete(delete_req).await.unwrap();
    assert_eq!(delete_res.status(), 200);

    // 7. Verify deletion
    let final_get = handle_get(Request::new(Body::Empty)).await.unwrap();
    match final_get.status().as_u16() {
        200 => {
            let body = final_get.body();
            let final_wishlists: Vec<Wishlist> = if body.is_empty() {
                Vec::new()
            } else {
                serde_json::from_slice(body).unwrap_or_default()
            };
            assert!(final_wishlists.is_empty());
        }
        404 => {
            // Expected behavior when resource is deleted
        }
        status => panic!("Unexpected status code: {}", status),
    }
}

#[tokio::test]
async fn test_item_operations() {
    // Clear and initialize storage for this test
    {
        let mut wishlists = wishlist_api::handlers::WISHLISTS.lock().unwrap();
        wishlists.clear();
    }
    // Create wishlist
    let create_req = Request::new(Body::from(
        json!({
            "id": format!("test-{}-{}", chrono::Utc::now().timestamp_nanos_opt().unwrap(), rand::random::<u32>()),
            "name": "Test Wishlist",
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
            "name": "Test Wishlist",
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
            "name": "Test Wishlist",
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
            "name": "Test Wishlist",
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
