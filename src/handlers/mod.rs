pub mod wishlist;

pub use crate::handlers::wishlist::Wishlist;

const TABLE_NAME: &str = "wishlist_table";
use aws_sdk_dynamodb::Client as DynamoDbClient;
use lambda_http::{Body, Error, Request, Response};
use serde_json::json;

pub async fn handle_get(
    event: Request,
    db_client: &DynamoDbClient,
) -> Result<Response<Body>, Error> {
    let path = event.uri().path();
    let cleaned_path = path.trim_start_matches("/prod"); // Remove /prod prefix
    println!("[DEBUG] GET request path: {}", path);
    println!("[DEBUG] Cleaned GET request path: {}", cleaned_path);
    match cleaned_path {
        "/health" => Ok(Response::builder()
            .status(200)
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(&json!({"status": "OK"}))?.into())?),
        "/wishlists" | "/wishlist" => {
            let scan_output = db_client.scan().table_name(TABLE_NAME).send().await;
            match scan_output {
                Ok(output) => {
                    let wishlists: Vec<Wishlist> = output
                        .items
                        .unwrap_or_default()
                        .into_iter()
                        .filter_map(|item| Wishlist::try_from(item).ok())
                        .collect();
                    Ok(Response::builder()
                        .status(200)
                        .header("Content-Type", "application/json")
                        .body(serde_json::to_string(&wishlists)?.into())?)
                }
                Err(e) => {
                    eprintln!("Error scanning DynamoDB: {:?}", e);
                    Ok(Response::builder()
                        .status(500)
                        .body("Internal Server Error".into())?)
                }
            }
        }
        path if path.starts_with("/wishlists/") => {
            let id = path.trim_start_matches("/wishlists/").trim_end_matches('/');
            let get_item_output = db_client
                .get_item()
                .table_name(TABLE_NAME)
                .key(
                    "id",
                    aws_sdk_dynamodb::types::AttributeValue::S(id.to_string()),
                )
                .send()
                .await;

            match get_item_output {
                Ok(output) => {
                    if let Some(item) = output.item {
                        match Wishlist::try_from(item) {
                            Ok(wishlist) => Ok(Response::builder()
                                .status(200)
                                .header("Content-Type", "application/json")
                                .body(serde_json::to_string(&wishlist)?.into())?),
                            Err(e) => {
                                eprintln!("Error converting item to Wishlist: {:?}", e);
                                Ok(Response::builder()
                                    .status(500)
                                    .body("Internal Server Error".into())?)
                            }
                        }
                    } else {
                        Ok(Response::builder().status(404).body("Not Found".into())?)
                    }
                }
                Err(e) => {
                    eprintln!("Error getting item from DynamoDB: {:?}", e);
                    Ok(Response::builder()
                        .status(500)
                        .body("Internal Server Error".into())?)
                }
            }
        }
        _ => Ok(Response::builder().status(404).body("Not Found".into())?),
    }
}

pub async fn handle_post(
    event: Request,
    db_client: &DynamoDbClient,
) -> Result<Response<Body>, Error> {
    let body = event.body().as_ref();
    println!("[DEBUG] POST body: {:?}", String::from_utf8_lossy(body));
    let wishlist: Wishlist = serde_json::from_slice(body)?;
    println!("[DEBUG] Parsed wishlist: {:?}", wishlist);
    let put_item_output = db_client
        .put_item()
        .table_name(TABLE_NAME)
        .item(
            "id",
            aws_sdk_dynamodb::types::AttributeValue::S(wishlist.id.clone()),
        )
        .item(
            "name",
            aws_sdk_dynamodb::types::AttributeValue::S(wishlist.name.clone()),
        )
        .item(
            "owner",
            aws_sdk_dynamodb::types::AttributeValue::S(wishlist.owner.clone()),
        )
        .item(
            "items",
            aws_sdk_dynamodb::types::AttributeValue::L(
                wishlist
                    .items
                    .iter()
                    .map(|item| aws_sdk_dynamodb::types::AttributeValue::S(item.clone()))
                    .collect(),
            ),
        )
        .send()
        .await;

    match put_item_output {
        Ok(_) => Ok(Response::builder()
            .status(201)
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(&wishlist)?.into())?),
        Err(e) => {
            eprintln!("Error putting item to DynamoDB: {:?}", e);
            Ok(Response::builder()
                .status(500)
                .body("Internal Server Error".into())?)
        }
    }
}

pub async fn handle_put(
    event: Request,
    db_client: &DynamoDbClient,
) -> Result<Response<Body>, Error> {
    let updated: Wishlist = serde_json::from_slice(event.body().as_ref())?;
    println!("[DEBUG] Updating wishlist with ID: {}", updated.id);
    let put_item_output = db_client
        .put_item()
        .table_name(TABLE_NAME)
        .item(
            "id",
            aws_sdk_dynamodb::types::AttributeValue::S(updated.id.clone()),
        )
        .item(
            "name",
            aws_sdk_dynamodb::types::AttributeValue::S(updated.name.clone()),
        )
        .item(
            "owner",
            aws_sdk_dynamodb::types::AttributeValue::S(updated.owner.clone()),
        )
        .item(
            "items",
            aws_sdk_dynamodb::types::AttributeValue::L(
                updated
                    .items
                    .iter()
                    .map(|item| aws_sdk_dynamodb::types::AttributeValue::S(item.clone()))
                    .collect(),
            ),
        )
        .send()
        .await;

    match put_item_output {
        Ok(_) => Ok(Response::builder()
            .status(200)
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(&updated)?.into())?),
        Err(e) => {
            eprintln!("Error updating item in DynamoDB: {:?}", e);
            Ok(Response::builder()
                .status(500)
                .body("Internal Server Error".into())?)
        }
    }
}

pub async fn handle_delete(
    event: Request,
    db_client: &DynamoDbClient,
) -> Result<Response<Body>, Error> {
    let body = event.body().as_ref();
    let id_map: std::collections::HashMap<String, String> = match serde_json::from_slice(body) {
        Ok(map) => map,
        Err(_) => {
            return Ok(Response::builder()
                .status(400)
                .body(Body::from("Invalid request body"))?)
        }
    };

    let id = match id_map.get("id") {
        Some(id) => id,
        None => {
            return Ok(Response::builder()
                .status(400)
                .body(Body::from("Missing id in request body"))?)
        }
    };

    let delete_item_output = db_client
        .delete_item()
        .table_name(TABLE_NAME)
        .key(
            "id",
            aws_sdk_dynamodb::types::AttributeValue::S(id.to_string()),
        )
        .send()
        .await;

    match delete_item_output {
        Ok(_) => Ok(Response::builder().status(204).body("".into())?),
        Err(e) => {
            eprintln!("Error deleting item from DynamoDB: {:?}", e);
            Ok(Response::builder()
                .status(500)
                .body("Internal Server Error".into())?)
        }
    }
}
