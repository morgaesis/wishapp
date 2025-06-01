use aws_smithy_runtime_api;
use lambda_http::http::StatusCode;
use lambda_http::Body;
use lambda_http::Response;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("DynamoDB error: {0}")]
    DynamoDb(String),
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("HTTP error: {0}")]
    Http(#[from] lambda_http::http::Error),
    #[error("Missing ID in request")]
    MissingId,
    #[error("Generic error: {0}")]
    Generic(String),
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

impl From<aws_sdk_dynamodb::Error> for AppError {
    fn from(err: aws_sdk_dynamodb::Error) -> Self {
        AppError::DynamoDb(err.to_string())
    }
}

impl<E, R> From<aws_smithy_runtime_api::client::result::SdkError<E, R>> for AppError
where
    E: std::error::Error + Send + Sync + 'static,
    R: std::fmt::Debug + 'static, // Add R as a generic parameter
{
    fn from(err: aws_smithy_runtime_api::client::result::SdkError<E, R>) -> Self {
        AppError::DynamoDb(err.to_string())
    }
}

impl From<hyper::Error> for AppError {
    fn from(err: hyper::Error) -> Self {
        AppError::Generic(format!("Hyper error: {}", err))
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::Generic(format!("IO error: {}", err))
    }
}

impl From<AppError> for Response<Body> {
    fn from(val: AppError) -> Self {
        let status_code = match val {
            AppError::DynamoDb(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Serialization(_) => StatusCode::BAD_REQUEST,
            AppError::Http(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::MissingId => StatusCode::BAD_REQUEST,
            AppError::Generic(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        Response::builder()
            .status(status_code)
            .body(Body::from(val.to_string()))
            .expect("Failed to build error response")
    }
}
