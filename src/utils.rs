use lambda_http::{Body, Response, Error};
use http::StatusCode;
use serde::Serialize;

pub fn build_response<T: Serialize>(
    status_code: StatusCode,
    body: Option<T>,
) -> Result<Response<Body>, Error> {
    let mut response_builder = Response::builder().status(status_code);

    if let Some(b) = body {
        response_builder = response_builder
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(&b)?.into());
    } else {
        response_builder = response_builder.body("".into());
    }

    Ok(response_builder.expect("Failed to build response"))
}

pub fn build_error_response(status_code: StatusCode, message: &str) -> Result<Response<Body>, Error> {
    Response::builder()
        .status(status_code)
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&serde_json::json!({"error": message}))?.into())
        .map_err(Error::from)
}
