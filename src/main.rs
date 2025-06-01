use wishlist_api::error::AppError;
use wishlist_api::handlers::handle_request;

use log::error;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    env_logger::init();
    use wishlist_api::db::get_db_client;
    let db_client = get_db_client().await;

    #[cfg(not(feature = "aws_lambda"))]
    {
        #[allow(unused_imports)]
        use lambda_http::{Body, Request, Response}; // Moved here
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
                                    Err(e) => Err(e),
                                }
                            }
                        }),
                    )
                    .await
                {
                    error!("Error serving connection: {:?}", err);
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
        .map_err(AppError::from)?
    }
}
