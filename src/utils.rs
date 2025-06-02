use crate::error::AppError;
use lambda_http::http::StatusCode;
use lambda_http::{Body, Response};
use serde::Serialize;
use serde_json::json;

pub fn build_response<T: Serialize>(
    status_code: StatusCode,
    body: Option<T>,
) -> Result<Response<Body>, AppError> {
    let response_body = if let Some(b) = body {
        serde_json::to_string(&b)?.into()
    } else {
        "".into()
    };

    let response = Response::builder()
        .status(status_code)
        .header("Content-Type", "application/json")
        .body(response_body)
        .map_err(AppError::from)?; // Convert the error from body() to AppError

    Ok(response)
}

pub fn build_error_response(
    status_code: StatusCode,
    message: &str,
) -> Result<Response<Body>, AppError> {
    let response = Response::builder()
        .status(status_code)
        .header("Content-Type", "application/json")
        .body(json!({"error": message}).to_string().into())
        .map_err(AppError::from)?; // Convert the error from body() to AppError

    Ok(response)
}
