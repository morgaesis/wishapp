use aws_config::SdkConfig;
use aws_sdk_dynamodb::Client as DynamoDbClient;
use lambda_http::{Body, Error, Request, Response};
mod handlers;
use handlers::{handle_delete, handle_get, handle_post, handle_put};

#[derive(Debug)]
struct AppError(Box<dyn std::error::Error + Send + Sync + 'static>);

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::error::Error for AppError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.0.source()
    }
}

impl From<lambda_http::Error> for AppError {
    fn from(err: lambda_http::Error) -> Self {
        AppError(err)
    }
}
impl From<hyper::Error> for AppError {
    fn from(err: hyper::Error) -> Self {
        AppError(Box::new(err))
    }
}

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
            .body("Method Not Allowed".into())?),
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let config = aws_config::load_from_env().await;
    let db_client = DynamoDbClient::new(&config);

    #[cfg(not(feature = "aws_lambda"))]
    {
        // Local server code
        use bytes::Bytes;
        use http_body_util::Full;
        use hyper::server::conn::http1;
        use hyper::service::service_fn;
        use hyper::Request as HyperRequest;
        use tokio::net::TcpListener;
        // use lambda_http::RequestExt; // To use .into_lambda_http_request()
        use http_body_util::BodyExt;
        use hyper_util::rt::tokio::TokioIo;
        use std::net::SocketAddr; // For .collect()

        let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
        let listener = TcpListener::bind(addr).await?;

        loop {
            let (stream, _) = listener.accept().await?;
            let db_client_clone = db_client.clone(); // Clone for each spawned task
            tokio::task::spawn(async move {
                if let Err(err) = http1::Builder::new()
                    .serve_connection(
                        TokioIo::new(stream),
                        service_fn(move |req: HyperRequest<hyper::body::Incoming>| {
                            let db_client_inner_clone = db_client_clone.clone(); // Clone for each request
                            async move {
                                let (parts, body) = req.into_parts();
                                let body_bytes = body.collect().await?.to_bytes();
                                let lambda_body = lambda_http::Body::from(body_bytes.to_vec());
                                let lambda_req =
                                    lambda_http::Request::from_parts(parts, lambda_body);
                                match handle_request(lambda_req, &db_client_inner_clone).await {
                                    Ok(resp) => {
                                        let (parts, body) = resp.into_parts();
                                        let hyper_resp_body = Full::new(Bytes::from(body.to_vec()));
                                        Ok(hyper::Response::from_parts(parts, hyper_resp_body))
                                    }
                                    Err(e) => Err(AppError::from(e)),
                                }
                            }
                        }),
                    )
                    .await
                {
                    eprintln!("Error serving connection: {:?}", err);
                }
            });
        }
    }

    #[cfg(feature = "aws_lambda")]
    {
        lambda_http::run(lambda_http::service_fn(|event| {
            handle_request(event, &db_client)
        }))
        .await
    }
}
