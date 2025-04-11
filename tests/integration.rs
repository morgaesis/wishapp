use lambda_http::{Body, Request};
use serde_json::json;
use wishlist_api::handlers::{handle_delete, handle_get, handle_post, handle_put};

#[tokio::test]
async fn test_get_handler() {
    let req = Request::new(Body::Empty);
    let res = handle_get(req).await.unwrap();
    assert_eq!(res.status(), 200);
}

#[tokio::test]
async fn test_post_handler() {
    let req = Request::new(Body::from(json!({"name": "New List"}).to_string()));
    let res = handle_post(req).await.unwrap();
    assert_eq!(res.status(), 201);
}

#[tokio::test]
async fn test_put_handler() {
    let req = Request::new(Body::from(json!({"name": "Updated List"}).to_string()));
    let res = handle_put(req).await.unwrap();
    assert_eq!(res.status(), 200);
}

#[tokio::test]
async fn test_delete_handler() {
    let req = Request::new(Body::Empty);
    let res = handle_delete(req).await.unwrap();
    assert_eq!(res.status(), 204);
}

#[tokio::test]
async fn test_full_lifecycle() {
    // Create
    let create_req = Request::new(Body::from(json!({"name": "Lifecycle Test"}).to_string()));
    let create_res = handle_post(create_req).await.unwrap();
    assert_eq!(create_res.status(), 201);
    let body_bytes = create_res.body();
    println!("Raw response: {:?}", body_bytes); // Debug output
    assert!(!body_bytes.is_empty(), "Expected non-empty response body");
    let body: serde_json::Value = serde_json::from_slice(body_bytes).expect("Failed to parse JSON");
    assert!(body["id"].is_string(), "Expected 'id' field in response");
    let list_id = body["id"].as_str().unwrap();

    // Read
    let get_req = Request::new(Body::Empty);
    let get_res = handle_get(get_req).await.unwrap();
    assert_eq!(get_res.status(), 200);

    // Update
    let update_req = Request::new(Body::from(json!({
        "id": list_id,
        "name": "Updated Lifecycle Test"
    }).to_string()));
    let update_res = handle_put(update_req).await.unwrap();
    assert_eq!(update_res.status(), 200);

    // Delete
    let delete_req = Request::new(Body::Empty);
    let delete_res = handle_delete(delete_req).await.unwrap();
    assert_eq!(delete_res.status(), 204);
}
