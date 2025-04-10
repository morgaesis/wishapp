use lambda_http::{Body, Request};
use serde_json::json;
use wishlist_api::handlers::{handle_get, handle_post, handle_put, handle_delete};

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