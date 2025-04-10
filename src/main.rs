use lambda_http::{run, service_fn, Body, Error, Request, Response};
mod handlers;
pub use handlers::{handle_delete, handle_get, handle_post, handle_put};

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
    run(service_fn(handle_request)).await
}
