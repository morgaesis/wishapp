use aws_config::SdkConfig;
use aws_credential_types::provider::SharedCredentialsProvider;
use aws_credential_types::Credentials;
use aws_sdk_dynamodb::error::SdkError;
use aws_sdk_dynamodb::Client as DynamoDbClient;
use lambda_http::{Body, Request};
use serde_json::json;
use wishlist_api::handlers::Wishlist;
use wishlist_api::handlers::{handle_delete, handle_get, handle_post, handle_put};

use tokio::time::{sleep, Duration};

async fn wait_for_table_status(
    client: &DynamoDbClient,
    table_name: &str,
    expected_status: aws_sdk_dynamodb::types::TableStatus,
) {
    for _ in 0..30 {
        // Max 10 retries
        let describe_table_result = client.describe_table().table_name(table_name).send().await;

        match describe_table_result {
            Ok(output) => {
                if let Some(table) = output.table {
                    if let Some(status) = table.table_status {
                        if status == expected_status {
                            println!(
                                "Table '{}' is now in {:?} status.",
                                table_name, expected_status
                            );
                            return;
                        }
                    }
                }
            }
            Err(e) => {
                if expected_status == aws_sdk_dynamodb::types::TableStatus::Deleting
                    && e.to_string().contains("ResourceNotFoundException")
                {
                    println!("Table '{}' is already gone.", table_name);
                    return;
                }
                eprintln!(
                    "Error describing table '{}': {:?}. Retrying...",
                    table_name, e
                );
            }
        }
        sleep(Duration::from_secs(1)).await; // Wait for 1 second before retrying
    }
    panic!(
        "Table '{}' did not reach {:?} status in time.",
        table_name, expected_status
    );
}

async fn create_table(client: &DynamoDbClient) {
    let table_name = "wishlist_table";
    let key_schema = aws_sdk_dynamodb::types::KeySchemaElement::builder()
        .attribute_name("id")
        .key_type(aws_sdk_dynamodb::types::KeyType::Hash)
        .build()
        .expect("Failed to build KeySchemaElement");
    let attribute_definition = aws_sdk_dynamodb::types::AttributeDefinition::builder()
        .attribute_name("id")
        .attribute_type(aws_sdk_dynamodb::types::ScalarAttributeType::S)
        .build()
        .expect("Failed to build AttributeDefinition");

    println!("Attempting to create table '{}'...", table_name);
    let create_table_result = client
        .create_table()
        .table_name(table_name)
        .key_schema(key_schema)
        .attribute_definitions(attribute_definition)
        .provisioned_throughput(
            aws_sdk_dynamodb::types::ProvisionedThroughput::builder()
                .read_capacity_units(1)
                .write_capacity_units(1)
                .build()
                .expect("Failed to build ProvisionedThroughput"),
        )
        .send()
        .await;

    match create_table_result {
        Ok(_) => {
            println!("Table '{}' created successfully.", table_name);
        }
        Err(e) => {
            if let SdkError::ServiceError(service_error) = &e {
                if service_error.err().is_resource_in_use_exception() {
                    println!(
                        "Table '{}' already exists. Proceeding with existing table.",
                        table_name
                    );
                } else {
                    panic!("Failed to create table: {:?}", e);
                }
            } else {
                panic!("Failed to create table: {:?}", e);
            }
        }
    }
    println!("Table '{}' creation initiated.", table_name);
    println!("Waiting for table '{}' to become active...", table_name);
    wait_for_table_status(
        client,
        table_name,
        aws_sdk_dynamodb::types::TableStatus::Active,
    )
    .await;
}

async fn wait_for_table_gone(client: &DynamoDbClient, table_name: &str) {
    for _ in 0..30 {
        // Max 10 retries
        let describe_table_result = client.describe_table().table_name(table_name).send().await;

        match describe_table_result {
            Ok(_) => {
                println!("Table '{}' still exists. Retrying...", table_name);
            }
            Err(e) => {
                if let SdkError::ServiceError(service_error) = &e {
                    if service_error.err().is_resource_not_found_exception() {
                        println!("Table '{}' is now gone.", table_name);
                        return;
                    }
                }
                eprintln!(
                    "Error describing table '{}': {:?}. Retrying...",
                    table_name, e
                );
            }
        }
        sleep(Duration::from_secs(1)).await; // Wait for 1 second before retrying
    }
    panic!("Table '{}' did not disappear in time.", table_name);
}

async fn delete_table(client: &DynamoDbClient) {
    let table_name = "wishlist_table";
    match client.delete_table().table_name(table_name).send().await {
        Ok(_) => {
            println!("Table '{}' deletion initiated.", table_name);
            wait_for_table_gone(client, table_name).await;
        }
        Err(e) => {
            if e.to_string().contains("ResourceNotFoundException") {
                println!("Table '{}' does not exist, no need to delete.", table_name);
            } else {
                eprintln!("Error deleting table '{}': {:?}", table_name, e);
            }
        }
    }
}

async fn setup_db_client() -> DynamoDbClient {
    let endpoint = std::env::var("DYNAMODB_ENDPOINT").unwrap_or_else(|_| "http://host.containers.internal:8000".to_string());
    let config = SdkConfig::builder()
        .endpoint_url(endpoint) // Use DynamoDB Local endpoint from env or default
        .region(aws_sdk_dynamodb::config::Region::new("eu-west-1")) // Specify a region
        .behavior_version(aws_config::BehaviorVersion::latest()) // Explicitly set behavior version
        .credentials_provider(SharedCredentialsProvider::new(Credentials::for_tests())) // Use dummy credentials for local testing
        .build();
    let client = DynamoDbClient::new(&config);
    println!("Setting up DynamoDB client. Deleting existing table (if any)...");
    delete_table(&client).await; // Ensure clean state
    println!("Creating fresh table...");
    create_table(&client).await; // Create fresh table
    println!("DynamoDB client setup complete.");
    client
}

#[tokio::test]
async fn test_health_check() {
    println!("Running test_health_check...");
    let db_client = setup_db_client().await;
    let mut event = Request::new(Body::Empty);
    *event.uri_mut() = "/health".parse().unwrap();

    println!("Request URI: {}", event.uri());

    let response = handle_get(event, &db_client)
        .await
        .expect("expected Ok(_) value");

    assert_eq!(response.status(), 200);
    match response.body() {
        Body::Text(text) => assert_eq!(text, "{\"status\":\"OK\"}"),
        _ => panic!("Expected text response body, got {:?}", response.body()),
    }
}
// Cleanup function moved to end of file for final cleanup
#[tokio::test]
async fn test_full_wishlist_lifecycle() {
    println!("Running test_full_wishlist_lifecycle...");
    let db_client = setup_db_client().await;

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
        let empty_res = handle_get(empty_req, &db_client).await.unwrap();
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
            let _ = handle_delete(delete_req, &db_client).await;
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
    let create_res = handle_post(create_req, &db_client).await.unwrap();
    assert_eq!(create_res.status(), 201, "Failed to create test wishlist");

    // Actively clean up any existing test wishlists
    let mut check_req = Request::new(Body::Empty);
    *check_req.uri_mut() = "/wishlists".parse().unwrap();
    let check_res = handle_get(check_req, &db_client).await.unwrap();
    let wishlists = if check_res.status() == 200 {
        serde_json::from_slice::<Vec<Wishlist>>(check_res.body()).unwrap_or_default()
    } else {
        Vec::new()
    };
    for wishlist in wishlists {
        if wishlist.id.starts_with("test-") || wishlist.id == "test-id-2" {
            let mut delete_req = Request::new(Body::Empty);
            *delete_req.uri_mut() = format!("/wishlists/{}/", wishlist.id).parse().unwrap();
            let _ = handle_delete(delete_req, &db_client).await.unwrap();
        }
    }

    // 1. Thorough cleanup of any existing test wishlists
    let mut cleanup_req = Request::new(Body::Empty);
    *cleanup_req.uri_mut() = "/wishlists".parse().unwrap();
    let cleanup_res = handle_get(cleanup_req, &db_client).await.unwrap();
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
                match handle_delete(delete_req, &db_client).await {
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
    let create_res = handle_post(create_req, &db_client).await.unwrap();
    assert_eq!(create_res.status(), 201);
    let created: Wishlist = serde_json::from_slice(create_res.body()).unwrap();
    assert_eq!(created.owner, "Christmas Owner");
    assert_eq!(created.items, vec!["Socks", "Chocolate"]);

    // 3. Verify wishlist appears in GET
    let mut get_req = Request::new(Body::Empty);
    *get_req.uri_mut() = "/wishlists".parse().unwrap();
    let get_res = handle_get(get_req, &db_client).await.unwrap();
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
    let update_res = handle_put(update_req, &db_client).await.unwrap();
    assert_eq!(update_res.status(), 200);
    println!("Update response body: {:?}", update_res.body().as_ref());

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
    let get_updated = handle_get(get_req, &db_client).await.unwrap();
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
    let delete_res = handle_delete(delete_req, &db_client).await.unwrap();
    assert_eq!(delete_res.status(), 204);

    // 7. Verify deletion
    let final_get = handle_get(Request::new(Body::Empty), &db_client)
        .await
        .unwrap();
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
    println!("Running test_item_operations...");
    let db_client = setup_db_client().await;

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
    let create_res = handle_post(create_req, &db_client).await.unwrap();
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
    let add_item_res = handle_put(add_item_req, &db_client).await.unwrap();
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
    let remove_item_res = handle_put(remove_item_req, &db_client).await.unwrap();
    assert_eq!(remove_item_res.status(), 200);
    let final_state: Wishlist = serde_json::from_slice(remove_item_res.body()).unwrap();
    assert_eq!(final_state.items.len(), 1);
    assert!(final_state.items.contains(&"Added".to_string()));
}

#[tokio::test]
async fn test_error_handling() {
    // Invalid JSON
    let invalid_json = Request::new(Body::from("invalid json"));
    let db_client = setup_db_client().await;
    let res = handle_post(invalid_json, &db_client).await;
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
    let update_res = handle_put(update_req, &db_client).await.unwrap();
    assert_eq!(update_res.status(), 404);

    let delete_req = Request::new(Body::from(json!({"id": fake_id}).to_string()));
    let delete_res = handle_delete(delete_req, &db_client).await.unwrap();
    assert_eq!(delete_res.status(), 404);
}
