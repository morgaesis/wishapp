use handlers::{handle_delete, handle_get, handle_post, handle_put};
use lambda_http::{Body, Error, Request, Response};

pub mod handlers;

use aws_sdk_dynamodb::Client as DynamoDbClient;

pub async fn handle_request(
    event: Request,
    db_client: &DynamoDbClient,
) -> Result<Response<Body>, Error> {
    match event.method().as_str() {
        "GET" => handle_get(event, db_client).await,
        "POST" => handle_post(event, db_client).await,
        "PUT" => handle_put(event, db_client).await,
        "DELETE" => handle_delete(event, db_client).await,
        _ => Ok(Response::builder()
            .status(405)
            .body("Method not allowed".into())?),
    }
}
