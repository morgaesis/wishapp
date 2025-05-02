use lambda_http::{run, service_fn, Body, Error, Request, Response};
mod handlers;
use handlers::{handle_delete, handle_get, handle_post, handle_put};

pub async fn handle_request(event: Request) -> Result<Response<Body>, Error> {
    println!("[DEBUG] Incoming request: path={}, method={}, headers={:?}", 
        event.uri().path(),
        event.method(),
        event.headers());
        
    // Special case for Lambda test invocations
    if event.uri().path().starts_with("/2015-03-31/functions") {
        let method = if let Some(method_override) = event.headers().get("x-http-method-override") {
            method_override.to_str()?
        } else {
            event.method().as_str()
        };
        
        match method {
            "GET" => handle_get(event).await,
            "POST" => handle_post(event).await,
            "PUT" => handle_put(event).await,
            "DELETE" => handle_delete(event).await,
            _ => Ok(Response::builder()
                .status(405)
                .body("Method Not Allowed".into())?),
        }
    } else {
        // Normal HTTP routing
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
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(handle_request)).await
}
