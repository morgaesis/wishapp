use lambda_http::{Body, Request, http};
use serial_test::serial;
use wishlist_api::handle_request;

#[tokio::test]
#[serial]
async fn test_health_check() {
    let request = http::Request::builder()
        .method("GET")
        .uri("/health")
        .body(Body::Empty)
        .unwrap();
    
    let response = handle_request(request)
        .await
        .expect("Health check failed");
    
    assert_eq!(response.status(), 200);
    assert_eq!(response.body(), &Body::Text("OK".to_string()));
}

#[tokio::test]
#[serial] 
async fn test_wishlist_endpoint() {
    let request = http::Request::builder()
        .method("GET")
        .uri("/api/wishlists")
        .body(Body::Empty)
        .unwrap();
        
    let response = handle_request(request)
        .await
        .expect("API request failed");
        
    assert_eq!(response.status(), 200);
}