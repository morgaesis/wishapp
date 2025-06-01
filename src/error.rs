use thiserror::Error;
use lambda_http::http::StatusCode;
use lambda_http::Response;
use lambda_http::Body;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("DynamoDB error: {0}")]
    DynamoDb(#[from] aws_sdk_dynamodb::Error),
    #[error("Serialization/Deserialization error: {0}")]
    SerdeJson(#[from] serde_json::Error),
    #[error("HTTP error: {0}")]
    Http(#[from] lambda_http::http::Error),
    #[error("Lambda HTTP error: {0}")]
    LambdaHttp(#[from] lambda_http::Error),
    #[error("Generic error: {0}")]
    Generic(String),
    #[error("Item not found")]
    NotFound,
    #[error("Bad request: {0}")]
    BadRequest(String),
    #[error("Missing ID in request body")]
    MissingId,
}

impl From<String> for AppError {
    fn from(s: String) -> Self {
        AppError::Generic(s)
    }
}

impl From<&str> for AppError {
    fn from(s: &str) -> Self {
        AppError::Generic(s.to_string())
    }
}

impl Into<Response<Body>> for AppError {
    fn into(self) -> Response<Body> {
        let status_code = match self {
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::BadRequest(_) | AppError::MissingId => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };
        Response::builder()
            .status(status_code)
            .body(Body::from(self.to_string()))
            .expect("Failed to build error response")
    }
}