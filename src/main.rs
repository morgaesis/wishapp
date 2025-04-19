//! WishApp Lambda Entry Point

use lambda_http::{run, service_fn};
use wishlist_api::handle_request;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    run(service_fn(handle_request)).await
}
