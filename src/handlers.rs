use lambda_http::{Body, Error, Request, Response};

pub async fn handle_get(_event: Request) -> Result<Response<Body>, Error> {
    Ok(Response::builder().status(200).body("GET handler".into())?)
}

use uuid::Uuid;
use serde_json::json;

pub async fn handle_post(_event: Request) -> Result<Response<Body>, Error> {
    let new_id = Uuid::new_v4();
    Ok(Response::builder()
        .status(201)
        .header("Content-Type", "application/json")
        .body(json!({"id": new_id.to_string()}).to_string().into())?)
}

pub async fn handle_put(_event: Request) -> Result<Response<Body>, Error> {
    Ok(Response::builder().status(200).body("PUT handler".into())?)
}

pub async fn handle_delete(_event: Request) -> Result<Response<Body>, Error> {
    Ok(Response::builder().status(204).body("".into())?)
}
