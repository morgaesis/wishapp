use log::{error, info};
use serde_json::json;
use lambda_http::{Body, Request, Response};
use lambda_http::http::StatusCode;

pub mod wishlist;

pub use crate::handlers::wishlist::Wishlist;


use crate::error::AppError;
use aws_sdk_dynamodb::Client as DynamoDbClient;

use crate::utils::{build_error_response, build_response};

pub async fn handle_request(
    event: Request,
    db_client: &DynamoDbClient,
) -> Result<Response<Body>, AppError> {
    let path = event.uri().path();
    let method = event.method();

    info!("[DEBUG] Request path: {}", path);
    info!("[DEBUG] Request method: {}", method);

    let cleaned_path = path.trim_start_matches("/prod"); // Remove /prod prefix

    match (method.as_str(), cleaned_path) {
        ("GET", _) => handle_get(event, db_client).await,
        ("POST", "/wishlists") => handle_post(event, db_client).await,
        ("PUT", "/wishlists") => handle_put(event, db_client).await,
        ("DELETE", "/wishlists") => handle_delete(event, db_client).await,
        _ => {
            error!("Unhandled request: {} {}", method, path);
            build_error_response(StatusCode::NOT_FOUND, "Not Found")
        }
    }
}
pub async fn handle_get(
    event: Request,
    db_client: &DynamoDbClient,
) -> Result<Response<Body>, AppError> {
    let path = event.uri().path();
    let cleaned_path = path.trim_start_matches("/prod"); // Remove /prod prefix
    info!("[DEBUG] GET request path: {}", path);
    info!("[DEBUG] Cleaned GET request path: {}", cleaned_path);
    match cleaned_path {
        "/health" => build_response(StatusCode::OK, Some(json!({"status": "OK"}))),
        "/wishlists" | "/wishlist" => match crate::db::scan_items(db_client).await {
            Ok(wishlists) => build_response(StatusCode::OK, Some(wishlists)),
            Err(e) => {
                error!("Error scanning DynamoDB: {:?}", e);
                build_error_response(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal Server Error",
                )
            }
        },
        path if path.starts_with("/wishlists/") => {
            let id = path.trim_start_matches("/wishlists/").trim_end_matches('/');
            match crate::db::get_item(db_client, id.to_string()).await {
                Ok(Some(wishlist)) => build_response(StatusCode::OK, Some(wishlist)),
                Ok(None) => build_error_response(StatusCode::NOT_FOUND, "Not Found"),
                Err(e) => {
                    error!("Error getting item from DynamoDB: {:?}", e);
                    build_error_response(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Internal Server Error",
                    )
                }
            }
        }
        _ => build_error_response(StatusCode::NOT_FOUND, "Not Found"),
    }
}

pub async fn handle_post(
    event: Request,
    db_client: &DynamoDbClient,
) -> Result<Response<Body>, AppError> {
    let body = event.body().as_ref();
    info!("[DEBUG] POST body: {:?}", String::from_utf8_lossy(body));
    let wishlist: Wishlist = serde_json::from_slice(body)?;
    info!("[DEBUG] Parsed wishlist: {:?}", wishlist);
    match crate::db::put_item(db_client, wishlist.clone()).await {
        Ok(_) => build_response(StatusCode::CREATED, Some(wishlist)),
        Err(e) => {
            error!("Error putting item to DynamoDB: {:?}", e);
            build_error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error",
            )
        }
    }
}

pub async fn handle_put(
    event: Request,
    db_client: &DynamoDbClient,
) -> Result<Response<Body>, AppError> {
    let updated: Wishlist = serde_json::from_slice(event.body().as_ref())?;
    info!("[DEBUG] Updating wishlist with ID: {}", updated.id);

    // Check if the item exists before attempting to update
    match crate::db::get_item(db_client, updated.id.clone()).await {
        Ok(Some(_)) => {
            // Item found, proceed with put_item
            match crate::db::put_item(db_client, updated.clone()).await {
                Ok(_) => build_response(StatusCode::OK, Some(updated)),
                Err(e) => {
                    error!("Error updating item in DynamoDB: {:?}", e);
                    build_error_response(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Internal Server Error",
                    )
                }
            }
        }
        Ok(None) => build_error_response(StatusCode::NOT_FOUND, "Not Found"),
        Err(e) => {
            error!("Error checking item existence in DynamoDB: {:?}", e);
            build_error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error",
            )
        }
    }
}

pub async fn handle_delete(
    event: Request,
    db_client: &DynamoDbClient,
) -> Result<Response<Body>, AppError> {
    let body = event.body().as_ref();
    let id_map: std::collections::HashMap<String, String> = match serde_json::from_slice(body) {
        Ok(map) => map,
        Err(e) => return Err(AppError::from(e)),
    };

    let id = match id_map.get("id") {
        Some(id) => id,
        None => return Err(AppError::MissingId),
    };

    // Check if the item exists before attempting to delete
    match crate::db::get_item(db_client, id.to_string()).await {
        Ok(Some(_)) => {
            // Item found, proceed with delete
            match crate::db::delete_item(db_client, id.to_string()).await {
                Ok(_) => build_response::<()>(StatusCode::NO_CONTENT, None),
                Err(e) => {
                    error!("Error deleting item from DynamoDB: {:?}", e);
                    build_error_response(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Internal Server Error",
                    )
                }
            }
        }
        Ok(None) => build_error_response(StatusCode::NOT_FOUND, "Not Found"),
        Err(e) => {
            error!(
                "Error checking item existence for deletion in DynamoDB: {:?}",
                e
            );
            build_error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error",
            )
        }
    }
}
