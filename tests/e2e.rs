use lambda_http::{http, Body};
use serial_test::serial;
use wishlist_api::handle_request;
use aws_sdk_dynamodb::Client as DynamoDbClient;
use aws_config::SdkConfig;

async fn setup_db_client() -> DynamoDbClient {
    let config = aws_config::load_from_env().await;
    DynamoDbClient::new(&config)
}

#[tokio::test]
#[serial]
#[ignore]
async fn test_health_check() {
    let request = http::Request::builder()
        .method("GET")
        .uri("/health")
        .body(Body::Empty)
        .unwrap();

    let db_client = setup_db_client().await;
    let response = handle_request(request, &db_client).await.expect("Health check failed");

    assert_eq!(response.status(), 200);
    assert_eq!(
        response.body(),
        &Body::Text("{\"status\":\"OK\"}".to_string())
    );
}

#[tokio::test]
#[serial]
#[ignore]
async fn test_wishlist_endpoint() {
    let request = http::Request::builder()
        .method("GET")
        .uri("/wishlists")
        .body(Body::Empty)
        .unwrap();

    let db_client = setup_db_client().await;
    let response = handle_request(request, &db_client).await.expect("API request failed");

    assert_eq!(response.status(), 200);
}
