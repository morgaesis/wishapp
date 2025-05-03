use lambda_http::{Body, Request};
use serde_json::json;
use wishlist_api::handle_request;

fn build_lambda_request(method: &str, path: &str, body: Option<Body>) -> Request {
    let mut request = Request::default();
    *request.method_mut() = method.parse().unwrap();
    *request.uri_mut() = format!("https://example.com{}", path).parse().unwrap();
    *request.headers_mut() = lambda_http::http::HeaderMap::new();
    
    if let Some(body) = body {
        *request.body_mut() = body;
    }
    
    request
}

#[tokio::test]
async fn test_health_endpoint() {
    let event = build_lambda_request(
        "GET",
        "/health",
        Some(Body::from(
            json!({
                "version": "2.0",
                "routeKey": "GET /health",
                "rawPath": "/health",
                "requestContext": {
                    "http": {
                        "method": "GET",
                        "path": "/health"
                    }
                }
            }).to_string()
        ))
    );

    let response = handle_request(event).await.unwrap();
    assert_eq!(response.status(), 200);
    assert_eq!(response.body(), &Body::from(r#"OK"#));
}

#[tokio::test] 
async fn test_wishlists_endpoint() {
    let event = build_lambda_request(
        "GET",
        "/wishlists", 
        Some(Body::from(
            json!({
                "version": "2.0",
                "routeKey": "GET /wishlists",
                "rawPath": "/wishlists",
                "requestContext": {
                    "http": {
                        "method": "GET",
                        "path": "/wishlists"
                    }
                }
            }).to_string()
        ))
    );

    let response = handle_request(event).await.unwrap();
    assert_eq!(response.status(), 200);
}

#[tokio::test]
async fn test_wishlist_creation() {
    let event = build_lambda_request(
        "POST", 
        "/wishlists",
        Some(Body::from(
            json!({
                "id": "test1",
                "name": "Test List",
                "owner": "test-user",
                "items": []
            }).to_string()
        ))
    );

    let response = handle_request(event).await.unwrap();
    assert_eq!(response.status(), 201);
}