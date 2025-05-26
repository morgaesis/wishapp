use lambda_http::{run, service_fn, Body, Error, Request, Response};
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


pub async fn handle_request(event: Request) -> Result<Response<Body>, Error> {
    match event.method().as_str() {
        "GET" => handle_get(event).await,
        "POST" => handle_post(event).await,
        "PUT" => handle_put(event).await,
        "DELETE" => handle_delete(event).await,
        _ => Ok(Response::builder()
            .status(405)
            .body("Method Not Allowed".into())?),
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    #[cfg(not(feature = "aws_lambda"))]
    {
        // Local server code
        use hyper::server::conn::http1;
        use hyper::service::service_fn;
        use hyper::Request as HyperRequest;
use http_body_util::Full;
use bytes::Bytes;
        use tokio::net::TcpListener;
        use lambda_http::RequestExt; // To use .into_lambda_http_request()
        use std::net::SocketAddr;
        use hyper_util::rt::tokio::TokioIo;
        use http_body_util::BodyExt; // For .collect()

        let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
        let listener = TcpListener::bind(addr).await?;

        loop {
            let (stream, _) = listener.accept().await?;
            tokio::task::spawn(async move {
                if let Err(err) = http1::Builder::new()
                    .serve_connection(
                        TokioIo::new(stream),
                        service_fn(move |req: HyperRequest<hyper::body::Incoming>| {
                            async move {
                                let (parts, body) = req.into_parts();
                                let body_bytes = body.collect().await?.to_bytes();
                                let lambda_body = lambda_http::Body::from(body_bytes.to_vec());
                                let lambda_req = lambda_http::Request::from_parts(parts, lambda_body);
                                match handle_request(lambda_req).await {
                                    Ok(resp) => {
                                        let (parts, body) = resp.into_parts();
                                        let hyper_resp_body = Full::new(Bytes::from(body.to_vec()));
                                        Ok(hyper::Response::from_parts(parts, hyper_resp_body))
                                    },
                                    Err(e) => {
                                        Err(AppError::from(e))
                                    }
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
        run(service_fn(handle_request)).await
    }
}
