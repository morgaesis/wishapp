//! WishApp API Library
//!
//! Provides the core functionality for the Wishlist application.

use handlers::{handle_delete, handle_get, handle_post, handle_put};
use lambda_http::{Body, Error, Request, Response};

pub mod handlers;

/// Main request handler for the WishApp API
///
/// Routes requests to appropriate handler functions based on HTTP method
/// # Arguments
/// * `event` - The incoming Lambda HTTP request
/// # Returns
/// Result containing either a response or error
pub async fn handle_request(event: Request) -> Result<Response<Body>, Error> {
    match event.method().as_str() {
        "GET" => handle_get(event).await,
        "POST" => handle_post(event).await,
        "PUT" => handle_put(event).await,
        "DELETE" => handle_delete(event).await,
        _ => Ok(Response::builder()
            .status(405)
            .body("Method not allowed".into())?),
    }
}
